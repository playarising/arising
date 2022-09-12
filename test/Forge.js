const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("Forge", () => {
  before(async () => {
    const [owner, receiver] = await ethers.getSigners();

    this.owner = owner;

    const Levels = await ethers.getContractFactory("Levels");
    const levels = await Levels.deploy();
    await levels.deployed();

    const Ard = await ethers.getContractFactory("Ard");
    this.ard = await Ard.deploy();
    await this.ard.deployed();

    const Civilizations = await ethers.getContractFactory("Civilizations");
    this.civ = await Civilizations.deploy(
      ethers.utils.parseEther("1"),
      receiver.address
    );
    await this.civ.deployed();
    await this.civ.addCivilization(this.ard.address);
    await this.civ.setInitialized();

    await this.ard.transferOwnership(this.civ.address);
    await this.civ.mint(this.ard.address, {
      value: ethers.utils.parseEther("1"),
    });
    const Experience = await ethers.getContractFactory("Experience");
    this.experience = await Experience.deploy(levels.address, this.civ.address);
    await this.experience.deployed();

    const Forge = await ethers.getContractFactory("Forge");
    this.forge = await Forge.deploy(this.civ.address, this.experience.address);
    await this.forge.deployed();
  });

  it("should deploy everything correctly", async () => {
    expect(await this.forge.owner()).to.eq(this.owner.address);
  });
});
