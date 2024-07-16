import { SuiClient } from '@mysten/sui.js/client'
import { getFullnodeUrl } from "@mysten/sui.js/client";
import { ChainListType, CoinListType } from '@/interface'
import { bigToSmallFixed0, getPointSix } from '@/utils/web3'

export const suiFullNode = getFullnodeUrl("testnet")
const client = new SuiClient({ url: suiFullNode })

export function useSuiJs() {
  async function getSuiBalance(info: ChainListType, userAddress: string, coinList: CoinListType[]) {
    const balanceList = await client.getAllBalances({ owner: userAddress })
    const list = [...coinList]
    for (let i = 0; i < list.length; i++) {
      const ele = list[i];
      if (ele.chain === info.chainName) {
        for (let j = 0; j < balanceList.length; j++) {
          if (ele.contract == balanceList[j].coinType) {
            ele.balance = getPointSix(bigToSmallFixed0(balanceList[j].totalBalance, ele.decimals), ele.decimals > 6 ? 6 : ele.decimals)
          }
        }
      }
    }
    return list
  }

  return {
    getSuiBalance
  }
}
