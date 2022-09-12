const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("Civilizations", () => {
  before(async () => {
    const [owner, receiver, minter] = await ethers.getSigners();

    this.owner = owner;
    this.receiver = receiver;
    this.minter = minter;

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

  it("should deploy an initialize the civilizations contract", async () => {
    const MockToken = await ethers.getContractFactory("MockToken");
    this.mock = await MockToken.deploy(ethers.utils.parseEther("100"));
    await this.mock.deployed();

    const Civilizations = await ethers.getContractFactory("Civilizations");
    this.civ = await Civilizations.deploy(this.mock.address);
    await this.civ.deployed();

    await this.civ.addCivilization(this.ard.address);
    await this.civ.addCivilization(this.zhand.address);
    await this.civ.addCivilization(this.ikarans.address);
    await this.civ.addCivilization(this.tarki.address);
    await this.civ.addCivilization(this.hartheim.address);
    await this.civ.addCivilization(this.shinkari.address);

    await this.ard.transferOwnership(this.civ.address);
    await this.zhand.transferOwnership(this.civ.address);
    await this.ikarans.transferOwnership(this.civ.address);
    await this.tarki.transferOwnership(this.civ.address);
    await this.hartheim.transferOwnership(this.civ.address);
    await this.shinkari.transferOwnership(this.civ.address);
  });

  it("should not be able to mint when not initialized", async () => {
    await expect(this.civ.mint(this.ard.address)).to.revertedWith(
      "Civilizations: contract is not initialized"
    );
  });

  it("should not be able to initialize by a non owner", async () => {
    await expect(
      this.civ.connect(this.receiver).setInitialized()
    ).to.revertedWith("Ownable: caller is not the owner");
  });

  it("should initialize correctly", async () => {
    expect(await this.civ.initialized()).to.be.eq(false);
    await this.civ.setInitialized();
    expect(await this.civ.initialized()).to.be.eq(true);
  });

  it("should try to mint to an empty instance", async () => {
    await expect(
      this.civ.mint("0x0000000000000000000000000000000000000000")
    ).to.revertedWith("Civilizations: instance address is null.");
  });

  it("should try to mint to a non arising civilization", async () => {
    await expect(this.civ.mint(this.receiver.address)).to.revertedWith(
      "Civilizations: instance is not an Arising civilization."
    );
  });

  it("should mint a token correctly", async () => {
    await this.civ.connect(this.minter).mint(this.ard.address);
    expect(await this.ard.balanceOf(this.minter.address)).to.eq(1);
  });

  it("should attempt to add a civilization contract from a non owner", async () => {
    await expect(
      this.civ.connect(this.minter).addCivilization(this.ard.address)
    ).to.revertedWith("Ownable: caller is not the owner");
  });

  it("should fail when trying to mint through a contract", async () => {
    const MockMinter = await ethers.getContractFactory("MockMinter");
    this.mock = await MockMinter.deploy(this.civ.address);
    await this.mock.deployed();
    await expect(this.mock.mintMock(this.ard.address)).to.revertedWith(
      "Civilizations: cannot mint from a contract"
    );
  });

  it("should return civilizations correctly", async () => {
    expect(await this.civ.getCivilizations()).to.eql([
      this.ard.address,
      this.zhand.address,
      this.ikarans.address,
      this.tarki.address,
      this.hartheim.address,
      this.shinkari.address,
    ]);
  });

  it("should return civilization ids correctly", async () => {
    expect(await this.civ.getID(this.ard.address)).to.eq(1);
    expect(await this.civ.getID(this.zhand.address)).to.eq(2);
    expect(await this.civ.getID(this.ikarans.address)).to.eq(3);
    expect(await this.civ.getID(this.tarki.address)).to.eq(4);
    expect(await this.civ.getID(this.hartheim.address)).to.eq(5);
    expect(await this.civ.getID(this.shinkari.address)).to.eq(6);
  });

  it("should error when request civilizations doesnt exist or are not from arising", async () => {
    await expect(
      this.civ.getID("0x0000000000000000000000000000000000000000")
    ).to.revertedWith("Civilizations: instance address is null.");
    await expect(this.civ.getID(this.receiver.address)).to.revertedWith(
      "Civilizations: instance is not an Arising civilization."
    );
  });

  it("should return civilization token ids correctly", async () => {
    expect(await this.civ.getTokenID(this.ard.address, 1)).to.eq(
      "0x00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000001"
    );
    expect(await this.civ.getTokenID(this.zhand.address, 1)).to.eq(
      "0x00000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000001"
    );
    expect(await this.civ.getTokenID(this.ikarans.address, 1)).to.eq(
      "0x00000000000000000000000000000000000000000000000000000000000000030000000000000000000000000000000000000000000000000000000000000001"
    );
    expect(await this.civ.getTokenID(this.tarki.address, 1)).to.eq(
      "0x00000000000000000000000000000000000000000000000000000000000000040000000000000000000000000000000000000000000000000000000000000001"
    );
    expect(await this.civ.getTokenID(this.hartheim.address, 1)).to.eq(
      "0x00000000000000000000000000000000000000000000000000000000000000050000000000000000000000000000000000000000000000000000000000000001"
    );
    expect(await this.civ.getTokenID(this.shinkari.address, 1)).to.eq(
      "0x00000000000000000000000000000000000000000000000000000000000000060000000000000000000000000000000000000000000000000000000000000001"
    );
  });

  it("should fail when try to get the civilization token ids from an invalid instance", async () => {
    await expect(
      this.civ.getTokenID("0x0000000000000000000000000000000000000000", 1)
    ).to.revertedWith("Civilizations: instance address is null.");
    await expect(this.civ.getTokenID(this.receiver.address, 1)).to.revertedWith(
      "Civilizations: instance is not an Arising civilization."
    );
  });

  it("should fail when try to get the civilization token ids from a non minted token", async () => {
    await expect(this.civ.getTokenID(this.ard.address, 3)).to.revertedWith(
      "Civilizations: token id is not minted."
    );
  });

  it("should return false for not allowance token", async () => {
    const id = await this.civ.getTokenID(this.ard.address, 1);
    expect(await this.civ.isAllowed(this.minter.address, id)).to.eq(false);
  });

  it("should return true when allowed and approved for all", async () => {
    const id = await this.civ.getTokenID(this.ard.address, 1);
    await this.ard.approve(this.receiver.address, 1);
    await this.ard.setApprovalForAll(this.minter.address, true);
    expect(await this.civ.isAllowed(this.minter.address, id)).to.eq(true);
    expect(await this.civ.isAllowed(this.receiver.address, id)).to.eq(true);
  });

  it("should revert when check allowance from a wrong civilization composed ID", async () => {
    await expect(
      this.civ.isAllowed(
        this.minter.address,
        "0x00000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000001"
      )
    ).to.revertedWith("Civilizations: id of the civilization is not valid.");
  });

  it("should check if a token is already minted", async () => {
    expect(
      await this.civ.exists(
        "0x00000000000000000000000000000000000000000000000000000000000000060000000000000000000000000000000000000000000000000000000000000001"
      )
    ).to.eq(true);
  });

  it("should fail when trying to check if a token exists from an invalid civilization", async () => {
    await expect(
      this.civ.exists(
        "0x00000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000001"
      )
    ).to.revertedWith("Civilizations: id of the civilization is not valid.");
  });

  it("should fail when trying to check if a token exists from a non minted id", async () => {
    expect(
      await this.civ.exists(
        "0x00000000000000000000000000000000000000000000000000000000000000060000000000000000000000000000000000000000000000000000000000000004"
      )
    ).to.eq(false);
  });
});
