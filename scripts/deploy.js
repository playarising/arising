const { ethers } = require("hardhat");

const PAYMENT_RECEIVER = "0x6a10415B984DeaC967425E157B77a852978002EF";

const ARD_CAP = 4500;
const ZHAND_CAP = 1000;
const IKARANS_CAP = 2500;
const TARKI_CAP = 2500;
const HARTHEIM_CAP = 1000;
const SHINKARI_CAP = 4500;

async function main() {
  const MintGuard = await ethers.getContractFactory("MintGuard");

  console.log("==> Deploying MintGuard...");
  const guard = await MintGuard.deploy();
  await guard.deployed();
  console.log("==> MintGuard deployed", guard.address);

  const Ard = await ethers.getContractFactory("Ard");
  const ard = await Ard.deploy(guard.address, ARD_CAP, PAYMENT_RECEIVER);
  await ard.deployed();

  const Zhand = await ethers.getContractFactory("Zhand");
  const zhand = await Zhand.deploy(guard.address, ZHAND_CAP, PAYMENT_RECEIVER);
  await zhand.deployed();

  const IKarans = await ethers.getContractFactory("IKarans");
  const ikarans = await IKarans.deploy(
    guard.address,
    IKARANS_CAP,
    PAYMENT_RECEIVER
  );
  await ikarans.deployed();

  const Tarki = await ethers.getContractFactory("Tarki");
  const tarki = await Tarki.deploy(guard.address, TARKI_CAP, PAYMENT_RECEIVER);
  await tarki.deployed();

  const Hartheim = await ethers.getContractFactory("Hartheim");
  const hartheim = await Hartheim.deploy(
    guard.address,
    HARTHEIM_CAP,
    PAYMENT_RECEIVER
  );
  await hartheim.deployed();

  const Shinkari = await ethers.getContractFactory("Shinkari");
  const shinkari = await Shinkari.deploy(
    guard.address,
    SHINKARI_CAP,
    PAYMENT_RECEIVER
  );
  await shinkari.deployed();

  console.log("==> Deployment finished");
  console.log("==> Ard", ard.address);
  console.log("==> Zhand", zhand.address);
  console.log("==> I'Karans", ikarans.address);
  console.log("==> Tark'i", tarki.address);
  console.log("==> Hartheim", hartheim.address);
  console.log("==> Shinkari", shinkari.address);

  console.log("==> Adding protected contracts to MintGuard");
  await guard.addProtected(ard.address);
  await guard.addProtected(zhand.address);
  await guard.addProtected(ikarans.address);
  await guard.addProtected(tarki.address);
  await guard.addProtected(hartheim.address);
  await guard.addProtected(shinkari.address);
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
