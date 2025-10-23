import { ethers } from "hardhat";

async function main() {
  console.log("Starting deployment...\n");

  // Get deployer account
  const [deployer] = await ethers.getSigners();
  console.log("Deploying contracts with account:", deployer.address);
  console.log("Account balance:", (await ethers.provider.getBalance(deployer.address)).toString());
  console.log();

  // Deploy BiyardToken
  console.log("Deploying BiyardToken...");
  const tokenName = "Biyard Token";
  const tokenSymbol = "BIYARD";
  const initialSupply = ethers.parseEther("1000000"); // 1 million tokens
  const maxSupply = ethers.parseEther("10000000"); // 10 million tokens max

  const BiyardToken = await ethers.getContractFactory("BiyardToken");
  const token = await BiyardToken.deploy(tokenName, tokenSymbol, initialSupply, maxSupply);
  await token.waitForDeployment();

  const tokenAddress = await token.getAddress();
  console.log("BiyardToken deployed to:", tokenAddress);
  console.log("Initial supply:", ethers.formatEther(initialSupply), tokenSymbol);
  console.log("Max supply:", ethers.formatEther(maxSupply), tokenSymbol);
  console.log();

  // Deploy DAOTreasury
  console.log("Deploying DAOTreasury...");
  const proposalThreshold = ethers.parseEther("100"); // 100 tokens to create proposal
  const votingPeriod = 3 * 24 * 60 * 60; // 3 days in seconds
  const quorumPercentage = 10; // 10% quorum

  const DAOTreasury = await ethers.getContractFactory("DAOTreasury");
  const treasury = await DAOTreasury.deploy(
    tokenAddress,
    proposalThreshold,
    votingPeriod,
    quorumPercentage
  );
  await treasury.waitForDeployment();

  const treasuryAddress = await treasury.getAddress();
  console.log("DAOTreasury deployed to:", treasuryAddress);
  console.log("Proposal threshold:", ethers.formatEther(proposalThreshold), tokenSymbol);
  console.log("Voting period:", votingPeriod / (24 * 60 * 60), "days");
  console.log("Quorum percentage:", quorumPercentage + "%");
  console.log();

  // Transfer some tokens to treasury for testing
  console.log("Transferring tokens to treasury...");
  const treasuryAmount = ethers.parseEther("100000"); // 100k tokens
  const transferTx = await token.transfer(treasuryAddress, treasuryAmount);
  await transferTx.wait();
  console.log("Transferred", ethers.formatEther(treasuryAmount), tokenSymbol, "to treasury");
  console.log();

  // Summary
  console.log("=".repeat(60));
  console.log("Deployment Summary:");
  console.log("=".repeat(60));
  console.log("BiyardToken:", tokenAddress);
  console.log("DAOTreasury:", treasuryAddress);
  console.log("Deployer:", deployer.address);
  console.log("=".repeat(60));
  console.log();

  // Save deployment info to file
  const deploymentInfo = {
    network: (await ethers.provider.getNetwork()).name,
    chainId: (await ethers.provider.getNetwork()).chainId.toString(),
    deployer: deployer.address,
    contracts: {
      BiyardToken: {
        address: tokenAddress,
        name: tokenName,
        symbol: tokenSymbol,
        initialSupply: initialSupply.toString(),
        maxSupply: maxSupply.toString(),
      },
      DAOTreasury: {
        address: treasuryAddress,
        governanceToken: tokenAddress,
        proposalThreshold: proposalThreshold.toString(),
        votingPeriod: votingPeriod,
        quorumPercentage: quorumPercentage,
      },
    },
    timestamp: new Date().toISOString(),
  };

  console.log("Deployment info:");
  console.log(JSON.stringify(deploymentInfo, null, 2));
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
