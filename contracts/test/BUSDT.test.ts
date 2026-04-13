import { expect } from "chai";
import { ethers } from "hardhat";

describe("BUSDT", function () {
  async function deployFixture() {
    const [deployer, alice] = await ethers.getSigners();
    const BUSDT = await ethers.getContractFactory("BUSDT");
    const busdt = await BUSDT.deploy();
    return { busdt, deployer, alice };
  }

  it("should have correct name, symbol, decimals", async function () {
    const { busdt } = await deployFixture();
    expect(await busdt.name()).to.equal("Biyard USD Tether");
    expect(await busdt.symbol()).to.equal("BUSDT");
    expect(await busdt.decimals()).to.equal(6);
  });

  it("should allow anyone to mint", async function () {
    const { busdt, alice } = await deployFixture();
    const amount = ethers.parseUnits("1000000", 6);
    await busdt.connect(alice).mint(alice.address, amount);
    expect(await busdt.balanceOf(alice.address)).to.equal(amount);
  });

  it("should allow minting to other addresses", async function () {
    const { busdt, deployer, alice } = await deployFixture();
    const amount = ethers.parseUnits("500", 6);
    await busdt.connect(deployer).mint(alice.address, amount);
    expect(await busdt.balanceOf(alice.address)).to.equal(amount);
  });

  it("should allow multiple mints", async function () {
    const { busdt, alice } = await deployFixture();
    const amt = ethers.parseUnits("100", 6);
    await busdt.connect(alice).mint(alice.address, amt);
    await busdt.connect(alice).mint(alice.address, amt);
    expect(await busdt.balanceOf(alice.address)).to.equal(amt * 2n);
  });
});
