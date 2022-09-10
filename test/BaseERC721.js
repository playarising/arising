const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("BaseERC721", () => {
  before(async () => {
    const [owner, minter] = await ethers.getSigners();

    this.owner = owner;
    this.minter = minter;

    const BaseERC721 = await ethers.getContractFactory("BaseERC721");
    this.token = await BaseERC721.deploy(
      "Test",
      "TEST",
      "https://test.uri/",
      5
    );
    await this.token.deployed();
  });

  it("should mint a token using the owner address", async () => {
    await this.token.mint(this.owner.address);
    expect(await this.token.balanceOf(this.owner.address)).to.eq(1);
  });

  it("should check the token allowance", async () => {
    expect(await this.token.isApprovedOrOwner(this.owner.address, 1)).to.eq(
      true
    );
    expect(await this.token.isApprovedOrOwner(this.minter.address, 1)).to.eq(
      false
    );
    await this.token.approve(this.minter.address, 1);
    expect(await this.token.isApprovedOrOwner(this.minter.address, 1)).to.eq(
      true
    );
  });

  it("should return the token URI correctly", async () => {
    expect(await this.token.tokenURI(1)).to.be.eq("https://test.uri/1");
  });

  it("should fail when trying to mint as a non owner", async () => {
    await expect(
      this.token.connect(this.minter).mint(this.minter.address)
    ).to.revertedWith("Ownable: caller is not the owner");
  });

  it("should mint until the cap is reached", async () => {
    await this.token.mint(this.owner.address);
    await this.token.mint(this.owner.address);
    await this.token.mint(this.owner.address);
    await this.token.mint(this.owner.address);
    await expect(this.token.mint(this.owner.address)).to.revertedWith(
      "BaseERC721: Max supply reached."
    );
  });
});
