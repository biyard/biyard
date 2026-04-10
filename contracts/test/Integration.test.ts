import { expect } from "chai";
import { ethers } from "hardhat";
import { time } from "@nomicfoundation/hardhat-network-helpers";

describe("Integration: Full Scenario", function () {
  const MONTH = 30 * 24 * 60 * 60;

  async function deployFullSystem() {
    const [deployer, claimSigner, marketing, partner, user1, user2] = await ethers.getSigners();

    const BUSDT = await ethers.getContractFactory("BUSDT");
    const busdt = await BUSDT.deploy();

    const Multisig = await ethers.getContractFactory("Multisig");
    const ms = await Multisig.deploy([deployer.address], 1);

    const BrandToken = await ethers.getContractFactory("BrandToken");
    const token = await BrandToken.deploy(
      "CafeToken", "CAFE",
      ethers.parseEther("10000000"),
      ethers.parseEther("1000000"),
      500,
      claimSigner.address,
      deployer.address
    );

    const Treasury = await ethers.getContractFactory("Treasury");
    const treasury = await Treasury.deploy(
      await busdt.getAddress(),
      await token.getAddress(),
      await ms.getAddress()
    );

    await token.setTreasury(await treasury.getAddress());
    await token.setDistributionSlots(
      [marketing.address, partner.address],
      [1000, 500]
    );
    await token.transferOwnership(await ms.getAddress());

    return { busdt, token, treasury, ms, deployer, claimSigner, marketing, partner, user1, user2 };
  }

  async function signClaim(token: any, signer: any, to: string, amount: bigint, nonce: bigint, deadline: bigint) {
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
    return await signer.signTypedData(domain, types, { to, amount, nonce, deadline });
  }

  it("Scenario: deploy -> mint -> deposit -> claim -> buyback -> governance", async function () {
    const { busdt, token, treasury, ms, deployer, claimSigner, marketing, partner, user1 } =
      await deployFullSystem();

    // Step 1: Mint BUSDT (faucet)
    await busdt.mint(deployer.address, ethers.parseUnits("1000000", 6));
    expect(await busdt.balanceOf(deployer.address)).to.equal(ethers.parseUnits("1000000", 6));

    // Step 2: Deposit 100K BUSDT into Treasury
    const depositAmount = ethers.parseUnits("100000", 6);
    await busdt.approve(await treasury.getAddress(), depositAmount);
    await treasury.deposit(depositAmount);
    expect(await busdt.balanceOf(await treasury.getAddress())).to.equal(depositAmount);

    // Step 3: triggerMonthlyMint via Multisig
    const mintData = token.interface.encodeFunctionData("triggerMonthlyMint");
    await ms.propose(await token.getAddress(), mintData, 0);
    await ms.approve(0);
    await ms.execute(0);

    expect(await token.balanceOf(marketing.address)).to.equal(ethers.parseEther("100000"));
    expect(await token.balanceOf(partner.address)).to.equal(ethers.parseEther("50000"));
    expect(await token.totalSupply()).to.equal(ethers.parseEther("1000000"));

    // Step 4: Check price
    const price = await treasury.getPrice();
    expect(price).to.be.greaterThan(0);

    // Step 5: User claims tokens
    const claimAmount = ethers.parseEther("10000");
    const deadline = BigInt(await time.latest()) + 3600n;
    const sig = await signClaim(token, claimSigner, user1.address, claimAmount, 1n, deadline);
    await token.connect(user1).claim(claimAmount, 1n, deadline, sig);
    expect(await token.balanceOf(user1.address)).to.equal(claimAmount);

    // Step 6: User does buyback
    const sellAmount = ethers.parseEther("5000");
    await token.connect(user1).approve(await treasury.getAddress(), sellAmount);
    const stableBefore = await busdt.balanceOf(user1.address);
    await treasury.connect(user1).buyback(sellAmount);
    const stableAfter = await busdt.balanceOf(user1.address);
    expect(stableAfter).to.be.greaterThan(stableBefore);
    expect(await token.balanceOf(await treasury.getAddress())).to.equal(sellAmount);

    // Step 7: Multisig withdraws BUSDT
    const withdrawAmount = ethers.parseUnits("1000", 6);
    const withdrawData = treasury.interface.encodeFunctionData(
      "withdrawStable", [deployer.address, withdrawAmount]
    );
    await ms.propose(await treasury.getAddress(), withdrawData, 0);
    await ms.approve(1);
    await ms.execute(1);

    // Step 8: Multisig changes emission via governance
    const setEmissionData = token.interface.encodeFunctionData(
      "setMonthlyEmission", [ethers.parseEther("500000")]
    );
    await ms.propose(await token.getAddress(), setEmissionData, 0);
    await ms.approve(2);
    await ms.execute(2);

    // Advance 1 month and mint with new emission
    await time.increase(MONTH);
    const mint2Data = token.interface.encodeFunctionData("triggerMonthlyMint");
    await ms.propose(await token.getAddress(), mint2Data, 0);
    await ms.approve(3);
    await ms.execute(3);

    // Month 1 ceiling: 500000 * 0.95 = 475000
    expect(await token.monthlyMinted(1)).to.equal(ethers.parseEther("475000"));
  });

  it("should handle skipped month correctly", async function () {
    const { token, ms } = await deployFullSystem();

    const mintData = token.interface.encodeFunctionData("triggerMonthlyMint");
    await ms.propose(await token.getAddress(), mintData, 0);
    await ms.approve(0);
    await ms.execute(0);

    // Skip month 1, go to month 2
    await time.increase(MONTH * 2);

    await ms.propose(await token.getAddress(), mintData, 0);
    await ms.approve(1);
    await ms.execute(1);

    // Month 2 ceiling: 1M * 0.95^2 = 902,500
    expect(await token.monthlyMinted(2)).to.equal(ethers.parseEther("902500"));
    expect(await token.monthlyMinted(1)).to.equal(0);
  });
});
