import BigNumber from "bignumber.js";

// bigNumber 计算方法
export enum BcType {
  plus = 'plus', //加
  minus = 'minus', //减
  times = 'times', //乘
  div = 'div' //除
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
export const bigToSmall = (from: number | string) => {
  const froms = new BigNumber(from)
  const x = new BigNumber(10).pow(-18)
  return froms.times(x).toFixed()
}
export const smallToBig = (from: number | string, wei: number = 18) => {
  const froms = new BigNumber(from)
  const x = new BigNumber(10).pow(wei)
  return froms.times(x).toFixed()  //toFixed() 转为字符串 可以防止位数过大被转为科学计数法 且可以传入参数保留多少位小数 不传入则保留原位数
}
//是否大于
export const bigGt = (one: number | string, two: number | string) => {
  const x = new BigNumber(one)
  const y = new BigNumber(two)
  return x.gt(y)
}
//是否大于等于
export const bigGte = (one: number | string, two: number | string) => {
  const x = new BigNumber(one)
  const y = new BigNumber(two)
  return x.gte(y)
}
//是否小于
export const bigLt = (one: number | string, two: number | string) => {
  const x = new BigNumber(one)
  const y = new BigNumber(two)
  return x.lt(y)
}
//是否小于等于
export const bigLte = (one: number | string, two: number | string) => {
  const x = new BigNumber(one)
  const y = new BigNumber(two)
  return x.lte(y)
}

//截取小数点后位数(默认2)
export const getPointTwo = (res: string, num: number = 2) => {
  let b = res.indexOf('.')
  if (b == -1) {
    return res
  } else {
    return res.slice(0, b + num + 1)
  }
}

//处理小数点后0过多问题 eg: 0.00000253 => 0.{5}253
export const formatZeroToNumber = (value: string | number) => {
  let a = value.toString()
  let b = a.indexOf('.')
  let c = 0;
  if (b !== -1) {
    for (let i = 0; i < a.length; i++) {
      if (i > b) {
        if (a[i] != '0') {
          c = i;
          break;
        }
      }
    }
    let d = c - b - 1
    if (d == 0) {
      return a;
    } else {
      return `0.{${c - b - 1}}${a.slice(c, a.length)}`
    }
  } else {
    return a;
  }
}

//处理位数过大转为kmb
export const formatBigNumberToletter = (value: string | number) => {
  const deck = '1000'
  const decm = '1000000'
  const decb = '1000000000'

  let a = value.toString();
  let res;
  if (bigGte(a, deck) && bigLt(a, decm)) {
    let b = bigNumberComputed(a, deck, BcType.div)
    res = getPointTwo(b);
    return `${res}K`
  } else if (bigGte(a, decm) && bigLt(a, decb)) {
    let b = bigNumberComputed(a, decm, BcType.div)
    res = getPointTwo(b);
    return `${res}M`
  } else if (bigGte(a, decb)) {
    let b = bigNumberComputed(a, decb, BcType.div)
    res = getPointTwo(b);
    return `${res}B`
  } else {
    return a
  }
}