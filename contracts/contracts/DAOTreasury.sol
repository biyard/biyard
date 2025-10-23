// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/utils/ReentrancyGuard.sol";

/**
 * @title DAOTreasury
 * @dev Decentralized Autonomous Organization Treasury with proposal-based governance
 * Manages funds through a voting mechanism where token holders can create and vote on proposals
 */
contract DAOTreasury is Ownable, ReentrancyGuard {
    using SafeERC20 for IERC20;

    // Proposal states
    enum ProposalState {
        Pending,
        Active,
        Succeeded,
        Defeated,
        Executed,
        Cancelled
    }

    // Proposal structure
    struct Proposal {
        uint256 id;
        address proposer;
        string description;
        address recipient;
        uint256 amount;
        address tokenAddress; // address(0) for native token (ETH/MATIC etc)
        uint256 votesFor;
        uint256 votesAgainst;
        uint256 startTime;
        uint256 endTime;
        ProposalState state;
        mapping(address => bool) hasVoted;
        mapping(address => uint256) votingPower;
    }

    // Configuration
    IERC20 public governanceToken;
    uint256 public proposalThreshold; // Minimum tokens needed to create proposal
    uint256 public votingPeriod; // Voting period in seconds
    uint256 public quorumPercentage; // Percentage of total supply needed for quorum (e.g., 10 = 10%)
    uint256 public proposalCount;

    // Storage
    mapping(uint256 => Proposal) public proposals;

    // Events
    event ProposalCreated(
        uint256 indexed proposalId,
        address indexed proposer,
        string description,
        address recipient,
        uint256 amount,
        address tokenAddress,
        uint256 startTime,
        uint256 endTime
    );
    event VoteCast(
        uint256 indexed proposalId,
        address indexed voter,
        bool support,
        uint256 votes
    );
    event ProposalExecuted(uint256 indexed proposalId);
    event ProposalCancelled(uint256 indexed proposalId);
    event FundsDeposited(address indexed from, uint256 amount);
    event TokensDeposited(address indexed from, address indexed token, uint256 amount);

    /**
     * @dev Constructor
     * @param _governanceToken Address of the governance token used for voting
     * @param _proposalThreshold Minimum tokens needed to create a proposal
     * @param _votingPeriod Voting period in seconds
     * @param _quorumPercentage Quorum percentage (e.g., 10 for 10%)
     */
    constructor(
        address _governanceToken,
        uint256 _proposalThreshold,
        uint256 _votingPeriod,
        uint256 _quorumPercentage
    ) Ownable(msg.sender) {
        require(_governanceToken != address(0), "Invalid governance token");
        require(_quorumPercentage > 0 && _quorumPercentage <= 100, "Invalid quorum percentage");

        governanceToken = IERC20(_governanceToken);
        proposalThreshold = _proposalThreshold;
        votingPeriod = _votingPeriod;
        quorumPercentage = _quorumPercentage;
    }

    /**
     * @dev Receive native tokens
     */
    receive() external payable {
        emit FundsDeposited(msg.sender, msg.value);
    }

    /**
     * @dev Deposit ERC20 tokens to the treasury
     * @param token Token address
     * @param amount Amount to deposit
     */
    function depositTokens(address token, uint256 amount) external {
        require(token != address(0), "Invalid token address");
        require(amount > 0, "Amount must be greater than 0");

        IERC20(token).safeTransferFrom(msg.sender, address(this), amount);
        emit TokensDeposited(msg.sender, token, amount);
    }

    /**
     * @dev Create a new proposal
     * @param description Proposal description
     * @param recipient Address to receive funds if proposal passes
     * @param amount Amount to transfer
     * @param tokenAddress Token address (address(0) for native token)
     */
    function createProposal(
        string memory description,
        address recipient,
        uint256 amount,
        address tokenAddress
    ) external returns (uint256) {
        require(
            governanceToken.balanceOf(msg.sender) >= proposalThreshold,
            "Insufficient tokens to create proposal"
        );
        require(recipient != address(0), "Invalid recipient");
        require(amount > 0, "Amount must be greater than 0");
        require(bytes(description).length > 0, "Description cannot be empty");

        // Verify treasury has sufficient funds
        if (tokenAddress == address(0)) {
            require(address(this).balance >= amount, "Insufficient native token balance");
        } else {
            require(
                IERC20(tokenAddress).balanceOf(address(this)) >= amount,
                "Insufficient token balance"
            );
        }

        proposalCount++;
        uint256 proposalId = proposalCount;

        Proposal storage newProposal = proposals[proposalId];
        newProposal.id = proposalId;
        newProposal.proposer = msg.sender;
        newProposal.description = description;
        newProposal.recipient = recipient;
        newProposal.amount = amount;
        newProposal.tokenAddress = tokenAddress;
        newProposal.startTime = block.timestamp;
        newProposal.endTime = block.timestamp + votingPeriod;
        newProposal.state = ProposalState.Active;

        emit ProposalCreated(
            proposalId,
            msg.sender,
            description,
            recipient,
            amount,
            tokenAddress,
            newProposal.startTime,
            newProposal.endTime
        );

        return proposalId;
    }

    /**
     * @dev Cast a vote on a proposal
     * @param proposalId Proposal ID
     * @param support True for yes, false for no
     */
    function castVote(uint256 proposalId, bool support) external {
        Proposal storage proposal = proposals[proposalId];

        require(proposal.state == ProposalState.Active, "Proposal is not active");
        require(block.timestamp <= proposal.endTime, "Voting period has ended");
        require(!proposal.hasVoted[msg.sender], "Already voted");

        uint256 votes = governanceToken.balanceOf(msg.sender);
        require(votes > 0, "No voting power");

        proposal.hasVoted[msg.sender] = true;
        proposal.votingPower[msg.sender] = votes;

        if (support) {
            proposal.votesFor += votes;
        } else {
            proposal.votesAgainst += votes;
        }

        emit VoteCast(proposalId, msg.sender, support, votes);
    }

    /**
     * @dev Execute a proposal if it has passed
     * @param proposalId Proposal ID
     */
    function executeProposal(uint256 proposalId) external nonReentrant {
        Proposal storage proposal = proposals[proposalId];

        require(proposal.state == ProposalState.Active, "Proposal is not active");
        require(block.timestamp > proposal.endTime, "Voting period not ended");

        // Check if proposal succeeded
        uint256 totalVotes = proposal.votesFor + proposal.votesAgainst;
        uint256 requiredQuorum = (governanceToken.totalSupply() * quorumPercentage) / 100;

        if (totalVotes >= requiredQuorum && proposal.votesFor > proposal.votesAgainst) {
            proposal.state = ProposalState.Succeeded;

            // Execute the transfer
            if (proposal.tokenAddress == address(0)) {
                // Transfer native token
                (bool success, ) = proposal.recipient.call{value: proposal.amount}("");
                require(success, "Native token transfer failed");
            } else {
                // Transfer ERC20 token
                IERC20(proposal.tokenAddress).safeTransfer(proposal.recipient, proposal.amount);
            }

            proposal.state = ProposalState.Executed;
            emit ProposalExecuted(proposalId);
        } else {
            proposal.state = ProposalState.Defeated;
        }
    }

    /**
     * @dev Cancel a proposal (only by proposer or owner)
     * @param proposalId Proposal ID
     */
    function cancelProposal(uint256 proposalId) external {
        Proposal storage proposal = proposals[proposalId];

        require(
            msg.sender == proposal.proposer || msg.sender == owner(),
            "Only proposer or owner can cancel"
        );
        require(
            proposal.state == ProposalState.Pending || proposal.state == ProposalState.Active,
            "Cannot cancel proposal in current state"
        );

        proposal.state = ProposalState.Cancelled;
        emit ProposalCancelled(proposalId);
    }

    /**
     * @dev Get proposal state
     * @param proposalId Proposal ID
     */
    function getProposalState(uint256 proposalId) external view returns (ProposalState) {
        return proposals[proposalId].state;
    }

    /**
     * @dev Get proposal details
     * @param proposalId Proposal ID
     */
    function getProposal(uint256 proposalId)
        external
        view
        returns (
            uint256 id,
            address proposer,
            string memory description,
            address recipient,
            uint256 amount,
            address tokenAddress,
            uint256 votesFor,
            uint256 votesAgainst,
            uint256 startTime,
            uint256 endTime,
            ProposalState state
        )
    {
        Proposal storage proposal = proposals[proposalId];
        return (
            proposal.id,
            proposal.proposer,
            proposal.description,
            proposal.recipient,
            proposal.amount,
            proposal.tokenAddress,
            proposal.votesFor,
            proposal.votesAgainst,
            proposal.startTime,
            proposal.endTime,
            proposal.state
        );
    }

    /**
     * @dev Check if an address has voted on a proposal
     * @param proposalId Proposal ID
     * @param voter Voter address
     */
    function hasVoted(uint256 proposalId, address voter) external view returns (bool) {
        return proposals[proposalId].hasVoted[voter];
    }

    /**
     * @dev Update governance parameters (only owner)
     */
    function updateProposalThreshold(uint256 _proposalThreshold) external onlyOwner {
        proposalThreshold = _proposalThreshold;
    }

    function updateVotingPeriod(uint256 _votingPeriod) external onlyOwner {
        votingPeriod = _votingPeriod;
    }

    function updateQuorumPercentage(uint256 _quorumPercentage) external onlyOwner {
        require(_quorumPercentage > 0 && _quorumPercentage <= 100, "Invalid quorum percentage");
        quorumPercentage = _quorumPercentage;
    }

    /**
     * @dev Get treasury balance for a specific token
     * @param tokenAddress Token address (address(0) for native token)
     */
    function getTreasuryBalance(address tokenAddress) external view returns (uint256) {
        if (tokenAddress == address(0)) {
            return address(this).balance;
        } else {
            return IERC20(tokenAddress).balanceOf(address(this));
        }
    }
}
