const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("Names", () => {
  before(async () => {
    const [owner, receiver, minter1, minter2] = await ethers.getSigners();

    this.owner = owner;
    this.minter1 = minter1;
    this.minter2 = minter2;

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
    await this.civ.mint(1);
    await this.civ.mint(1);

    const Levels = await ethers.getContractFactory("Levels");
    const levels = await Levels.deploy();
    await levels.deployed();

    const Experience = await ethers.getContractFactory("Experience");
    this.experience = await Experience.deploy(levels.address, this.civ.address);
    await this.experience.deployed();

    const Names = await ethers.getContractFactory("Names");
    this.names = await Names.deploy(this.civ.address, this.experience.address);
    await this.names.deployed();
  });

  it("should not be able to claim a name when paused", async () => {
    const id = this.civ.getTokenID(1, 1);
    expect(await this.names.paused()).to.eq(false);
    await this.names.pause();
    expect(await this.names.paused()).to.eq(true);
    await expect(this.names.claimName(id, "Hello")).to.revertedWith(
      "Pausable: paused"
    );
    await this.names.unpause();
    expect(await this.names.paused()).to.eq(false);
  });

  it("should not be able pause when not owner", async () => {
    await expect(this.names.connect(this.minter1).pause()).to.revertedWith(
      "Ownable: caller is not the owner"
    );
    await expect(this.names.connect(this.minter1).unpause()).to.revertedWith(
      "Ownable: caller is not the owner"
    );
  });

  it("should validate names correctly", async () => {
    expect(await this.names.isNameValid("conan")).to.eq(true);
    expect(await this.names.isNameValid("Conan")).to.eq(true);
    expect(await this.names.isNameValid("Conan the Barbarian")).to.eq(true);
    expect(await this.names.isNameValid("abcdefghijklmnopqrstuvw")).to.eq(true);
    expect(await this.names.isNameValid("xyz0123456789ABCDEFGHIJ")).to.eq(true);
    expect(await this.names.isNameValid("KLMNOPQRSTUVWXYZ")).to.eq(true);
    expect(await this.names.isNameValid("this is 25 characters aaa")).to.eq(
      true
    );
    expect(await this.names.isNameValid("1")).to.eq(true);

    expect(await this.names.isNameValid("this is 26 characters aaaa")).to.eq(
      false
    );
    expect(await this.names.isNameValid("")).to.eq(false);
    expect(await this.names.isNameValid("use  two space")).to.eq(false);
    expect(await this.names.isNameValid(" leading space")).to.eq(false);
    expect(await this.names.isNameValid("trailing space ")).to.eq(false);
    expect(await this.names.isNameValid("cönan")).to.eq(false);
  });

  it("should convert names to lowercase correctly", async () => {
    expect(await this.names.toLowerCase("CoNaN tH3 B4rb")).to.eq(
      "conan th3 b4rb"
    );
  });

  it("should fail trying to claim a name from a non level 5 character", async () => {
    const id = this.civ.getTokenID(1, 1);
    await expect(
      this.names.claimName(id, "Conan de Barbarian")
    ).to.revertedWith("Name: claimName() not enough level.");
  });

  it("should fail trying to replacing a name from a non level 5 character", async () => {
    const id = this.civ.getTokenID(1, 1);
    await expect(
      this.names.replaceName(id, "Conan de Barbarian")
    ).to.revertedWith("Name: replaceName() not enough level.");
  });

  it("should fail trying to claim an invalid name", async () => {
    const id = this.civ.getTokenID(1, 1);
    await this.experience.assignExperience(id, 20000);
    await expect(this.names.claimName(id, "trailing space ")).to.revertedWith(
      "Name: claimName() invalid name."
    );
  });

  it("should claim a name for a token", async () => {
    const id = this.civ.getTokenID(1, 1);
    expect(await this.names.getCharacterName(id)).to.eq("");
    await this.names.claimName(id, "Conan de Barbarian");
    expect(await this.names.getCharacterName(id)).to.eq("Conan de Barbarian");
  });

  it("should fail trying to claim a name already claimed for a token", async () => {
    const id = this.civ.getTokenID(1, 2);
    await this.experience.assignExperience(id, 20000);
    await expect(
      this.names.claimName(id, "Conan de Barbarian")
    ).to.revertedWith("Name: claimName() name not available.");
  });

  it("should fail to claim a name for a token with a name already claimed", async () => {
    const id = this.civ.getTokenID(1, 1);
    await expect(this.names.claimName(id, "Conan")).to.revertedWith(
      "Name: claimName() already named."
    );
  });

  it("should fail trying to replace an invalid name", async () => {
    const id = this.civ.getTokenID(1, 1);
    await expect(this.names.replaceName(id, "trailing space ")).to.revertedWith(
      "Name: replaceName() invalid name."
    );
  });

  it("should fail trying to replace a name for a token with no name", async () => {
    const id2 = this.civ.getTokenID(1, 2);
    await expect(
      this.names.replaceName(id2, "Gandalf the White")
    ).to.revertedWith("Name: replaceName() no name assigned.");
  });

  it("should fail trying to replace an already used name", async () => {
    const id = this.civ.getTokenID(1, 1);
    const id2 = this.civ.getTokenID(1, 2);
    await this.names.claimName(id2, "Gandalf the White");
    await expect(
      this.names.replaceName(id, "Gandalf the White")
    ).to.revertedWith("Name: replaceName() name not available.");
  });

  it("should replace a name correctly", async () => {
    const id = this.civ.getTokenID(1, 1);
    expect(await this.names.getCharacterName(id)).to.eq("Conan de Barbarian");
    await this.names.replaceName(id, "Radagast The Brown");
    expect(await this.names.getCharacterName(id)).to.eq("Radagast The Brown");
    expect(await this.names.isNameAvailable("Conan de Barbarian")).to.eq(true);
  });

  it("should fail trying to clear a name from a token without name", async () => {
    const id = this.civ.getTokenID(1, 3);
    await expect(this.names.clearName(id)).to.revertedWith(
      "Name: clearName() no name assigned."
    );
  });

  it("should clear a name correctly", async () => {
    const id = this.civ.getTokenID(1, 1);
    expect(await this.names.getCharacterName(id)).to.eq("Radagast The Brown");
    await this.names.clearName(id);
    expect(await this.names.getCharacterName(id)).to.eq("");
    expect(await this.names.isNameAvailable("Radagast The Brown")).to.eq(true);
  });

  it("should fail to rename a token from a non allowed", async () => {
    const id = this.civ.getTokenID(1, 1);
    await expect(
      this.names.connect(this.minter1).replaceName(id, "Conan")
    ).to.revertedWith(
      "Names: onlyAllowed() msg.sender is not allowed to access this token."
    );
  });

  it("should be able to replace a name with approval and allowance", async () => {
    const id = this.civ.getTokenID(1, 1);
    await this.collection.approve(this.minter1.address, 1);
    await this.collection.setApprovalForAll(this.minter2.address, true);
    expect(await this.names.getCharacterName(id)).to.eq("");
    await this.names.connect(this.minter1).claimName(id, "Conan The Barb");
    expect(await this.names.getCharacterName(id)).to.eq("Conan The Barb");
    await this.names
      .connect(this.minter2)
      .replaceName(id, "Conan The Barbarian");
    expect(await this.names.getCharacterName(id)).to.eq("Conan The Barbarian");
  });
});