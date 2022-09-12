const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("Names", () => {
  before(async () => {
    const [owner, receiver] = await ethers.getSigners();

    this.owner = owner;

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

    const Names = await ethers.getContractFactory("Names");
    this.names = await Names.deploy(this.civ.address);
    await this.names.deployed();
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

  it("should fail trying to claim an invalid name", async () => {
    const id = this.civ.getTokenID(this.ard.address, 1);
    await expect(this.names.claimName(id, "trailing space ")).to.revertedWith(
      "Name: name trying to claim is not valid."
    );
  });

  it("should claim a name for a token", async () => {
    const id = this.civ.getTokenID(this.ard.address, 1);
    expect(await this.names.getTokenName(id)).to.eq("");
    await this.names.claimName(id, "Conan de Barbarian");
    expect(await this.names.getTokenName(id)).to.eq("Conan de Barbarian");
  });

  it("should fail trying to claim a name already claimed for a token", async () => {
    const id = this.civ.getTokenID(this.ard.address, 2);
    await expect(
      this.names.claimName(id, "Conan de Barbarian")
    ).to.revertedWith("Name: name trying to claim is already claimed.");
  });

  it("should fail to claim a name for a token with a name already claimed", async () => {
    const id = this.civ.getTokenID(this.ard.address, 1);
    await expect(this.names.claimName(id, "Conan")).to.revertedWith(
      "Name: token already have a name."
    );
  });
});
