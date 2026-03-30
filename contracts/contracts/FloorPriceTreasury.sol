// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/utils/ReentrancyGuard.sol";

interface IBiyardToken is IERC20 {
    function mint(address to, uint256 amount) external;
    function burn(uint256 amount) external;
    function burnFrom(address account, uint256 amount) external;
}

/**
 * @title FloorPriceTreasury
 * @dev Treasury contract that guarantees a floor price for brand tokens.
 *
 * Core Mechanism:
 * - Purchases deposit USDT into treasury → floor price rises
 * - Floor Price = USDT Reserve / Circulating Token Supply
 * - Token holders can ALWAYS sell at floor price (treasury buyback)
 * - Bought-back tokens are burned → remaining holders benefit
 * - Floor price NEVER decreases (only rises with more purchases)
 *
 * Kaia Chain compatible (EVM).
 */
contract FloorPriceTreasury is Ownable, ReentrancyGuard {
    using SafeERC20 for IERC20;

    // ============================================
    // STATE
    // ============================================

    IBiyardToken public brandToken;
    IERC20 public stableToken; // USDT or USDC

    // Treasury accounting
    uint256 public totalTreasuryDeposited; // Total USDT ever deposited
    uint256 public totalBuybackSpent;      // Total USDT spent on buybacks
    uint256 public totalTokensBurned;      // Total tokens burned via buyback

    // Project configuration
    address public projectOwner;           // Enterprise that owns this project
    uint256 public rewardRateBps;          // Purchase reward rate in basis points (200 = 2%)
    bool public buybackEnabled;

    // ============================================
    // EVENTS
    // ============================================

    event TreasuryDeposit(
        address indexed from,
        uint256 usdtAmount,
        uint256 newFloorPrice,
        uint256 newTreasuryBalance
    );

    event PurchaseReward(
        address indexed customer,
        uint256 purchaseAmount,
        uint256 rewardTokens,
        uint256 treasuryContribution
    );

    event FloorPriceBuyback(
        address indexed seller,
        uint256 tokenAmount,
        uint256 usdtPaid,
        uint256 tokensBurned,
        uint256 newFloorPrice
    );

    event TokensMinted(
        address indexed to,
        uint256 amount,
        string reason
    );

    event RewardRateUpdated(uint256 oldRate, uint256 newRate);
    event ProjectOwnerUpdated(address oldOwner, address newOwner);

    // ============================================
    // MODIFIERS
    // ============================================

    modifier onlyProjectOwner() {
        require(
            msg.sender == projectOwner || msg.sender == owner(),
            "Not project owner"
        );
        _;
    }

    // ============================================
    // CONSTRUCTOR
    // ============================================

    /**
     * @param _brandToken Brand's ERC20 token (must have mint/burn)
     * @param _stableToken USDT/USDC address
     * @param _projectOwner Enterprise wallet address
     * @param _rewardRateBps Purchase reward rate (200 = 2%)
     */
    constructor(
        address _brandToken,
        address _stableToken,
        address _projectOwner,
        uint256 _rewardRateBps
    ) Ownable(msg.sender) {
        require(_brandToken != address(0), "Invalid brand token");
        require(_stableToken != address(0), "Invalid stable token");
        require(_projectOwner != address(0), "Invalid project owner");
        require(_rewardRateBps <= 5000, "Rate too high"); // Max 50%

        brandToken = IBiyardToken(_brandToken);
        stableToken = IERC20(_stableToken);
        projectOwner = _projectOwner;
        rewardRateBps = _rewardRateBps;
        buybackEnabled = true;
    }

    // ============================================
    // CORE: FLOOR PRICE CALCULATION
    // ============================================

    /**
     * @dev Calculate current floor price
     * Floor Price = Treasury USDT Balance / Circulating Token Supply
     * @return floorPrice Price in stable token units (scaled by 1e18)
     */
    function getFloorPrice() public view returns (uint256 floorPrice) {
        uint256 treasuryBalance = stableToken.balanceOf(address(this));
        uint256 circulatingSupply = getCirculatingSupply();

        if (circulatingSupply == 0 || treasuryBalance == 0) {
            return 0;
        }

        // Scale by 1e18 for precision
        floorPrice = (treasuryBalance * 1e18) / circulatingSupply;
    }

    /**
     * @dev Get circulating supply (total - treasury held - burned)
     */
    function getCirculatingSupply() public view returns (uint256) {
        uint256 totalSupply = brandToken.totalSupply();
        uint256 treasuryHeld = brandToken.balanceOf(address(this));
        return totalSupply - treasuryHeld;
    }

    /**
     * @dev Get treasury status
     */
    function getTreasuryInfo()
        external
        view
        returns (
            uint256 usdtBalance,
            uint256 floorPrice,
            uint256 circulatingSupply,
            uint256 totalSupply,
            uint256 totalDeposited,
            uint256 totalBuybacks,
            uint256 totalBurned
        )
    {
        usdtBalance = stableToken.balanceOf(address(this));
        floorPrice = getFloorPrice();
        circulatingSupply = getCirculatingSupply();
        totalSupply = brandToken.totalSupply();
        totalDeposited = totalTreasuryDeposited;
        totalBuybacks = totalBuybackSpent;
        totalBurned = totalTokensBurned;
    }

    // ============================================
    // CORE: PURCHASE → TREASURY DEPOSIT
    // ============================================

    /**
     * @dev Record a purchase and deposit treasury contribution
     * Called by the enterprise's backend when a customer makes a purchase.
     *
     * Flow:
     * 1. Enterprise sends USDT (purchase amount * reward rate) to treasury
     * 2. Treasury mints brand tokens to customer as reward
     * 3. Floor price increases because treasury grew
     *
     * @param customer Customer wallet address
     * @param purchaseAmountUsdt Full purchase amount in USDT (for calculation)
     * @param rewardTokens Number of brand tokens to mint as reward
     */
    function recordPurchase(
        address customer,
        uint256 purchaseAmountUsdt,
        uint256 rewardTokens
    ) external onlyProjectOwner nonReentrant {
        require(customer != address(0), "Invalid customer");
        require(purchaseAmountUsdt > 0, "Invalid amount");
        require(rewardTokens > 0, "Invalid reward");

        // Calculate treasury contribution
        uint256 treasuryContribution = (purchaseAmountUsdt * rewardRateBps) / 10000;
        require(treasuryContribution > 0, "Contribution too small");

        // Transfer USDT from project owner to treasury
        stableToken.safeTransferFrom(msg.sender, address(this), treasuryContribution);
        totalTreasuryDeposited += treasuryContribution;

        // Mint reward tokens to customer
        brandToken.mint(customer, rewardTokens);

        uint256 newFloorPrice = getFloorPrice();

        emit TreasuryDeposit(msg.sender, treasuryContribution, newFloorPrice, stableToken.balanceOf(address(this)));
        emit PurchaseReward(customer, purchaseAmountUsdt, rewardTokens, treasuryContribution);
    }

    /**
     * @dev Direct USDT deposit to treasury (no token minting)
     * Used for manual treasury funding or revenue sharing
     */
    function depositToTreasury(uint256 amount) external nonReentrant {
        require(amount > 0, "Invalid amount");

        stableToken.safeTransferFrom(msg.sender, address(this), amount);
        totalTreasuryDeposited += amount;

        uint256 newFloorPrice = getFloorPrice();

        emit TreasuryDeposit(msg.sender, amount, newFloorPrice, stableToken.balanceOf(address(this)));
    }

    // ============================================
    // CORE: FLOOR PRICE BUYBACK + BURN
    // ============================================

    /**
     * @dev Sell tokens back to treasury at floor price
     *
     * This is the KEY mechanism:
     * 1. User sends brand tokens to this contract
     * 2. Treasury pays USDT at floor price
     * 3. Tokens are BURNED (not held)
     * 4. Floor price remains the same or INCREASES
     *
     * Math proof:
     *   Before: FloorPrice = Treasury / Supply
     *   User sells X tokens → gets X * FloorPrice USDT
     *   After: FloorPrice = (Treasury - X*FP) / (Supply - X)
     *        = (T - X*T/S) / (S - X)
     *        = T*(S - X)/S / (S - X)
     *        = T/S = same floor price ✓
     *
     * @param tokenAmount Amount of brand tokens to sell
     */
    function sellAtFloorPrice(uint256 tokenAmount) external nonReentrant {
        require(buybackEnabled, "Buyback disabled");
        require(tokenAmount > 0, "Invalid amount");
        require(
            brandToken.balanceOf(msg.sender) >= tokenAmount,
            "Insufficient token balance"
        );

        uint256 floorPrice = getFloorPrice();
        require(floorPrice > 0, "Floor price not set");

        // Calculate USDT to pay: tokenAmount * floorPrice / 1e18
        uint256 usdtToPay = (tokenAmount * floorPrice) / 1e18;
        require(usdtToPay > 0, "Amount too small");

        uint256 treasuryBalance = stableToken.balanceOf(address(this));
        require(treasuryBalance >= usdtToPay, "Insufficient treasury");

        // 1. Transfer tokens from seller to treasury
        IERC20(address(brandToken)).safeTransferFrom(msg.sender, address(this), tokenAmount);

        // 2. Burn the tokens (reduces supply)
        brandToken.burn(tokenAmount);

        // 3. Pay USDT to seller
        stableToken.safeTransfer(msg.sender, usdtToPay);

        // Track stats
        totalBuybackSpent += usdtToPay;
        totalTokensBurned += tokenAmount;

        uint256 newFloorPrice = getFloorPrice();

        emit FloorPriceBuyback(
            msg.sender,
            tokenAmount,
            usdtToPay,
            tokenAmount,
            newFloorPrice
        );
    }

    /**
     * @dev Calculate how much USDT would be received for selling tokens
     * @param tokenAmount Amount of tokens to sell
     * @return usdtAmount USDT that would be received
     */
    function calculateSellAmount(uint256 tokenAmount)
        external
        view
        returns (uint256 usdtAmount)
    {
        uint256 floorPrice = getFloorPrice();
        usdtAmount = (tokenAmount * floorPrice) / 1e18;
    }

    // ============================================
    // ADMIN: MINT TOKENS (for challenges, rewards)
    // ============================================

    /**
     * @dev Mint tokens for activity rewards (walking, running, etc.)
     * Note: This dilutes floor price slightly, but continuous purchases
     * bring in more treasury to compensate.
     */
    function mintRewardTokens(
        address to,
        uint256 amount,
        string calldata reason
    ) external onlyProjectOwner {
        require(to != address(0), "Invalid recipient");
        require(amount > 0, "Invalid amount");

        brandToken.mint(to, amount);

        emit TokensMinted(to, amount, reason);
    }

    // ============================================
    // CONFIG
    // ============================================

    function setRewardRate(uint256 newRateBps) external onlyProjectOwner {
        require(newRateBps <= 5000, "Rate too high");
        uint256 oldRate = rewardRateBps;
        rewardRateBps = newRateBps;
        emit RewardRateUpdated(oldRate, newRateBps);
    }

    function setBuybackEnabled(bool enabled) external onlyProjectOwner {
        buybackEnabled = enabled;
    }

    function setProjectOwner(address newOwner) external onlyOwner {
        require(newOwner != address(0), "Invalid owner");
        address oldOwner = projectOwner;
        projectOwner = newOwner;
        emit ProjectOwnerUpdated(oldOwner, newOwner);
    }
}
