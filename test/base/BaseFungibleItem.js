const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("BaseFungibleItems", () => {
  before(async () => {
    const [owner, minter, minter2] = await ethers.getSigners();

    this.owner = owner;
    this.minter = minter;
    this.minter2 = minter2;

    const MockToken = await ethers.getContractFactory("MockToken");
    this.mock = await MockToken.deploy(ethers.utils.parseEther("100"));
    await this.mock.deployed();

    const Civilizations = await ethers.getContractFactory("Civilizations");
    this.civ = await Civilizations.deploy();
    await this.civ.deployed();
    await this.civ.initialize(this.mock.address);

    const BaseERC721 = await ethers.getContractFactory("BaseERC721");
    this.collection = await BaseERC721.deploy();
    await this.collection.deployed();
    await this.collection.initialize(
      "Test Collection",
      "TEST",
      "https://test.uri/",
      this.civ.address,
    );

    await this.collection.addAuthority(this.civ.address);

    await this.civ.addCivilization(this.collection.address);

    await this.civ.mint(1);

    const BaseFungibleItem =
      await ethers.getContractFactory("BaseFungibleItem");
    this.token = await BaseFungibleItem.deploy();
    await this.token.deployed();
    await this.token.initialize("Test", "TEST", this.civ.address);
    const BaseERC20Wrapper =
      await ethers.getContractFactory("BaseERC20Wrapper");
    this.wrapped_token = BaseERC20Wrapper.attach(await this.token.wrapper());
  });

  it("should deploy everything correctly", async () => {
    expect(await this.token.name()).to.eq("Test");
    expect(await this.token.symbol()).to.eq("TEST");
  });

  it("should add and remove tokens", async () => {
    const id = await this.civ.getTokenID(1, 1);
    await this.token.mintTo(id, 10);
    expect(await this.token.balanceOf(id)).to.eq(10);
    await this.token.consume(id, 5);
    expect(await this.token.balanceOf(id)).to.eq(5);
  });

  it("should fail to mint tokens when not authorized", async () => {
    const id = await this.civ.getTokenID(1, 1);
    await expect(
      this.token.connect(this.minter).mintTo(id, 10),
    ).to.revertedWith(
      "BaseFungibleItem: onlyAuthorized() msg.sender not authorized.",
    );
  });

  it("should fail consume when not allowed", async () => {
    const id = await this.civ.getTokenID(1, 1);
    await expect(
      this.token.connect(this.minter).consume(id, 5),
    ).to.revertedWith(
      "BaseFungibleItem: onlyAllowed() msg.sender is not allowed to access this token.",
    );
  });

  it("should be able consume when allowance assigned", async () => {
    const id = await this.civ.getTokenID(1, 1);
    await this.collection.approve(this.minter.address, 1);
    await this.token.connect(this.minter).consume(id, 1);
    expect(await this.token.balanceOf(id)).to.eq(4);

    await this.collection.setApprovalForAll(this.minter2.address, true);
    await this.token.connect(this.minter2).consume(id, 1);
    expect(await this.token.balanceOf(id)).to.eq(3);
  });

  it("should fail to consume when no enough balance", async () => {
    const id = await this.civ.getTokenID(1, 1);
    await this.token.consume(id, 3);
    await expect(this.token.consume(id, 1)).to.revertedWith(
      "BaseFungibleItem: consume() not enough balance.",
    );
  });

  it("should fail to wrap and unwrap when not enabled", async () => {
    const id = await this.civ.getTokenID(1, 1);
    await expect(this.token.wrap(id, 5)).to.revertedWith(
      "BaseFungibleItem: onlyEnabled() wrap is not enabled.",
    );
  });

  it("should fail to enable wrap when not owner", async () => {
    const id = await this.civ.getTokenID(1, 1);
    await expect(
      this.token.connect(this.minter).setWrapFunction(true),
    ).to.revertedWith("Ownable: caller is not the owner");
  });

  it("should remove an authority and prevent from assigning experience from a non authority", async () => {
    const id = await this.civ.getTokenID(1, 1);
    await this.token.removeAuthority(this.owner.address);
    await expect(this.token.mintTo(id, 1020)).to.revertedWith(
      "BaseFungibleItem: onlyAuthorized() msg.sender not authorized.",
    );
  });

  it("should prevent adding an authority from non owner", async () => {
    await expect(
      this.token.connect(this.minter).addAuthority(this.owner.address),
    ).to.revertedWith("Ownable: caller is not the owner");
  });

  it("should add a second authority and mint items", async () => {
    const id = await this.civ.getTokenID(1, 1);
    await this.token.addAuthority(this.owner.address);
    await this.token.mintTo(id, 500);
    const balance = await this.token.balanceOf(id);
    expect(balance).to.eq(500);
    await this.token.addAuthority(this.minter.address);
    await this.token.connect(this.minter).mintTo(id, 540);
    expect(await this.token.balanceOf(id)).to.eq(1040);
  });

  it("should wrap and unwrap tokens correctly", async () => {
    const id = await this.civ.getTokenID(1, 1);
    expect(await this.wrapped_token.balanceOf(this.owner.address)).to.eq(0);
    await this.token.setWrapFunction(true);
    expect(await this.token.balanceOf(id)).to.eq(1040);
    await this.token.wrap(id, 5);
    expect(await this.wrapped_token.balanceOf(this.owner.address)).to.eq(
      ethers.utils.parseEther("5"),
    );
    expect(await this.token.balanceOf(id)).to.eq(1035);
    await this.wrapped_token.approve(
      this.token.address,
      ethers.utils.parseEther("10"),
    );
    await this.token.unwrap(id, 5);
    expect(await this.wrapped_token.balanceOf(this.owner.address)).to.eq(0);
    expect(await this.token.balanceOf(id)).to.eq(1040);
  });
});
