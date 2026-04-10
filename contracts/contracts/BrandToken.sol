// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import "@openzeppelin/contracts/utils/cryptography/EIP712.sol";

/// @title BrandToken — ERC-20 with scheduled monthly emission and signature-based claim
/// @notice Owner (Multisig) triggers monthly mints. Users claim from pre-minted pool via server signature.
contract BrandToken is ERC20, Ownable, EIP712 {
    using ECDSA for bytes32;

    event MonthlyMint(uint256 indexed month, uint256 amount);
    event Claimed(address indexed user, uint256 amount, uint256 nonce);
    event ClaimSignerUpdated(address indexed newSigner);
    event MonthlyEmissionUpdated(uint256 newEmission);
    event DecayRateUpdated(uint16 newDecayRateBps);
    event DistributionSlotsUpdated(uint256 slotCount);
    event TreasuryUpdated(address indexed newTreasury);

    struct DistributionSlot {
        address wallet;
        uint16 bps; // basis points, e.g. 1000 = 10%
    }

    uint256 public immutable maxSupply;
    uint256 public monthlyEmission;
    uint16 public decayRateBps; // e.g. 500 = 5%
    uint256 public immutable deployTimestamp;

    address public claimSigner;
    address public treasury;
    DistributionSlot[] public distributionSlots;

    mapping(uint256 => uint256) public monthlyMinted;
    mapping(uint256 => bool) public claimedNonces;

    bytes32 private constant CLAIM_TYPEHASH =
        keccak256("Claim(address to,uint256 amount,uint256 nonce,uint256 deadline)");

    constructor(
        string memory name_,
        string memory symbol_,
        uint256 _maxSupply,
        uint256 _monthlyEmission,
        uint16 _decayRateBps,
        address _claimSigner,
        address _owner
    ) ERC20(name_, symbol_) Ownable(_owner) EIP712(name_, "1") {
        require(_maxSupply > 0, "BrandToken: zero maxSupply");
        require(_monthlyEmission > 0, "BrandToken: zero emission");
        require(_decayRateBps < 10000, "BrandToken: decay >= 100%");
        require(_claimSigner != address(0), "BrandToken: zero claimSigner");

        maxSupply = _maxSupply;
        monthlyEmission = _monthlyEmission;
        decayRateBps = _decayRateBps;
        claimSigner = _claimSigner;
        deployTimestamp = block.timestamp;
    }

    function currentMonth() public view returns (uint256) {
        return (block.timestamp - deployTimestamp) / 30 days;
    }

    function monthlyCeiling(uint256 month) public view returns (uint256) {
        uint256 ceiling = monthlyEmission;
        for (uint256 i = 0; i < month; i++) {
            ceiling = ceiling * (10000 - decayRateBps) / 10000;
        }
        return ceiling;
    }

    function triggerMonthlyMint() external onlyOwner {
        uint256 month = currentMonth();
        require(monthlyMinted[month] == 0, "BrandToken: already minted this month");

        uint256 ceiling = monthlyCeiling(month);
        uint256 remaining = maxSupply - totalSupply();
        if (ceiling > remaining) {
            ceiling = remaining;
        }
        require(ceiling > 0, "BrandToken: nothing to mint");

        uint256 distributed = 0;
        for (uint256 i = 0; i < distributionSlots.length; i++) {
            uint256 slotAmount = ceiling * distributionSlots[i].bps / 10000;
            if (slotAmount > 0) {
                _mint(distributionSlots[i].wallet, slotAmount);
                distributed += slotAmount;
            }
        }

        uint256 claimPoolAmount = ceiling - distributed;
        if (claimPoolAmount > 0) {
            _mint(address(this), claimPoolAmount);
        }

        monthlyMinted[month] = ceiling;
        emit MonthlyMint(month, ceiling);
    }

    function claim(
        uint256 amount,
        uint256 nonce,
        uint256 deadline,
        bytes calldata signature
    ) external {
        require(block.timestamp <= deadline, "BrandToken: expired");
        require(!claimedNonces[nonce], "BrandToken: nonce used");
        require(balanceOf(address(this)) >= amount, "BrandToken: insufficient claim pool");

        bytes32 structHash = keccak256(abi.encode(
            CLAIM_TYPEHASH,
            msg.sender,
            amount,
            nonce,
            deadline
        ));
        bytes32 digest = _hashTypedDataV4(structHash);
        address recovered = ECDSA.recover(digest, signature);
        require(recovered == claimSigner, "BrandToken: invalid signature");

        claimedNonces[nonce] = true;
        _transfer(address(this), msg.sender, amount);
        emit Claimed(msg.sender, amount, nonce);
    }

    function claimPool() external view returns (uint256) {
        return balanceOf(address(this));
    }

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
