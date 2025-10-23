import { expect } from "chai";
import { ethers } from "hardhat";
import { BiyardToken } from "../typechain-types";
import { SignerWithAddress } from "@nomicfoundation/hardhat-ethers/signers";

describe("BiyardToken", function () {
  let token: BiyardToken;
  let owner: SignerWithAddress;
  let minter: SignerWithAddress;
  let user1: SignerWithAddress;
  let user2: SignerWithAddress;

  const TOKEN_NAME = "Biyard Token";
  const TOKEN_SYMBOL = "BIYARD";
  const INITIAL_SUPPLY = ethers.parseEther("1000000");
  const MAX_SUPPLY = ethers.parseEther("10000000");

  beforeEach(async function () {
    [owner, minter, user1, user2] = await ethers.getSigners();

    const BiyardToken = await ethers.getContractFactory("BiyardToken");
    token = await BiyardToken.deploy(TOKEN_NAME, TOKEN_SYMBOL, INITIAL_SUPPLY, MAX_SUPPLY);
    await token.waitForDeployment();
  });

  describe("Deployment", function () {
    it("Should set the correct name and symbol", async function () {
      expect(await token.name()).to.equal(TOKEN_NAME);
      expect(await token.symbol()).to.equal(TOKEN_SYMBOL);
    });

    it("Should mint initial supply to owner", async function () {
      expect(await token.balanceOf(owner.address)).to.equal(INITIAL_SUPPLY);
    });

    it("Should set the correct max supply", async function () {
      expect(await token.maxSupply()).to.equal(MAX_SUPPLY);
    });

    it("Should add deployer as initial minter", async function () {
      expect(await token.minters(owner.address)).to.be.true;
    });
  });

  describe("Minting", function () {
    it("Should allow owner to mint tokens", async function () {
      const mintAmount = ethers.parseEther("1000");
      await token.mint(user1.address, mintAmount);
      expect(await token.balanceOf(user1.address)).to.equal(mintAmount);
    });

    it("Should allow authorized minter to mint tokens", async function () {
      await token.addMinter(minter.address);
      const mintAmount = ethers.parseEther("1000");
      await token.connect(minter).mint(user1.address, mintAmount);
      expect(await token.balanceOf(user1.address)).to.equal(mintAmount);
    });

    it("Should not allow non-minters to mint", async function () {
      const mintAmount = ethers.parseEther("1000");
      await expect(token.connect(user1).mint(user2.address, mintAmount)).to.be.revertedWith(
        "Caller is not a minter"
      );
    });

    it("Should not allow minting beyond max supply", async function () {
      const exceedAmount = MAX_SUPPLY;
      await expect(token.mint(user1.address, exceedAmount)).to.be.revertedWith(
        "Minting would exceed max supply"
      );
    });
  });

  describe("Minter Management", function () {
    it("Should allow owner to add minter", async function () {
      await expect(token.addMinter(minter.address))
        .to.emit(token, "MinterAdded")
        .withArgs(minter.address);
      expect(await token.minters(minter.address)).to.be.true;
    });

    it("Should allow owner to remove minter", async function () {
      await token.addMinter(minter.address);
      await expect(token.removeMinter(minter.address))
        .to.emit(token, "MinterRemoved")
        .withArgs(minter.address);
      expect(await token.minters(minter.address)).to.be.false;
    });

    it("Should not allow non-owner to add minter", async function () {
      await expect(token.connect(user1).addMinter(minter.address)).to.be.reverted;
    });

    it("Should not allow adding zero address as minter", async function () {
      await expect(token.addMinter(ethers.ZeroAddress)).to.be.revertedWith(
        "Cannot add zero address as minter"
      );
    });
  });

  describe("Burning", function () {
    beforeEach(async function () {
      await token.transfer(user1.address, ethers.parseEther("1000"));
    });

    it("Should allow users to burn their tokens", async function () {
      const burnAmount = ethers.parseEther("100");
      const initialBalance = await token.balanceOf(user1.address);

      await token.connect(user1).burn(burnAmount);
      expect(await token.balanceOf(user1.address)).to.equal(initialBalance - burnAmount);
    });

    it("Should reduce total supply when burning", async function () {
      const burnAmount = ethers.parseEther("100");
      const initialSupply = await token.totalSupply();

      await token.connect(user1).burn(burnAmount);
      expect(await token.totalSupply()).to.equal(initialSupply - burnAmount);
    });
  });

  describe("Pausable", function () {
    it("Should allow owner to pause", async function () {
      await token.pause();
      await expect(token.transfer(user1.address, ethers.parseEther("100"))).to.be.reverted;
    });

    it("Should allow owner to unpause", async function () {
      await token.pause();
      await token.unpause();
      await expect(token.transfer(user1.address, ethers.parseEther("100"))).to.not.be.reverted;
    });

    it("Should not allow non-owner to pause", async function () {
      await expect(token.connect(user1).pause()).to.be.reverted;
    });

    it("Should prevent minting when paused", async function () {
      await token.pause();
      await expect(token.mint(user1.address, ethers.parseEther("100"))).to.be.reverted;
    });
  });

  describe("Transfers", function () {
    beforeEach(async function () {
      await token.transfer(user1.address, ethers.parseEther("1000"));
    });

    it("Should transfer tokens between accounts", async function () {
      const transferAmount = ethers.parseEther("100");
      await token.connect(user1).transfer(user2.address, transferAmount);
      expect(await token.balanceOf(user2.address)).to.equal(transferAmount);
    });

    it("Should fail if sender doesn't have enough tokens", async function () {
      const initialBalance = await token.balanceOf(user1.address);
      await expect(
        token.connect(user1).transfer(user2.address, initialBalance + BigInt(1))
      ).to.be.reverted;
    });
  });
});
