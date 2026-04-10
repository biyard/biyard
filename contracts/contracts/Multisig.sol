// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

/// @title Multisig — N-of-M proposal-based governance
/// @notice Signers propose, approve, and execute arbitrary calls.
///         Initial deployment: 1-of-1 for simplicity, expandable later.
contract Multisig {
    event SignerAdded(address indexed signer);
    event SignerRemoved(address indexed signer);
    event ThresholdUpdated(uint256 newThreshold);
    event Proposed(uint256 indexed proposalId, address indexed proposer, address target);
    event Approved(uint256 indexed proposalId, address indexed approver);
    event Executed(uint256 indexed proposalId);

    struct Proposal {
        address target;
        bytes data;
        uint256 value;
        uint256 approvalCount;
        bool executed;
    }

    mapping(address => bool) public isSigner;
    address[] public signers;
    uint256 public threshold;

    Proposal[] public proposals;
    // proposalId => signer => approved
    mapping(uint256 => mapping(address => bool)) public hasApproved;

    modifier onlySigner() {
        require(isSigner[msg.sender], "Multisig: not a signer");
        _;
    }

    modifier onlySelf() {
        require(msg.sender == address(this), "Multisig: only via proposal");
        _;
    }

    constructor(address[] memory _signers, uint256 _threshold) {
        require(_signers.length > 0, "Multisig: no signers");
        require(_threshold > 0 && _threshold <= _signers.length, "Multisig: bad threshold");

        for (uint256 i = 0; i < _signers.length; i++) {
            address s = _signers[i];
            require(s != address(0), "Multisig: zero address");
            require(!isSigner[s], "Multisig: duplicate signer");
            isSigner[s] = true;
            signers.push(s);
        }
        threshold = _threshold;
    }

    function propose(
        address target,
        bytes calldata data,
        uint256 value
    ) external onlySigner returns (uint256 proposalId) {
        proposalId = proposals.length;
        proposals.push(Proposal({
            target: target,
            data: data,
            value: value,
            approvalCount: 0,
            executed: false
        }));
        emit Proposed(proposalId, msg.sender, target);
    }

    function approve(uint256 proposalId) external onlySigner {
        require(proposalId < proposals.length, "Multisig: invalid id");
        Proposal storage p = proposals[proposalId];
        require(!p.executed, "Multisig: already executed");
        require(!hasApproved[proposalId][msg.sender], "Multisig: already approved");

        hasApproved[proposalId][msg.sender] = true;
        p.approvalCount++;
        emit Approved(proposalId, msg.sender);
    }

    function execute(uint256 proposalId) external {
        require(proposalId < proposals.length, "Multisig: invalid id");
        Proposal storage p = proposals[proposalId];
        require(!p.executed, "Multisig: already executed");
        require(p.approvalCount >= threshold, "Multisig: not enough approvals");

        p.executed = true;
        (bool ok, ) = p.target.call{value: p.value}(p.data);
        require(ok, "Multisig: call failed");
        emit Executed(proposalId);
    }

    function addSigner(address signer) external onlySelf {
        require(signer != address(0), "Multisig: zero address");
        require(!isSigner[signer], "Multisig: already signer");
        isSigner[signer] = true;
        signers.push(signer);
        emit SignerAdded(signer);
    }

    function removeSigner(address signer) external onlySelf {
        require(isSigner[signer], "Multisig: not a signer");
        require(signers.length - 1 >= threshold, "Multisig: would break threshold");
        isSigner[signer] = false;
        // Swap-and-pop
        for (uint256 i = 0; i < signers.length; i++) {
            if (signers[i] == signer) {
                signers[i] = signers[signers.length - 1];
                signers.pop();
                break;
            }
        }
        emit SignerRemoved(signer);
    }

    function setThreshold(uint256 _threshold) external onlySelf {
        require(_threshold > 0 && _threshold <= signers.length, "Multisig: bad threshold");
        threshold = _threshold;
        emit ThresholdUpdated(_threshold);
    }

    function proposalCount() external view returns (uint256) {
        return proposals.length;
    }

    function signerCount() external view returns (uint256) {
        return signers.length;
    }

    receive() external payable {}
}
