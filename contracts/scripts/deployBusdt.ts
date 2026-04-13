import { ethers } from "hardhat";

async function main() {
  const [deployer] = await ethers.getSigners();
  console.log("Deployer:", deployer.address);
  console.log("Network:", (await ethers.provider.getNetwork()).chainId.toString());

  const balance = await ethers.provider.getBalance(deployer.address);
  console.log("Balance:", ethers.formatEther(balance), "KAIA");

  const BUSDT = await ethers.getContractFactory("BUSDT");
  const busdt = await BUSDT.deploy();
  await busdt.waitForDeployment();

  const address = await busdt.getAddress();
  console.log("\n=== BUSDT Deployed ===");
  console.log("Address:", address);
  console.log("\nUpdate this address in:");
  console.log("  - console/src/common/types/supported_chain.rs (KaiaKairos stable_token_options)");
  console.log("  - contracts/dapp/buyback.html (if hardcoding)");
}

main().catch((err) => {
  console.error(err);
  process.exitCode = 1;
});
