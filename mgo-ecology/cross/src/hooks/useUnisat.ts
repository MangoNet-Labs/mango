import { bigToSmallFixed0, getPointSix } from '@/utils/web3'
import { CoinListType } from '@/interface'

export function useUniSat() {
  async function connectUnisat() {
    try {
      if (typeof window.unisat !== 'undefined') {
        let network = await window.unisat.getNetwork();
        if (network !== 'testnet') {
          await window.unisat.switchNetwork("testnet");
        }
        let accounts = await window.unisat.requestAccounts();
        console.log('connect success', accounts);
        return accounts[0]
      } else {
        throw new Error('UniSat wallet is not installed!')
      }
    } catch (error) {
      console.log(error);
    }
  }
  async function getUnisatBalance(coinList: CoinListType[]) {
    if (!coinList.length) {
      return []
    }
    const coinInfo = coinList[0]
    let ba = await window.unisat.getBalance();
    const bba = ba.confirmed
    const res = getPointSix(bigToSmallFixed0(bba.toString(), coinInfo.decimals), coinInfo.decimals > 6 ? 6 : coinInfo.decimals)
    coinInfo.balance = res
    return [coinInfo]
  }

  return {
    connectUnisat,
    getUnisatBalance
  }
}