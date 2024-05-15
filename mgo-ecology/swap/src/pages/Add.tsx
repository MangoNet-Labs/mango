import styled from 'styled-components';
import { SwapOutBox, SwapTransationBox, SwapTransationBoxLess1Px } from './Swap';
import { PopOutBox, SubmitButton, TbInputOutBox, TbInput, TbInputUsd } from '@/components/TransactionBody';
import { Image, FlexBetween, TextCommon, GridCommon, FlexCenter, Pt, FlexStart, TextWhite } from '@/assets/style/common';
import { ThOuterPadding, ThText1 } from '@/components/TransactionHeader';
import img_back from '@/assets/image/back.png';
import { coinList, contractAddress, factoryAddress, fullNode, global_pause_status, lp_front } from '@/utils/info';
import { useState, useEffect } from 'react';
import img_downarr from '@/assets/image/downarr.png';
import img_add2 from '@/assets/image/add.png';
import { Popup, DotLoading, Toast } from 'antd-mobile'
import { CoinInfoType } from '@/interface';
import img_help from '@/assets/image/help.png';
import { getBalance, toHexString } from '@/utils/fc';
import { useWalletKit } from '@mgonetwork/wallet-kit';
import { useNavigate } from 'react-router-dom';
import { TransactionBlock } from '@mgonetwork/mango.js/transactions';
import { MgoClient } from '@mgonetwork/mango.js/client';
import { bcs } from '@mgonetwork/mango.js/bcs';
import { bigToSmallFixed0, smallToBig, bigGt, getPointSix, calculateSquareRoot } from '@/utils/web3';

//#region 
const AddStyle = styled(ThOuterPadding)`
  padding: 16px;
`
const AddPairItem = styled.div`
  background: #212833;
  width: 100%;
  height: 40px;
  border-radius: 16px;
  box-shadow: rgba(14, 14, 44, 0.4) 0px -1px 0px 0px inset;
  opacity: 1;
  padding: 0;
  outline: 0px;
  border: 0px;
  /* transition: background-color 0.2s ease 0s, opacity 0.2s ease 0s; */
  min-width: 136px;
  &:active:not(:disabled){
    opacity: 0.85;
    transform: translateY(1px);
    box-shadow: none;
  }
`
const AddPd = styled(FlexBetween)`
  padding: 0 16px;
  height: 100%;
`
const LiqStyle = styled.div`
  padding: 16px;
`

//#endregion

export default function Add() {
  const client = new MgoClient({ url: fullNode })
  const router = useNavigate()
  const { currentAccount, signAndExecuteTransactionBlock } = useWalletKit();
  const [coinsList, setCoinsList] = useState(coinList)
  const [fromPopupShow, setFromPopupShow] = useState(false) 
  const [coinInfo, setCoinInfo] = useState<[CoinInfoType?, CoinInfoType?]>([])
  type PairType = 'coin1' | 'coin2'
  const [cointype, setCoinType] = useState<PairType>()
  const chooseCoinType = (type: PairType) => {
    setCoinType(type)
    setFromPopupShow(true)
  }

  const chooseAndSetInfo = (item: CoinInfoType) => {
    if (item.disabled) return
    if (cointype == 'coin1') {
      setCoinInfo([item, coinInfo[1]])
    } else {
      setCoinInfo([coinInfo[0], item])
    }
    setFromPopupShow(false)
  }

  const [poolAddress, setPoolAddress] = useState('')
  const [isAtoB, setIsAtoB] = useState(false)
  
  const checkPool = async () => {
    if (!currentAccount?.address || !coinInfo[0] || !coinInfo[1]) return
    try {
      setIsLoading(true)
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
      console.log(results![0].returnValues![1][0][0] === 1);
      setIsAtoB(results![0].returnValues![1][0][0] === 1)
      setPoolAddress(poolId)
      setIsLoading(false)
    } catch (error: any) {
      setIsLoading(false)
      console.log(error);
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

  const backPage = () => {
    router(-1)
  }

  const [isLoading, setIsLoading] = useState(false)
  const [showLiquidity, setShowLiquidity] = useState(false)

  const [mainValue, setMainValue] = useState<number | string>('')
  const [secondValue, setSecondValue] = useState<number | string>('')
  const [isMainInput, setIsMainInput] = useState<boolean>()  
  const mainValueChange = async (e: any) => {
    setMainValue(e.target.value)
    setIsMainInput(true)
    if (!currentAccount?.address || !coinInfo[0] || !coinInfo[1]) return
    if (!e.target.value || e.target.value == 0) {
      setSecondValue(0)
      return
    }
    try {
      const txb = new TransactionBlock()
      txb.moveCall({
        target: `${contractAddress}::amm_utils::quote`,
        arguments: [
          txb.pure(smallToBig(e.target.value, coinInfo[0].coin.decimal)),
          txb.pure(smallToBig(isAtoB ? reservesA : reservesB, coinInfo[0].coin.decimal)),
          txb.pure(smallToBig(isAtoB ? reservesB : reservesA, coinInfo[1].coin.decimal))
        ]
      })
      const { results } = await client.devInspectTransactionBlock({
        sender: currentAccount.address,
        transactionBlock: txb
      })
      if (!results) {
        throw new Error('error!')
      }
      const res = bcs.de('u64', Uint8Array.from(results![0].returnValues![0][0])) * 1.05
      const result = getPointSix(bigToSmallFixed0(res, coinInfo[1].coin.decimal), coinInfo[1].coin.decimal)
      console.log(result);
      if (!e.target.value || e.target.value == 0) {
        setSecondValue(0)
      } else {
        setSecondValue(result)
      }
    } catch (error) {
      console.log(error);
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
      const txb = new TransactionBlock()
      txb.moveCall({
        target: `${contractAddress}::amm_utils::quote`,
        arguments: [
          txb.pure(smallToBig(e.target.value, coinInfo[1].coin.decimal)),
          txb.pure(smallToBig(isAtoB ? reservesB : reservesA, coinInfo[1].coin.decimal)),
          txb.pure(smallToBig(isAtoB ? reservesA : reservesB, coinInfo[0].coin.decimal))
        ]
      })
      const { results } = await client.devInspectTransactionBlock({
        sender: currentAccount.address,
        transactionBlock: txb
      })
      if (!results) {
        throw new Error('error!')
      }
      const res = bcs.de('u64', Uint8Array.from(results![0].returnValues![0][0])) * 1.05
      const result = getPointSix(bigToSmallFixed0(res, coinInfo[0].coin.decimal), coinInfo[0].coin.decimal)
      console.log(result);
      if (!e.target.value || e.target.value == 0) {
        setMainValue(0)
      } else {
        setMainValue(result)
      }
    } catch (error) {
      console.log(error);
    }
  }

  const [initPoolLoading, setInitPoolLoading] = useState(false)
  const initPool = async () => {
    if (!currentAccount?.address || !coinInfo[0] || !coinInfo[1]) return
    try {
      setInitPoolLoading(true)
      const txb = new TransactionBlock()
      txb.moveCall({
        target: `${contractAddress}::amm_script::init_pool`,
        typeArguments: [coinInfo[0].address, coinInfo[1].address],
        arguments: [txb.object(factoryAddress)]
      })
      console.log(txb);
      const result = await signAndExecuteTransactionBlock({
        transactionBlock: txb,
      })
      console.log(result);
      await checkPool()
      setTimeout(() => {
        setInitPoolLoading(false)
      }, 1000);
      Toast.show({
        content: 'success!'
      })
    } catch (error) {
      setInitPoolLoading(false)
      console.log(error);
    }
  }

  const [reservesA, setReservesA] = useState('') 
  const [reservesB, setReservesB] = useState('') 
  const getReserves = async () => {
    if (!currentAccount?.address || !coinInfo[0] || !coinInfo[1]) return
    try {
      const txb = new TransactionBlock()
      txb.moveCall({
        target: `${contractAddress}::amm_swap::get_reserves`,
        typeArguments: isAtoB ? [coinInfo[0].address, coinInfo[1].address] : [coinInfo[1].address, coinInfo[0].address],
        arguments: [txb.object(poolAddress)]
      })
      const { results } = await client.devInspectTransactionBlock({
        sender: currentAccount.address,
        transactionBlock: txb
      })
      if (!results) {
        throw new Error('error!')
      }
      const res1 = bcs.de('u64', Uint8Array.from(results![0].returnValues![0][0]))
      const res2 = bcs.de('u64', Uint8Array.from(results![0].returnValues![1][0]))
      if (isAtoB) {
        const reserves1 = bigToSmallFixed0(res1, coinInfo[0].coin.decimal)
        setReservesA(reserves1)
        const reserves2 = bigToSmallFixed0(res2, coinInfo[1].coin.decimal)
        setReservesB(reserves2)
      } else {
        const reserves1 = bigToSmallFixed0(res1, coinInfo[1].coin.decimal)
        setReservesA(reserves1)
        const reserves2 = bigToSmallFixed0(res2, coinInfo[0].coin.decimal)
        setReservesB(reserves2)
      }
    } catch (error) {
      console.log(error);
    }
  }

  const [isFirst, setIsFirst] = useState(false)
  const getPoolInfo = async () => {
    try {
      // const txb = new TransactionBlock()
      const poolInfo: any = await client.getObject({
        id: poolAddress,
        options: {
          showContent: true
        }
      })
      console.log('locked', poolInfo.data?.content?.fields?.lp_locked);
      const result = poolInfo.data?.content?.fields?.lp_locked && poolInfo.data.content.fields.lp_locked == '10' ? false : true
      setIsFirst(result)
    } catch (error) {
      console.log(error);
    }
  }

  useEffect(() => {
    if (!poolAddress) return
    setMainValue('')
    setSecondValue('')
    getReserves()
    getLpBalance()
    getPoolInfo()
  }, [poolAddress])

  const [addLoading, setAddLoading] = useState(false)
  const addLiquidity = async () => {
    if (!currentAccount?.address || !mainValue || !secondValue || !coinInfo[0] || !coinInfo[1]) return

    if (bigGt(mainValue, coinInfo[0].balance) || bigGt(secondValue, coinInfo[1].balance)) {
      Toast.show({
        content: 'Insufficient balance!'
      })
      return
    }
    if (isFirst) {
      if (calculateSquareRoot(mainValue, secondValue)) {
        Toast.show({
          content: 'Value is too small!'
        })
        return
      }
    }

    try {
      setAddLoading(true)
      const txb = new TransactionBlock()

      let coin0Obj: any;
      let coin1Obj: any;
      if (coinInfo[0].address === '0x2::mgo::MGO') {
        [coin0Obj] = txb.splitCoins(
          txb.gas,
          [txb.pure(smallToBig(mainValue, coinInfo[0].coin.decimal))]
        )
      } else {
        const coins = await client.getCoins({
          owner: currentAccount.address,
          coinType: coinInfo[0].address
        })
        if (!coins.data.length) return
        if (coins.data.length > 1) {
          txb.mergeCoins(txb.object(coins.data[0]['coinObjectId']), coins.data.slice(1).map((e: any) => txb.object(e['coinObjectId'])))
        }
        coin0Obj = txb.object(coins.data[0]['coinObjectId'])
      }
      if (coinInfo[1].address === '0x2::mgo::MGO') {
        [coin1Obj] = txb.splitCoins(
          txb.gas,
          [txb.pure(smallToBig(secondValue, coinInfo[1].coin.decimal))]
        )
      } else {
        const coins = await client.getCoins({
          owner: currentAccount.address,
          coinType: coinInfo[1].address
        })
        if (!coins.data.length) return
        if (coins.data.length > 1) {
          txb.mergeCoins(txb.object(coins.data[0]['coinObjectId']), coins.data.slice(1).map((e: any) => txb.object(e['coinObjectId'])))
        }
        coin1Obj = txb.object(coins.data[0]['coinObjectId'])
      }

      const argA = isAtoB ? txb.pure(smallToBig(mainValue, coinInfo[0].coin.decimal)) : txb.pure(smallToBig(secondValue, coinInfo[1].coin.decimal))
      const argB = isAtoB ? txb.pure(smallToBig(secondValue, coinInfo[1].coin.decimal)) : txb.pure(smallToBig(mainValue, coinInfo[0].coin.decimal))
      txb.moveCall({
        target: `${contractAddress}::amm_script::add_liquidity`,
        typeArguments: isAtoB ? [coinInfo[0].address, coinInfo[1].address] : [coinInfo[1].address, coinInfo[0].address],
        arguments: [
          txb.object(poolAddress),
          txb.object(global_pause_status),
          isAtoB ? coin0Obj : coin1Obj,
          isAtoB ? coin1Obj : coin0Obj,
          argA,
          argB,
          (isAtoB && isMainInput) || (!isAtoB && !isMainInput) ? argA : txb.pure(0),
          (isAtoB && !isMainInput) || (!isAtoB && isMainInput) ? argB : txb.pure(0)
        ]
      })
      console.log(txb);
      const result = await signAndExecuteTransactionBlock({
        transactionBlock: txb,
      })
      console.log(result);
      setMainValue('')
      setSecondValue('')
      setTimeout(() => {
        getAllBalance()
        getReserves()
        setShowLiquidity(false)
        setAddLoading(false)
      }, 1000);
      Toast.show({
        content: 'success!'
      })
    } catch (error) {
      console.log(error);
      setAddLoading(false)
    }
  }

  const [showRemove, setShowRemove] = useState(false)
  const [lpValue, setLpVlaue] = useState('')
  const [removeLoading, setRemoveLoading] = useState(false)
  const [removeInfo, setRemoveInfo] = useState({
    amountA: '--',
    amountB: '--'
  })
  // setRemoveInfo((prevInfo) => ({
  //   ...prevInfo,
  //   amountB: 'newValue'
  // }))
  const lpValueChange = async (e: any) => {
    setLpVlaue(e.target.value)
    if (!currentAccount?.address || !coinInfo[0] || !coinInfo[1]) return
    if (!e.target.value || e.target.value == 0) {
      setRemoveInfo({
        amountA: '--',
        amountB: '--'
      })
      return
    }
    try {
      setRemoveLoading(true)
      const txb = new TransactionBlock()
      txb.moveCall({
        target: `${contractAddress}::amm_swap::lp_to_ab`,
        typeArguments: isAtoB ? [coinInfo[0].address, coinInfo[1].address] : [coinInfo[1].address, coinInfo[0].address],
        arguments: [
          txb.object(poolAddress),
          txb.pure(e.target.value)
        ]
      })
      const { results } = await client.devInspectTransactionBlock({
        sender: currentAccount.address,
        transactionBlock: txb
      })
      const res1 = bcs.de('u64', Uint8Array.from(results![0].returnValues![0][0]))
      const res2 = bcs.de('u64', Uint8Array.from(results![0].returnValues![1][0]))
      const resMain = isAtoB ? bigToSmallFixed0(res1, coinInfo[0].coin.decimal) : bigToSmallFixed0(res1, coinInfo[1].coin.decimal)
      const resSecond = isAtoB ? bigToSmallFixed0(res2, coinInfo[1].coin.decimal) : bigToSmallFixed0(res2, coinInfo[0].coin.decimal)
      console.log(resMain, '--', resSecond);
      if (!e.target.value || e.target.value == 0) {
        setRemoveInfo({
          amountA: '--',
          amountB: '--'
        })
      } else {
        setRemoveInfo({
          amountA: resMain,
          amountB: resSecond
        })
      }
      setRemoveLoading(false)
    } catch (error) {
      console.log(error);
      setRemoveLoading(false)
    }
  }

  const getCoinType = () => {
    if (!coinInfo[0] || !coinInfo[1]) return
    return `${lp_front}<${isAtoB ? coinInfo[0].address : coinInfo[1].address},${isAtoB ? coinInfo[1].address : coinInfo[0].address}>`
  }

  const [lpBalance, setLpBalance] = useState('')
  const getLpBalance = async () => {
    if (!currentAccount?.address || !coinInfo[0] || !coinInfo[1]) return
    try {
      const res = await client.getBalance({
        owner: currentAccount.address,
        coinType: getCoinType()
      })
      console.log(res.totalBalance);
      setLpBalance(res.totalBalance)
    } catch (error) {
      console.log(error);
    }
  }

  const removeLiquidity = async () => {
    if (!currentAccount?.address || !lpValue || !coinInfo[0] || !coinInfo[1]) return
    try {
      setRemoveLoading(true)
      const txb = new TransactionBlock()

      const coins = await client.getCoins({
        owner: currentAccount.address,
        coinType: getCoinType()
      })
      if (!coins.data.length) return
      if (coins.data.length > 1) {
        txb.mergeCoins(txb.object(coins.data[0]['coinObjectId']), coins.data.slice(1).map((e: any) => txb.object(e['coinObjectId'])))
      }
      const lpObject = txb.object(coins.data[0]['coinObjectId'])  

      txb.moveCall({
        target: `${contractAddress}::amm_script::remove_liquidity`,
        typeArguments: isAtoB ? [coinInfo[0].address, coinInfo[1].address] : [coinInfo[1].address, coinInfo[0].address],
        arguments: [
          txb.object(poolAddress),
          txb.object(global_pause_status),
          lpObject,
          txb.pure(lpValue),
          txb.pure(smallToBig(removeInfo.amountA, isAtoB ? coinInfo[0].coin.decimal : coinInfo[1].coin.decimal)),
          txb.pure(smallToBig(removeInfo.amountB, isAtoB ? coinInfo[1].coin.decimal : coinInfo[0].coin.decimal))
        ]
      })
      console.log(txb);
      const result = await signAndExecuteTransactionBlock({
        transactionBlock: txb,
      })
      console.log(result);
      setLpVlaue('')
      setRemoveInfo({
        amountA: '--',
        amountB: '--'
      })
      setTimeout(() => {
        getAllBalance()
        getLpBalance()
        getReserves()
        setRemoveLoading(false)
        setShowRemove(false)
      }, 1000);
      Toast.show({
        content: 'success!'
      })
    } catch (error) {
      console.log(error);
      setRemoveLoading(false)
    }
  }

  return (
    <>
      <SwapOutBox>
        <SwapTransationBox>
          <SwapTransationBoxLess1Px>
            <ThOuterPadding>
              <FlexBetween $gap={10}>
                <Image src={img_back} $width='24px' onClick={backPage} />
                <div>
                  <ThText1>Liquidity</ThText1>
                  {/* <ThText2>Receive LP tokens and earn 0.17% trading fees</ThText2> */}
                </div>
                <div style={{ width: '10px' }}></div>
              </FlexBetween>
            </ThOuterPadding>
            <AddStyle>
              <TextWhite $size={14} $weight={600} $pb={24}>Choose a valid pair</TextWhite>
              <FlexBetween $gap={4}>
                <AddPairItem onClick={() => chooseCoinType('coin1')}>
                  <AddPd $gap={5}>
                    <Image src={coinInfo[0]?.icon ?? img_help} $width='24px' />
                    <TextCommon style={{ flex: 'auto' }} $weight={600}>{coinInfo[0]?.symbol ?? '--'}</TextCommon>
                    <Image src={img_downarr} $width='14px' />
                  </AddPd>
                </AddPairItem>
                <Image src={img_add2} $width='14px' />
                <AddPairItem onClick={() => chooseCoinType('coin2')}>
                  <AddPd $gap={5}>
                    <Image src={coinInfo[1]?.icon ?? img_help} $width='24px' />
                    <TextCommon style={{ flex: 'auto' }} $weight={600}>{coinInfo[1]?.symbol ?? '--'}</TextCommon>
                    <Image src={img_downarr} $width='14px' />
                  </AddPd>
                </AddPairItem>
              </FlexBetween>
              {
                reservesA && reservesB ? <Pt $pt={15}>
                  <FlexStart $gap={8}>
                    <FlexStart $gap={4}>
                      <Image $width='20px' src={coinInfo[0]?.icon ?? img_help} alt="" />
                      <TextCommon>{isAtoB ? reservesA : reservesB}</TextCommon>
                    </FlexStart>
                    <TextCommon>/</TextCommon>
                    <FlexStart $gap={4}>
                      <Image $width='20px' src={coinInfo[1]?.icon ?? img_help} alt="" />
                      <TextCommon>{isAtoB ? reservesB : reservesA}</TextCommon>
                    </FlexStart>
                  </FlexStart>
                </Pt> : <></>
              }
            </AddStyle>
            <AddStyle>
              {
                !coinInfo[0] || !coinInfo[1] ?
                  <SubmitButton $isDisabled={true}>Please select a token</SubmitButton> :
                  isLoading ?
                    <SubmitButton $isDisabled={true}>
                      <DotLoading color='#02070F' />
                    </SubmitButton> :
                    poolAddress ?
                      <div>
                        <SubmitButton onClick={() => setShowLiquidity(true)}>Go to add</SubmitButton>
                        <div style={{ height: '10px' }}></div>
                        <SubmitButton onClick={() => setShowRemove(true)}>to remove</SubmitButton>
                      </div> :
                      <SubmitButton $isDisabled={initPoolLoading} onClick={initPool}>{
                        initPoolLoading ? <DotLoading color='#02070F' /> : 'init pool'
                      }</SubmitButton>
              }
            </AddStyle>
          </SwapTransationBoxLess1Px>
        </SwapTransationBox>
      </SwapOutBox>
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
      <Popup
        visible={showLiquidity}
        onMaskClick={() => {
          setShowLiquidity(false)
        }}
        onClose={() => {
          setShowLiquidity(false)
        }}
        // bodyStyle={{
        //   paddingBottom: '30px'
        // }}
        showCloseButton
        closeOnSwipe
      >
        <PopOutBox>
          <div className='poptitle'>Add Liquidity</div>
          <LiqStyle>
            <TextWhite $size={12} $leading={1.2} >The actual payment of tokens is subject to the transaction</TextWhite>
            <Pt $pt={15}></Pt>
            <GridCommon $rowgap={20}>
              <TbInputOutBox>
                <TbInput value={mainValue} onInput={mainValueChange} />
                <TbInputUsd>{coinInfo[0]?.symbol}</TbInputUsd>
              </TbInputOutBox>
              <FlexCenter>
                <Image src={img_add2} $width='16px' />
              </FlexCenter>
              <TbInputOutBox>
                <TbInput value={secondValue} onInput={secondValueChange} />
                <TbInputUsd>{coinInfo[1]?.symbol}</TbInputUsd>
              </TbInputOutBox>
            </GridCommon>
            <Pt $pt={50}>
              <SubmitButton $isDisabled={!mainValue || !secondValue || addLoading} onClick={addLiquidity}>{
                addLoading ? <DotLoading color='#02070F' /> : 'Add Liquidity'
              }</SubmitButton>
            </Pt>
          </LiqStyle>
        </PopOutBox>
      </Popup>
      <Popup
        visible={showRemove}
        onMaskClick={() => {
          setShowRemove(false)
        }}
        onClose={() => {
          setShowRemove(false)
        }}
        // bodyStyle={{
        //   paddingBottom: '30px'
        // }}
        showCloseButton
        closeOnSwipe
      >
        <PopOutBox>
          <div className='poptitle'>Remove Liquidity</div>
          <LiqStyle>
            <GridCommon $rowgap={20}>
              <TbInputOutBox>
                <TbInput value={lpValue} onInput={lpValueChange} />
                <TbInputUsd>
                  Balance: {lpBalance ?? '--'}
                </TbInputUsd>
              </TbInputOutBox>
              <GridCommon $rowgap={4}>
                <FlexBetween>
                  <TextWhite $size={14}>available {coinInfo[0]?.symbol}</TextWhite>
                  <TextWhite $size={14} $weight={600}>{
                    isAtoB ? removeInfo.amountA : removeInfo.amountB
                  }</TextWhite>
                </FlexBetween>
                <FlexBetween>
                  <TextWhite $size={14}>available {coinInfo[1]?.symbol}</TextWhite>
                  <TextWhite $size={14} $weight={600}>{
                    isAtoB ? removeInfo.amountB : removeInfo.amountA
                  }</TextWhite>
                </FlexBetween>
              </GridCommon>
            </GridCommon>
            <Pt $pt={50}>
              <SubmitButton $isDisabled={!lpValue || removeLoading || bigGt(lpValue, lpBalance)} onClick={removeLiquidity}>{
                removeLoading ? <DotLoading color='#02070F' /> : bigGt(lpValue, lpBalance) ? 'Insufficient balance' : 'Remove Liquidity'
              }</SubmitButton>
            </Pt>
          </LiqStyle>
        </PopOutBox>
      </Popup>
    </>
  )
}
