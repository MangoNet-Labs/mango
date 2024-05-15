import { MgoClient } from '@mgonetwork/mango.js/client';
import { coinList, fullNode, contractAddress } from '@/utils/info';
import { bigToSmallFixed0, bigNumberComputed, BcType, getPointSix } from './web3';
import { TransactionBlock } from '@mgonetwork/mango.js/transactions';
import { AmountOutType, AmountInType } from '@/interface';
import { bcs } from '@mgonetwork/mango.js/bcs';

const client = new MgoClient({ url: fullNode })

export const getBalance = async (userAddress: string) => {
  const balanceList = await client.getAllBalances({ owner: userAddress })
  for (let i = 0; i < coinList.length; i++) {
    const ele = coinList[i];
    for (let j = 0; j < balanceList.length; j++) {
      if (ele.address == balanceList[j].coinType) {
        ele.balance = bigToSmallFixed0(balanceList[j].totalBalance, ele.coin.decimal)
      }
    }
  }
  return coinList
}

export const getTwoCoins = (str: string) => {
  const pattern = new RegExp(`<(.*?)>`);
  const matches = str.match(pattern);
  if (matches) {
    const coinstr = matches[1]
    const arr = coinstr.split(',')
    const arr1 = arr[1].slice(1, arr[1].length)
    return [arr[0], arr1]
  } else {
    throw new Error('error!')
  }
}

export function toHexString(byteArray: number[]) {
  return Array.from(byteArray, function (byte) {
    return ('0' + (byte & 0xFF).toString(16)).slice(-2);
  }).join('')
}


// const poolObj = await client.getObject({ id: poolId, options: { showType: true } })
// if (poolObj.data?.type) {
//   const res = getTwoCoins(poolObj.data.type)
//   console.log(res);
//   let li = coinInfo
//   for (let i = 0; i < li.length; i++) {
//     const element = li[i];
//     if (element?.address === res[0]) {
//       element.isMain = true
//     }
//   }
//   setCoinInfo(li)
// }

export const getAmountOut = async ({
  typeArg,
  poolId,
  is_a_to_b,
  amountIn,
  userAddress,
  decimal
}: AmountOutType) => {
  const txb = new TransactionBlock()
  txb.moveCall({
    target: `${contractAddress}::amm_router::compute_out`,
    typeArguments: typeArg,
    arguments: [txb.object(poolId), txb.pure(amountIn), txb.pure(is_a_to_b)]
  })
  const { results } = await client.devInspectTransactionBlock({
    sender: userAddress,
    transactionBlock: txb
  })
  if (!results) {
    throw new Error('Insufficient pool balance!')
  }
  const res = bcs.de('u64', Uint8Array.from(results![0].returnValues![0][0]))
  const res1 = bigToSmallFixed0(res, decimal)
  return res1
}

export const getAmountIn = async ({
  typeArg,
  poolId,
  is_a_to_b,
  amountOut,
  userAddress,
  decimal
}: AmountInType) => {
  const txb = new TransactionBlock()
  txb.moveCall({
    target: `${contractAddress}::amm_router::compute_in`,
    typeArguments: typeArg,
    arguments: [txb.object(poolId), txb.pure(amountOut), txb.pure(is_a_to_b)]
  })
  const { results } = await client.devInspectTransactionBlock({
    sender: userAddress,
    transactionBlock: txb
  })
  if (!results) {
    throw new Error('Insufficient pool balance!')
  }
  const res = bcs.de('u64', Uint8Array.from(results![0].returnValues![0][0]))
  const res1 = bigToSmallFixed0(res, decimal)
  return res1
}

export const OneMainGetSecond = (one: string | number, two: string | number) => {
  return bigNumberComputed(two, one, BcType.div)
}

export const getSlippageEndValue = (value: string | number, sli: number, dec: number) => {
  const se = sli / 100
  const res1 = bigNumberComputed(value, se, BcType.times)
  const res2 = bigNumberComputed(value, res1, BcType.minus)
  const res3 = getPointSix(res2, dec)
  return res3
}

export const getSlippageAddValue = (value: string | number, sli: number, dec: number) => {
  const se = sli / 100
  const res1 = bigNumberComputed(value, se, BcType.times)
  const res2 = bigNumberComputed(value, res1, BcType.plus)
  const res3 = getPointSix(res2, dec)
  return res3
}
