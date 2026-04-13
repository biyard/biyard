// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import "@openzeppelin/contracts/utils/cryptography/EIP712.sol";
import "@openzeppelin/contracts/utils/introspection/ERC165.sol";

/// @title BrandToken — ERC-20 with monthly emission ceiling and signature-based claim
/// @notice Users claim tokens via server signature. Each claim mints on demand within
///         the target month's ceiling. Brand allocation wallets receive their share
///         proportionally with each claim. Claims are for past months only (month must
///         be < currentMonth). Server signs (month, maxClaimable) so the contract enforces
///         per-user per-month limits even if the signing key is compromised.
contract BrandToken is ERC20, Ownable, EIP712, ERC165 {
    using ECDSA for bytes32;

    event Claimed(address indexed user, uint256 indexed month, uint256 amount, uint256 nonce);
    event ClaimSignerUpdated(address indexed newSigner);
    event MonthlyEmissionUpdated(uint256 newEmission);
    event DecayRateUpdated(uint16 newDecayRateBps);
    event DistributionSlotsUpdated(uint256 slotCount);
    event TreasuryUpdated(address indexed newTreasury);

    struct DistributionSlot {
        address wallet;
        uint16 bps; // basis points, e.g. 1000 = 10%
    }

    uint256 public monthlyEmission;
    uint16 public decayRateBps; // e.g. 500 = 5%
    uint256 public immutable startTimestamp;

    address public claimSigner;
    address public treasury;
    DistributionSlot[] public distributionSlots;

    /// Total amount already minted in a given month (claims + brand allocation).
    mapping(uint256 => uint256) public monthlyMinted;
    mapping(uint256 => bool) public claimedNonces;

    bytes32 private constant CLAIM_TYPEHASH =
        keccak256("Claim(address to,uint256 month,uint256 amount,uint256 maxClaimable,uint256 nonce,uint256 deadline)");

    constructor(
        string memory name_,
        string memory symbol_,
        uint256 _monthlyEmission,
        uint16 _decayRateBps,
        address _claimSigner,
        address _owner,
        uint256 _startTimestamp
    ) ERC20(name_, symbol_) Ownable(_owner) EIP712(name_, "1") {
        require(_monthlyEmission > 0, "BrandToken: zero emission");
        require(_decayRateBps < 10000, "BrandToken: decay >= 100%");
        require(_claimSigner != address(0), "BrandToken: zero claimSigner");
        require(_startTimestamp <= block.timestamp, "BrandToken: start in future");

        monthlyEmission = _monthlyEmission;
        decayRateBps = _decayRateBps;
        claimSigner = _claimSigner;
        startTimestamp = _startTimestamp;
    }

    function currentMonth() public view returns (uint256) {
        return (block.timestamp - startTimestamp) / 30 days;
    }

    /// The emission ceiling for a given month, factoring in decay.
    function monthlyCeiling(uint256 month) public view returns (uint256) {
        uint256 ceiling = monthlyEmission;
        for (uint256 i = 0; i < month; i++) {
            ceiling = ceiling * (10000 - decayRateBps) / 10000;
        }
        return ceiling;
    }

    /// How much of a given month's ceiling has NOT yet been minted.
    /// Treasury uses this for the current month in floor price calculation.
    function monthRemaining(uint256 month) public view returns (uint256) {
        uint256 ceiling = monthlyCeiling(month);
        uint256 minted = monthlyMinted[month];
        if (minted >= ceiling) return 0;
        return ceiling - minted;
    }

    /// Convenience: remaining for current month.
    function currentMonthRemaining() external view returns (uint256) {
        return monthRemaining(currentMonth());
    }

    /// Cumulative emission ceiling from month 0 through currentMonth (inclusive).
    /// This represents the total tokens that *should* exist regardless of claim timing.
    function cumulativeEmission() external view returns (uint256) {
        uint256 cm = currentMonth();
        uint256 total = 0;
        for (uint256 i = 0; i <= cm; i++) {
            total += monthlyCeiling(i);
        }
        return total;
    }

    /// @notice Claim tokens for a specific past month via server signature.
    /// @param month       Which month's points to claim (must be < currentMonth).
    /// @param amount      How many tokens to claim this transaction.
    /// @param maxClaimable Server-calculated max this user can ever claim for this month
    ///                     = (userMonthPoints / totalMonthPoints) * monthUserPool.
    ///                     The contract enforces cumulative claims <= maxClaimable.
    function claim(
        uint256 month,
        uint256 amount,
        uint256 maxClaimable,
        uint256 nonce,
        uint256 deadline,
        bytes calldata signature
    ) external {
        require(block.timestamp <= deadline, "BrandToken: expired");
        require(!claimedNonces[nonce], "BrandToken: nonce used");
        require(amount > 0, "BrandToken: zero amount");
        require(month < currentMonth(), "BrandToken: month not ended");

        _verifyClaim(msg.sender, month, amount, maxClaimable, nonce, deadline, signature);

        // Server signature already encodes per-user per-month maxClaimable.
        // The nonce prevents replay. No on-chain per-user tracking needed.
        require(amount <= maxClaimable, "BrandToken: exceeds claimable");

        // Calculate total mint including brand allocation
        uint256 totalMint = _calcTotalMint(amount);

        // Monthly ceiling check
        require(monthlyMinted[month] + totalMint <= monthlyCeiling(month), "BrandToken: exceeds monthly ceiling");

        // Update state
        claimedNonces[nonce] = true;
        monthlyMinted[month] += totalMint;

        // Mint brand allocation
        _mintSlots(totalMint);

        // Mint user's claim
        _mint(msg.sender, amount);

        emit Claimed(msg.sender, month, amount, nonce);
    }

    function _verifyClaim(
        address to,
        uint256 month,
        uint256 amount,
        uint256 maxClaimable,
        uint256 nonce,
        uint256 deadline,
        bytes calldata signature
    ) internal view {
        bytes32 structHash = keccak256(abi.encode(
            CLAIM_TYPEHASH, to, month, amount, maxClaimable, nonce, deadline
        ));
        address recovered = ECDSA.recover(_hashTypedDataV4(structHash), signature);
        require(recovered == claimSigner, "BrandToken: invalid signature");
    }

    function _calcTotalMint(uint256 userAmount) internal view returns (uint256) {
        uint256 slotBps = _totalSlotBps();
        if (slotBps >= 10000) return userAmount;
        return userAmount * 10000 / (10000 - slotBps);
    }

    function _mintSlots(uint256 totalMint) internal {
        for (uint256 i = 0; i < distributionSlots.length; i++) {
            uint256 slotAmount = totalMint * distributionSlots[i].bps / 10000;
            if (slotAmount > 0) {
                _mint(distributionSlots[i].wallet, slotAmount);
            }
        }
    }

    function _totalSlotBps() internal view returns (uint256) {
        uint256 total = 0;
        for (uint256 i = 0; i < distributionSlots.length; i++) {
            total += distributionSlots[i].bps;
        }
        return total;
    }

    // --- KIP-7 / KIP-13 support (Kaia token standard) ---

    /// @dev KIP-7 interface ID = 0x65787371, KIP-7 Metadata = 0xa219a025
    function supportsInterface(bytes4 interfaceId) public view virtual override returns (bool) {
        return
            interfaceId == 0x65787371 || // KIP-7
            interfaceId == 0xa219a025 || // KIP-7 Metadata (name, symbol, decimals)
            super.supportsInterface(interfaceId);
    }

    // --- Owner-only configuration (via Multisig) ---

    function setMonthlyEmission(uint256 _monthlyEmission) external onlyOwner {
        require(_monthlyEmission > 0, "BrandToken: zero emission");
        monthlyEmission = _monthlyEmission;
        emit MonthlyEmissionUpdated(_monthlyEmission);
    }

    function setDecayRateBps(uint16 _decayRateBps) external onlyOwner {
        require(_decayRateBps < 10000, "BrandToken: decay >= 100%");
        decayRateBps = _decayRateBps;
        emit DecayRateUpdated(_decayRateBps);
    }

    function setDistributionSlots(
        address[] calldata wallets,
        uint16[] calldata bps
    ) external onlyOwner {
        require(wallets.length == bps.length, "BrandToken: length mismatch");
        uint256 totalBps = 0;
        for (uint256 i = 0; i < bps.length; i++) {
            totalBps += bps[i];
        }
        require(totalBps < 10000, "BrandToken: slots >= 100%");

        delete distributionSlots;
        for (uint256 i = 0; i < wallets.length; i++) {
            require(wallets[i] != address(0), "BrandToken: zero wallet");
            distributionSlots.push(DistributionSlot({
                wallet: wallets[i],
                bps: bps[i]
            }));
        }
        emit DistributionSlotsUpdated(wallets.length);
    }

    function setClaimSigner(address _claimSigner) external onlyOwner {
        require(_claimSigner != address(0), "BrandToken: zero claimSigner");
        claimSigner = _claimSigner;
        emit ClaimSignerUpdated(_claimSigner);
    }

    function setTreasury(address _treasury) external onlyOwner {
        require(_treasury != address(0), "BrandToken: zero treasury");
        treasury = _treasury;
        emit TreasuryUpdated(_treasury);
    }

    function distributionSlotCount() external view returns (uint256) {
        return distributionSlots.length;
    }
}
