const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("Experience", () => {
  before(async () => {
    const [owner, receiver, authority] = await ethers.getSigners();

    this.owner = owner;
    this.receiver = receiver;
    this.authority = authority;

    const Levels = await ethers.getContractFactory("Levels");
    const levels = await Levels.deploy();
    await levels.deployed();

    const BaseERC721 = await ethers.getContractFactory("BaseERC721");
    this.collection = await BaseERC721.deploy(
      "Test Collection",
      "TEST",
      "https://test.uri/"
    );
    await this.collection.deployed();

    const Civilizations = await ethers.getContractFactory("Civilizations");
    this.civ = await Civilizations.deploy(receiver.address);
    await this.civ.deployed();
    await this.civ.addCivilization(this.collection.address);

    await this.collection.transferOwnership(this.civ.address);
    await this.civ.mint(1);
    const Experience = await ethers.getContractFactory("Experience");
    this.experience = await Experience.deploy(levels.address, this.civ.address);
    await this.experience.deployed();
  });

  it("should deploy everything correctly", async () => {
    expect(await this.experience.owner()).to.eq(this.owner.address);
  });

  it("should assign the experience correctly", async () => {
    const id = this.civ.getTokenID(1, 1);
    expect(await this.experience.getExperience(id)).to.eq(0);
    await this.experience.assignExperience(id, 1000);
    expect(await this.experience.getExperience(id)).to.eq(1000);
  });

  it("should return the level correctly", async () => {
    const id = this.civ.getTokenID(1, 1);
    expect(await this.experience.getLevel(id)).to.eq(1);
    await this.experience.assignExperience(id, 1020);
    expect(await this.experience.getLevel(id)).to.eq(2);
  });

  it("should remove an authority and prevent from assigning experience from a non authority", async () => {
    const id = this.civ.getTokenID(1, 1);
    await this.experience.removeAuthority(this.owner.address);
    await expect(this.experience.assignExperience(id, 1020)).to.revertedWith(
      "Experience: onlyAuthorized() msg.sender not authorized."
    );
  });

  it("should prevent adding an authority from non owner", async () => {
    const id = this.civ.getTokenID(1, 1);
    await expect(
      this.experience.connect(this.receiver).addAuthority(this.owner.address)
    ).to.revertedWith("Ownable: caller is not the owner");
  });

  it("should add a second authority and check missing experience for next level", async () => {
    const id = this.civ.getTokenID(1, 1);
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
    ).to.revertedWith("Civilizations: exists() invalid civilization id.");
    await expect(
      this.experience.assignExperience(
        "0x00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000002",
        500
      )
    ).to.revertedWith("Experience: assignExperience() token not minted.");
  });
});