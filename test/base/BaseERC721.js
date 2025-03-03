const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("BaseERC721", () => {
  before(async () => {
    const [owner, minter] = await ethers.getSigners();

    this.owner = owner;
    this.minter = minter;

    const MockEmitter = await ethers.getContractFactory("MockEmitter");
    this.mock = await MockEmitter.deploy();
    await this.mock.deployed();

    const BaseERC721 = await ethers.getContractFactory("BaseERC721");
    this.token = await BaseERC721.deploy();
    await this.token.deployed();
    await this.token.initialize(
      "Test",
      "TEST",
      "https://test.uri/",
      this.mock.address,
    );
  });

  it("should mint a token using the owner address", async () => {
    await this.token.mint(this.owner.address);
    expect(await this.token.balanceOf(this.owner.address)).to.eq(1);
  });

  it("should return the token URI correctly", async () => {
    expect(await this.token.tokenURI(1)).to.be.eq("https://test.uri/1");
  });

  it("should fail when trying to mint as a non owner", async () => {
    await expect(
      this.token.connect(this.minter).mint(this.minter.address),
    ).to.revertedWith(
      "BaseERC721: onlyAuthorized() msg.sender not authorized.",
    );
  });

  it("should check if a token exists", async () => {
    expect(await this.token.exists(1)).to.eq(true);
    expect(await this.token.exists(2)).to.eq(false);
  });

  it("should check the token allowance", async () => {
    await this.token.mint(this.owner.address);
    expect(await this.token.isApprovedOrOwner(this.owner.address, 1)).to.eq(
      true,
    );
    expect(await this.token.isApprovedOrOwner(this.minter.address, 1)).to.eq(
      false,
    );
    await this.token.approve(this.minter.address, 1);
    expect(await this.token.isApprovedOrOwner(this.minter.address, 1)).to.eq(
      true,
    );
    await this.token.setApprovalForAll(this.minter.address, true);
    expect(await this.token.isApprovedOrOwner(this.minter.address, 2)).to.eq(
      true,
    );
  });

  it("should prevent adding or removing an authority from non owner", async () => {
    await expect(
      this.token.connect(this.minter).addAuthority(this.owner.address),
    ).to.revertedWith("Ownable: caller is not the owner");
    await expect(
      this.token.connect(this.minter).removeAuthority(this.owner.address),
    ).to.revertedWith("Ownable: caller is not the owner");
  });

  it("should add a second authority and mint tokens correctly", async () => {
    await this.token.addAuthority(this.minter.address);
    await this.token.connect(this.minter).mint(this.minter.address);
    expect(await this.token.balanceOf(this.minter.address)).to.eq(1);
  });

  it("should remove the authority and prevent minting", async () => {
    await this.token.removeAuthority(this.minter.address);
    await expect(
      this.token.connect(this.minter).mint(this.minter.address),
    ).to.revertedWith(
      "BaseERC721: onlyAuthorized() msg.sender not authorized.",
    );
  });
});
