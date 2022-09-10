const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("Civilizations", () => {
  before(async () => {
    const [owner] = await ethers.getSigners();

    this.owner = owner;

    const Ard = await ethers.getContractFactory("Ard");
    this.ard = await Ard.deploy();
    await this.ard.deployed();

    const Zhand = await ethers.getContractFactory("Zhand");
    this.zhand = await Zhand.deploy();
    await this.zhand.deployed();

    const IKarans = await ethers.getContractFactory("IKarans");
    this.ikarans = await IKarans.deploy();
    await this.ikarans.deployed();

    const Tarki = await ethers.getContractFactory("Tarki");
    this.tarki = await Tarki.deploy();
    await this.tarki.deployed();

    const Hartheim = await ethers.getContractFactory("Hartheim");
    this.hartheim = await Hartheim.deploy();
    await this.hartheim.deployed();

    const Shinkari = await ethers.getContractFactory("Shinkari");
    this.shinkari = await Shinkari.deploy();
    await this.shinkari.deployed();
  });

  it("should deploy everything correctly", async () => {
    expect(await this.ard.name()).to.be.eq("Arising: Ard");
    expect(await this.zhand.name()).to.be.eq("Arising: Zhand");
    expect(await this.ikarans.name()).to.be.eq("Arising: I'Karans");
    expect(await this.tarki.name()).to.be.eq("Arising: Tark'i");
    expect(await this.hartheim.name()).to.be.eq("Arising: Hartheim");
    expect(await this.shinkari.name()).to.be.eq("Arising: Shinkari");

    expect(await this.ard.symbol()).to.be.eq("ARISING");
    expect(await this.zhand.symbol()).to.be.eq("ARISING");
    expect(await this.ikarans.symbol()).to.be.eq("ARISING");
    expect(await this.tarki.symbol()).to.be.eq("ARISING");
    expect(await this.hartheim.symbol()).to.be.eq("ARISING");
    expect(await this.shinkari.symbol()).to.be.eq("ARISING");

    expect(await this.ard.baseURI()).to.be.eq(
      "https://characters.playarising.com/ard/"
    );
    expect(await this.zhand.baseURI()).to.be.eq(
      "https://characters.playarising.com/zhand/"
    );
    expect(await this.ikarans.baseURI()).to.be.eq(
      "https://characters.playarising.com/ikarans/"
    );
    expect(await this.tarki.baseURI()).to.be.eq(
      "https://characters.playarising.com/tarki/"
    );
    expect(await this.hartheim.baseURI()).to.be.eq(
      "https://characters.playarising.com/hartheim/"
    );
    expect(await this.shinkari.baseURI()).to.be.eq(
      "https://characters.playarising.com/shinkari/"
    );
  });

  it("should initialize and mint an ard character", async () => {
    await this.ard.mint(this.owner.address);
    expect(await this.ard.tokenURI(1)).to.be.eq(
      "https://characters.playarising.com/ard/1"
    );
  });

  it("should initialize and mint an zhand character", async () => {
    await this.zhand.mint(this.owner.address);

    expect(await this.zhand.tokenURI(1)).to.be.eq(
      "https://characters.playarising.com/zhand/1"
    );
  });

  it("should initialize and mint an ikaran character", async () => {
    await this.ikarans.mint(this.owner.address);

    expect(await this.ikarans.tokenURI(1)).to.be.eq(
      "https://characters.playarising.com/ikarans/1"
    );
  });

  it("should initialize and mint an tarki character", async () => {
    await this.tarki.mint(this.owner.address);

    expect(await this.tarki.tokenURI(1)).to.be.eq(
      "https://characters.playarising.com/tarki/1"
    );
  });

  it("should initialize and mint an hartheim character", async () => {
    await this.hartheim.mint(this.owner.address);

    expect(await this.hartheim.tokenURI(1)).to.be.eq(
      "https://characters.playarising.com/hartheim/1"
    );
  });

  it("should initialize and mint an shinkari character", async () => {
    await this.shinkari.mint(this.owner.address);
    expect(await this.shinkari.tokenURI(1)).to.be.eq(
      "https://characters.playarising.com/shinkari/1"
    );
  });
});
