const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("BaseERC721", () => {
  before(async () => {
    const [owner, receiver, minter1, minter2, minter3, minter4, minter5] =
      await ethers.getSigners();

    this.owner = owner;
    this.receiver = receiver;
    this.minter1 = minter1;
    this.minter2 = minter2;
    this.minter3 = minter3;
    this.minter4 = minter4;
    this.minter5 = minter5;

    const MintGuard = await ethers.getContractFactory("MintGuard");
    this.guard = await MintGuard.deploy();
    await this.guard.deployed();

    const BaseERC721 = await ethers.getContractFactory("BaseERC721");
    this.token = await BaseERC721.deploy(
      "Test",
      "TEST",
      this.guard.address,
      "https://test.uri/",
      5,
      receiver.address
    );
    await this.token.deployed();

    await this.guard.addProtected(this.token.address);
  });

  it("should not be able to mint when not initialized", async () => {
    await expect(this.token.mint()).to.revertedWith(
      "BaseERC721: contract is not initialized"
    );
  });

  it("should not be able to initialize when no price set", async () => {
    await expect(this.token.setInitialized()).to.revertedWith(
      "BaseERC721: can't initialize when price is 0"
    );
  });

  it("should not be able to initialize by a non owner", async () => {
    await expect(
      this.token.connect(this.receiver).setInitialized()
    ).to.revertedWith("Ownable: caller is not the owner");
  });

  it("should set the price correctly", async () => {
    expect(await this.token.price()).to.be.eq(0);
    await this.token.setPrice(ethers.utils.parseEther("10"));
    expect(await this.token.price()).to.be.eq(ethers.utils.parseEther("10"));
  });

  it("should initialize correctly", async () => {
    expect(await this.token.initialized()).to.be.eq(false);
    await this.token.setInitialized();
    expect(await this.token.initialized()).to.be.eq(true);
  });

  it("should try to mint sending less than required", async () => {
    await expect(
      this.token.mint({ value: ethers.utils.parseEther("9") })
    ).to.revertedWith("BaseERC721: Tx doesn't include enough to pay the mint");
  });

  it("should mint 1 token using the owner address", async () => {
    const initialReceiverBalance = await ethers.provider.getBalance(
      this.receiver.address
    );
    const initialMinterBalance = await ethers.provider.getBalance(
      this.owner.address
    );
    await this.token.mint({ value: ethers.utils.parseEther("10") });
    const minterBalance = await ethers.provider.getBalance(this.owner.address);
    const receiverBalance = await ethers.provider.getBalance(
      this.receiver.address
    );
    expect(
      (parseFloat(ethers.utils.formatEther(initialMinterBalance)) - 10).toFixed(
        0
      )
    ).to.be.eq(parseFloat(ethers.utils.formatEther(minterBalance)).toFixed(0));
    expect(
      (
        parseFloat(ethers.utils.formatEther(initialReceiverBalance)) + 10
      ).toFixed(0)
    ).to.be.eq(
      parseFloat(ethers.utils.formatEther(receiverBalance)).toFixed(0)
    );

    expect(await this.token.balanceOf(this.owner.address)).to.be.eq(1);
  });

  it("should fail when trying to mint a second token from owner address", async () => {
    await expect(
      this.token.mint({ value: ethers.utils.parseEther("10") })
    ).to.revertedWith("BaseERC721: Address has already minted");
  });

  it("should return the token URI correctly", async () => {
    expect(await this.token.tokenURI(1)).to.be.eq("https://test.uri/1");
  });

  it("should change the max supply", async () => {
    await expect(await this.token.cap()).to.be.eq(5);
    await this.token.setCap(10);
    await expect(await this.token.cap()).to.be.eq(10);
    await this.token.setCap(5);
  });

  it("should mint until the cap is reached", async () => {
    await this.token
      .connect(this.minter1)
      .mint({ value: ethers.utils.parseEther("10") });
    expect(await this.token.balanceOf(this.minter1.address)).to.be.eq(1);
    await this.token
      .connect(this.minter2)
      .mint({ value: ethers.utils.parseEther("10") });
    expect(await this.token.balanceOf(this.minter2.address)).to.be.eq(1);
    await this.token
      .connect(this.minter3)
      .mint({ value: ethers.utils.parseEther("10") });
    expect(await this.token.balanceOf(this.minter3.address)).to.be.eq(1);
    await this.token
      .connect(this.minter4)
      .mint({ value: ethers.utils.parseEther("10") });
    expect(await this.token.balanceOf(this.minter4.address)).to.be.eq(1);
    await expect(
      this.token
        .connect(this.minter5)
        .mint({ value: ethers.utils.parseEther("10") })
    ).to.revertedWith(
      "BaseERC721: Max supply reached, wait for more tokens to be available"
    );
  });

  it("should increase the cap for 1 more and mint it", async () => {
    await expect(
        this.token
            .connect(this.minter5)
            .mint({ value: ethers.utils.parseEther("10") })
    ).to.revertedWith(
        "BaseERC721: Max supply reached, wait for more tokens to be available"
    );
    await this.token.setCap(6);
    await this.token
        .connect(this.minter5)
        .mint({ value: ethers.utils.parseEther("10") });
    expect(await this.token.balanceOf(this.minter5.address)).to.be.eq(1);
  });
});
