import { expect } from "chai";
import { ethers } from "hardhat";
import { time } from "@nomicfoundation/hardhat-network-helpers";

describe("Treasury", function () {
  const MONTH = 30 * 24 * 60 * 60;
  const MAX_SUPPLY = ethers.parseEther("10000000");
  const MONTHLY_EMISSION = ethers.parseEther("1000000");

  async function deployFixture() {
    const [owner, claimSigner, user1, user2] = await ethers.getSigners();

    const BUSDT = await ethers.getContractFactory("BUSDT");
    const busdt = await BUSDT.deploy();

    const BrandToken = await ethers.getContractFactory("BrandToken");
    const token = await BrandToken.deploy(
      "CafeToken", "CAFE", MAX_SUPPLY, MONTHLY_EMISSION, 500,
      claimSigner.address, owner.address
    );

    const Multisig = await ethers.getContractFactory("Multisig");
    const ms = await Multisig.deploy([owner.address], 1);

    const Treasury = await ethers.getContractFactory("Treasury");
    const treasury = await Treasury.deploy(
      await busdt.getAddress(),
      await token.getAddress(),
      await ms.getAddress()
    );

    await token.setTreasury(await treasury.getAddress());

    return { busdt, token, treasury, ms, owner, claimSigner, user1, user2 };
  }

  async function deployWithLiquidityFixture() {
    const fixture = await deployFixture();
    const { busdt, token, treasury, owner, user1 } = fixture;

    // Mint month 0 (no slots -> all to claim pool)
    await token.triggerMonthlyMint();

    // Deposit 100K BUSDT into treasury
    const depositAmount = ethers.parseUnits("100000", 6);
    await busdt.mint(owner.address, depositAmount);
    await busdt.approve(await treasury.getAddress(), depositAmount);
    await treasury.deposit(depositAmount);

    // Give user1 some tokens via claim
    const amount = ethers.parseEther("100000");
    const nonce = 1n;
    const deadline = BigInt(await time.latest()) + 3600n;

    const domain = {
      name: "CafeToken",
      version: "1",
      chainId: (await ethers.provider.getNetwork()).chainId,
      verifyingContract: await token.getAddress(),
    };
    const types = {
      Claim: [
        { name: "to", type: "address" },
        { name: "amount", type: "uint256" },
        { name: "nonce", type: "uint256" },
        { name: "deadline", type: "uint256" },
      ],
    };
    const sig = await fixture.claimSigner.signTypedData(domain, types, {
      to: user1.address,
      amount,
      nonce,
      deadline,
    });
    await token.connect(user1).claim(amount, nonce, deadline, sig);

    return fixture;
  }

  describe("Deployment", function () {
    it("should set correct addresses", async function () {
      const { treasury, busdt, token, ms } = await deployFixture();
      expect(await treasury.stableToken()).to.equal(await busdt.getAddress());
      expect(await treasury.brandToken()).to.equal(await token.getAddress());
      expect(await treasury.multisig()).to.equal(await ms.getAddress());
    });
  });

  describe("Deposit", function () {
    it("should accept stable deposits", async function () {
      const { treasury, busdt, owner } = await deployFixture();
      const amount = ethers.parseUnits("1000", 6);
      await busdt.mint(owner.address, amount);
      await busdt.approve(await treasury.getAddress(), amount);
      await expect(treasury.deposit(amount))
        .to.emit(treasury, "Deposited")
        .withArgs(owner.address, amount);
      expect(await busdt.balanceOf(await treasury.getAddress())).to.equal(amount);
    });

    it("should revert zero deposit", async function () {
      const { treasury } = await deployFixture();
      await expect(treasury.deposit(0)).to.be.revertedWith("Treasury: zero amount");
    });
  });

  describe("Price", function () {
    it("should return 0 when no circulating supply", async function () {
      const { treasury } = await deployFixture();
      expect(await treasury.getPrice()).to.equal(0);
    });

    it("should calculate price correctly", async function () {
      const { treasury } = await deployWithLiquidityFixture();
      const price = await treasury.getPrice();
      // 100K BUSDT (6 dec) in treasury, 100K CAFE circulating
      // price = 100000e6 * 1e18 / 100000e18 = 1e6
      expect(price).to.equal(1000000n);
    });
  });

  describe("Circulating Supply", function () {
    it("should exclude claim pool and treasury holdings", async function () {
      const { treasury, token } = await deployWithLiquidityFixture();
      expect(await treasury.circulatingSupply()).to.equal(ethers.parseEther("100000"));
    });
  });

  describe("Buyback", function () {
    it("should exchange tokens for stable at floor price", async function () {
      const { treasury, token, busdt, user1 } = await deployWithLiquidityFixture();
      const sellAmount = ethers.parseEther("10000");
      await token.connect(user1).approve(await treasury.getAddress(), sellAmount);
      await treasury.connect(user1).buyback(sellAmount);

      const stableReceived = await busdt.balanceOf(user1.address);
      expect(stableReceived).to.be.greaterThan(0);
      expect(await token.balanceOf(await treasury.getAddress())).to.equal(sellAmount);
    });

    it("should maintain price after buyback", async function () {
      const { treasury, token, user1 } = await deployWithLiquidityFixture();
      const priceBefore = await treasury.getPrice();

      const sellAmount = ethers.parseEther("10000");
      await token.connect(user1).approve(await treasury.getAddress(), sellAmount);
      await treasury.connect(user1).buyback(sellAmount);

      const priceAfter = await treasury.getPrice();
      expect(priceAfter).to.be.closeTo(priceBefore, 1);
    });

    it("should revert when price is zero", async function () {
      const { treasury, user1 } = await deployFixture();
      await expect(treasury.connect(user1).buyback(ethers.parseEther("100")))
        .to.be.revertedWith("Treasury: price is zero");
    });
  });

  describe("Multisig Withdrawals", function () {
    it("should allow multisig to withdraw stable", async function () {
      const { treasury, busdt, ms, owner, user1 } = await deployWithLiquidityFixture();
      const amount = ethers.parseUnits("1000", 6);
      const data = treasury.interface.encodeFunctionData("withdrawStable", [user1.address, amount]);

      await ms.propose(await treasury.getAddress(), data, 0);
      await ms.approve(0);
      await ms.execute(0);

      expect(await busdt.balanceOf(user1.address)).to.equal(amount);
    });

    it("should reject non-multisig withdrawal", async function () {
      const { treasury, user1 } = await deployWithLiquidityFixture();
      await expect(treasury.connect(user1).withdrawStable(user1.address, 100))
        .to.be.revertedWith("Treasury: not multisig");
    });

    it("should allow multisig to withdraw any token", async function () {
      const { treasury, token, ms, owner, user1 } = await deployWithLiquidityFixture();
      const sellAmount = ethers.parseEther("10000");
      await token.connect(user1).approve(await treasury.getAddress(), sellAmount);
      await treasury.connect(user1).buyback(sellAmount);

      const data = treasury.interface.encodeFunctionData(
        "withdrawToken",
        [await token.getAddress(), owner.address, sellAmount]
      );
      await ms.propose(await treasury.getAddress(), data, 0);
      await ms.approve(0);
      await ms.execute(0);

      expect(await token.balanceOf(owner.address)).to.equal(sellAmount);
    });
  });
});
