const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("BaseFungibleItems", () => {
  before(async () => {
    const [owner, minter, minter2] = await ethers.getSigners();

    this.owner = owner;
    this.minter = minter;
    this.minter2 = minter2;

    const Ard = await ethers.getContractFactory("Ard");
    this.ard = await Ard.deploy();
    await this.ard.deployed();
    const Civilizations = await ethers.getContractFactory("Civilizations");
    this.civ = await Civilizations.deploy(this.owner.address);
    await this.civ.deployed();

    await this.civ.addCivilization(this.ard.address);
    await this.ard.transferOwnership(this.civ.address);

    await this.civ.mint(1);

    const BaseFungibleItem = await ethers.getContractFactory(
      "BaseFungibleItem"
    );
    this.token = await BaseFungibleItem.deploy(
      "Test",
      "TEST",
      "https://test.test",
      this.civ.address
    );
    await this.token.deployed();
  });

  it("should deploy everything correctly", async () => {
    expect(await this.token.name()).to.eq("Test");
    expect(await this.token.symbol()).to.eq("TEST");
    expect(await this.token.image()).to.eq("https://test.test");
  });

  it("should add and remove tokens", async () => {
    const id = await this.civ.getTokenID(1, 1);
    await this.token.mintTo(id, 10);
    expect(await this.token.balanceOf(id)).to.eq(10);
    await this.token.consume(id, 5);
    expect(await this.token.balanceOf(id)).to.eq(5);
  });

  it("should fail to mint tokens when not owner", async () => {
    const id = await this.civ.getTokenID(1, 1);
    await expect(
      this.token.connect(this.minter).mintTo(id, 10)
    ).to.revertedWith("Ownable: caller is not the owner");
  });

  it("should fail consume when not allowed", async () => {
    const id = await this.civ.getTokenID(1, 1);
    await expect(
      this.token.connect(this.minter).consume(id, 5)
    ).to.revertedWith(
      "BaseFungibleItem: onlyAllowed() msg.sender is not allowed to access this token."
    );
  });

  it("should be able consume when allowance assigned", async () => {
    const id = await this.civ.getTokenID(1, 1);
    await this.ard.approve(this.minter.address, 1);
    await this.token.connect(this.minter).consume(id, 1);
    expect(await this.token.balanceOf(id)).to.eq(4);

    await this.ard.setApprovalForAll(this.minter2.address, true);
    await this.token.connect(this.minter2).consume(id, 1);
    expect(await this.token.balanceOf(id)).to.eq(3);
  });

  it("should fail to consume when no enough balance", async () => {
    const id = await this.civ.getTokenID(1, 1);
    await this.token.consume(id, 3);
    await expect(this.token.consume(id, 1)).to.revertedWith(
      "BaseFungibleItem: consume() not enough balance."
    );
  });
});
