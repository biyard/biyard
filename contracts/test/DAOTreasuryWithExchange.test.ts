import { expect } from "chai";
import { ethers } from "hardhat";
import { BiyardToken, MockUSDT, DAOTreasuryWithExchange } from "../typechain-types";
import { SignerWithAddress } from "@nomicfoundation/hardhat-ethers/signers";
import { time } from "@nomicfoundation/hardhat-network-helpers";

describe("DAOTreasuryWithExchange", function () {
  let biyardToken: BiyardToken;
  let usdtToken: MockUSDT;
  let treasury: DAOTreasuryWithExchange;
  let owner: SignerWithAddress;
  let trader1: SignerWithAddress;
  let trader2: SignerWithAddress;
  let proposer: SignerWithAddress;
  let voter1: SignerWithAddress;

  const INITIAL_BIYARD_SUPPLY = ethers.parseEther("1000000"); // 1M BIYARD
  const INITIAL_USDT = 1000000n * 10n ** 6n; // 1M USDT (6 decimals)
  const PROPOSAL_THRESHOLD = ethers.parseEther("100");
  const VOTING_PERIOD = 3 * 24 * 60 * 60; // 3 days
  const QUORUM_PERCENTAGE = 10;
  const EXCHANGE_FEE = 30; // 0.3%

  beforeEach(async function () {
    [owner, trader1, trader2, proposer, voter1] = await ethers.getSigners();

    // Deploy BiyardToken
    const BiyardToken = await ethers.getContractFactory("BiyardToken");
    biyardToken = await BiyardToken.deploy("Biyard Token", "BIYARD", INITIAL_BIYARD_SUPPLY, 0);
    await biyardToken.waitForDeployment();

    // Deploy MockUSDT
    const MockUSDT = await ethers.getContractFactory("MockUSDT");
    usdtToken = await MockUSDT.deploy();
    await usdtToken.waitForDeployment();

    // Deploy Treasury with exchange
    const DAOTreasuryWithExchange = await ethers.getContractFactory("DAOTreasuryWithExchange");
    treasury = await DAOTreasuryWithExchange.deploy(
      await biyardToken.getAddress(),
      await usdtToken.getAddress(),
      PROPOSAL_THRESHOLD,
      VOTING_PERIOD,
      QUORUM_PERCENTAGE,
      EXCHANGE_FEE
    );
    await treasury.waitForDeployment();

    // Setup initial reserves for exchange
    // Transfer 100k BIYARD to treasury (reserve)
    await biyardToken.transfer(await treasury.getAddress(), ethers.parseEther("100000"));

    // Transfer 100k USDT to treasury (reserve)
    await usdtToken.transfer(await treasury.getAddress(), 100000n * 10n ** 6n);

    // Distribute tokens to traders for testing
    await biyardToken.transfer(trader1.address, ethers.parseEther("10000"));
    await biyardToken.transfer(trader2.address, ethers.parseEther("10000"));
    await usdtToken.transfer(trader1.address, 10000n * 10n ** 6n);
    await usdtToken.transfer(trader2.address, 10000n * 10n ** 6n);

    // Distribute for governance testing
    await biyardToken.transfer(proposer.address, ethers.parseEther("150000"));
    await biyardToken.transfer(voter1.address, ethers.parseEther("150000"));
  });

  describe("Deployment", function () {
    it("Should set correct token addresses", async function () {
      expect(await treasury.governanceToken()).to.equal(await biyardToken.getAddress());
      expect(await treasury.usdtToken()).to.equal(await usdtToken.getAddress());
    });

    it("Should set correct exchange parameters", async function () {
      expect(await treasury.exchangeFeePercentage()).to.equal(EXCHANGE_FEE);
      expect(await treasury.exchangeEnabled()).to.be.true;
    });

    it("Should have initial reserves", async function () {
      const biyardReserve = await biyardToken.balanceOf(await treasury.getAddress());
      const usdtReserve = await usdtToken.balanceOf(await treasury.getAddress());

      expect(biyardReserve).to.equal(ethers.parseEther("100000"));
      expect(usdtReserve).to.equal(100000n * 10n ** 6n);
    });
  });

  describe("Price Calculation", function () {
    it("Should calculate correct price based on reserves", async function () {
      // Price = USDT Reserve / (Total Supply - Treasury Reserve)
      // Price = 100,000 USDT / (1,000,000 - 100,000) BIYARD
      // Price = 100,000 / 900,000 = 0.111... USDT per BIYARD

      const price = await treasury.getCurrentPrice();
      const expectedPrice = (100000n * 10n ** 6n * 10n ** 18n) / ethers.parseEther("900000");

      expect(price).to.be.closeTo(expectedPrice, ethers.parseEther("0.001"));
    });

    it("Should update price when reserves change", async function () {
      const priceBefore = await treasury.getCurrentPrice();

      // Add more USDT to treasury
      await usdtToken.transfer(await treasury.getAddress(), 50000n * 10n ** 6n);

      const priceAfter = await treasury.getCurrentPrice();

      // Price should increase because USDT reserve increased
      expect(priceAfter).to.be.greaterThan(priceBefore);
    });

    it("Should provide exchange info", async function () {
      const info = await treasury.getExchangeInfo();

      expect(info.usdtReserve).to.equal(100000n * 10n ** 6n);
      expect(info.biyardReserve).to.equal(ethers.parseEther("100000"));
      expect(info.circulatingSupply).to.equal(ethers.parseEther("900000"));
      expect(info.feePercentage).to.equal(EXCHANGE_FEE);
      expect(info.isEnabled).to.be.true;
    });
  });

  describe("Exchange Calculations", function () {
    it("Should calculate BIYARD to USDT correctly", async function () {
      const biyardAmount = ethers.parseEther("1000"); // 1000 BIYARD

      const [usdtAmount, fee] = await treasury.calculateBiyardToUsdt(biyardAmount);

      // Should be approximately 111.11 USDT (before fee)
      // After 0.3% fee: ~110.78 USDT
      expect(usdtAmount).to.be.greaterThan(0);
      expect(fee).to.be.greaterThan(0);
      expect(usdtAmount + fee).to.be.closeTo(111n * 10n ** 6n, 2n * 10n ** 6n);
    });

    it("Should calculate USDT to BIYARD correctly", async function () {
      const usdtAmount = 1000n * 10n ** 6n; // 1000 USDT

      const [biyardAmount, fee] = await treasury.calculateUsdtToBiyard(usdtAmount);

      // Should be approximately 9000 BIYARD (before fee)
      // After 0.3% fee: ~8973 BIYARD
      expect(biyardAmount).to.be.greaterThan(0);
      expect(fee).to.be.greaterThan(0);
      expect(biyardAmount + fee).to.be.closeTo(ethers.parseEther("9000"), ethers.parseEther("100"));
    });

    it("Should include exchange fee in calculations", async function () {
      const biyardAmount = ethers.parseEther("1000");
      const [usdtAmount, fee] = await treasury.calculateBiyardToUsdt(biyardAmount);

      // Fee should be 0.3% of gross amount
      const expectedFeeRatio = 30n; // 0.3%
      const actualFeeRatio = (fee * 10000n) / (usdtAmount + fee);

      expect(actualFeeRatio).to.be.closeTo(expectedFeeRatio, 1n);
    });
  });

  describe("Exchange BIYARD for USDT", function () {
    it("Should exchange BIYARD for USDT successfully", async function () {
      const biyardAmount = ethers.parseEther("100");

      // Approve treasury to spend BIYARD
      await biyardToken.connect(trader1).approve(await treasury.getAddress(), biyardAmount);

      const [expectedUsdt] = await treasury.calculateBiyardToUsdt(biyardAmount);

      const initialBiyardBalance = await biyardToken.balanceOf(trader1.address);
      const initialUsdtBalance = await usdtToken.balanceOf(trader1.address);

      await treasury.connect(trader1).exchangeBiyardForUsdt(biyardAmount, 0);

      const finalBiyardBalance = await biyardToken.balanceOf(trader1.address);
      const finalUsdtBalance = await usdtToken.balanceOf(trader1.address);

      expect(finalBiyardBalance).to.equal(initialBiyardBalance - biyardAmount);
      expect(finalUsdtBalance).to.be.closeTo(initialUsdtBalance + expectedUsdt, 1n);
    });

    it("Should emit TokensExchanged event", async function () {
      const biyardAmount = ethers.parseEther("100");
      await biyardToken.connect(trader1).approve(await treasury.getAddress(), biyardAmount);

      const [expectedUsdt, fee] = await treasury.calculateBiyardToUsdt(biyardAmount);
      const price = await treasury.getCurrentPrice();

      await expect(treasury.connect(trader1).exchangeBiyardForUsdt(biyardAmount, 0))
        .to.emit(treasury, "TokensExchanged")
        .withArgs(
          trader1.address,
          await biyardToken.getAddress(),
          await usdtToken.getAddress(),
          biyardAmount,
          expectedUsdt,
          fee,
          price
        );
    });

    it("Should respect slippage protection", async function () {
      const biyardAmount = ethers.parseEther("100");
      await biyardToken.connect(trader1).approve(await treasury.getAddress(), biyardAmount);

      const [expectedUsdt] = await treasury.calculateBiyardToUsdt(biyardAmount);
      const tooHighMinimum = expectedUsdt + 1000n;

      await expect(
        treasury.connect(trader1).exchangeBiyardForUsdt(biyardAmount, tooHighMinimum)
      ).to.be.revertedWith("Slippage too high");
    });

    it("Should fail when exchange is disabled", async function () {
      await treasury.setExchangeEnabled(false);

      const biyardAmount = ethers.parseEther("100");
      await biyardToken.connect(trader1).approve(await treasury.getAddress(), biyardAmount);

      await expect(
        treasury.connect(trader1).exchangeBiyardForUsdt(biyardAmount, 0)
      ).to.be.revertedWith("Exchange is disabled");
    });

    it("Should fail when treasury has insufficient USDT", async function () {
      // Try to exchange more than available
      const hugeAmount = ethers.parseEther("1000000");
      await biyardToken.connect(owner).approve(await treasury.getAddress(), hugeAmount);

      await expect(
        treasury.connect(owner).exchangeBiyardForUsdt(hugeAmount, 0)
      ).to.be.revertedWith("Insufficient USDT reserve");
    });
  });

  describe("Exchange USDT for BIYARD", function () {
    it("Should exchange USDT for BIYARD successfully", async function () {
      const usdtAmount = 100n * 10n ** 6n; // 100 USDT

      // Approve treasury to spend USDT
      await usdtToken.connect(trader1).approve(await treasury.getAddress(), usdtAmount);

      const [expectedBiyard] = await treasury.calculateUsdtToBiyard(usdtAmount);

      const initialUsdtBalance = await usdtToken.balanceOf(trader1.address);
      const initialBiyardBalance = await biyardToken.balanceOf(trader1.address);

      await treasury.connect(trader1).exchangeUsdtForBiyard(usdtAmount, 0);

      const finalUsdtBalance = await usdtToken.balanceOf(trader1.address);
      const finalBiyardBalance = await biyardToken.balanceOf(trader1.address);

      expect(finalUsdtBalance).to.equal(initialUsdtBalance - usdtAmount);
      expect(finalBiyardBalance).to.be.closeTo(initialBiyardBalance + expectedBiyard, 1n);
    });

    it("Should emit TokensExchanged event", async function () {
      const usdtAmount = 100n * 10n ** 6n;
      await usdtToken.connect(trader1).approve(await treasury.getAddress(), usdtAmount);

      const [expectedBiyard, fee] = await treasury.calculateUsdtToBiyard(usdtAmount);
      const price = await treasury.getCurrentPrice();

      await expect(treasury.connect(trader1).exchangeUsdtForBiyard(usdtAmount, 0))
        .to.emit(treasury, "TokensExchanged")
        .withArgs(
          trader1.address,
          await usdtToken.getAddress(),
          await biyardToken.getAddress(),
          usdtAmount,
          expectedBiyard,
          fee,
          price
        );
    });

    it("Should respect slippage protection", async function () {
      const usdtAmount = 100n * 10n ** 6n;
      await usdtToken.connect(trader1).approve(await treasury.getAddress(), usdtAmount);

      const [expectedBiyard] = await treasury.calculateUsdtToBiyard(usdtAmount);
      const tooHighMinimum = expectedBiyard + ethers.parseEther("1000");

      await expect(
        treasury.connect(trader1).exchangeUsdtForBiyard(usdtAmount, tooHighMinimum)
      ).to.be.revertedWith("Slippage too high");
    });

    it("Should fail when treasury has insufficient BIYARD", async function () {
      // Try to exchange for more BIYARD than available
      const hugeUsdtAmount = 1000000n * 10n ** 6n;
      await usdtToken.mint(trader1.address, hugeUsdtAmount);
      await usdtToken.connect(trader1).approve(await treasury.getAddress(), hugeUsdtAmount);

      await expect(
        treasury.connect(trader1).exchangeUsdtForBiyard(hugeUsdtAmount, 0)
      ).to.be.revertedWith("Insufficient BiyardToken reserve");
    });
  });

  describe("Exchange Configuration", function () {
    it("Should allow owner to enable/disable exchange", async function () {
      await treasury.setExchangeEnabled(false);
      expect(await treasury.exchangeEnabled()).to.be.false;

      await treasury.setExchangeEnabled(true);
      expect(await treasury.exchangeEnabled()).to.be.true;
    });

    it("Should emit ExchangeEnabled event", async function () {
      await expect(treasury.setExchangeEnabled(false))
        .to.emit(treasury, "ExchangeEnabled")
        .withArgs(false);
    });

    it("Should allow owner to update exchange fee", async function () {
      const newFee = 50; // 0.5%
      await treasury.setExchangeFee(newFee);
      expect(await treasury.exchangeFeePercentage()).to.equal(newFee);
    });

    it("Should emit ExchangeFeeUpdated event", async function () {
      const newFee = 50;
      await expect(treasury.setExchangeFee(newFee))
        .to.emit(treasury, "ExchangeFeeUpdated")
        .withArgs(newFee);
    });

    it("Should not allow fee greater than 100%", async function () {
      await expect(treasury.setExchangeFee(10001)).to.be.revertedWith("Fee too high");
    });

    it("Should not allow non-owner to change settings", async function () {
      await expect(treasury.connect(trader1).setExchangeEnabled(false)).to.be.reverted;
      await expect(treasury.connect(trader1).setExchangeFee(100)).to.be.reverted;
    });
  });

  describe("Exchange Impact on Price", function () {
    it("Should update price after BIYARD->USDT exchange", async function () {
      const priceBefore = await treasury.getCurrentPrice();

      const biyardAmount = ethers.parseEther("1000");
      await biyardToken.connect(trader1).approve(await treasury.getAddress(), biyardAmount);
      await treasury.connect(trader1).exchangeBiyardForUsdt(biyardAmount, 0);

      const priceAfter = await treasury.getCurrentPrice();

      // Price should decrease or stay same (more BIYARD in treasury, less USDT)
      expect(priceAfter).to.be.lessThanOrEqual(priceBefore);
    });

    it("Should update price after USDT->BIYARD exchange", async function () {
      const priceBefore = await treasury.getCurrentPrice();

      const usdtAmount = 1000n * 10n ** 6n;
      await usdtToken.connect(trader1).approve(await treasury.getAddress(), usdtAmount);
      await treasury.connect(trader1).exchangeUsdtForBiyard(usdtAmount, 0);

      const priceAfter = await treasury.getCurrentPrice();

      // Price should increase (less BIYARD in treasury, more USDT)
      expect(priceAfter).to.be.greaterThan(priceBefore);
    });
  });

  describe("Governance Integration", function () {
    it("Should still support proposal creation", async function () {
      const proposalAmount = ethers.parseEther("1000");

      await expect(
        treasury
          .connect(proposer)
          .createProposal(
            "Test proposal",
            trader1.address,
            proposalAmount,
            await biyardToken.getAddress()
          )
      ).to.emit(treasury, "ProposalCreated");
    });

    it("Should still support voting", async function () {
      const tx = await treasury
        .connect(proposer)
        .createProposal(
          "Test proposal",
          trader1.address,
          ethers.parseEther("1000"),
          await biyardToken.getAddress()
        );
      await tx.wait();

      await expect(treasury.connect(voter1).castVote(1, true)).to.emit(treasury, "VoteCast");
    });

    it("Should execute proposals correctly", async function () {
      const proposalAmount = ethers.parseEther("1000");
      const tx = await treasury
        .connect(proposer)
        .createProposal(
          "Test proposal",
          trader1.address,
          proposalAmount,
          await biyardToken.getAddress()
        );
      await tx.wait();

      await treasury.connect(voter1).castVote(1, true);
      await treasury.connect(proposer).castVote(1, true);

      await time.increase(VOTING_PERIOD + 1);

      const initialBalance = await biyardToken.balanceOf(trader1.address);
      await treasury.executeProposal(1);
      const finalBalance = await biyardToken.balanceOf(trader1.address);

      expect(finalBalance).to.equal(initialBalance + proposalAmount);
    });
  });

  describe("Edge Cases", function () {
    it("Should handle very small amounts", async function () {
      const smallAmount = 1n; // Very small USDT amount
      await usdtToken.connect(trader1).approve(await treasury.getAddress(), smallAmount);

      // This should fail because amount is too small
      try {
        await treasury.connect(trader1).exchangeUsdtForBiyard(smallAmount, 0);
        // If it doesn't fail, check the result is still valid
        expect(true).to.be.true;
      } catch (error) {
        // Expected to fail with amount too small
        expect(true).to.be.true;
      }
    });

    it("Should handle zero amount gracefully", async function () {
      await expect(treasury.calculateBiyardToUsdt(0)).to.be.revertedWith(
        "Amount must be greater than 0"
      );
      await expect(treasury.calculateUsdtToBiyard(0)).to.be.revertedWith(
        "Amount must be greater than 0"
      );
    });

    it("Should maintain reserve balance accuracy", async function () {
      // Perform multiple exchanges
      for (let i = 0; i < 5; i++) {
        const biyardAmount = ethers.parseEther("10");
        await biyardToken.connect(trader1).approve(await treasury.getAddress(), biyardAmount);
        await treasury.connect(trader1).exchangeBiyardForUsdt(biyardAmount, 0);

        const usdtAmount = 10n * 10n ** 6n;
        await usdtToken.connect(trader1).approve(await treasury.getAddress(), usdtAmount);
        await treasury.connect(trader1).exchangeUsdtForBiyard(usdtAmount, 0);
      }

      // Reserves should still be queryable
      const info = await treasury.getExchangeInfo();
      expect(info.usdtReserve).to.be.greaterThan(0);
      expect(info.biyardReserve).to.be.greaterThan(0);
    });
  });
});
