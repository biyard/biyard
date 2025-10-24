import { ethers } from "hardhat";

async function main() {
  console.log("Starting deployment with exchange functionality...\n");

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
  const biyardToken = await BiyardToken.deploy(tokenName, tokenSymbol, initialSupply, maxSupply);
  await biyardToken.waitForDeployment();

  const biyardTokenAddress = await biyardToken.getAddress();
  console.log("BiyardToken deployed to:", biyardTokenAddress);
  console.log("Initial supply:", ethers.formatEther(initialSupply), tokenSymbol);
  console.log("Max supply:", ethers.formatEther(maxSupply), tokenSymbol);
  console.log();

  // Deploy MockUSDT (for testing - replace with real USDT address in production)
  console.log("Deploying MockUSDT...");
  const MockUSDT = await ethers.getContractFactory("MockUSDT");
  const usdtToken = await MockUSDT.deploy();
  await usdtToken.waitForDeployment();

  const usdtTokenAddress = await usdtToken.getAddress();
  console.log("MockUSDT deployed to:", usdtTokenAddress);
  console.log();

  // Deploy DAOTreasuryWithExchange
  console.log("Deploying DAOTreasuryWithExchange...");
  const proposalThreshold = ethers.parseEther("100"); // 100 tokens to create proposal
  const votingPeriod = 3 * 24 * 60 * 60; // 3 days in seconds
  const quorumPercentage = 10; // 10% quorum
  const exchangeFee = 30; // 0.3% exchange fee

  const DAOTreasuryWithExchange = await ethers.getContractFactory("DAOTreasuryWithExchange");
  const treasury = await DAOTreasuryWithExchange.deploy(
    biyardTokenAddress,
    usdtTokenAddress,
    proposalThreshold,
    votingPeriod,
    quorumPercentage,
    exchangeFee
  );
  await treasury.waitForDeployment();

  const treasuryAddress = await treasury.getAddress();
  console.log("DAOTreasuryWithExchange deployed to:", treasuryAddress);
  console.log("Proposal threshold:", ethers.formatEther(proposalThreshold), tokenSymbol);
  console.log("Voting period:", votingPeriod / (24 * 60 * 60), "days");
  console.log("Quorum percentage:", quorumPercentage + "%");
  console.log("Exchange fee:", exchangeFee / 100 + "%");
  console.log();

  // Setup initial reserves for the exchange
  console.log("Setting up initial reserves...");

  // Transfer BIYARD to treasury (reserve)
  const biyardReserve = ethers.parseEther("100000"); // 100k BIYARD
  let tx = await biyardToken.transfer(treasuryAddress, biyardReserve);
  await tx.wait();
  console.log("Transferred", ethers.formatEther(biyardReserve), tokenSymbol, "to treasury");

  // Transfer USDT to treasury (reserve)
  const usdtReserve = 100000n * 10n ** 6n; // 100k USDT
  tx = await usdtToken.transfer(treasuryAddress, usdtReserve);
  await tx.wait();
  console.log("Transferred", Number(usdtReserve) / 1e6, "USDT to treasury");
  console.log();

  // Calculate and display initial price
  const price = await treasury.getCurrentPrice();
  console.log("Initial BIYARD price:", Number(price) / 1e18, "USDT per BIYARD");
  console.log();

  // Get exchange info
  const info = await treasury.getExchangeInfo();
  console.log("Exchange Status:");
  console.log("- USDT Reserve:", Number(info.usdtReserve) / 1e6, "USDT");
  console.log("- BIYARD Reserve:", ethers.formatEther(info.biyardReserve), "BIYARD");
  console.log("- Circulating Supply:", ethers.formatEther(info.circulatingSupply), "BIYARD");
  console.log("- Exchange Enabled:", info.isEnabled);
  console.log();

  // Summary
  console.log("=".repeat(60));
  console.log("Deployment Summary:");
  console.log("=".repeat(60));
  console.log("BiyardToken:", biyardTokenAddress);
  console.log("MockUSDT:", usdtTokenAddress);
  console.log("DAOTreasuryWithExchange:", treasuryAddress);
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
        address: biyardTokenAddress,
        name: tokenName,
        symbol: tokenSymbol,
        initialSupply: initialSupply.toString(),
        maxSupply: maxSupply.toString(),
      },
      MockUSDT: {
        address: usdtTokenAddress,
        symbol: "USDT",
        decimals: 6,
      },
      DAOTreasuryWithExchange: {
        address: treasuryAddress,
        governanceToken: biyardTokenAddress,
        usdtToken: usdtTokenAddress,
        proposalThreshold: proposalThreshold.toString(),
        votingPeriod: votingPeriod,
        quorumPercentage: quorumPercentage,
        exchangeFee: exchangeFee,
        initialReserves: {
          biyard: biyardReserve.toString(),
          usdt: usdtReserve.toString(),
        },
        initialPrice: price.toString(),
      },
    },
    timestamp: new Date().toISOString(),
  };

  console.log("Deployment info:");
  console.log(JSON.stringify(deploymentInfo, null, 2));
  console.log();

  console.log("Next steps:");
  console.log("1. Verify contracts on block explorer");
  console.log("2. Configure frontend to use these contract addresses");
  console.log("3. Test exchange functionality through UI");
  console.log("4. Set up event listeners for TokensExchanged events");
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
