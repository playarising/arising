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

    const ArdCharacters = await ethers.getContractFactory("ArdCharacters");
    this.ard = await ArdCharacters.deploy(
      this.guard.address,
      10,
      receiver.address
    );

    await this.ard.deployed();

    const ZhandCharacters = await ethers.getContractFactory("ZhandCharacters");
    this.zhand = await ZhandCharacters.deploy(
      this.guard.address,
      10,
      receiver.address
    );
    await this.zhand.deployed();

    const IkaranCharacters = await ethers.getContractFactory(
      "IKaransCharacters"
    );
    this.ikarans = await IkaranCharacters.deploy(
      this.guard.address,
      10,
      receiver.address
    );
    await this.ikarans.deployed();

    const TarkiCharacters = await ethers.getContractFactory("TarkiCharacters");
    this.tarki = await TarkiCharacters.deploy(
      this.guard.address,
      10,
      receiver.address
    );
    await this.tarki.deployed();

    const HeartheimCharacters = await ethers.getContractFactory(
      "HeartheimCharacters"
    );
    this.heartheim = await HeartheimCharacters.deploy(
      this.guard.address,
      10,
      receiver.address
    );
    await this.heartheim.deployed();

    const ShinkariCharacters = await ethers.getContractFactory(
      "ShinkariCharacters"
    );
    this.shinkari = await ShinkariCharacters.deploy(
      this.guard.address,
      10,
      receiver.address
    );
    await this.shinkari.deployed();

    await this.guard.addProtected(this.ard.address);
    await this.guard.addProtected(this.zhand.address);
    await this.guard.addProtected(this.ikarans.address);
    await this.guard.addProtected(this.tarki.address);
    await this.guard.addProtected(this.heartheim.address);
    await this.guard.addProtected(this.shinkari.address);
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

  it("should initialize and mint an heartheim character", async () => {
    await this.heartheim.setPrice(ethers.utils.parseEther("10"));
    await this.heartheim.setInitialized();
    await this.heartheim
      .connect(this.minter4)
      .mint({ value: ethers.utils.parseEther("10") });
    expect(await this.heartheim.tokenURI(1)).to.be.eq(
      "https://characters.playarising.com/heartheim/1"
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
