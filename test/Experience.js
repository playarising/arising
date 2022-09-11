const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("Civilizations", () => {
  before(async () => {
    const [owner, receiver] = await ethers.getSigners();

    this.owner = owner;
    this.receiver = receiver;

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
    await this.civ.mint(this.ard.address, {
      value: ethers.utils.parseEther("1"),
    });

    const Experience = await ethers.getContractFactory("Experience");
    this.experience = await Experience.deploy(levels.address, this.civ.address);
    await this.experience.deployed();
  });

  it("should deploy everything correctly", async () => {
    expect(await this.experience.owner()).to.eq(this.owner.address);
  });

  it("should assign the experience correctly", async () => {
    const id = this.civ.getTokenID(this.ard.address, 1);
    expect(await this.experience.getExperience(id)).to.eq(0);
    await this.experience.assignExperience(id, 1000);
    expect(await this.experience.getExperience(id)).to.eq(1000);
  });

  it("should return the level correctly", async () => {
    const id = this.civ.getTokenID(this.ard.address, 1);
    expect(await this.experience.getLevel(id)).to.eq(1);
    await this.experience.assignExperience(id, 1020);
    expect(await this.experience.getLevel(id)).to.eq(2);
  });

  it("should remove an authority and prevent from assigning experience from a non authority", async () => {
    const id = this.civ.getTokenID(this.ard.address, 1);
    await this.experience.removeAuthority(this.owner.address);
    await expect(this.experience.assignExperience(id, 1020)).to.revertedWith(
      "Experience: msg.sender is not authorized to assign experience"
    );
  });

  it("should prevent adding an authority from non owner", async () => {
    const id = this.civ.getTokenID(this.ard.address, 1);
    await expect(
      this.experience.connect(this.receiver).addAuthority(this.owner.address)
    ).to.revertedWith("Ownable: caller is not the owner");
  });
});
