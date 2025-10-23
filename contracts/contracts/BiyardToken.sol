// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/token/ERC20/extensions/ERC20Burnable.sol";
import "@openzeppelin/contracts/token/ERC20/extensions/ERC20Pausable.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

/**
 * @title BiyardToken
 * @dev ERC20 token for the Biyard platform with minting, burning, and pausable features
 */
contract BiyardToken is ERC20, ERC20Burnable, ERC20Pausable, Ownable {
    // Maximum supply cap (optional - can be set to 0 for unlimited)
    uint256 public immutable maxSupply;

    // Minter role mapping
    mapping(address => bool) public minters;

    // Events
    event MinterAdded(address indexed account);
    event MinterRemoved(address indexed account);

    // Modifiers
    modifier onlyMinter() {
        require(minters[msg.sender] || msg.sender == owner(), "Caller is not a minter");
        _;
    }

    /**
     * @dev Constructor
     * @param name Token name
     * @param symbol Token symbol
     * @param initialSupply Initial supply to mint to deployer
     * @param _maxSupply Maximum supply (0 for unlimited)
     */
    constructor(
        string memory name,
        string memory symbol,
        uint256 initialSupply,
        uint256 _maxSupply
    ) ERC20(name, symbol) Ownable(msg.sender) {
        maxSupply = _maxSupply;

        if (initialSupply > 0) {
            require(_maxSupply == 0 || initialSupply <= _maxSupply, "Initial supply exceeds max supply");
            _mint(msg.sender, initialSupply);
        }

        // Add deployer as initial minter
        minters[msg.sender] = true;
        emit MinterAdded(msg.sender);
    }

    /**
     * @dev Mint new tokens
     * @param to Recipient address
     * @param amount Amount to mint
     */
    function mint(address to, uint256 amount) public onlyMinter {
        if (maxSupply > 0) {
            require(totalSupply() + amount <= maxSupply, "Minting would exceed max supply");
        }
        _mint(to, amount);
    }

    /**
     * @dev Add a new minter
     * @param account Address to grant minter role
     */
    function addMinter(address account) public onlyOwner {
        require(account != address(0), "Cannot add zero address as minter");
        require(!minters[account], "Account is already a minter");
        minters[account] = true;
        emit MinterAdded(account);
    }

    /**
     * @dev Remove a minter
     * @param account Address to revoke minter role
     */
    function removeMinter(address account) public onlyOwner {
        require(minters[account], "Account is not a minter");
        minters[account] = false;
        emit MinterRemoved(account);
    }

    /**
     * @dev Pause token transfers
     */
    function pause() public onlyOwner {
        _pause();
    }

    /**
     * @dev Unpause token transfers
     */
    function unpause() public onlyOwner {
        _unpause();
    }

    /**
     * @dev Required override for _update function
     */
    function _update(address from, address to, uint256 value)
        internal
        override(ERC20, ERC20Pausable)
    {
        super._update(from, to, value);
    }
}
