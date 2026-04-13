import { ethers } from "hardhat";

async function main() {
  const [deployer] = await ethers.getSigners();
  console.log("Deployer:", deployer.address);

  const BUSDT = await ethers.getContractFactory("BUSDT");
  const busdt = await BUSDT.deploy();
  await busdt.waitForDeployment();
  console.log("BUSDT:", await busdt.getAddress());

  const Multisig = await ethers.getContractFactory("Multisig");
  const ms = await Multisig.deploy([deployer.address], 1);
  await ms.waitForDeployment();
  console.log("Multisig:", await ms.getAddress());

  const BrandToken = await ethers.getContractFactory("BrandToken");
  const token = await BrandToken.deploy(
    "CafeToken", "CAFE",
    ethers.parseEther("1000000"),
    500,
    deployer.address,
    deployer.address
  );
  await token.waitForDeployment();
  console.log("BrandToken:", await token.getAddress());

  const Treasury = await ethers.getContractFactory("Treasury");
  const treasury = await Treasury.deploy(
    await busdt.getAddress(), await token.getAddress(), await ms.getAddress()
  );
  await treasury.waitForDeployment();
  console.log("Treasury:", await treasury.getAddress());

  await token.setTreasury(await treasury.getAddress());
  await token.setDistributionSlots(
    [deployer.address],
    [1000]
  );
  await token.transferOwnership(await ms.getAddress());

  await busdt.mint(deployer.address, ethers.parseUnits("1000000", 6));
  const depositAmount = ethers.parseUnits("100000", 6);
  await busdt.approve(await treasury.getAddress(), depositAmount);
  await treasury.deposit(depositAmount);

  const mintData = token.interface.encodeFunctionData("triggerMonthlyMint");
  await ms.propose(await token.getAddress(), mintData, 0);
  await ms.approve(0);
  await ms.execute(0);

  const price = await treasury.getPrice();
  console.log("\n=== Deployment Complete ===");
  console.log(JSON.stringify({
    busdt: await busdt.getAddress(),
    multisig: await ms.getAddress(),
    brandToken: await token.getAddress(),
    treasury: await treasury.getAddress(),
    initialPrice: price.toString(),
    totalSupply: (await token.totalSupply()).toString(),
    circulatingSupply: (await treasury.circulatingSupply()).toString(),
    treasuryBalance: (await busdt.balanceOf(await treasury.getAddress())).toString(),
  }, null, 2));
}

main().catch((err) => {
  console.error(err);
  process.exitCode = 1;
});
