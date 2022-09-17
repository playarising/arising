const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("Gadgets", () => {
  before(async () => {
    const [owner] = await ethers.getSigners();

    this.owner = owner;

    const MockToken = await ethers.getContractFactory("MockToken");
    this.mock = await MockToken.deploy(ethers.utils.parseEther("10"));
    await this.mock.deployed();

    const Refresher = await ethers.getContractFactory("Refresher");
    this.refresher = await Refresher.deploy(
      this.mock.address,
      ethers.utils.parseEther("1")
    );
    await this.refresher.deployed();

    const Vitalizer = await ethers.getContractFactory("Vitalizer");
    this.vitalizer = await Vitalizer.deploy(
      this.mock.address,
      ethers.utils.parseEther("1")
    );
    await this.vitalizer.deployed();
  });

  it("should deploy everything correctly", async () => {
    expect(await this.refresher.name()).to.eq("Arising: Refresh Token");
    expect(await this.vitalizer.name()).to.eq("Arising: Vitalizer Token");

    expect(await this.refresher.symbol()).to.eq("REFRESHER");
    expect(await this.vitalizer.symbol()).to.eq("VITALIZER");

    expect(await this.refresher.token()).to.eq(this.mock.address);
    expect(await this.vitalizer.token()).to.eq(this.mock.address);

    expect(await this.refresher.price()).to.eq(ethers.utils.parseEther("1"));
    expect(await this.vitalizer.price()).to.eq(ethers.utils.parseEther("1"));
  });
});
