const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("Collections", () => {
  before(async () => {
    const [owner, receiver, minter1, minter2, minter3, minter4, minter5] =
      await ethers.getSigners();

    this.owner = owner;
    this.minter1 = minter1;
    this.minter2 = minter2;
    this.minter3 = minter3;
    this.minter4 = minter4;
    this.minter5 = minter5;

    const MintGuard = await ethers.getContractFactory("MintGuard");
    this.guard = await MintGuard.deploy();
    await this.guard.deployed();

    const Ard = await ethers.getContractFactory("Ard");
    this.ard = await Ard.deploy(this.guard.address, 10, receiver.address);

    await this.ard.deployed();

    const Zhand = await ethers.getContractFactory("Zhand");
    this.zhand = await Zhand.deploy(this.guard.address, 10, receiver.address);
    await this.zhand.deployed();

    const IKarans = await ethers.getContractFactory("IKarans");
    this.ikarans = await IKarans.deploy(
      this.guard.address,
      10,
      receiver.address
    );
    await this.ikarans.deployed();

    const Tarki = await ethers.getContractFactory("Tarki");
    this.tarki = await Tarki.deploy(this.guard.address, 10, receiver.address);
    await this.tarki.deployed();

    const Hartheim = await ethers.getContractFactory("Hartheim");
    this.hartheim = await Hartheim.deploy(
      this.guard.address,
      10,
      receiver.address
    );
    await this.hartheim.deployed();

    const Shinkari = await ethers.getContractFactory("Shinkari");
    this.shinkari = await Shinkari.deploy(
      this.guard.address,
      10,
      receiver.address
    );
    await this.shinkari.deployed();

    await this.guard.addProtected(this.ard.address);
    await this.guard.addProtected(this.zhand.address);
    await this.guard.addProtected(this.ikarans.address);
    await this.guard.addProtected(this.tarki.address);
    await this.guard.addProtected(this.hartheim.address);
    await this.guard.addProtected(this.shinkari.address);
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
    await this.ard.setPrice(ethers.utils.parseEther("10"));
    await this.ard.setInitialized();
    await this.ard.mint({ value: ethers.utils.parseEther("10") });
    expect(await this.ard.tokenURI(1)).to.be.eq(
      "https://characters.playarising.com/ard/1"
    );
  });

  it("should initialize and mint an zhand character", async () => {
    await this.zhand.setPrice(ethers.utils.parseEther("10"));
    await this.zhand.setInitialized();
    await this.zhand
      .connect(this.minter1)
      .mint({ value: ethers.utils.parseEther("10") });
    expect(await this.zhand.tokenURI(1)).to.be.eq(
      "https://characters.playarising.com/zhand/1"
    );
  });

  it("should initialize and mint an ikaran character", async () => {
    await this.ikarans.setPrice(ethers.utils.parseEther("10"));
    await this.ikarans.setInitialized();
    await this.ikarans
      .connect(this.minter2)
      .mint({ value: ethers.utils.parseEther("10") });
    expect(await this.ikarans.tokenURI(1)).to.be.eq(
      "https://characters.playarising.com/ikarans/1"
    );
  });

  it("should initialize and mint an tarki character", async () => {
    await this.tarki.setPrice(ethers.utils.parseEther("10"));
    await this.tarki.setInitialized();
    await this.tarki
      .connect(this.minter3)
      .mint({ value: ethers.utils.parseEther("10") });
    expect(await this.tarki.tokenURI(1)).to.be.eq(
      "https://characters.playarising.com/tarki/1"
    );
  });

  it("should initialize and mint an hartheim character", async () => {
    await this.hartheim.setPrice(ethers.utils.parseEther("10"));
    await this.hartheim.setInitialized();
    await this.hartheim
      .connect(this.minter4)
      .mint({ value: ethers.utils.parseEther("10") });
    expect(await this.hartheim.tokenURI(1)).to.be.eq(
      "https://characters.playarising.com/hartheim/1"
    );
  });

  it("should initialize and mint an shinkari character", async () => {
    await this.shinkari.setPrice(ethers.utils.parseEther("10"));
    await this.shinkari.setInitialized();
    await this.shinkari
      .connect(this.minter5)
      .mint({ value: ethers.utils.parseEther("10") });
    expect(await this.shinkari.tokenURI(1)).to.be.eq(
      "https://characters.playarising.com/shinkari/1"
    );
  });
});
