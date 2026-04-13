import { ethers } from "hardhat";

async function main() {
  const [deployer] = await ethers.getSigners();
  console.log("Deployer:", deployer.address);

  const stableTokenAddress = process.env.STABLE_TOKEN_ADDRESS;
  if (!stableTokenAddress) throw new Error("Set STABLE_TOKEN_ADDRESS env var");

  const claimSignerAddress = process.env.CLAIM_SIGNER_ADDRESS;
  if (!claimSignerAddress) throw new Error("Set CLAIM_SIGNER_ADDRESS env var");

  const monthlyEmission = ethers.parseEther(process.env.MONTHLY_EMISSION || "1000000");
  const decayBps = parseInt(process.env.DECAY_BPS || "500");
  const tokenName = process.env.TOKEN_NAME || "BrandToken";
  const tokenSymbol = process.env.TOKEN_SYMBOL || "BRAND";

  // Default: start from the 1st of last month so month 0 is already claimable.
  const now = new Date();
  const lastMonth1st = new Date(Date.UTC(now.getUTCFullYear(), now.getUTCMonth() - 1, 1));
  const startTimestamp = parseInt(process.env.START_TIMESTAMP || String(Math.floor(lastMonth1st.getTime() / 1000)));

  const Multisig = await ethers.getContractFactory("Multisig");
  const ms = await Multisig.deploy([deployer.address], 1);
  await ms.waitForDeployment();
  console.log("Multisig:", await ms.getAddress());

  const BrandToken = await ethers.getContractFactory("BrandToken");
  const token = await BrandToken.deploy(
    tokenName, tokenSymbol, monthlyEmission, decayBps,
    claimSignerAddress, deployer.address, startTimestamp
  );
  await token.waitForDeployment();
  console.log("BrandToken:", await token.getAddress());

  const Treasury = await ethers.getContractFactory("Treasury");
  const treasury = await Treasury.deploy(
    stableTokenAddress, await token.getAddress(), await ms.getAddress()
  );
  await treasury.waitForDeployment();
  console.log("Treasury:", await treasury.getAddress());

  await token.setTreasury(await treasury.getAddress());
  await token.transferOwnership(await ms.getAddress());
  console.log("BrandToken ownership transferred to Multisig");

  console.log(JSON.stringify({
    multisig: await ms.getAddress(),
    brandToken: await token.getAddress(),
    treasury: await treasury.getAddress(),
    stableToken: stableTokenAddress,
    claimSigner: claimSignerAddress,
    monthlyEmission: monthlyEmission.toString(),
    decayBps,
    startTimestamp,
  }, null, 2));
}

main().catch((err) => {
  console.error(err);
  process.exitCode = 1;
});
