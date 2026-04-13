// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import "@openzeppelin/contracts/utils/ReentrancyGuard.sol";

interface IBrandToken is IERC20 {
    function cumulativeEmission() external view returns (uint256);
}

/// @title Treasury — USDT vault with floor-price buyback
/// @notice Price = stableToken balance / effective circulating supply.
///         Effective circulating supply includes tokens already minted to users
///         PLUS the current month's remaining emission (not yet claimed but expected).
///         This ensures the floor price is consistent regardless of claim timing.
contract Treasury is ReentrancyGuard {
    using SafeERC20 for IERC20;

    event Deposited(address indexed from, uint256 amount);
    event Buyback(address indexed user, uint256 tokenAmount, uint256 stableOut);
    event StableWithdrawn(address indexed to, uint256 amount);
    event TokenWithdrawn(address indexed token, address indexed to, uint256 amount);

    IERC20 public immutable stableToken;
    IBrandToken public immutable brandToken;
    address public immutable multisig;

    modifier onlyMultisig() {
        require(msg.sender == multisig, "Treasury: not multisig");
        _;
    }

    constructor(address _stableToken, address _brandToken, address _multisig) {
        require(_stableToken != address(0), "Treasury: zero stableToken");
        require(_brandToken != address(0), "Treasury: zero brandToken");
        require(_multisig != address(0), "Treasury: zero multisig");
        stableToken = IERC20(_stableToken);
        brandToken = IBrandToken(_brandToken);
        multisig = _multisig;
    }

    /// Effective circulating supply for floor price calculation.
    /// = cumulativeEmission - treasuryHeld
    ///
    /// - cumulativeEmission: sum of monthlyCeiling(0..currentMonth), the total
    ///   tokens that *should* exist regardless of whether users have claimed.
    ///   This makes floor price independent of claim timing.
    /// - treasuryHeld: tokens bought back into treasury (out of circulation)
    function circulatingSupply() public view returns (uint256) {
        uint256 cumulative = brandToken.cumulativeEmission();
        uint256 inTreasury = brandToken.balanceOf(address(this));

        if (inTreasury >= cumulative) return 0;
        return cumulative - inTreasury;
    }

    function getPrice() public view returns (uint256) {
        uint256 circ = circulatingSupply();
        if (circ == 0) return 0;
        uint256 stableBal = stableToken.balanceOf(address(this));
        return stableBal * 1e18 / circ;
    }

    function buyback(uint256 tokenAmount) external nonReentrant {
        require(tokenAmount > 0, "Treasury: zero amount");
        uint256 price = getPrice();
        require(price > 0, "Treasury: price is zero");

        uint256 stableOut = tokenAmount * price / 1e18;
        require(stableOut > 0, "Treasury: output is zero");
        require(stableToken.balanceOf(address(this)) >= stableOut, "Treasury: insufficient stable");

        IERC20(address(brandToken)).safeTransferFrom(msg.sender, address(this), tokenAmount);
        stableToken.safeTransfer(msg.sender, stableOut);
        emit Buyback(msg.sender, tokenAmount, stableOut);
    }

    function withdrawStable(address to, uint256 amount) external onlyMultisig {
        require(to != address(0), "Treasury: zero address");
        stableToken.safeTransfer(to, amount);
        emit StableWithdrawn(to, amount);
    }

    function withdrawToken(address token, address to, uint256 amount) external onlyMultisig {
        require(to != address(0), "Treasury: zero address");
        IERC20(token).safeTransfer(to, amount);
        emit TokenWithdrawn(token, to, amount);
    }
}
