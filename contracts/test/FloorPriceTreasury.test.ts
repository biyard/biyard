import { expect } from "chai";
import { ethers } from "hardhat";
import { loadFixture } from "@nomicfoundation/hardhat-toolbox/network-helpers";

describe("FloorPriceTreasury", function () {
  async function deployFixture() {
    const [owner, enterprise, customer1, customer2, whale] =
      await ethers.getSigners();

    // Deploy MockUSDT (6 decimals)
    const MockUSDT = await ethers.getContractFactory("MockUSDT");
    const usdt = await MockUSDT.deploy();

    // Deploy BiyardToken (18 decimals, no max supply)
    const BiyardToken = await ethers.getContractFactory("BiyardToken");
    const token = await BiyardToken.deploy("Le Mouton Token", "LMT", 0, 0);

    // Deploy FloorPriceTreasury (2% reward rate = 200 bps)
    const FloorPriceTreasury = await ethers.getContractFactory(
      "FloorPriceTreasury"
    );
    const treasury = await FloorPriceTreasury.deploy(
      await token.getAddress(),
      await usdt.getAddress(),
      enterprise.address,
      200 // 2%
    );

    // Setup: Give treasury minter role
    await token.addMinter(await treasury.getAddress());

    // Setup: Give enterprise USDT for deposits
    await usdt.mint(enterprise.address, ethers.parseUnits("100000", 6));
    // Give customers USDT too (for direct deposits)
    await usdt.mint(customer1.address, ethers.parseUnits("10000", 6));

    // Enterprise approves treasury to spend USDT
    await usdt
      .connect(enterprise)
      .approve(await treasury.getAddress(), ethers.MaxUint256);

    return { treasury, token, usdt, owner, enterprise, customer1, customer2, whale };
  }

  describe("Floor Price Calculation", function () {
    it("should return 0 when no tokens or treasury", async function () {
      const { treasury } = await loadFixture(deployFixture);
      expect(await treasury.getFloorPrice()).to.equal(0);
    });

    it("should calculate correct floor price after deposit", async function () {
      const { treasury, token, usdt, enterprise, customer1 } =
        await loadFixture(deployFixture);

      // Simulate: Customer buys ₩100,000 product ($83.33 USDT)
      // 2% goes to treasury = $1.67 USDT
      // Reward: 100 LMT tokens
      const purchaseUsdt = ethers.parseUnits("83.33", 6);
      const rewardTokens = ethers.parseUnits("100", 18);

      await treasury
        .connect(enterprise)
        .recordPurchase(customer1.address, purchaseUsdt, rewardTokens);

      const floorPrice = await treasury.getFloorPrice();
      const circulatingSupply = await treasury.getCirculatingSupply();

      expect(circulatingSupply).to.equal(rewardTokens);
      expect(floorPrice).to.be.gt(0);

      console.log(
        "Floor Price after first purchase:",
        ethers.formatUnits(floorPrice, 18),
        "USDT per LMT"
      );
    });
  });

  describe("Purchase → Treasury Growth", function () {
    it("floor price should increase with more purchases", async function () {
      const { treasury, enterprise, customer1, customer2 } =
        await loadFixture(deployFixture);

      // Purchase 1
      await treasury
        .connect(enterprise)
        .recordPurchase(
          customer1.address,
          ethers.parseUnits("100", 6),
          ethers.parseUnits("100", 18)
        );

      const price1 = await treasury.getFloorPrice();

      // Purchase 2 (same tokens, more USDT → price goes up)
      await treasury
        .connect(enterprise)
        .recordPurchase(
          customer2.address,
          ethers.parseUnits("200", 6),
          ethers.parseUnits("100", 18)
        );

      const price2 = await treasury.getFloorPrice();

      // Floor price should increase because treasury grew faster than supply
      expect(price2).to.be.gt(price1);

      console.log("Price after purchase 1:", ethers.formatUnits(price1, 18));
      console.log("Price after purchase 2:", ethers.formatUnits(price2, 18));
    });
  });

  describe("Floor Price Buyback + Burn", function () {
    it("should buy back tokens at floor price and burn them", async function () {
      const { treasury, token, usdt, enterprise, customer1 } =
        await loadFixture(deployFixture);

      // Setup: Create some treasury and tokens
      await treasury
        .connect(enterprise)
        .recordPurchase(
          customer1.address,
          ethers.parseUnits("1000", 6), // $1000 purchase
          ethers.parseUnits("500", 18) // 500 LMT reward
        );

      const floorBefore = await treasury.getFloorPrice();
      const balanceBefore = await token.balanceOf(customer1.address);
      const usdtBefore = await usdt.balanceOf(customer1.address);

      console.log("--- Before Buyback ---");
      console.log("Floor Price:", ethers.formatUnits(floorBefore, 18));
      console.log("Customer LMT:", ethers.formatUnits(balanceBefore, 18));
      console.log("Customer USDT:", ethers.formatUnits(usdtBefore, 6));

      // Customer sells 100 LMT at floor price
      const sellAmount = ethers.parseUnits("100", 18);
      await token
        .connect(customer1)
        .approve(await treasury.getAddress(), sellAmount);
      await treasury.connect(customer1).sellAtFloorPrice(sellAmount);

      const floorAfter = await treasury.getFloorPrice();
      const balanceAfter = await token.balanceOf(customer1.address);
      const usdtAfter = await usdt.balanceOf(customer1.address);

      console.log("--- After Buyback ---");
      console.log("Floor Price:", ethers.formatUnits(floorAfter, 18));
      console.log("Customer LMT:", ethers.formatUnits(balanceAfter, 18));
      console.log("Customer USDT:", ethers.formatUnits(usdtAfter, 6));

      // Floor price should be maintained (equal or slightly higher due to rounding)
      expect(floorAfter).to.be.gte(floorBefore);

      // Customer should have fewer tokens
      expect(balanceAfter).to.equal(balanceBefore - sellAmount);

      // Customer should have received USDT
      expect(usdtAfter).to.be.gt(usdtBefore);
    });

    it("should maintain floor price even after massive dump", async function () {
      const { treasury, token, usdt, enterprise, customer1 } =
        await loadFixture(deployFixture);

      // Setup: Large treasury
      await treasury
        .connect(enterprise)
        .recordPurchase(
          customer1.address,
          ethers.parseUnits("10000", 6), // $10,000
          ethers.parseUnits("1000", 18) // 1000 LMT
        );

      const floorBefore = await treasury.getFloorPrice();
      const supplyBefore = await treasury.getCirculatingSupply();

      console.log("=== WHALE DUMP TEST ===");
      console.log("Floor Before:", ethers.formatUnits(floorBefore, 18));
      console.log("Supply Before:", ethers.formatUnits(supplyBefore, 18));

      // Customer dumps 500 tokens (50% of supply!)
      const dumpAmount = ethers.parseUnits("500", 18);
      await token
        .connect(customer1)
        .approve(await treasury.getAddress(), dumpAmount);
      await treasury.connect(customer1).sellAtFloorPrice(dumpAmount);

      const floorAfter = await treasury.getFloorPrice();
      const supplyAfter = await treasury.getCirculatingSupply();

      console.log("Floor After:", ethers.formatUnits(floorAfter, 18));
      console.log("Supply After:", ethers.formatUnits(supplyAfter, 18));

      // CRITICAL: Floor price must NOT decrease
      expect(floorAfter).to.be.gte(floorBefore);

      // Supply should be halved
      expect(supplyAfter).to.equal(supplyBefore - dumpAmount);

      // Remaining holders' floor price is MAINTAINED
      console.log("✅ Floor price maintained after 50% dump!");
    });

    it("should handle 90% dump without floor price decrease", async function () {
      const { treasury, token, enterprise, customer1 } =
        await loadFixture(deployFixture);

      await treasury
        .connect(enterprise)
        .recordPurchase(
          customer1.address,
          ethers.parseUnits("5000", 6),
          ethers.parseUnits("1000", 18)
        );

      const floorBefore = await treasury.getFloorPrice();

      // Dump 900 out of 1000 tokens (90%!)
      const dumpAmount = ethers.parseUnits("900", 18);
      await token
        .connect(customer1)
        .approve(await treasury.getAddress(), dumpAmount);
      await treasury.connect(customer1).sellAtFloorPrice(dumpAmount);

      const floorAfter = await treasury.getFloorPrice();

      // Floor price STILL maintained
      expect(floorAfter).to.be.gte(floorBefore);
      console.log("✅ Floor price maintained after 90% dump!");
    });
  });

  describe("Treasury Info", function () {
    it("should return correct treasury info", async function () {
      const { treasury, enterprise, customer1 } =
        await loadFixture(deployFixture);

      await treasury
        .connect(enterprise)
        .recordPurchase(
          customer1.address,
          ethers.parseUnits("1000", 6),
          ethers.parseUnits("500", 18)
        );

      const info = await treasury.getTreasuryInfo();
      expect(info.usdtBalance).to.be.gt(0);
      expect(info.floorPrice).to.be.gt(0);
      expect(info.circulatingSupply).to.equal(ethers.parseUnits("500", 18));
      expect(info.totalDeposited).to.be.gt(0);
    });
  });

  describe("Access Control", function () {
    it("should only allow project owner to record purchases", async function () {
      const { treasury, customer1 } = await loadFixture(deployFixture);

      await expect(
        treasury
          .connect(customer1)
          .recordPurchase(
            customer1.address,
            ethers.parseUnits("100", 6),
            ethers.parseUnits("10", 18)
          )
      ).to.be.revertedWith("Not project owner");
    });

    it("should only allow project owner to mint reward tokens", async function () {
      const { treasury, customer1 } = await loadFixture(deployFixture);

      await expect(
        treasury
          .connect(customer1)
          .mintRewardTokens(
            customer1.address,
            ethers.parseUnits("100", 18),
            "test"
          )
      ).to.be.revertedWith("Not project owner");
    });
  });
});
