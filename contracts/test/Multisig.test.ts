import { expect } from "chai";
import { ethers } from "hardhat";

describe("Multisig", function () {
  async function deployFixture() {
    const [owner, alice, bob, charlie] = await ethers.getSigners();
    const Multisig = await ethers.getContractFactory("Multisig");
    const ms = await Multisig.deploy([owner.address], 1); // 1-of-1
    return { ms, owner, alice, bob, charlie };
  }

  async function deploy2of3Fixture() {
    const [owner, alice, bob, charlie] = await ethers.getSigners();
    const Multisig = await ethers.getContractFactory("Multisig");
    const ms = await Multisig.deploy([owner.address, alice.address, bob.address], 2);
    return { ms, owner, alice, bob, charlie };
  }

  describe("Deployment", function () {
    it("should set signers and threshold", async function () {
      const { ms, owner } = await deployFixture();
      expect(await ms.isSigner(owner.address)).to.be.true;
      expect(await ms.threshold()).to.equal(1);
      expect(await ms.signerCount()).to.equal(1);
    });

    it("should revert with zero signers", async function () {
      const Multisig = await ethers.getContractFactory("Multisig");
      await expect(Multisig.deploy([], 1)).to.be.revertedWith("Multisig: no signers");
    });

    it("should revert with threshold > signers", async function () {
      const [owner] = await ethers.getSigners();
      const Multisig = await ethers.getContractFactory("Multisig");
      await expect(Multisig.deploy([owner.address], 2)).to.be.revertedWith("Multisig: bad threshold");
    });
  });

  describe("Propose and Execute (1-of-1)", function () {
    it("should propose, approve, and execute", async function () {
      const { ms, owner, alice } = await deployFixture();
      // Send ETH to multisig
      await owner.sendTransaction({ to: await ms.getAddress(), value: ethers.parseEther("1") });

      // Propose sending 0.5 ETH to alice
      const tx = await ms.propose(alice.address, "0x", ethers.parseEther("0.5"));
      const receipt = await tx.wait();
      const proposalId = 0;

      await ms.approve(proposalId);
      const balBefore = await ethers.provider.getBalance(alice.address);
      await ms.execute(proposalId);
      const balAfter = await ethers.provider.getBalance(alice.address);
      expect(balAfter - balBefore).to.equal(ethers.parseEther("0.5"));
    });

    it("should revert execute without approval", async function () {
      const { ms, alice } = await deployFixture();
      await ms.propose(alice.address, "0x", 0);
      await expect(ms.execute(0)).to.be.revertedWith("Multisig: not enough approvals");
    });

    it("should revert double execute", async function () {
      const { ms, alice } = await deployFixture();
      await ms.propose(alice.address, "0x", 0);
      await ms.approve(0);
      await ms.execute(0);
      await expect(ms.execute(0)).to.be.revertedWith("Multisig: already executed");
    });
  });

  describe("Propose and Execute (2-of-3)", function () {
    it("should require 2 approvals", async function () {
      const { ms, owner, alice, bob } = await deploy2of3Fixture();
      await ms.propose(bob.address, "0x", 0);
      await ms.connect(owner).approve(0);
      await expect(ms.execute(0)).to.be.revertedWith("Multisig: not enough approvals");
      await ms.connect(alice).approve(0);
      await expect(ms.execute(0)).to.not.be.reverted;
    });

    it("should revert double approval", async function () {
      const { ms, owner } = await deploy2of3Fixture();
      await ms.propose(owner.address, "0x", 0);
      await ms.approve(0);
      await expect(ms.approve(0)).to.be.revertedWith("Multisig: already approved");
    });

    it("should reject non-signer propose", async function () {
      const { ms, charlie } = await deploy2of3Fixture();
      await expect(ms.connect(charlie).propose(charlie.address, "0x", 0))
        .to.be.revertedWith("Multisig: not a signer");
    });
  });

  describe("Self-governance", function () {
    it("should add signer via proposal", async function () {
      const { ms, owner, alice } = await deployFixture();
      const data = ms.interface.encodeFunctionData("addSigner", [alice.address]);
      await ms.propose(await ms.getAddress(), data, 0);
      await ms.approve(0);
      await ms.execute(0);
      expect(await ms.isSigner(alice.address)).to.be.true;
      expect(await ms.signerCount()).to.equal(2);
    });

    it("should update threshold via proposal", async function () {
      const { ms, owner, alice } = await deployFixture();
      // First add alice
      const addData = ms.interface.encodeFunctionData("addSigner", [alice.address]);
      await ms.propose(await ms.getAddress(), addData, 0);
      await ms.approve(0);
      await ms.execute(0);

      // Then set threshold to 2
      const threshData = ms.interface.encodeFunctionData("setThreshold", [2]);
      await ms.propose(await ms.getAddress(), threshData, 0);
      await ms.approve(1);
      await ms.execute(1);
      expect(await ms.threshold()).to.equal(2);
    });

    it("should reject removeSigner if it breaks threshold", async function () {
      const { ms, owner } = await deployFixture();
      const data = ms.interface.encodeFunctionData("removeSigner", [owner.address]);
      await ms.propose(await ms.getAddress(), data, 0);
      await ms.approve(0);
      await expect(ms.execute(0)).to.be.revertedWith("Multisig: call failed");
    });

    it("should reject direct addSigner call", async function () {
      const { ms, alice } = await deployFixture();
      await expect(ms.addSigner(alice.address)).to.be.revertedWith("Multisig: only via proposal");
    });
  });
});
