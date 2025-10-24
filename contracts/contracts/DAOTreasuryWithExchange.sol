// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/utils/ReentrancyGuard.sol";

/**
 * @title DAOTreasuryWithExchange
 * @dev Enhanced DAO Treasury with token exchange mechanism
 * Supports governance proposals and automated token exchange between USDT and BiyardToken
 * Price is dynamically calculated based on treasury reserves using the formula:
 * Price = USDT Reserve / (Total BiyardToken Supply - BiyardToken Reserve)
 */
contract DAOTreasuryWithExchange is Ownable, ReentrancyGuard {
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
        address tokenAddress;
        uint256 votesFor;
        uint256 votesAgainst;
        uint256 startTime;
        uint256 endTime;
        ProposalState state;
        mapping(address => bool) hasVoted;
        mapping(address => uint256) votingPower;
    }

    // Configuration
    IERC20 public governanceToken; // BiyardToken
    IERC20 public usdtToken; // USDT token
    uint256 public proposalThreshold;
    uint256 public votingPeriod;
    uint256 public quorumPercentage;
    uint256 public proposalCount;

    // Exchange configuration
    uint256 public exchangeFeePercentage; // Fee percentage (e.g., 30 = 0.3%)
    bool public exchangeEnabled;

    // Storage
    mapping(uint256 => Proposal) public proposals;

    // Events - Governance
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
    event VoteCast(uint256 indexed proposalId, address indexed voter, bool support, uint256 votes);
    event ProposalExecuted(uint256 indexed proposalId);
    event ProposalCancelled(uint256 indexed proposalId);
    event FundsDeposited(address indexed from, uint256 amount);
    event TokensDeposited(address indexed from, address indexed token, uint256 amount);

    // Events - Exchange
    event TokensExchanged(
        address indexed user,
        address indexed fromToken,
        address indexed toToken,
        uint256 amountIn,
        uint256 amountOut,
        uint256 fee,
        uint256 price
    );
    event ExchangeEnabled(bool enabled);
    event ExchangeFeeUpdated(uint256 newFee);

    /**
     * @dev Constructor
     */
    constructor(
        address _governanceToken,
        address _usdtToken,
        uint256 _proposalThreshold,
        uint256 _votingPeriod,
        uint256 _quorumPercentage,
        uint256 _exchangeFeePercentage
    ) Ownable(msg.sender) {
        require(_governanceToken != address(0), "Invalid governance token");
        require(_usdtToken != address(0), "Invalid USDT token");
        require(_governanceToken != _usdtToken, "Tokens must be different");
        require(_quorumPercentage > 0 && _quorumPercentage <= 100, "Invalid quorum percentage");
        require(_exchangeFeePercentage <= 10000, "Fee too high"); // Max 100%

        governanceToken = IERC20(_governanceToken);
        usdtToken = IERC20(_usdtToken);
        proposalThreshold = _proposalThreshold;
        votingPeriod = _votingPeriod;
        quorumPercentage = _quorumPercentage;
        exchangeFeePercentage = _exchangeFeePercentage;
        exchangeEnabled = true;
    }

    /**
     * @dev Receive native tokens
     */
    receive() external payable {
        emit FundsDeposited(msg.sender, msg.value);
    }

    /**
     * @dev Deposit ERC20 tokens to the treasury
     */
    function depositTokens(address token, uint256 amount) external {
        require(token != address(0), "Invalid token address");
        require(amount > 0, "Amount must be greater than 0");

        IERC20(token).safeTransferFrom(msg.sender, address(this), amount);
        emit TokensDeposited(msg.sender, token, amount);
    }

    // ============================================
    // EXCHANGE FUNCTIONS
    // ============================================

    /**
     * @dev Calculate the current price of BiyardToken in USDT
     * Price = USDT Reserve / Circulating BiyardToken Supply
     * Circulating Supply = Total Supply - Treasury Reserve
     * @return price Price in USDT with 6 decimals (USDT standard)
     */
    function getCurrentPrice() public view returns (uint256 price) {
        uint256 usdtReserve = usdtToken.balanceOf(address(this));
        uint256 biyardReserve = governanceToken.balanceOf(address(this));
        uint256 biyardTotalSupply = governanceToken.totalSupply();

        require(biyardTotalSupply > biyardReserve, "Invalid token supply");

        uint256 circulatingSupply = biyardTotalSupply - biyardReserve;
        require(circulatingSupply > 0, "No circulating supply");
        require(usdtReserve > 0, "Insufficient USDT reserve");

        // Price = (USDT Reserve * 1e18) / Circulating Supply
        // Then normalize to USDT decimals (6)
        // Assuming BiyardToken has 18 decimals
        price = (usdtReserve * 1e18) / circulatingSupply;
    }

    /**
     * @dev Get exchange rate details
     */
    function getExchangeInfo()
        external
        view
        returns (
            uint256 currentPrice,
            uint256 usdtReserve,
            uint256 biyardReserve,
            uint256 circulatingSupply,
            uint256 feePercentage,
            bool isEnabled
        )
    {
        usdtReserve = usdtToken.balanceOf(address(this));
        biyardReserve = governanceToken.balanceOf(address(this));
        uint256 totalSupply = governanceToken.totalSupply();
        circulatingSupply = totalSupply > biyardReserve ? totalSupply - biyardReserve : 0;

        currentPrice = circulatingSupply > 0 && usdtReserve > 0 ? getCurrentPrice() : 0;
        feePercentage = exchangeFeePercentage;
        isEnabled = exchangeEnabled;
    }

    /**
     * @dev Calculate how much USDT will be received for given BiyardToken amount
     * @param biyardAmount Amount of BiyardToken to exchange
     * @return usdtAmount Amount of USDT to receive (after fee)
     * @return fee Fee amount in USDT
     */
    function calculateBiyardToUsdt(uint256 biyardAmount)
        public
        view
        returns (uint256 usdtAmount, uint256 fee)
    {
        require(biyardAmount > 0, "Amount must be greater than 0");

        uint256 price = getCurrentPrice();

        // Calculate USDT amount: (biyardAmount * price) / 1e18
        uint256 grossAmount = (biyardAmount * price) / 1e18;

        // Calculate fee
        fee = (grossAmount * exchangeFeePercentage) / 10000;
        usdtAmount = grossAmount - fee;

        require(usdtAmount > 0, "Amount too small");
    }

    /**
     * @dev Calculate how much BiyardToken will be received for given USDT amount
     * @param usdtAmount Amount of USDT to exchange
     * @return biyardAmount Amount of BiyardToken to receive (after fee)
     * @return fee Fee amount in BiyardToken
     */
    function calculateUsdtToBiyard(uint256 usdtAmount)
        public
        view
        returns (uint256 biyardAmount, uint256 fee)
    {
        require(usdtAmount > 0, "Amount must be greater than 0");

        uint256 price = getCurrentPrice();

        // Calculate BiyardToken amount: (usdtAmount * 1e18) / price
        uint256 grossAmount = (usdtAmount * 1e18) / price;

        // Calculate fee
        fee = (grossAmount * exchangeFeePercentage) / 10000;
        biyardAmount = grossAmount - fee;

        require(biyardAmount > 0, "Amount too small");
    }

    /**
     * @dev Exchange BiyardToken for USDT
     * @param biyardAmount Amount of BiyardToken to exchange
     * @param minUsdtOut Minimum USDT to receive (slippage protection)
     */
    function exchangeBiyardForUsdt(uint256 biyardAmount, uint256 minUsdtOut)
        external
        nonReentrant
    {
        require(exchangeEnabled, "Exchange is disabled");
        require(biyardAmount > 0, "Amount must be greater than 0");

        (uint256 usdtAmount, uint256 fee) = calculateBiyardToUsdt(biyardAmount);
        require(usdtAmount >= minUsdtOut, "Slippage too high");
        require(usdtToken.balanceOf(address(this)) >= usdtAmount, "Insufficient USDT reserve");

        uint256 price = getCurrentPrice();

        // Transfer BiyardToken from user to treasury
        governanceToken.safeTransferFrom(msg.sender, address(this), biyardAmount);

        // Transfer USDT from treasury to user
        usdtToken.safeTransfer(msg.sender, usdtAmount);

        emit TokensExchanged(
            msg.sender,
            address(governanceToken),
            address(usdtToken),
            biyardAmount,
            usdtAmount,
            fee,
            price
        );
    }

    /**
     * @dev Exchange USDT for BiyardToken
     * @param usdtAmount Amount of USDT to exchange
     * @param minBiyardOut Minimum BiyardToken to receive (slippage protection)
     */
    function exchangeUsdtForBiyard(uint256 usdtAmount, uint256 minBiyardOut)
        external
        nonReentrant
    {
        require(exchangeEnabled, "Exchange is disabled");
        require(usdtAmount > 0, "Amount must be greater than 0");

        (uint256 biyardAmount, uint256 fee) = calculateUsdtToBiyard(usdtAmount);
        require(biyardAmount >= minBiyardOut, "Slippage too high");
        require(
            governanceToken.balanceOf(address(this)) >= biyardAmount,
            "Insufficient BiyardToken reserve"
        );

        uint256 price = getCurrentPrice();

        // Transfer USDT from user to treasury
        usdtToken.safeTransferFrom(msg.sender, address(this), usdtAmount);

        // Transfer BiyardToken from treasury to user
        governanceToken.safeTransfer(msg.sender, biyardAmount);

        emit TokensExchanged(
            msg.sender,
            address(usdtToken),
            address(governanceToken),
            usdtAmount,
            biyardAmount,
            fee,
            price
        );
    }

    /**
     * @dev Enable or disable exchange functionality
     */
    function setExchangeEnabled(bool enabled) external onlyOwner {
        exchangeEnabled = enabled;
        emit ExchangeEnabled(enabled);
    }

    /**
     * @dev Update exchange fee percentage
     * @param newFee New fee percentage (e.g., 30 = 0.3%)
     */
    function setExchangeFee(uint256 newFee) external onlyOwner {
        require(newFee <= 10000, "Fee too high"); // Max 100%
        exchangeFeePercentage = newFee;
        emit ExchangeFeeUpdated(newFee);
    }

    // ============================================
    // GOVERNANCE FUNCTIONS (from original DAOTreasury)
    // ============================================

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

    function executeProposal(uint256 proposalId) external nonReentrant {
        Proposal storage proposal = proposals[proposalId];

        require(proposal.state == ProposalState.Active, "Proposal is not active");
        require(block.timestamp > proposal.endTime, "Voting period not ended");

        uint256 totalVotes = proposal.votesFor + proposal.votesAgainst;
        uint256 requiredQuorum = (governanceToken.totalSupply() * quorumPercentage) / 100;

        if (totalVotes >= requiredQuorum && proposal.votesFor > proposal.votesAgainst) {
            proposal.state = ProposalState.Succeeded;

            if (proposal.tokenAddress == address(0)) {
                (bool success, ) = proposal.recipient.call{value: proposal.amount}("");
                require(success, "Native token transfer failed");
            } else {
                IERC20(proposal.tokenAddress).safeTransfer(proposal.recipient, proposal.amount);
            }

            proposal.state = ProposalState.Executed;
            emit ProposalExecuted(proposalId);
        } else {
            proposal.state = ProposalState.Defeated;
        }
    }

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

    function getProposalState(uint256 proposalId) external view returns (ProposalState) {
        return proposals[proposalId].state;
    }

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

    function hasVoted(uint256 proposalId, address voter) external view returns (bool) {
        return proposals[proposalId].hasVoted[voter];
    }

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

    function getTreasuryBalance(address tokenAddress) external view returns (uint256) {
        if (tokenAddress == address(0)) {
            return address(this).balance;
        } else {
            return IERC20(tokenAddress).balanceOf(address(this));
        }
    }
}
