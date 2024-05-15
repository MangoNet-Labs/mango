import BigNumber from "bignumber.js";

export const bigToSmallFixed0 = (from: number | string, wei: number = 9) => {
  const froms = new BigNumber(from)
  const x = new BigNumber(10).pow(-wei)
  return froms.times(x).toFixed()
}

export const smallToBig = (from: number | string, wei: number = 9) => {
  const froms = new BigNumber(from)
  const x = new BigNumber(10).pow(wei)
  return froms.times(x).toFixed()  
}

export const smallToBigFixed0 = (from: number | string, wei: number = 9) => {
  const froms = new BigNumber(from)
  const x = new BigNumber(10).pow(wei)
  return froms.times(x).toFixed(0) 
}

// bigNumber 
export enum BcType {
  plus = 'plus', 
  minus = 'minus', 
  times = 'times', 
  div = 'div' 
}
export const bigNumberComputed = (one: number | string, two: number | string, method: BcType) => {
  const firstNum = new BigNumber(one)
  const second = new BigNumber(two)
  if (method == BcType.plus) {
    return firstNum.plus(second).toFixed()
  } else if (method == BcType.minus) {
    return firstNum.minus(second).toFixed()
  } else if (method == BcType.times) {
    return firstNum.times(second).toFixed()
  } else if (method == BcType.div) {
    return firstNum.div(second).toFixed()
  } else {
    return '0'
  }
}


export const getPointSix = (res: string, num: number = 6) => {
  let b = res.indexOf('.')
  if (b == -1) {
    return res
  } else {
    return res.slice(0, b + num + 1)
  }
}


export const bigGt = (one: number | string, two: number | string) => {
  const x = new BigNumber(one)
  const y = new BigNumber(two)
  return x.gt(y)
}

export const bigLt = (one: number | string, two: number | string) => {
  const x = new BigNumber(one)
  const y = new BigNumber(two)
  return x.lt(y)
}


export function calculateSquareRoot(a:string | number, b: string | number) {
  const x = new BigNumber(a)
  const y = new BigNumber(b)
  // const result = x.times(y).squareRoot().minus(10).max(0);
  const result = x.times(y).squareRoot();
  return bigLt(result.toFixed(),10)
}