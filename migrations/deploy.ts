import * as anchor from "@coral-xyz/anchor";
import { deployWithAutomaticCluster } from "./deploy-script";

module.exports = async function (provider: anchor.AnchorProvider) {
  anchor.setProvider(provider);
  await deployWithAutomaticCluster(provider);
};
