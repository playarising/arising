import * as anchor from "@coral-xyz/anchor";
import { runDeployment, detectClusterFromEndpoint } from "./deploy-script";

module.exports = async function (provider: anchor.AnchorProvider) {
  anchor.setProvider(provider);
  const cluster =
    detectClusterFromEndpoint(provider.connection.rpcEndpoint) ?? "devnet";
  await runDeployment(provider, cluster);
};
