export interface CoinInfoType {
  address: string;
  coin: {
    decimal: number;
  };
  symbol: string;
  icon: string;
  Introduction: string;
  balance: string;
  disabled: boolean;
  isMain: boolean;
}

interface AmountType {
  typeArg: string[];
  poolId: string;
  is_a_to_b: boolean;
  userAddress: string;
  decimal: number;
}

export interface AmountOutType extends AmountType {
  amountIn: string;
}
export interface AmountInType extends AmountType {
  amountOut: string;
}