import { expect } from "chai";
import { ethers } from "hardhat";
import { time } from "@nomicfoundation/hardhat-network-helpers";

describe("BrandToken", function () {
  const MONTH = 30 * 24 * 60 * 60;
  const MAX_SUPPLY = ethers.parseEther("10000000");
  const MONTHLY_EMISSION = ethers.parseEther("1000000");
  const DECAY_BPS = 500;

  async function deployFixture() {
    const [owner, claimSigner, marketing, partner, user1, user2] = await ethers.getSigners();
    const BrandToken = await ethers.getContractFactory("BrandToken");
    const token = await BrandToken.deploy(
      "CafeToken", "CAFE", MAX_SUPPLY, MONTHLY_EMISSION, DECAY_BPS,
      claimSigner.address, owner.address
    );
    return { token, owner, claimSigner, marketing, partner, user1, user2 };
  }

  async function deployWithSlotsFixture() {
    const fixture = await deployFixture();
    const { token, marketing, partner } = fixture;
    await token.setDistributionSlots(
      [marketing.address, partner.address],
      [1000, 500]
    );
    return fixture;
  }

  async function signClaim(
    token: any, signer: any,
    to: string, amount: bigint, nonce: bigint, deadline: bigint
  ) {
    const domain = {
      name: await token.name(),
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
    const value = { to, amount, nonce, deadline };
    return await signer.signTypedData(domain, types, value);
  }

  describe("Deployment", function () {
    it("should set correct params", async function () {
      const { token, owner, claimSigner } = await deployFixture();
      expect(await token.name()).to.equal("CafeToken");
      expect(await token.symbol()).to.equal("CAFE");
      expect(await token.maxSupply()).to.equal(MAX_SUPPLY);
      expect(await token.monthlyEmission()).to.equal(MONTHLY_EMISSION);
      expect(await token.decayRateBps()).to.equal(DECAY_BPS);
      expect(await token.claimSigner()).to.equal(claimSigner.address);
      expect(await token.owner()).to.equal(owner.address);
      expect(await token.totalSupply()).to.equal(0);
    });
  });

  describe("Monthly Minting", function () {
    it("should mint month 0 with distribution slots", async function () {
      const { token, marketing, partner } = await deployWithSlotsFixture();
      await token.triggerMonthlyMint();
      expect(await token.balanceOf(marketing.address)).to.equal(ethers.parseEther("100000"));
      expect(await token.balanceOf(partner.address)).to.equal(ethers.parseEther("50000"));
      expect(await token.claimPool()).to.equal(ethers.parseEther("850000"));
      expect(await token.totalSupply()).to.equal(MONTHLY_EMISSION);
    });

    it("should revert double mint same month", async function () {
      const { token } = await deployWithSlotsFixture();
      await token.triggerMonthlyMint();
      await expect(token.triggerMonthlyMint())
        .to.be.revertedWith("BrandToken: already minted this month");
    });

    it("should apply decay for month 1", async function () {
      const { token } = await deployWithSlotsFixture();
      await token.triggerMonthlyMint();
      await time.increase(MONTH);
      await token.triggerMonthlyMint();
      const month1 = await token.monthlyMinted(1);
      expect(month1).to.equal(ethers.parseEther("950000"));
    });

    it("should revert if not owner", async function () {
      const { token, user1 } = await deployWithSlotsFixture();
      await expect(token.connect(user1).triggerMonthlyMint())
        .to.be.revertedWithCustomError(token, "OwnableUnauthorizedAccount");
    });

    it("should mint all to claim pool when no slots", async function () {
      const { token } = await deployFixture();
      await token.triggerMonthlyMint();
      expect(await token.claimPool()).to.equal(MONTHLY_EMISSION);
    });

    it("should cap at maxSupply", async function () {
      const [owner, signer] = await ethers.getSigners();
      const BrandToken = await ethers.getContractFactory("BrandToken");
      const token = await BrandToken.deploy(
        "Test", "TST", ethers.parseEther("500000"), ethers.parseEther("1000000"),
        0, signer.address, owner.address
      );
      await token.triggerMonthlyMint();
      expect(await token.totalSupply()).to.equal(ethers.parseEther("500000"));
    });
  });

  describe("Claim", function () {
    it("should claim with valid signature", async function () {
      const { token, claimSigner, user1 } = await deployFixture();
      await token.triggerMonthlyMint();
      const amount = ethers.parseEther("10000");
      const nonce = 1n;
      const deadline = BigInt(await time.latest()) + 3600n;
      const sig = await signClaim(token, claimSigner, user1.address, amount, nonce, deadline);
      await token.connect(user1).claim(amount, nonce, deadline, sig);
      expect(await token.balanceOf(user1.address)).to.equal(amount);
      expect(await token.claimedNonces(nonce)).to.be.true;
    });

    it("should revert with expired deadline", async function () {
      const { token, claimSigner, user1 } = await deployFixture();
      await token.triggerMonthlyMint();
      const amount = ethers.parseEther("100");
      const deadline = BigInt(await time.latest()) - 1n;
      const sig = await signClaim(token, claimSigner, user1.address, amount, 1n, deadline);
      await expect(token.connect(user1).claim(amount, 1n, deadline, sig))
        .to.be.revertedWith("BrandToken: expired");
    });

    it("should revert with reused nonce", async function () {
      const { token, claimSigner, user1 } = await deployFixture();
      await token.triggerMonthlyMint();
      const amount = ethers.parseEther("100");
      const nonce = 42n;
      const deadline = BigInt(await time.latest()) + 3600n;
      const sig = await signClaim(token, claimSigner, user1.address, amount, nonce, deadline);
      await token.connect(user1).claim(amount, nonce, deadline, sig);
      await expect(token.connect(user1).claim(amount, nonce, deadline, sig))
        .to.be.revertedWith("BrandToken: nonce used");
    });

    it("should revert with wrong signer", async function () {
      const { token, user1, user2 } = await deployFixture();
      await token.triggerMonthlyMint();
      const amount = ethers.parseEther("100");
      const deadline = BigInt(await time.latest()) + 3600n;
      const sig = await signClaim(token, user2, user1.address, amount, 1n, deadline);
      await expect(token.connect(user1).claim(amount, 1n, deadline, sig))
        .to.be.revertedWith("BrandToken: invalid signature");
    });

    it("should revert if claim pool insufficient", async function () {
      const { token, claimSigner, user1 } = await deployFixture();
      const amount = ethers.parseEther("100");
      const deadline = BigInt(await time.latest()) + 3600n;
      const sig = await signClaim(token, claimSigner, user1.address, amount, 1n, deadline);
      await expect(token.connect(user1).claim(amount, 1n, deadline, sig))
        .to.be.revertedWith("BrandToken: insufficient claim pool");
    });
  });

  describe("Owner Setters", function () {
    it("should update monthlyEmission", async function () {
      const { token } = await deployFixture();
      await token.setMonthlyEmission(ethers.parseEther("500000"));
      expect(await token.monthlyEmission()).to.equal(ethers.parseEther("500000"));
    });

    it("should update decayRateBps", async function () {
      const { token } = await deployFixture();
      await token.setDecayRateBps(1000);
      expect(await token.decayRateBps()).to.equal(1000);
    });

    it("should update distribution slots", async function () {
      const { token, marketing, partner } = await deployFixture();
      await token.setDistributionSlots(
        [marketing.address, partner.address],
        [2000, 1000]
      );
      expect(await token.distributionSlotCount()).to.equal(2);
      const slot0 = await token.distributionSlots(0);
      expect(slot0.wallet).to.equal(marketing.address);
      expect(slot0.bps).to.equal(2000);
    });

    it("should revert slots totaling >= 100%", async function () {
      const { token, marketing, partner } = await deployFixture();
      await expect(token.setDistributionSlots(
        [marketing.address, partner.address],
        [5000, 5000]
      )).to.be.revertedWith("BrandToken: slots >= 100%");
    });
  });
});
