const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("Civilizations", () => {
  before(async () => {
    const [owner, receiver, authority] = await ethers.getSigners();

    this.owner = owner;
    this.receiver = receiver;
    this.authority = authority;

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
    await this.civ.setInitialized(true);

    await this.ard.transferOwnership(this.civ.address);
    await this.civ.mint(this.ard.address);
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

  it("should add a second authority and check missing experience for next level", async () => {
    const id = this.civ.getTokenID(this.ard.address, 1);
    await this.experience.addAuthority(this.owner.address);
    await this.experience.assignExperience(id, 500);
    const missing = await this.experience.getExperienceForNextLevel(id);
    expect(missing).to.eq(540);
    await this.experience.addAuthority(this.authority.address);
    await this.experience.connect(this.authority).assignExperience(id, 540);
    expect(await this.experience.getLevel(id)).to.eq(3);
  });

  it("should fail adding experience when token not minted and collection not available", async () => {
    await expect(
      this.experience.assignExperience(
        "0x00000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000001",
        500
      )
    ).to.revertedWith("Civilizations: id of the civilization is not valid.");
    await expect(
      this.experience.assignExperience(
        "0x00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000002",
        500
      )
    ).to.revertedWith(
      "Experience: can't assign experience to non minted token."
    );
  });
});
