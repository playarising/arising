const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("BaseGadgetToken", () => {
  before(async () => {
    const [owner, minter] = await ethers.getSigners();

    this.owner = owner;
    this.minter = minter;

    const MockToken = await ethers.getContractFactory("MockToken");
    this.mock = await MockToken.deploy(ethers.utils.parseEther("10"));
    await this.mock.deployed();

    this.mock2 = await MockToken.deploy(ethers.utils.parseEther("10"));
    await this.mock2.deployed();

    const BaseGadgetToken = await ethers.getContractFactory("BaseGadgetToken");
    this.token = await BaseGadgetToken.deploy();

    await this.token.deployed();
    await this.token.initialize(
      "Test",
      "TEST",
      this.mock.address,
      ethers.utils.parseEther("1"),
    );
  });

  it("should deploy everything correctly", async () => {
    expect(await this.token.owner()).to.eq(this.owner.address);
    expect(await this.token.decimals()).to.eq(0);
    expect(await this.token.price()).to.eq(ethers.utils.parseEther("1"));
    expect(await this.token.token()).to.eq(this.mock.address);
    expect(await this.mock.balanceOf(this.owner.address)).to.eq(
      ethers.utils.parseEther("10"),
    );
  });

  it("should not be able to mint tokens with no balance of payment token", async () => {
    await expect(this.token.connect(this.minter).mint(5)).to.revertedWith(
      "BaseGadgetToken: mint() not enough balance to mint tokens.",
    );
  });

  it("should return the total cost correctly", async () => {
    expect(await this.token.getTotalCost(1)).to.eq(
      ethers.utils.parseEther("1"),
    );
    expect(await this.token.getTotalCost(2)).to.eq(
      ethers.utils.parseEther("2"),
    );
    expect(await this.token.getTotalCost(4)).to.eq(
      ethers.utils.parseEther("4"),
    );
    expect(await this.token.getTotalCost(10)).to.eq(
      ethers.utils.parseEther("10"),
    );
  });

  it("should not be able to mint tokens for not enough allowance", async () => {
    expect(await this.mock.balanceOf(this.minter.address)).to.eq(0);
    await this.mock.transfer(this.minter.address, ethers.utils.parseEther("5"));
    expect(await this.mock.balanceOf(this.minter.address)).to.eq(
      ethers.utils.parseEther("5"),
    );
    await expect(this.token.connect(this.minter).mint(5)).to.revertedWith(
      "BaseGadgetToken: mint() not enough allowance to mint tokens.",
    );
  });

  it("should be able to mint tokens", async () => {
    expect(await this.token.balanceOf(this.minter.address)).to.eq(0);
    await this.mock
      .connect(this.minter)
      .approve(this.token.address, ethers.utils.parseEther("5"));
    await this.token.connect(this.minter).mint(5);
    expect(await this.token.balanceOf(this.minter.address)).to.eq(5);
  });

  it("should not be able to mint tokens when paused", async () => {
    expect(await this.token.paused()).to.eq(false);
    await this.token.pause();
    expect(await this.token.paused()).to.eq(true);
    await expect(this.token.mint(5)).to.revertedWith("Pausable: paused");
    await expect(this.token.mintFree(this.minter.address, 5)).to.revertedWith(
      "Pausable: paused",
    );
    await this.token.unpause();
    expect(await this.token.paused()).to.eq(false);
  });

  it("should not be able pause when not owner", async () => {
    await expect(this.token.connect(this.minter).pause()).to.revertedWith(
      "Ownable: caller is not the owner",
    );
    await expect(this.token.connect(this.minter).unpause()).to.revertedWith(
      "Ownable: caller is not the owner",
    );
  });

  it("should not be able to set price from non owner", async () => {
    await expect(
      this.token.connect(this.minter).setPrice(ethers.utils.parseEther("5")),
    ).to.revertedWith("Ownable: caller is not the owner");
  });

  it("should not be able to set payment token from non owner", async () => {
    await expect(
      this.token.connect(this.minter).setToken(this.mock.address),
    ).to.revertedWith("Ownable: caller is not the owner");
  });

  it("should be able to set a new price", async () => {
    await this.token.setPrice(ethers.utils.parseEther("5"));
    expect(await this.token.price()).to.eq(ethers.utils.parseEther("5"));
  });

  it("should be able to set a new token", async () => {
    await this.token.setToken(this.mock2.address);
    expect(await this.token.token()).to.eq(this.mock2.address);
  });

  it("should be able to mint tokens for free", async () => {
    await this.token.mintFree(this.owner.address, 5);
    expect(await this.token.balanceOf(this.owner.address)).to.eq(5);
  });

  it("should fail when try to mint for free from non owner", async () => {
    await expect(
      this.token.connect(this.minter).mintFree(this.minter.address, 1),
    ).to.revertedWith("Ownable: caller is not the owner");
  });
});
