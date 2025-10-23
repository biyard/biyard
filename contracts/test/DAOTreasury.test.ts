import { expect } from "chai";
import { ethers } from "hardhat";
import { BiyardToken, DAOTreasury } from "../typechain-types";
import { SignerWithAddress } from "@nomicfoundation/hardhat-ethers/signers";
import { time } from "@nomicfoundation/hardhat-network-helpers";

describe("DAOTreasury", function () {
  let token: BiyardToken;
  let treasury: DAOTreasury;
  let owner: SignerWithAddress;
  let proposer: SignerWithAddress;
  let voter1: SignerWithAddress;
  let voter2: SignerWithAddress;
  let recipient: SignerWithAddress;

  const INITIAL_SUPPLY = ethers.parseEther("1000000");
  const PROPOSAL_THRESHOLD = ethers.parseEther("100");
  const VOTING_PERIOD = 3 * 24 * 60 * 60; // 3 days
  const QUORUM_PERCENTAGE = 10; // 10%

  beforeEach(async function () {
    [owner, proposer, voter1, voter2, recipient] = await ethers.getSigners();

    // Deploy token
    const BiyardToken = await ethers.getContractFactory("BiyardToken");
    token = await BiyardToken.deploy("Biyard Token", "BIYARD", INITIAL_SUPPLY, 0);
    await token.waitForDeployment();

    // Deploy treasury
    const DAOTreasury = await ethers.getContractFactory("DAOTreasury");
    treasury = await DAOTreasury.deploy(
      await token.getAddress(),
      PROPOSAL_THRESHOLD,
      VOTING_PERIOD,
      QUORUM_PERCENTAGE
    );
    await treasury.waitForDeployment();

    // Distribute tokens
    await token.transfer(proposer.address, ethers.parseEther("150000"));
    await token.transfer(voter1.address, ethers.parseEther("150000"));
    await token.transfer(voter2.address, ethers.parseEther("150000"));

    // Fund treasury
    await token.transfer(await treasury.getAddress(), ethers.parseEther("100000"));
  });

  describe("Deployment", function () {
    it("Should set the correct governance token", async function () {
      expect(await treasury.governanceToken()).to.equal(await token.getAddress());
    });

    it("Should set the correct proposal threshold", async function () {
      expect(await treasury.proposalThreshold()).to.equal(PROPOSAL_THRESHOLD);
    });

    it("Should set the correct voting period", async function () {
      expect(await treasury.votingPeriod()).to.equal(VOTING_PERIOD);
    });

    it("Should set the correct quorum percentage", async function () {
      expect(await treasury.quorumPercentage()).to.equal(QUORUM_PERCENTAGE);
    });
  });

  describe("Deposits", function () {
    it("Should receive native tokens", async function () {
      const depositAmount = ethers.parseEther("10");
      await expect(
        owner.sendTransaction({
          to: await treasury.getAddress(),
          value: depositAmount,
        })
      )
        .to.emit(treasury, "FundsDeposited")
        .withArgs(owner.address, depositAmount);

      expect(await ethers.provider.getBalance(await treasury.getAddress())).to.equal(
        depositAmount
      );
    });

    it("Should accept ERC20 token deposits", async function () {
      const depositAmount = ethers.parseEther("1000");
      await token.approve(await treasury.getAddress(), depositAmount);
      await expect(treasury.depositTokens(await token.getAddress(), depositAmount))
        .to.emit(treasury, "TokensDeposited")
        .withArgs(owner.address, await token.getAddress(), depositAmount);
    });

    it("Should get treasury balance correctly", async function () {
      const balance = await treasury.getTreasuryBalance(await token.getAddress());
      expect(balance).to.equal(ethers.parseEther("100000"));
    });
  });

  describe("Proposal Creation", function () {
    it("Should allow users with sufficient tokens to create proposal", async function () {
      const proposalAmount = ethers.parseEther("1000");
      const description = "Fund development";

      await expect(
        treasury
          .connect(proposer)
          .createProposal(description, recipient.address, proposalAmount, await token.getAddress())
      )
        .to.emit(treasury, "ProposalCreated")
        .withArgs(
          1,
          proposer.address,
          description,
          recipient.address,
          proposalAmount,
          await token.getAddress(),
          await time.latest(),
          (await time.latest()) + VOTING_PERIOD
        );
    });

    it("Should not allow users without sufficient tokens to create proposal", async function () {
      const proposalAmount = ethers.parseEther("1000");
      // voter2 has tokens but we'll use a fresh account
      const [, , , , , noTokenUser] = await ethers.getSigners();

      await expect(
        treasury
          .connect(noTokenUser)
          .createProposal(
            "Test proposal",
            recipient.address,
            proposalAmount,
            await token.getAddress()
          )
      ).to.be.revertedWith("Insufficient tokens to create proposal");
    });

    it("Should not allow proposal with insufficient treasury funds", async function () {
      const excessiveAmount = ethers.parseEther("200000");
      await expect(
        treasury
          .connect(proposer)
          .createProposal(
            "Excessive proposal",
            recipient.address,
            excessiveAmount,
            await token.getAddress()
          )
      ).to.be.revertedWith("Insufficient token balance");
    });

    it("Should increment proposal count", async function () {
      await treasury
        .connect(proposer)
        .createProposal(
          "Proposal 1",
          recipient.address,
          ethers.parseEther("1000"),
          await token.getAddress()
        );
      expect(await treasury.proposalCount()).to.equal(1);

      await treasury
        .connect(proposer)
        .createProposal(
          "Proposal 2",
          recipient.address,
          ethers.parseEther("1000"),
          await token.getAddress()
        );
      expect(await treasury.proposalCount()).to.equal(2);
    });
  });

  describe("Voting", function () {
    let proposalId: number;

    beforeEach(async function () {
      const tx = await treasury
        .connect(proposer)
        .createProposal(
          "Test proposal",
          recipient.address,
          ethers.parseEther("1000"),
          await token.getAddress()
        );
      await tx.wait();
      proposalId = 1;
    });

    it("Should allow token holders to vote", async function () {
      const voterBalance = await token.balanceOf(voter1.address);
      await expect(treasury.connect(voter1).castVote(proposalId, true))
        .to.emit(treasury, "VoteCast")
        .withArgs(proposalId, voter1.address, true, voterBalance);
    });

    it("Should not allow voting twice", async function () {
      await treasury.connect(voter1).castVote(proposalId, true);
      await expect(treasury.connect(voter1).castVote(proposalId, true)).to.be.revertedWith(
        "Already voted"
      );
    });

    it("Should not allow voting without tokens", async function () {
      const [, , , , , noTokenUser] = await ethers.getSigners();
      await expect(treasury.connect(noTokenUser).castVote(proposalId, true)).to.be.revertedWith(
        "No voting power"
      );
    });

    it("Should not allow voting after period ends", async function () {
      await time.increase(VOTING_PERIOD + 1);
      await expect(treasury.connect(voter1).castVote(proposalId, true)).to.be.revertedWith(
        "Voting period has ended"
      );
    });

    it("Should track votes correctly", async function () {
      await treasury.connect(voter1).castVote(proposalId, true);
      await treasury.connect(voter2).castVote(proposalId, false);

      const proposal = await treasury.getProposal(proposalId);
      expect(proposal.votesFor).to.equal(await token.balanceOf(voter1.address));
      expect(proposal.votesAgainst).to.equal(await token.balanceOf(voter2.address));
    });
  });

  describe("Proposal Execution", function () {
    let proposalId: number;

    beforeEach(async function () {
      const tx = await treasury
        .connect(proposer)
        .createProposal(
          "Test proposal",
          recipient.address,
          ethers.parseEther("1000"),
          await token.getAddress()
        );
      await tx.wait();
      proposalId = 1;
    });

    it("Should execute successful proposal", async function () {
      // Vote in favor
      await treasury.connect(voter1).castVote(proposalId, true);
      await treasury.connect(voter2).castVote(proposalId, true);

      // Wait for voting period to end
      await time.increase(VOTING_PERIOD + 1);

      const initialBalance = await token.balanceOf(recipient.address);

      await expect(treasury.executeProposal(proposalId))
        .to.emit(treasury, "ProposalExecuted")
        .withArgs(proposalId);

      expect(await token.balanceOf(recipient.address)).to.equal(
        initialBalance + ethers.parseEther("1000")
      );

      const proposal = await treasury.getProposal(proposalId);
      expect(proposal.state).to.equal(4); // Executed
    });

    it("Should mark defeated proposal as defeated", async function () {
      // Vote against
      await treasury.connect(voter1).castVote(proposalId, false);
      await treasury.connect(voter2).castVote(proposalId, false);

      await time.increase(VOTING_PERIOD + 1);

      await treasury.executeProposal(proposalId);

      const proposal = await treasury.getProposal(proposalId);
      expect(proposal.state).to.equal(3); // Defeated
    });

    it("Should not execute before voting period ends", async function () {
      await treasury.connect(voter1).castVote(proposalId, true);
      await expect(treasury.executeProposal(proposalId)).to.be.revertedWith(
        "Voting period not ended"
      );
    });

    it("Should require quorum to pass", async function () {
      // Only one small voter (not enough for quorum)
      const [, , , , , smallVoter] = await ethers.getSigners();
      await token.transfer(smallVoter.address, ethers.parseEther("1"));

      await treasury.connect(smallVoter).castVote(proposalId, true);
      await time.increase(VOTING_PERIOD + 1);

      await treasury.executeProposal(proposalId);

      const proposal = await treasury.getProposal(proposalId);
      expect(proposal.state).to.equal(3); // Defeated due to lack of quorum
    });
  });

  describe("Proposal Cancellation", function () {
    let proposalId: number;

    beforeEach(async function () {
      const tx = await treasury
        .connect(proposer)
        .createProposal(
          "Test proposal",
          recipient.address,
          ethers.parseEther("1000"),
          await token.getAddress()
        );
      await tx.wait();
      proposalId = 1;
    });

    it("Should allow proposer to cancel their proposal", async function () {
      await expect(treasury.connect(proposer).cancelProposal(proposalId))
        .to.emit(treasury, "ProposalCancelled")
        .withArgs(proposalId);

      const proposal = await treasury.getProposal(proposalId);
      expect(proposal.state).to.equal(5); // Cancelled
    });

    it("Should allow owner to cancel any proposal", async function () {
      await expect(treasury.connect(owner).cancelProposal(proposalId))
        .to.emit(treasury, "ProposalCancelled")
        .withArgs(proposalId);
    });

    it("Should not allow others to cancel proposal", async function () {
      await expect(treasury.connect(voter1).cancelProposal(proposalId)).to.be.revertedWith(
        "Only proposer or owner can cancel"
      );
    });
  });

  describe("Parameter Updates", function () {
    it("Should allow owner to update proposal threshold", async function () {
      const newThreshold = ethers.parseEther("200");
      await treasury.updateProposalThreshold(newThreshold);
      expect(await treasury.proposalThreshold()).to.equal(newThreshold);
    });

    it("Should allow owner to update voting period", async function () {
      const newPeriod = 7 * 24 * 60 * 60; // 7 days
      await treasury.updateVotingPeriod(newPeriod);
      expect(await treasury.votingPeriod()).to.equal(newPeriod);
    });

    it("Should allow owner to update quorum percentage", async function () {
      const newQuorum = 20;
      await treasury.updateQuorumPercentage(newQuorum);
      expect(await treasury.quorumPercentage()).to.equal(newQuorum);
    });

    it("Should not allow non-owner to update parameters", async function () {
      await expect(
        treasury.connect(voter1).updateProposalThreshold(ethers.parseEther("200"))
      ).to.be.reverted;
    });

    it("Should validate quorum percentage", async function () {
      await expect(treasury.updateQuorumPercentage(0)).to.be.revertedWith(
        "Invalid quorum percentage"
      );
      await expect(treasury.updateQuorumPercentage(101)).to.be.revertedWith(
        "Invalid quorum percentage"
      );
    });
  });

  describe("View Functions", function () {
    let proposalId: number;

    beforeEach(async function () {
      const tx = await treasury
        .connect(proposer)
        .createProposal(
          "Test proposal",
          recipient.address,
          ethers.parseEther("1000"),
          await token.getAddress()
        );
      await tx.wait();
      proposalId = 1;
    });

    it("Should return correct proposal state", async function () {
      expect(await treasury.getProposalState(proposalId)).to.equal(1); // Active
    });

    it("Should return correct proposal details", async function () {
      const proposal = await treasury.getProposal(proposalId);
      expect(proposal.proposer).to.equal(proposer.address);
      expect(proposal.recipient).to.equal(recipient.address);
      expect(proposal.amount).to.equal(ethers.parseEther("1000"));
      expect(proposal.tokenAddress).to.equal(await token.getAddress());
    });

    it("Should track who has voted", async function () {
      expect(await treasury.hasVoted(proposalId, voter1.address)).to.be.false;
      await treasury.connect(voter1).castVote(proposalId, true);
      expect(await treasury.hasVoted(proposalId, voter1.address)).to.be.true;
    });
  });
});
