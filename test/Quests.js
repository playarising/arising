const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("Quests", () => {
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
    this.civ = await Civilizations.deploy(receiver.address);
    await this.civ.deployed();
    await this.civ.addCivilization(this.ard.address);
    await this.civ.setInitialized();

    await this.ard.transferOwnership(this.civ.address);
    await this.civ.mint(this.ard.address);
    const Experience = await ethers.getContractFactory("Experience");
    this.experience = await Experience.deploy(levels.address, this.civ.address);
    await this.experience.deployed();

    const Quests = await ethers.getContractFactory("Quests");
    this.quests = await Quests.deploy(
      this.civ.address,
      this.experience.address
    );
    await this.quests.deployed();
  });

  it("should deploy everything correctly", async () => {
    expect(await this.quests.owner()).to.eq(this.owner.address);
  });
});
