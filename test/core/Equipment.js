const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("Equipment", () => {
  before(async () => {
    const [owner, minter] = await ethers.getSigners();

    this.owner = owner;
    this.minter = minter;

    const Items = await ethers.getContractFactory("Items");
    this.items = await Items.deploy();
    await this.items.deployed();
    await this.items.initialize();
    await this.items.addItem(
      "test",
      "test 2",
      1,
      0,
      {
        might: 2,
        speed: 2,
        intellect: 2,
        might_reducer: 1,
        speed_reducer: 1,
        intellect_reducer: 1,
      },
      {
        atk: 2,
        atk_reducer: 1,
        def: 2,
        def_reducer: 1,
        range: 2,
        range_reducer: 1,
        mag_atk: 2,
        mag_atk_reducer: 1,
        mag_def: 2,
        mag_def_reducer: 1,
        rate: 2,
        rate_reducer: 1,
      },
    );
    await this.items.addItem(
      "test",
      "test 2",
      3,
      1,
      {
        might: 4,
        speed: 4,
        intellect: 4,
        might_reducer: 2,
        speed_reducer: 2,
        intellect_reducer: 2,
      },
      {
        atk: 3,
        atk_reducer: 2,
        def: 3,
        def_reducer: 2,
        range: 3,
        range_reducer: 2,
        mag_atk: 3,
        mag_atk_reducer: 2,
        mag_def: 3,
        mag_def_reducer: 2,
        rate: 3,
        rate_reducer: 2,
      },
    );

    await this.items.addItem(
      "test",
      "test 2",
      1,
      0,
      {
        might: 4,
        speed: 4,
        intellect: 4,
        might_reducer: 2,
        speed_reducer: 2,
        intellect_reducer: 2,
      },
      {
        atk: 3,
        atk_reducer: 2,
        def: 3,
        def_reducer: 2,
        range: 2,
        range_reducer: 5,
        mag_atk: 2,
        mag_atk_reducer: 2,
        mag_def: 3,
        mag_def_reducer: 2,
        rate: 3,
        rate_reducer: 2,
      },
    );

    await this.items.addItem(
      "test",
      "test 2",
      1,
      11,
      {
        might: 4,
        speed: 4,
        intellect: 4,
        might_reducer: 2,
        speed_reducer: 2,
        intellect_reducer: 2,
      },
      {
        atk: 3,
        atk_reducer: 2,
        def: 3,
        def_reducer: 2,
        range: 3,
        range_reducer: 2,
        mag_atk: 2,
        mag_atk_reducer: 0,
        mag_def: 3,
        mag_def_reducer: 2,
        rate: 3,
        rate_reducer: 2,
      },
    );

    await this.items.addItem(
      "test",
      "test 2",
      1,
      12,
      {
        might: 4,
        speed: 4,
        intellect: 4,
        might_reducer: 2,
        speed_reducer: 2,
        intellect_reducer: 2,
      },
      {
        atk: 3,
        atk_reducer: 2,
        def: 3,
        def_reducer: 2,
        range: 3,
        range_reducer: 2,
        mag_atk: 0,
        mag_atk_reducer: 0,
        mag_def: 3,
        mag_def_reducer: 2,
        rate: 3,
        rate_reducer: 2,
      },
    );

    await this.items.mint(this.owner.address, 1);
    await this.items.mint(this.owner.address, 2);
    await this.items.mint(this.owner.address, 3);
    await this.items.mint(this.owner.address, 4);
    await this.items.mint(this.owner.address, 4);
    await this.items.mint(this.owner.address, 5);

    const MockToken = await ethers.getContractFactory("MockToken");
    this.mock = await MockToken.deploy(ethers.utils.parseEther("5000"));
    await this.mock.deployed();

    const Civilizations = await ethers.getContractFactory("Civilizations");
    this.civ = await Civilizations.deploy();
    await this.civ.deployed();
    await this.civ.initialize(this.mock.address);
    const BaseERC721 = await ethers.getContractFactory("BaseERC721");
    this.collection = await BaseERC721.deploy();
    await this.collection.deployed();
    await this.collection.initialize(
      "Test Collection",
      "TEST",
      "https://test.uri/",
      this.civ.address,
    );
    await this.collection.addAuthority(this.civ.address);

    await this.civ.addCivilization(this.collection.address);

    await this.civ.mint(1);

    const Levels = await ethers.getContractFactory("Levels");
    const levels = await Levels.deploy();
    await levels.deployed();
    await levels.initialize();
    const Experience = await ethers.getContractFactory("Experience");
    this.experience = await Experience.deploy();
    await this.experience.deployed();
    await this.experience.initialize(this.civ.address, levels.address);
    const Equipment = await ethers.getContractFactory("Equipment");
    this.equipment = await Equipment.deploy();
    await this.equipment.deployed();
    await this.equipment.initialize(
      this.civ.address,
      this.experience.address,
      this.items.address,
    );
  });

  it("should not be able to equip an item when paused", async () => {
    const id = await this.civ.getTokenID(1, 1);
    expect(await this.equipment.paused()).to.eq(false);
    await this.equipment.pause();
    expect(await this.equipment.paused()).to.eq(true);
    await expect(this.equipment.equip(id, 0, 1)).to.revertedWith(
      "Pausable: paused",
    );
    await this.equipment.unpause();
    expect(await this.equipment.paused()).to.eq(false);
  });

  it("should not be able pause when no owner", async () => {
    await expect(this.equipment.connect(this.minter).pause()).to.revertedWith(
      "Ownable: caller is not the owner",
    );
  });

  it("should fail when trying to equip for a non owned token", async () => {
    const id = await this.civ.getTokenID(1, 1);
    await expect(
      this.equipment.connect(this.minter).equip(id, 1, 1),
    ).to.revertedWith(
      "Equipment: onlyAllowed() msg.sender is not allowed to access this token.",
    );
  });

  it("should fail when trying to unequip for a non owned token", async () => {
    const id = await this.civ.getTokenID(1, 1);
    await expect(
      this.equipment.connect(this.minter).unequip(id, 1),
    ).to.revertedWith(
      "Equipment: onlyAllowed() msg.sender is not allowed to access this token.",
    );
  });

  it("should fail when trying to equip for a non minted token", async () => {
    await expect(
      this.equipment
        .connect(this.minter)
        .equip(
          "0x00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000002",
          1,
          1,
        ),
    ).to.revertedWith("Equipment: onlyAllowed() token not minted.");
  });

  it("should fail when trying to unequip for a non minted token", async () => {
    await expect(
      this.equipment
        .connect(this.minter)
        .unequip(
          "0x00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000002",
          1,
        ),
    ).to.revertedWith("Equipment: onlyAllowed() token not minted.");
  });

  it("should fail when trying to equip a token to a wrong slot", async () => {
    const id = await this.civ.getTokenID(1, 1);
    await expect(this.equipment.equip(id, 2, 1)).to.revertedWith(
      "Equipment: equip() item type not for this slot.",
    );
  });

  it("should fail when trying to equip a token with not enough level", async () => {
    const id = await this.civ.getTokenID(1, 1);
    await expect(this.equipment.equip(id, 0, 1)).to.revertedWith(
      "Equipment: equip() not enough level to equip item.",
    );
  });

  it("should equip a token correctly", async () => {
    const id = await this.civ.getTokenID(1, 1);
    await this.experience.assignExperience(id, 5000);
    await this.items.setApprovalForAll(this.equipment.address, true);
    await expect(this.equipment.equip(id, 0, 1))
      .to.emit(this.equipment, "Equipped")
      .withArgs(id, 0, 1);
    const equipment = await this.equipment.getCharacterEquipment(id);
    expect(equipment.helmet.id).to.eq(1);
    expect(equipment.helmet.equiped).to.eq(true);
    const stats = await this.equipment.getCharacterTotalStatsModifiers(id);
    expect(stats.might).to.eq(1);
    expect(stats.speed).to.eq(1);
    expect(stats.intellect).to.eq(1);
    const attributes = await this.equipment.getCharacterTotalAttributes(id);
    expect(attributes.atk).to.eq(1);
    expect(attributes.def).to.eq(1);
    expect(attributes.mag_atk).to.eq(1);
    expect(attributes.mag_def).to.eq(1);
    expect(attributes.range).to.eq(1);
    expect(attributes.rate).to.eq(1);

    expect(await this.items.balanceOf(this.owner.address, 1)).to.eq(0);
    expect(await this.items.balanceOf(this.owner.address, 3)).to.eq(1);
    expect(await this.items.balanceOf(this.equipment.address, 1)).to.eq(1);
    expect(await this.items.balanceOf(this.equipment.address, 3)).to.eq(0);
  });

  it("should equip by replacing a token correctly", async () => {
    const id = await this.civ.getTokenID(1, 1);
    await this.experience.assignExperience(id, 5000);
    await this.items.setApprovalForAll(this.equipment.address, true);
    await this.equipment.equip(id, 0, 3);
    const equipment = await this.equipment.getCharacterEquipment(id);
    expect(equipment.helmet.id).to.eq(3);
    expect(equipment.helmet.equiped).to.eq(true);
    const stats = await this.equipment.getCharacterTotalStatsModifiers(id);
    expect(stats.might).to.eq(2);
    expect(stats.speed).to.eq(2);
    expect(stats.intellect).to.eq(2);
    const attributes = await this.equipment.getCharacterTotalAttributes(id);
    expect(attributes.atk).to.eq(1);
    expect(attributes.def).to.eq(1);
    expect(attributes.mag_atk).to.eq(0);
    expect(attributes.mag_def).to.eq(1);
    expect(attributes.range).to.eq(0);
    expect(attributes.rate).to.eq(1);

    expect(await this.items.balanceOf(this.owner.address, 1)).to.eq(1);
    expect(await this.items.balanceOf(this.owner.address, 3)).to.eq(0);
    expect(await this.items.balanceOf(this.equipment.address, 1)).to.eq(0);
    expect(await this.items.balanceOf(this.equipment.address, 3)).to.eq(1);
  });

  it("should fail when trying to unequip an empty slot", async () => {
    const id = await this.civ.getTokenID(1, 1);
    await expect(this.equipment.unequip(id, 5)).to.revertedWith(
      "Equipment: unequip() item slot not equiped.",
    );
  });

  it("should unequip an item successfully", async () => {
    const id = await this.civ.getTokenID(1, 1);
    await expect(this.equipment.unequip(id, 0))
      .to.emit(this.equipment, "Unequipped")
      .withArgs(id, 0);
  });

  it("should remove both hands slots when equiping a two handed item", async () => {
    const id = await this.civ.getTokenID(1, 1);
    await this.equipment.equip(id, 0, 3);
    expect(await this.items.balanceOf(this.owner.address, 4)).to.eq(2);
    await this.equipment.equip(id, 11, 4);
    await this.equipment.equip(id, 12, 4);
    expect(await this.items.balanceOf(this.owner.address, 4)).to.eq(0);
    let equipment = await this.equipment.getCharacterEquipment(id);
    expect(equipment.left_hand.id).to.eq(4);
    expect(equipment.right_hand.id).to.eq(4);
    expect(equipment.left_hand.equiped).to.eq(true);
    expect(equipment.right_hand.equiped).to.eq(true);

    await expect(this.equipment.equip(id, 12, 5)).to.revertedWith(
      "Equipment: equip() item type not for this slot.",
    );
    await this.equipment.equip(id, 11, 5);
    equipment = await this.equipment.getCharacterEquipment(id);
    expect(equipment.left_hand.id).to.eq(5);
    expect(equipment.right_hand.id).to.eq(0);
    expect(equipment.left_hand.equiped).to.eq(true);
    expect(equipment.right_hand.equiped).to.eq(false);
    expect(await this.items.balanceOf(this.owner.address, 4)).to.eq(2);
    const stats = await this.equipment.getCharacterTotalStatsModifiers(id);
    expect(stats.might).to.eq(4);
    expect(stats.speed).to.eq(4);
    expect(stats.intellect).to.eq(4);
    const attributes = await this.equipment.getCharacterTotalAttributes(id);
    expect(attributes.atk).to.eq(2);
    expect(attributes.def).to.eq(2);
    expect(attributes.mag_atk).to.eq(0);
    expect(attributes.mag_def).to.eq(2);
    expect(attributes.range).to.eq(0);
    expect(attributes.rate).to.eq(2);
  });
});
