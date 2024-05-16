import styled from 'styled-components';
import img_help from '@/assets/image/help.png';
import img_copy from '@/assets/image/copy.png';
import img_cg from '@/assets/image/cg.png';
import { DownOutline, EditSOutline } from 'antd-mobile-icons'
import { Pt, FlexBetween, FlexStart, FlexCenter, TextGreen, TextCommon, GridCommon, Image, FlexEnd, TextWhite } from '@/assets/style/common';
import { Popup, DotLoading, Toast } from 'antd-mobile'
import { useState, useEffect } from 'react';
import '@/assets/style/common.css';
import { useWalletKit } from '@mgonetwork/wallet-kit';
import { TransactionBlock } from '@mgonetwork/mango.js/transactions';
import { MgoClient } from '@mgonetwork/mango.js/client';
import { coinList, fullNode, contractAddress, factoryAddress, global_pause_status } from '@/utils/info';
import { getBalance, getAmountOut, toHexString, getAmountIn, OneMainGetSecond, getSlippageEndValue, getSlippageAddValue } from '@/utils/fc';
import { CoinInfoType } from '@/interface';
import Copy from '@/components/Copy';
import { smallToBig, getPointSix, bigGt } from '@/utils/web3';

//#region
const TbOuterBox = styled.div`
  padding: 16px;
  display: grid;
  row-gap: 8px;
`
const TbInputInfoButton = styled.button`
  background-color: transparent;
  font-size: 16px;
  border: 0px;
  display: flex;
  justify-content: center;
  align-items: center;
  cursor: pointer;
  outline: 0px;
  height: 32px;
  padding: 0 4px;
  gap: 6px;
  &:active:not(:disabled){
    opacity: 0.85;
    transform: translateY(1px);
    box-shadow: none;
  }
`
export const TbInputOutBox = styled.label`
  background-color: #212833;
  border-radius: 16px;
  box-shadow: inset 0px 2px 2px -1px rgba(74,74,104,.1);
  height: 75px;
  padding: 12px 16px;
`
export const TbInput = styled.input.attrs({
  type: 'number'
})`
  text-align: right;
  border: 0px;
  outline: 0px;
  background: transparent;
  width: 100%;
  font-size: 16px;
  font-weight: bold;
  color: #fff;
`
export const TbInputUsd = styled.div`
  color: #fff;
  font-size: 12px;
  text-align: right;
  height: 18px;
  padding-top: 2px;
`
const TbPriceBoxText1 = styled.div`
  color: #fff;
  font-size: 12px;
  font-weight: bold;
  white-space: nowrap;
`
const TbPriceBoxEndSpan = styled.span`
  color: #fff;
  font-size: 14px;
`
export const SubmitButton = styled.button.attrs<{
  $isDisabled?: boolean
}>(({ $isDisabled }) => ({
  disabled: $isDisabled ?? false
}))`
  height: 48px;
  border: 0px;
  outline: 0px;
  cursor: pointer;
  box-shadow: rgba(14, 14, 44, 0.4) 0px -1px 0px 0px inset;
  background: #fff;
  color: #02070F;
  font-size: 16px;
  font-weight: bold;
  width: 100%;
  border-radius: 16px;
  &:disabled{
    opacity: 0.6;
  }
  &:active:not(:disabled){
    opacity: 0.85;
    transform: translateY(1px);
    box-shadow: none;
  }
`
export const PopOutBox = styled.div`
  background: #131821;
  height: 100%;
  .poptitle{
    padding: 12px 12px 20px 16px;
    color: #fff;
    font-size: 16px;
    font-weight: bold;
    border-bottom: 1px solid rgba(151, 151, 151, 0.7);
  }
  .poplist{
    padding: 5px 24px 24px;
    .popitem{
      padding: 5px 0;
      display: flex;
      justify-content: space-between;
      align-items: center;
      gap: 10px;
      img{
        width: 24px;
        height: 24px;
      }
    }
    .popitemopa{
      opacity: 0.5;
    }
  }
`
const Slippage = styled.input.attrs({
  type: 'number',
  id: 'slippage'
})`
  border: none;
  outline: none;
  text-align: right;
  font-size: 16px;
  color: #81D8CF;
  flex: auto;
  width: 100%;
  background: transparent;
`
//#endregion

export default function TransactionBody() {
  const [fromPopupShow, setFromPopupShow] = useState(false) 
  const [slippage, setSlippage] = useState(0.5) 
  const { currentAccount, signAndExecuteTransactionBlock } = useWalletKit();
  const client = new MgoClient({ url: fullNode })

  const [coinsList, setCoinsList] = useState(coinList)
  const getAllBalance = async () => {
    if (!currentAccount?.address) return
    try {
      const res = await getBalance(currentAccount.address)
      setCoinsList(res)
    } catch (error) {
      console.log(error);
    }
  }

  useEffect(() => {
    if (!currentAccount) return
    getAllBalance()
  }, [currentAccount])

  type PairType = 'coin1' | 'coin2'
  const [cointype, setCoinType] = useState<PairType>()
  const chooseCoinType = (type: PairType) => {
    setCoinType(type)
    setFromPopupShow(true)
  }

  const [coinInfo, setCoinInfo] = useState<[CoinInfoType?, CoinInfoType?]>([])

  const chooseAndSetInfo = (item: CoinInfoType) => {
    if (item.disabled) return
    if (cointype == 'coin1') {
      setCoinInfo([item, coinInfo[1]])
    } else {
      setCoinInfo([coinInfo[0], item])
    }
    setFromPopupShow(false)
  }

  const [isAtoB, setIsAtoB] = useState(false)
  const [poolAddress, setPoolAddress] = useState('')

  const checkPool = async () => {
    if (!currentAccount?.address || !coinInfo[0] || !coinInfo[1]) return
    try {
      const txb = new TransactionBlock()
      txb.moveCall({
        target: `${contractAddress}::amm_swap::get_pool_id`,
        typeArguments: [coinInfo[0].address, coinInfo[1].address],
        arguments: [txb.object(factoryAddress)]
      })
      const { results } = await client.devInspectTransactionBlock({
        sender: currentAccount.address,
        transactionBlock: txb
      })
      if (!results) {
        throw new Error('error!')
      }
      const poolId = toHexString(results![0].returnValues![0][0])
      if (poolId === '0000000000000000000000000000000000000000000000000000000000000000') {
        throw new Error('No trading pair!')
      }
      setPoolAddress(poolId)
      console.log(results![0].returnValues![1][0][0] === 1);
      setIsAtoB(results![0].returnValues![1][0][0] === 1)
    } catch (error: any) {
      console.log(error);
      Toast.show({
        content: error.message
      })
      setPoolAddress('')
    }
  }

  useEffect(() => {
    if (!coinInfo.length) return
    let li = coinsList.map(item => ({
      ...item,
      disabled: false
    }))
    for (let i = 0; i < li.length; i++) {
      const element = li[i];
      const coinInfoItem = coinInfo.find(ele => ele?.address === element.address)
      if (coinInfoItem) {
        element.disabled = true
      }
    }
    setCoinsList(li)
    if (coinInfo[0] && coinInfo[1]) {
      checkPool()
    }
  }, [coinInfo])

  const [mainValue, setMainValue] = useState<number | string>('')
  const [secondValue, setSecondValue] = useState<number | string>('')
  const [oneGetTwo, setOneGetTwo] = useState('')
  const [isMainInput, setIsMainInput] = useState<boolean>()  
  const [minimum, setMinimum] = useState('') 
  const [maxsold, setMaxsold] = useState('') 
  const mainValueChange = async (e: any) => {
    setMainValue(e.target.value)
    setIsMainInput(true)
    if (!currentAccount?.address || !coinInfo[0] || !coinInfo[1]) return
    if (!e.target.value || e.target.value == 0) {
      setSecondValue(0)
      return
    }
    try {
      const result = await getAmountOut({
        amountIn: smallToBig(e.target.value, coinInfo[0].coin.decimal),
        typeArg: isAtoB ? [coinInfo[0].address, coinInfo[1].address] : [coinInfo[1].address, coinInfo[0].address],
        poolId: poolAddress,
        is_a_to_b: isAtoB,
        userAddress: currentAccount.address,
        decimal: coinInfo[1].coin.decimal
      })
      console.log(result);
      if (!e.target.value || e.target.value == 0) {
        setSecondValue(0)
      } else {
        setSecondValue(result)
      }
      const gt = getPointSix(OneMainGetSecond(e.target.value, result))
      setOneGetTwo(gt)
      const minNum = getSlippageEndValue(result, slippage, coinInfo[1].coin.decimal)
      setMinimum(minNum)
    } catch (error: any) {
      console.log(error);
      Toast.show({
        content: error.message
      })
    }
  }
  const secondValueChange = async (e: any) => {
    setSecondValue(e.target.value)
    setIsMainInput(false)
    if (!currentAccount?.address || !coinInfo[0] || !coinInfo[1]) return
    if (!e.target.value || e.target.value == 0) {
      setMainValue(0)
      return
    }
    try {
      const result = await getAmountIn({
        amountOut: smallToBig(e.target.value, coinInfo[1].coin.decimal),
        typeArg: isAtoB ? [coinInfo[0].address, coinInfo[1].address] : [coinInfo[1].address, coinInfo[0].address],
        poolId: poolAddress,
        is_a_to_b: isAtoB,
        userAddress: currentAccount.address,
        decimal: coinInfo[0].coin.decimal
      })
      console.log(result);
      if (!e.target.value || e.target.value == 0) {
        setMainValue(0)
      } else {
        setMainValue(result)
      }
      const maxNum = getSlippageAddValue(result, slippage, coinInfo[0].coin.decimal)
      setMaxsold(maxNum)
    } catch (error: any) {
      console.log(error);
      Toast.show({
        content: error.message
      })
    }
  }

  const [isLoading, setIsLoading] = useState(false)
  const submit = async () => {
    if (!currentAccount?.address || !coinInfo[0] || !coinInfo[1]) return

    if ((isMainInput === true && bigGt(mainValue, coinInfo[0].balance)) || (isMainInput === false && bigGt(maxsold, coinInfo[0].balance))) {
      Toast.show({
        content: 'Insufficient balance!'
      })
      return
    }

    try {
      setIsLoading(true)
      const txb = new TransactionBlock()

      let coins: any;
      let mgoCoins: any;
      if (coinInfo[0].address === '0x2::mgo::MGO') {
        [mgoCoins] = txb.splitCoins(
          txb.gas,
          isMainInput ? [txb.pure(smallToBig(mainValue, coinInfo[0].coin.decimal))] : [txb.pure(smallToBig(maxsold, coinInfo[0].coin.decimal))]
        )
        console.log(mgoCoins);
      } else {
        coins = await client.getCoins({
          owner: currentAccount.address,
          coinType: coinInfo[0].address
        })
        console.log(coins.data);
        if (!coins.data.length) return
        if (coins.data.length > 1) {
          txb.mergeCoins(txb.object(coins.data[0]['coinObjectId']), coins.data.slice(1).map((e: any) => txb.object(e['coinObjectId'])))
        }
      }
      let fcName: string;
      if (isAtoB) {
        fcName = isMainInput ? 'swap_exact_coinA_for_coinB' : 'swap_coinA_for_exact_coinB'
      } else {
        fcName = isMainInput ? 'swap_exact_coinB_for_coinA' : 'swap_coinB_for_exact_coinA'
      }
      txb.moveCall({
        target: `${contractAddress}::amm_script::${fcName}`,
        typeArguments: isAtoB ? [coinInfo[0].address, coinInfo[1].address] : [coinInfo[1].address, coinInfo[0].address],
        arguments: [
          txb.object(poolAddress),
          txb.object(global_pause_status),
          coinInfo[0].address === '0x2::mgo::MGO' ? mgoCoins : txb.object(coins.data[0]['coinObjectId']),
          isMainInput ? txb.pure(smallToBig(mainValue, coinInfo[0].coin.decimal)) : txb.pure(smallToBig(maxsold, coinInfo[0].coin.decimal)),
          isMainInput ? txb.pure(smallToBig(minimum, coinInfo[1].coin.decimal)) : txb.pure(smallToBig(secondValue, coinInfo[1].coin.decimal))
        ]
      })
      console.log(txb);
      const result = await signAndExecuteTransactionBlock({
        transactionBlock: txb,
      })
      console.log(result);
      setMainValue('')
      setSecondValue('')
      getAllBalance()
      setIsLoading(false)
      Toast.show({
        content: 'success!'
      })
    } catch (error) {
      setIsLoading(false)
      console.log(error);
    }
  }

  const slippageChange = (e: any) => {
    setSlippage(e.target.value)
    if (!coinInfo[0] || !coinInfo[1]) return
    if (isMainInput === true) {
      const result = getSlippageEndValue(secondValue, e.target.value, coinInfo[1].coin.decimal)
      console.log(result);
      setMinimum(result)
    } else if (isMainInput === false) {
      const result = getSlippageAddValue(mainValue, e.target.value, coinInfo[0].coin.decimal)
      console.log(result);
      setMaxsold(result)
    }
  }

  const changeCoinInfo = () => {
    if (!coinInfo[0] || !coinInfo[1]) return
    const coinobj: [CoinInfoType?, CoinInfoType?] = [coinInfo[1], coinInfo[0]]
    setCoinInfo(coinobj)
  }

  useEffect(() => {
    if (!coinInfo[0] || !coinInfo[1]) return
    if (isMainInput === true) {
      mainValueChange({
        target: {
          value: mainValue
        }
      })
    } else if (isMainInput === false) {
      secondValueChange({
        target: {
          value: secondValue
        }
      })
    }
  }, [isAtoB, poolAddress])

  return (
    <>
      <TbOuterBox>
        <GridCommon $rowgap={4}>
          <FlexStart $gap={8}>
            <TbInputInfoButton onClick={() => chooseCoinType('coin1')}>
              <Image $width='24px' src={coinInfo[0]?.icon ?? img_help} alt="" />
              <TextCommon>{coinInfo[0]?.symbol ?? '--'}</TextCommon>
              <DownOutline color='#DAFCF8' />
            </TbInputInfoButton>
            <Copy value={coinInfo[0]?.address ?? 'Please select currency first'}>
              <Image $width='16px' src={img_copy} alt="" />
            </Copy>
          </FlexStart>
          <TbInputOutBox>
            <TbInput value={mainValue} onInput={mainValueChange} />
            <TbInputUsd></TbInputUsd>
          </TbInputOutBox>
        </GridCommon>
        <FlexCenter>
          <Image src={img_cg} $width='24px' onClick={changeCoinInfo} />
        </FlexCenter>
        <GridCommon $rowgap={4}>
          <FlexStart $gap={8}>
            <TbInputInfoButton onClick={() => chooseCoinType('coin2')}>
              <Image $width='24px' src={coinInfo[1]?.icon ?? img_help} alt="" />
              <TextCommon>{coinInfo[1]?.symbol ?? '--'}</TextCommon>
              <DownOutline color='#DAFCF8' />
            </TbInputInfoButton>
            <Copy value={coinInfo[1]?.address ?? 'Please select currency first'}>
              <Image $width='16px' src={img_copy} alt="" />
            </Copy>
          </FlexStart>
          <TbInputOutBox>
            <TbInput value={secondValue} onInput={secondValueChange} />
            <TbInputUsd></TbInputUsd>
          </TbInputOutBox>
        </GridCommon>
        {
          mainValue && secondValue ? <FlexBetween>
            <TbPriceBoxText1>Price</TbPriceBoxText1>
            <FlexEnd $gap={5}>
              <TbPriceBoxEndSpan>1 {coinInfo[0]?.symbol} ≈ {oneGetTwo ?? 0} {coinInfo[1]?.symbol}</TbPriceBoxEndSpan>
              {/* <UndoOutline /> */}
            </FlexEnd>
          </FlexBetween> : <></>
        }
        <FlexBetween>
          <FlexStart $gap={5}>
            <TbPriceBoxText1>Slippage Tolerance</TbPriceBoxText1>
            <label htmlFor="slippage">
              <EditSOutline color='#81D8CF' />
            </label>
          </FlexStart>
          <Slippage value={slippage} onChange={slippageChange} />
          <TextGreen>%</TextGreen>
        </FlexBetween>
        <SubmitButton $isDisabled={!currentAccount?.address || !mainValue || mainValue == 0 || !secondValue || isLoading || !poolAddress} onClick={submit}>{
          !currentAccount?.address ? 'please Connect Wallet' : !poolAddress ? 'no pool' : isLoading ? <DotLoading color='#02070F' /> : 'Submit'
        }</SubmitButton>
        <Pt $pt={30}>
          {
            isMainInput === true ? <FlexBetween>
              <TextWhite $size={14}>Minimum received</TextWhite>
              <TextCommon $size={14}>{minimum} {coinInfo[1]?.symbol}</TextCommon>
            </FlexBetween> : isMainInput === false ? <FlexBetween>
              <TextWhite $size={14}>Max sold</TextWhite>
              <TextCommon $size={14}>{maxsold} {coinInfo[0]?.symbol}</TextCommon>
            </FlexBetween> : <></>
          }
        </Pt>
      </TbOuterBox>
      <Popup
        visible={fromPopupShow}
        onMaskClick={() => {
          setFromPopupShow(false)
        }}
        onClose={() => {
          setFromPopupShow(false)
        }}
        bodyStyle={{
          height: '90vh'
        }}
        showCloseButton
        closeOnSwipe
      >
        <PopOutBox>
          <div className='poptitle'>Select a Token</div>
          <div className='poplist'>
            {
              coinsList.map(item => (
                <div className={item.disabled ? 'popitem popitemopa' : 'popitem'} key={item.address} onClick={() => chooseAndSetInfo(item)}>
                  <img src={item.icon} alt="" />
                  <div style={{ flex: 'auto' }}>
                    <TextCommon $size={14} $weight={600}>{item.symbol}</TextCommon>
                    <TextWhite $size={12}>{item.Introduction}</TextWhite>
                  </div>
                  <TextCommon>{item.balance}</TextCommon>
                </div>
              ))
            }
          </div>
        </PopOutBox>
      </Popup>
    </>
  )
}
