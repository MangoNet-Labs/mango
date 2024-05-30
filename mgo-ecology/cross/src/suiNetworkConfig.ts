import { getFullnodeUrl } from "@mysten/sui.js/client";
import { createNetworkConfig } from "@mysten/dapp-kit";

const { networkConfig } = createNetworkConfig({
  mainnet: {
    url: getFullnodeUrl("mainnet")
  },
});

export { networkConfig as suiNetworkConfig }