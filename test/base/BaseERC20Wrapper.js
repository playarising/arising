const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("BaseERC20Wrapper", () => {
  before(async () => {
    const [owner, minter] = await ethers.getSigners();

    this.owner = owner;
    this.minter = minter;

    const BaseERC20Wrapper =
      await ethers.getContractFactory("BaseERC20Wrapper");
    this.token = await BaseERC20Wrapper.deploy("Test", "TEST");
    await this.token.deployed();
  });

  it("should mint a token using the owner address", async () => {
    await this.token.mint(this.owner.address, ethers.utils.parseEther("1"));
    expect(await this.token.balanceOf(this.owner.address)).to.eq(
      ethers.utils.parseEther("1"),
    );
  });

  it("should return the name and symbol correctly", async () => {
    expect(await this.token.name()).to.be.eq("Test");
    expect(await this.token.symbol()).to.be.eq("TEST");
  });

  it("should fail when trying to mint as a non owner", async () => {
    await expect(
      this.token
        .connect(this.minter)
        .mint(this.minter.address, ethers.utils.parseEther("1")),
    ).to.revertedWith("Ownable: caller is not the owner");
  });
});
