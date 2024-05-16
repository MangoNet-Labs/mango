import Web3 from 'web3'
import BigNumber from "bignumber.js";
import { token20_abi } from '@/utils/abi'
import { ChainListType, CoinListType } from '@/interface'
import { bigToSmallFixed0, getPointSix } from '@/utils/web3'
import { Toast } from 'antd-mobile';

export function useWeb3Js() {
  async function getAccount() {
    try {
      if (window.ethereum) {
        const accounts = await window.ethereum.request({ method: 'eth_requestAccounts' })
        return accounts[0]
      } else {
        throw new Error('Please install MetaMask or other Ethereum wallet!')
      }
    } catch (error) {
      throw error;
    }
  }
  async function getCoinsBalance(info: ChainListType, userAddress: string, coinList: CoinListType[]) {
    try {
      const web3 = new Web3(info.rpc)
      const asyncOperations = coinList.map(async (item) => {
        if (item.chain === info.chainName) {
          if (item.isEther) {
            const ba = await web3.eth.getBalance(userAddress)
            const res = getPointSix(bigToSmallFixed0(ba.toString(), item.decimals))
            return {
              ...item,
              balance: res
            }
          } else {
            const method = new web3.eth.Contract(token20_abi, item.contract).methods
            const ba = await method.balanceOf(userAddress).call() as BigInt
            const res = getPointSix(bigToSmallFixed0(ba.toString(), item.decimals))
            return {
              ...item,
              balance: res
            }
          }
        } else {
          return item
        }
      })
      const results = await Promise.all(asyncOperations)
      return results
    } catch (error) {
      throw error
    }
  }
  async function getNetworkId() {
    try {
      if (window.ethereum) {
        const id = await window.ethereum.request({ method: 'eth_chainId' })
        return Number(id)
      } else {
        throw new Error('Please install MetaMask or other Ethereum wallet')
      }
    } catch (error) {
      throw error;
    }
  }
  async function changeNetWork(info: ChainListType) {
    const web3 = new Web3(window.ethereum)
    try {
      await window.ethereum.request({
        method: 'wallet_switchEthereumChain',
        params: [{
          chainId: web3.utils.numberToHex(info.chainId) 
        }]
      })
    } catch (e: any) {
      if (e.code === 4902) {
        try {
          await window.ethereum.request({
            method: 'wallet_addEthereumChain',
            params: [
              {
                chainId: web3.utils.numberToHex(info.chainId),
                chainName: info.chainName,
                nativeCurrency: {
                  name: info.nativeCurrency.name,
                  symbol: info.nativeCurrency.symbol,
                  decimals: Number(info.nativeCurrency.decimals)
                },
                rpcUrls: [info.rpc],
                blockExplorerUrls: [info.blockUrl],
                iconUrls: [info.logo]
              }
            ]
          })
        } catch (ee) {
          //
        }
      } else if (e.code === 4001) return
    }
  }
  async function allowQuota(quota: string, contract_from: string, user_address: string, contract_to: string) {
    const web3 = new Web3(window.ethereum)
    let f_max_num = new BigNumber(2).pow(256).minus(1);
    const max_num = f_max_num.toString(16)
    const contract_proto = new web3.eth.Contract(token20_abi, contract_from, {
      from: user_address
    });
    const isAllowance: any = await contract_proto.methods.allowance(user_address, contract_to).call();
    console.log(isAllowance);
    if (new BigNumber(isAllowance).gte(quota)) {
      return true
    } else {
      await contract_proto.methods.approve(contract_to, '0x' + max_num).send()
      const isAllowance: any = await contract_proto.methods.allowance(user_address, contract_to).call();
      if (new BigNumber(isAllowance).gte(quota)) {
        return true
      } else {
        Toast.show('Insufficient authorization amount, please try again!')
      }
    }
  }

  return {
    getAccount,
    getCoinsBalance,
    getNetworkId,
    changeNetWork,
    allowQuota
  }
}