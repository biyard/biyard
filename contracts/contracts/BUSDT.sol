// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";

/// @title BUSDT — Biyard USD Tether (test-only faucet)
/// @notice Anyone can mint unlimited tokens. 6 decimals like real USDT.
contract BUSDT is ERC20 {
    constructor() ERC20("Biyard USD Tether", "BUSDT") {}

    function decimals() public pure override returns (uint8) {
        return 6;
    }

    /// @notice Faucet — anyone can mint any amount to any address.
    function mint(address to, uint256 amount) external {
        _mint(to, amount);
    }
}
