import React from "react";
import ReactDOM from "react-dom/client";
import "@mysten/dapp-kit/dist/index.css";
import { BrowserRouter } from 'react-router-dom';
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { getFullnodeUrl } from "@mgonetwork/mango.js/client";
import { MgoClientProvider, WalletProvider } from "@mgonetwork/dapp-kit";
import { createNetworkConfig } from "@mysten/dapp-kit";
import { WalletKitProvider } from '@mgonetwork/wallet-kit';
import { fullNode } from '@/utils/info';

import App from "./App.tsx";
import '@/locales/i18n.ts'
const queryClient = new QueryClient();

const { networkConfig } =
  createNetworkConfig({
    testnet: {
      url: getFullnodeUrl("testnet")
    },
    mainnet: {
      url: fullNode
    },
  });

ReactDOM.createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <QueryClientProvider client={queryClient}>
      <MgoClientProvider networks={networkConfig} defaultNetwork="mainnet">
        <WalletKitProvider enableUnsafeBurner={false}>
          <WalletProvider autoConnect>
            <BrowserRouter>
              <App />
            </BrowserRouter>
          </WalletProvider>
        </WalletKitProvider>
      </MgoClientProvider>
    </QueryClientProvider>
  </React.StrictMode>,
);
