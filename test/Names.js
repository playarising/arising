const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("Names", () => {
  before(async () => {
    const [owner, receiver, minter1, minter2] = await ethers.getSigners();

    this.owner = owner;
    this.minter1 = minter1;
    this.minter2 = minter2;

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

  it("should fail trying to replace an invalid name", async () => {
    const id = this.civ.getTokenID(this.ard.address, 1);
    await expect(this.names.replaceName(id, "trailing space ")).to.revertedWith(
      "Name: name trying to replace with is not valid."
    );
  });

  it("should fail trying to replace a name for a token with no name", async () => {
    const id2 = this.civ.getTokenID(this.ard.address, 2);
    await expect(
      this.names.replaceName(id2, "Gandalf the White")
    ).to.revertedWith("Name: can't replace name of token without a name.");
  });

  it("should fail trying to replace an already used name", async () => {
    const id = this.civ.getTokenID(this.ard.address, 1);
    const id2 = this.civ.getTokenID(this.ard.address, 2);
    await this.names.claimName(id2, "Gandalf the White");
    await expect(
      this.names.replaceName(id, "Gandalf the White")
    ).to.revertedWith("Name: name trying to replace with is already claimed.");
  });

  it("should replace a name correctly", async () => {
    const id = this.civ.getTokenID(this.ard.address, 1);
    expect(await this.names.getTokenName(id)).to.eq("Conan de Barbarian");
    await this.names.replaceName(id, "Radagast The Brown");
    expect(await this.names.getTokenName(id)).to.eq("Radagast The Brown");
    expect(await this.names.isNameAvailable("Conan de Barbarian")).to.eq(true);
  });

  it("should fail trying to clear a name from a token without name", async () => {
    const id = this.civ.getTokenID(this.ard.address, 3);
    await expect(this.names.clearName(id)).to.revertedWith(
      "Name: can't clear name of token without a name."
    );
  });

  it("should clear a name correctly", async () => {
    const id = this.civ.getTokenID(this.ard.address, 1);
    expect(await this.names.getTokenName(id)).to.eq("Radagast The Brown");
    await this.names.clearName(id);
    expect(await this.names.getTokenName(id)).to.eq("");
    expect(await this.names.isNameAvailable("Radagast The Brown")).to.eq(true);
  });

  it("should fail to assign a name from a non existing civilization and non minted token", async () => {
    await expect(
      this.names.claimName(
        "0x00000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000001",
        "Conan"
      )
    ).to.revertedWith("Civilizations: id of the civilization is not valid.");
    await expect(
      this.names.claimName(
        "0x00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000005",
        "Conan"
      )
    ).to.revertedWith("Names: can't get access to a non minted token.");
  });

  it("should fail to rename a token from a non allowed", async () => {
    const id = this.civ.getTokenID(this.ard.address, 1);
    await expect(
      this.names.connect(this.minter1).replaceName(id, "Conan")
    ).to.revertedWith("Names: msg.sender is not allowed to access this token.");
  });

  it("should be able to replace a name with approval and allowance", async () => {
    const id = this.civ.getTokenID(this.ard.address, 1);
    await this.ard.approve(this.minter1.address, 1);
    await this.ard.setApprovalForAll(this.minter2.address, true);
    expect(await this.names.getTokenName(id)).to.eq("");
    await this.names.connect(this.minter1).claimName(id, "Conan The Barb");
    expect(await this.names.getTokenName(id)).to.eq("Conan The Barb");
    await this.names
      .connect(this.minter2)
      .replaceName(id, "Conan The Barbarian");
    expect(await this.names.getTokenName(id)).to.eq("Conan The Barbarian");
  });
});
