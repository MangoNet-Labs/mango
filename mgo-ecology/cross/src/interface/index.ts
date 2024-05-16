export interface ChainListType {
  chainName: string;
  logo: string;
  chainId: number;
  series: 'evm' | 'mgo' | 'sui' | 'btc';
  blockUrl: string;
  rpc: string;
  tokens: CoinListType[];
  nativeCurrency: NativeCurrencyType;
}

interface NativeCurrencyType {
  decimals: string;
  name: string;
  symbol: string;
}

export interface CoinListType {
  name: string;
  symbol: string;
  logo: string;
  contract: string;
  decimals: number;
  balance: string;
  alias: string;
  isEther: boolean;
  bridges: Bridge[];
  chain: string;
}
interface Bridge {
  from: string;
  to: string;
  contract: string;
  bridgeObject: string | null;
}

export interface ConfigType {
  fee: string;
  feeType: '0' | '1'; 
  minLimit: string;
  maxLimit: string;
  feeCalculationType: '0' | '1'; 
}

export interface ActualInfoType {
  ActualDeduction: string;
  ActualGain: string;
}