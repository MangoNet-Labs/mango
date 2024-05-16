import ReactDOM from "react-dom/client";
import "@mysten/dapp-kit/dist/index.css";
import { BrowserRouter } from 'react-router-dom';
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";

import { MgoClientProvider, WalletProvider } from "@mgonetwork/dapp-kit";
import { WalletKitProvider } from '@mgonetwork/wallet-kit';
import { fullNode } from '@/utils/info';

import { createNetworkConfig, SuiClientProvider, WalletProvider as SuiWalletProvider } from "@mysten/dapp-kit";
import { suiNetworkConfig } from './suiNetworkConfig.ts'


const { networkConfig } = createNetworkConfig({
  mainnet: {
    url: fullNode
  },
});


import App from "./App.tsx";
import '@/locales/i18n.ts'
const queryClient = new QueryClient();



ReactDOM.createRoot(document.getElementById("root")!).render(
  <QueryClientProvider client={queryClient}>
    <MgoClientProvider networks={networkConfig} defaultNetwork="mainnet">
      <SuiClientProvider networks={suiNetworkConfig} defaultNetwork="mainnet">
        <WalletKitProvider>
          <WalletProvider autoConnect={false}>
            <SuiWalletProvider autoConnect={false}>
              <BrowserRouter>
                <App />
              </BrowserRouter>
            </SuiWalletProvider>
          </WalletProvider>
        </WalletKitProvider>
      </SuiClientProvider>
    </MgoClientProvider>
  </QueryClientProvider>
);
