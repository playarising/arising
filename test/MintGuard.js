const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("MintGuard", () => {
  before(async () => {
    const [owner, receiver, minter] = await ethers.getSigners();

    this.owner = owner;
    this.minter = minter;

    const MintGuard = await ethers.getContractFactory("MintGuard");
    this.guard = await MintGuard.deploy();
    await this.guard.deployed();

    const BaseERC721 = await ethers.getContractFactory("BaseERC721");
    this.token = await BaseERC721.deploy(
      "Test",
      "TEST",
      this.guard.address,
      "https://test.uri/",
      5,
      receiver.address
    );
    await this.token.deployed();

    await this.token.setPrice("10");
    await this.token.setInitialized();
    const MockMinter = await ethers.getContractFactory("MockMinter");
    this.mock = await MockMinter.deploy(this.guard.address, this.token.address);

    await this.mock.deployed();
  });

  it("should return the list of protected contracts empty", async () => {
    await expect(await this.guard.getProtected()).to.be.empty;
  });

  it("should attempt to add a protected contract from no owner", async () => {
    await expect(
      this.guard.connect(this.minter).addProtected(this.mock.address)
    ).to.revertedWith("Ownable: caller is not the owner");
  });

  it("should fail to mark a minter from a non protected contract", async () => {
    await expect(this.mock.mintMock()).to.revertedWith(
      "MintGuard: contract is not defined on the list of protected contracts"
    );
  });

  it("should add a protected contract", async () => {
    await this.guard.addProtected(this.mock.address);
    expect(await this.guard.getProtected()).to.include(this.mock.address);
  });

  it("should mark minter as minted", async () => {
    expect(await this.guard.hasMinted(this.minter.address)).to.be.eq(false);
    await this.mock.connect(this.minter).mintMock();
    await this.mock.connect(this.minter).mintMock();
    await this.mock.connect(this.minter).mintMock();
    await this.mock.connect(this.minter).mintMock();
    await this.mock.connect(this.minter).mintMock();
    expect(await this.guard.hasMinted(this.minter.address)).to.be.eq(true);
  });

  it("should fail when trying to mint through a contract", async () => {
    await expect(this.mock.testMint()).to.revertedWith(
      "MintGuard: cannot mint from a contract"
    );
  });
});
