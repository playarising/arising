const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("RefresherToken", () => {
  before(async () => {
    const [owner, minter] = await ethers.getSigners();

    this.owner = owner;
    this.minter = minter;

    const MockToken = await ethers.getContractFactory("MockToken");
    this.mock = await MockToken.deploy(ethers.utils.parseEther("10"));
    await this.mock.deployed();

    const RefresherToken = await ethers.getContractFactory("RefresherToken");
    this.token = await RefresherToken.deploy(this.mock.address, 1);
    await this.token.deployed();
  });

  it("should deploy everything correctly", async () => {
    expect(await this.token.owner()).to.eq(this.owner.address);
    expect(await this.token.decimals()).to.eq(0);
    expect(await this.mock.balanceOf(this.owner.address)).to.eq(
      ethers.utils.parseEther("10")
    );
  });

  it("should be able to mint tokens", async () => {
    expect(await this.mock.balanceOf(this.minter.address)).to.eq(0);
    await this.mock.transfer(this.minter.address, ethers.utils.parseEther("5"));
    expect(await this.mock.balanceOf(this.minter.address)).to.eq(
      ethers.utils.parseEther("5")
    );
    expect(await this.mock.balanceOf(this.owner.address)).to.eq(
      ethers.utils.parseEther("5")
    );
    expect(await this.token.balanceOf(this.minter.address)).to.eq(0);
    await this.mock
      .connect(this.minter)
      .approve(this.token.address, ethers.utils.parseEther("5"));
    await this.token.connect(this.minter).mint(5);
    expect(await this.token.balanceOf(this.minter.address)).to.eq(5);
  });

  it("should not be able to withdraw from non owner", async () => {
    await expect(this.token.connect(this.minter).withdraw()).to.revertedWith(
      "Ownable: caller is not the owner"
    );
  });

  it("should be able to withdraw the tokens by the owner", async () => {
    expect(await this.mock.balanceOf(this.owner.address)).to.eq(
      ethers.utils.parseEther("5")
    );
    expect(await this.mock.balanceOf(this.token.address)).to.eq(
      ethers.utils.parseEther("5")
    );
    await this.token.withdraw();
    expect(await this.mock.balanceOf(this.owner.address)).to.eq(
      ethers.utils.parseEther("10")
    );
    expect(await this.mock.balanceOf(this.token.address)).to.eq(
      ethers.utils.parseEther("0")
    );
  });
});
