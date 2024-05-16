import styled from 'styled-components';
import { GridCommon, Image, FlexCenter } from '@/assets/style/common'
import Container from './Container'
import Submit from './Submit'
import img_change from '@/assets/image/change.png'
import SelectChain from '@/components/SelectChain'
import SelectCoin from '@/components/SelectCoin'
import { useState, useCallback, useMemo, useEffect } from 'react'
import { ChainListType, CoinListType, ConfigType, ActualInfoType } from '@/interface'
import { useWeb3Js } from '@/hooks/useWeb3Js'
import { useMangoJs } from '@/hooks/useMangoJs'
import { useConnectWallet } from '@mgonetwork/dapp-kit'
import { bigGt, bigGte, bigLte, bigLt, smallToBig } from '@/utils/web3'
import { Toast } from 'antd-mobile';
import { useConnectWallet as useConnectSuiWallet, useWallets as useSuiWallets, useSignAndExecuteTransactionBlock } from '@mysten/dapp-kit'
import { useSuiJs, suiFullNode } from '@/hooks/useSuiJs'
import { $get } from '@/request'
import { bigNumberComputed, BcType, bigToSmallFixed0 } from '@/utils/web3'
import Web3 from 'web3'
import { bridgeAbi } from '@/utils/abi'
import { TransactionBlock } from '@mgonetwork/mango.js/transactions';
import { MgoClient } from '@mgonetwork/mango.js/client';
import { useWalletKit } from '@mgonetwork/wallet-kit';
import { fullNode } from '@/utils/info'
import { TransactionBlock as SuiTransactionBlock } from '@mysten/sui.js/transactions'
import { SuiClient } from '@mysten/sui.js/client';
import { useUniSat } from '@/hooks/useUnisat'

//#region 
const MainOutBox = styled.div`
  padding: 16px;
  width: 100%;
  max-width: 900px;
  @media (max-width: 499.95px) {
    margin: 0 auto;
  }
  margin: 40px auto;
`
const MainBox = styled(GridCommon)`
  margin: auto;
  max-width: 650px;
`
const ChangeBox = styled.button.attrs<{
  $diabled?: boolean
}>(({ $diabled }) => ({
  disabled: $diabled ?? false
}))`
  border-radius: 4px;
  cursor: pointer;
  background: rgba(255,255,255,0.1);
  padding: 4px 4px 1px;
  border: none;
  outline: none;
  &:hover:not(:disabled){
    background: rgba(255,255,255,0.2);
  }
  &:disabled{
    opacity: 0.6;
  }
`
//#endregion

export default function Main() {

 
  const [fromChainList, setFromChainList] = useState<ChainListType[]>([])
  const [toChainList, setToChainList] = useState<ChainListType[]>([])


  const getFromChainList = () => {
    $get('/chains').then((res: any) => {
      setFromChainList(res)
    })
  }
  useEffect(() => {
    getFromChainList()
  }, [])

  const getToChainList = () => {
    $get(`/chains?alias=${fromCoin?.alias}`).then((res: any) => {
      setToChainList(res)
    })
  }


  useEffect(() => {
    if (toChainList.length && toChainList[0].tokens.length) {
      setToInfo(toChainList[0])
      setToCoinList(toChainList[0].tokens)
      setToCoin(toChainList[0].tokens[0])
    } else if (toChainList.length && !toChainList[0].tokens.length) {
      setToInfo(toChainList[0])
      setToCoinList([])
      setToCoin(undefined)
    } else {
      setToInfo(undefined)
      setToCoinList([])
      setToCoin(undefined)
    }
  }, [toChainList])

  const [fromCoinList, setFromCoinList] = useState<CoinListType[]>([])
  const [toCoinList, setToCoinList] = useState<CoinListType[]>([])
  const [showSelectChain, setShowSelectChain] = useState(false) 
  const [isFrom, setIsFrom] = useState(true)  
  const [fromInfo, setFromInfo] = useState<ChainListType>()  
  const [isChooseChain, setIsChooseChain] = useState(false) 
  const [toInfo, setToInfo] = useState<ChainListType>()  
  const [fromCoin, setFromCoin] = useState<CoinListType>() 
  const [toCoin, setToCoin] = useState<CoinListType>()  

 
  useEffect(() => {
    if (fromInfo && fromCoin) {
      getToChainList()
    }
  }, [fromInfo, fromCoin])


  const openShowSelectChain = useCallback((from: boolean) => {
    if (!from && (!fromInfo || !fromCoin)) {
      setShowToError(true)
      setToErrorMsg('Please select the information to complete From first.')
      return
    } else {
      setShowToError(false)
    }
    setIsFrom(from)
    setShowSelectChain(true)
  }, [fromInfo, fromCoin])

  const canUseChainList = useMemo(() => {
    if (isFrom) {
      return fromChainList.filter(ele => (ele.chainName !== fromInfo?.chainName))
    } else if (!isFrom) {
      return toChainList.filter(ele => (ele.chainName !== toInfo?.chainName))
    } else {
      return []
    }
  }, [isFrom, showSelectChain])


  const chooseChain = (item: ChainListType) => {
    setIsChooseChain(true)
    isFrom ? setFromInfo(item) : setToInfo(item)
    isFrom ? setFromCoinList(item.tokens) : setToCoinList(item.tokens)
    isFrom ? setFromCoin(undefined) : setToCoin(undefined)
    setShowSelectChain(false)
  }

  // const [showFromError, setShowFromError] = useState(false)
  const [showFromError] = useState(false)
  const [showToError, setShowToError] = useState(false)
  // const [fromErrorMsg, setFromErrorMsg] = useState('')
  const [fromErrorMsg] = useState('')
  const [toErrorMsg, setToErrorMsg] = useState('')
 
  const [showSelectCoin, setShowSelectCoin] = useState(false) //是否显示选择代币的弹窗
 
  const openShowSelectCoin = useCallback((from: boolean) => {
    if (!from && (!fromInfo || !fromCoin)) {
      setShowToError(true)
      setToErrorMsg('Please select the information to complete From first.')
      return
    } else {
      setShowToError(false)
    }
    setIsFrom(from)
    setShowSelectCoin(true)
  }, [fromInfo, fromCoin])


  const canUseCoinList = useMemo(() => {
    if (isFrom && fromInfo) {
      return fromCoinList.filter(ele => ele.chain === fromInfo.chainName)
    } else if (!isFrom && toInfo) {
      return toCoinList.filter(ele => ele.chain === toInfo.chainName)
    } else {
      return []
    }
  }, [isFrom, showSelectCoin])
 
  const chooseCoin = (item: CoinListType) => {
    isFrom ? setFromCoin(item) : setToCoin(item)
    setShowSelectCoin(false)
  }


  const [fromInput, setFromInput] = useState('')
  const toInput = useMemo(() => {
    return fromInput
  }, [fromInput])

  const { getAccount, getCoinsBalance, getNetworkId, changeNetWork, allowQuota } = useWeb3Js()
  const { getMgoBalance } = useMangoJs()
  const { getSuiBalance } = useSuiJs()
  const suiWallets = useSuiWallets()
  const { mutate: connectSui } = useConnectSuiWallet()
  const { signAndExecuteTransactionBlock, connect, wallets } = useWalletKit();
  const client = new MgoClient({ url: fullNode })
  const { mutate: suiSignAndExecuteTransactionBlock } = useSignAndExecuteTransactionBlock();
  const suiClient = new SuiClient({ url: suiFullNode })
  const { connectUnisat, getUnisatBalance } = useUniSat()

  const [refreshbalance, setRefreshbalance] = useState(false)
  useEffect(() => {
    if (refreshbalance) {
      console.log('Start refreshing');
    } else {
      console.log('End refresh');
    }
  }, [refreshbalance])

  const [fromUserAddress, setFromUserAddress] = useState('')

  const { mutate: mangoConnect } = useConnectWallet();
  const mgoFromConnect = async () => {
    await connect(wallets[0].name)
    mangoConnect({
      wallet: wallets[0]
    }, {
      onSuccess: (e) => {
        setFromUserAddress(e.accounts[0].address)
      }
    })
  }
  const mgoToConnect = async () => {
    await connect(wallets[0].name)
    mangoConnect({
      wallet: wallets[0]
    }, {
      onSuccess: (e) => {
        setToUserAddress(e.accounts[0].address)
      }
    })
  }
  const unisatConnectFrom = async () => {
    const address = await connectUnisat()
    setFromUserAddress(address)
  }
  const unisatConnectTo = async () => {
    const address = await connectUnisat()
    setToUserAddress(address)
  }

  const connectFromWallet = useCallback(() => {
    if (!fromInfo) return
    if (fromInfo.series === 'mgo') {
      mgoFromConnect()
    } else if (fromInfo.series === 'evm') {
      getAccount().then(addr => {
        setFromUserAddress(addr)
      })
    } else if (fromInfo.series === 'sui') {
      connectSui({
        wallet: suiWallets[0]
      }, {
        onSuccess: (e) => {
          setFromUserAddress(e.accounts[0].address)
        }
      })
    } else if (fromInfo.series === 'btc') {
      unisatConnectFrom()
    }
  }, [fromInfo])
  useEffect(() => {
    if (fromInfo?.series && isChooseChain) {
      setFromUserAddress('')
    }
  }, [fromInfo?.series])

  useEffect(() => {
    if (!fromInfo || !fromUserAddress) return
    const changeBalances = async () => {
      try {
        const res = await getCoinsBalance(fromInfo, fromUserAddress, fromCoinList)
        setFromCoinList(res)
        setRefreshbalance(false)
      } catch (error) {
        console.log(error);
      }
    }
    const changeMgoBalances = async () => {
      try {
        const res = await getMgoBalance(fromInfo, fromUserAddress, fromCoinList)
        setFromCoinList(res)
        setRefreshbalance(false)
      } catch (error) {
        console.log(error);
      }
    }
    const changeSuiBalances = async () => {
      try {
        const res = await getSuiBalance(fromInfo, fromUserAddress, fromCoinList)
        setFromCoinList(res)
        setRefreshbalance(false)
      } catch (error) {
        console.log(error);
      }
    }
    const changeUnisatBalances = async () => {
      try {
        let res = await getUnisatBalance(fromCoinList);
        setFromCoinList(res)
        setRefreshbalance(false)
      } catch (error) {
        console.log(error);
      }
    }
    if (fromInfo.series === 'mgo') {
      changeMgoBalances()
    } else if (fromInfo.series === 'evm') {
      changeBalances()
    } else if (fromInfo.series === 'sui') {
      changeSuiBalances()
    } else if (fromInfo.series === 'btc') {
      changeUnisatBalances()
    }
  }, [fromUserAddress, fromInfo, refreshbalance])

  const [toUserAddress, setToUserAddress] = useState('')

  const connectToWallet = useCallback(() => {
    if (!toInfo) return
    if (toInfo.series === 'mgo') {
      mgoToConnect()
    } else if (toInfo.series === 'evm') {
      getAccount().then(addr => {
        setToUserAddress(addr)
      })
    } else if (toInfo.series === 'sui') {
      connectSui({
        wallet: suiWallets[0]
      }, {
        onSuccess: (e) => {
          setToUserAddress(e.accounts[0].address)
        }
      })
    }
    else if (toInfo.series === 'btc') {
      unisatConnectTo()
    }
  }, [toInfo])
  useEffect(() => {
    if (toInfo?.series && isChooseChain) {
      setToUserAddress('')
    }
  }, [toInfo?.series])

  useEffect(() => {
    if (!toInfo || !toUserAddress) return
    const changeBalances = async () => {
      try {
        const res = await getCoinsBalance(toInfo, toUserAddress, toCoinList)
        setToCoinList(res)
        setRefreshbalance(false)
      } catch (error) {
        console.log(error);
      }
    }
    const changeMgoBalances = async () => {
      try {
        const res = await getMgoBalance(toInfo, toUserAddress, toCoinList)
        setToCoinList(res)
        setRefreshbalance(false)
      } catch (error) {
        console.log(error);
      }
    }
    const changeSuiBalances = async () => {
      try {
        const res = await getSuiBalance(toInfo, toUserAddress, toCoinList)
        setToCoinList(res)
        setRefreshbalance(false)
      } catch (error) {
        console.log(error);
      }
    }
    const changeUnisatBalances = async () => {
      try {
        let res = await getUnisatBalance(toCoinList);
        setToCoinList(res)
        setRefreshbalance(false)
      } catch (error) {
        console.log(error);
      }
    }
    if (toInfo.series === 'mgo') {
      changeMgoBalances()
    } else if (toInfo.series === 'evm') {
      changeBalances()
    } else if (toInfo.series === 'sui') {
      changeSuiBalances()
    } else if (toInfo.series === 'btc') {
      changeUnisatBalances()
    }
  }, [toUserAddress, toInfo, refreshbalance])

  const changeFromAndTo = () => {
    console.log('change');
    // setIsChooseChain(false)
    // const fromChainData = fromInfo
    // const toChainData = toInfo
    // const fromCoinData = fromCoin
    // const toCoinData = toCoin
    // const fromAddressData = fromUserAddress
    // const toAddressData = toUserAddress
    // setFromUserAddress(toAddressData)
    // setToUserAddress(fromAddressData)
    // setFromInfo(toChainData)
    // setToInfo(fromChainData)
    // setFromCoin(toCoinData)
    // setToCoin(fromCoinData)
  }

  const changeFromInput = (num: string) => {
    setFromInput(num)
  }

  const [actualInfo, setActualInfo] = useState<ActualInfoType>({
    ActualDeduction: '0',
    ActualGain: '0'
  })

  const getFeeInfo = () => {
 
    if (config.feeCalculationType === '0') {
      if (config.feeType === '0') {
        const arg2 = bigNumberComputed(1, config.fee, BcType.minus)
        const res = bigNumberComputed(fromInput, arg2, BcType.times)
        setActualInfo({
          ActualDeduction: fromInput,
          ActualGain: res
        })
      } else {
        const res = bigNumberComputed(fromInput, config.fee, BcType.minus)
        setActualInfo({
          ActualDeduction: fromInput,
          ActualGain: res
        })
      }
    } else {
      if (config.feeType === '0') {
        const arg2 = bigNumberComputed(1, config.fee, BcType.minus)
        const res = bigNumberComputed(toInput, arg2, BcType.div)
        setActualInfo({
          ActualDeduction: res,
          ActualGain: toInput
        })
      } else {
        const res = bigNumberComputed(toInput, config.fee, BcType.plus)
        setActualInfo({
          ActualDeduction: res,
          ActualGain: toInput
        })
      }
    }
  }

  const initConfig: ConfigType = {
    fee: '0',
    feeType: '0',
    minLimit: '0',
    maxLimit: '0',
    feeCalculationType: '0'
  }
  const [config, setConfig] = useState<ConfigType>(initConfig)
  const getConfig = () => {
    $get('/bridges/tokens/config', {
      fromChainName: fromCoin?.chain,
      fromTokenContract: fromCoin?.contract,
      toTokenAlias: toCoin?.alias,
      toChainSeries: toInfo?.series,
      toTokenContract: toCoin?.contract
    }).then((res: any) => {
      const config: ConfigType = {
        fee: res.feeType === '1' ? bigToSmallFixed0(res.fee, fromCoin?.decimals) : res.fee,
        minLimit: res.minLimit,
        maxLimit: res.maxLimit,
        feeType: res.feeType,
        feeCalculationType: res.feeCalculationType
      }
      setConfig(config)
    })
  }
  useEffect(() => {
    if (fromInfo && fromCoin && toInfo && toCoin) {
      getConfig()
    }
  }, [toInfo, toCoin])

  useEffect(() => {
    if (toInput) {
      getFeeInfo()
    }
  }, [toInput, config])

  useEffect(() => {
    if (fromInfo && fromCoin && fromUserAddress) {
      const item = fromCoinList.find(ele => ele.alias === fromCoin.alias)!
      const result = { ...item }
      setFromCoin(result)
    }
  }, [fromCoinList])
  useEffect(() => {
    if (toCoinList.length && toInfo && toCoin && toUserAddress) {
      const item = toCoinList.find(ele => ele.alias === toCoin.alias)!
      const result = { ...item }
      setToCoin(result)
    }
  }, [toCoinList])

  const isSubmitDisabled = useMemo(() => {
    if (fromInfo && fromCoin && fromUserAddress && fromInput && Number(fromInput) != 0 && toInfo && toCoin && toUserAddress && !showFromError && !showToError && bigGte(fromCoin.balance, fromInput) && bigGte(fromInput, config.minLimit) && bigLte(fromInput, config.maxLimit)) {
      return false
    } else {
      return true
    }
  }, [fromInfo, fromCoin, fromUserAddress, fromInput, toInfo, toCoin, toUserAddress, showFromError, showToError, config])

  const submitText = useMemo(() => {
    if (!fromInfo || !toInfo) {
      return 'Please select a network'
    } else if (!fromCoin || !toCoin) {
      return 'Please select a token'
    } else if (!fromUserAddress || !toUserAddress) {
      return 'Please connect wallet'
    } else if (!fromInput) {
      return 'Please enter the redemption quantity'
    } else if (Number(fromInput) == 0) {
      return 'Incorrect redemption amount'
    } else if (bigGt(fromInput, fromCoin.balance)) {
      return 'Insufficient balance'
    } else if (bigLt(fromInput, config.minLimit)) {
      return `The minimum value is ${config.minLimit}`
    } else if (bigGt(fromInput, config.maxLimit)) {
      return `The maximum value is ${config.maxLimit}`
    } else {
      return 'Submit'
    }
  }, [fromInfo, fromCoin, fromUserAddress, fromInput, toInfo, toCoin, toUserAddress, config])

  const [isLoading, setIsLoading] = useState(false)
  const submit = useCallback(async () => {
    if (!fromInfo || !fromCoin || !toCoin || isLoading) return
    try {
      setIsLoading(true)
      if (fromInfo.series === 'evm') {
        const chainId = await getNetworkId()
        if (chainId !== fromInfo.chainId) {
          changeNetWork(fromInfo!)
          throw new Error(`Please switch to ${fromInfo.chainName} Network`)
        }
        if (!fromCoin.isEther) {
          await allowQuota(
            smallToBig(actualInfo.ActualDeduction, fromCoin.decimals),
            fromCoin.contract,
            fromUserAddress,
            fromCoin.bridges[0]?.contract
          )
        }
        const web3 = new Web3(window.ethereum)
        const { methods } = new web3.eth.Contract(bridgeAbi, fromCoin.bridges[0]?.contract, {
          from: fromUserAddress
        })
        methods.bridgeToken([
          fromCoin.contract,
          smallToBig(actualInfo.ActualDeduction, fromCoin.decimals),
          toCoin.alias,
          toUserAddress
        ]).send(
          fromCoin.isEther ? { value: smallToBig(actualInfo.ActualDeduction, fromCoin.decimals) } : undefined
        ).on("transactionHash", (hash: string) => {
          console.log(hash);
          Toast.show('The transaction has been submitted to the chain')
          setTimeout(() => {
            setIsLoading(false)
          }, 1000);
        }).on("receipt", (receipt: any) => {
          console.log(receipt);
          setRefreshbalance(true)
        }).on("error", (error: any) => {
          console.log(error);
          Toast.show(error.message)
          setIsLoading(false)
        })
      } else if (fromInfo.series === 'mgo') {
        const txb = new TransactionBlock()
        let coins: any;
        if (fromCoin.isEther) {
          [coins] = txb.splitCoins(
            txb.gas,
            [txb.pure(smallToBig(actualInfo.ActualDeduction, fromCoin.decimals))]
          )
        } else {
          let coinlist = await client.getCoins({
            owner: fromUserAddress,
            coinType: fromCoin.contract
          })
          if (!coinlist.data.length) {
            throw new Error('Insufficient balance')
          } else if (coinlist.data.length > 1) {
            txb.mergeCoins(txb.object(coinlist.data[0]['coinObjectId']), coinlist.data.slice(1).map((e: any) => txb.object(e['coinObjectId'])))
          }
          [coins] = txb.splitCoins(txb.object(coinlist.data[0]['coinObjectId']), [txb.pure(smallToBig(actualInfo.ActualDeduction, fromCoin.decimals))])
        }
        txb.moveCall({
          target: `${fromCoin.bridges[0].contract}::bridge::bridge_token`,
          typeArguments: [fromCoin.contract],
          arguments: [
            txb.pure(fromCoin.bridges[0].bridgeObject),
            coins,
            txb.pure(toUserAddress),
            txb.pure(toCoin.alias),
            txb.object('0x6')
          ]
        })
        console.log(txb);
        const result = await signAndExecuteTransactionBlock({
          transactionBlock: txb,
        })
        console.log(result);
        Toast.show('Transaction successful')
        setTimeout(() => {
          setRefreshbalance(true)
          setIsLoading(false)
        }, 1000);
      } else if (fromInfo.series === 'sui') {
        const txb = new SuiTransactionBlock()
        let coins: any;
        if (fromCoin.isEther) {
          [coins] = txb.splitCoins(
            txb.gas,
            [txb.pure(smallToBig(actualInfo.ActualDeduction, fromCoin.decimals))]
          )
        } else {
          let coinlist = await suiClient.getCoins({
            owner: fromUserAddress,
            coinType: fromCoin.contract
          })
          if (!coinlist.data.length) {
            throw new Error('Insufficient balance')
          } else if (coinlist.data.length > 1) {
            txb.mergeCoins(txb.object(coinlist.data[0]['coinObjectId']), coinlist.data.slice(1).map((e: any) => txb.object(e['coinObjectId'])))
          }
          [coins] = txb.splitCoins(txb.object(coinlist.data[0]['coinObjectId']), [txb.pure(smallToBig(actualInfo.ActualDeduction, fromCoin.decimals))])
        }
        txb.moveCall({
          target: `${fromCoin.bridges[0].contract}::bridge::bridge_token`,
          typeArguments: [fromCoin.contract],
          arguments: [
            txb.pure(fromCoin.bridges[0].bridgeObject),
            coins,
            txb.pure(toUserAddress),
            txb.pure(toCoin.alias),
            txb.object('0x6')
          ]
        })
        console.log(txb);
        suiSignAndExecuteTransactionBlock(
          {
            transactionBlock: txb
          },
          {
            onSuccess: (tx) => {
              console.log(tx);
              Toast.show('Transaction successful')
              setTimeout(() => {
                setRefreshbalance(true)
                setIsLoading(false)
              }, 1000);
            },
            onError: (err) => {
              console.log(err);
              setIsLoading(false)
            }
          }
        )
      } else if (fromInfo.series === 'btc') {
        const res = await window.unisat.sendBitcoin(toUserAddress, smallToBig(actualInfo.ActualDeduction, fromCoin.decimals));
        console.log(res);
      }
    } catch (error: any) {
      console.log(error);
      Toast.show(error.message)
      setIsLoading(false)
    }
  }, [fromInfo, fromCoin, toCoin, actualInfo, fromUserAddress, toUserAddress, isLoading])


  const testUnisat = async () => {
    let txid = await window.unisat.sendBitcoin("tb1q7wfazmxf8de9guguvu2esxsjt27h094kzfpnjs", Number(smallToBig(0.00000001, 8)));
    console.log(txid)
    let publicKey = await window.unisat.getPublicKey();
    console.log(publicKey);
    const signData = JSON.stringify({
      amount: '0.00000001',
      inscriptionId: txid,
      l1address: 'tb1q7wfazmxf8de9guguvu2esxsjt27h094kzfpnjs',
      l2address: '0x395c896d18918374cf86d404c9fdfabd28340bec'
    })
    let res = await window.unisat.signMessage(signData);
    console.log(res);
  }

  return (
    <>
      <MainOutBox>
        <MainBox $rowgap={16}>
          <Container
            openSelectChain={openShowSelectChain}
            isFrom={true}
            info={fromInfo}
            showError={showFromError}
            errorMessage={fromErrorMsg}
            openSelectCoin={openShowSelectCoin}
            coin={fromCoin}
            inputValue={fromInput}
            changeInputValue={changeFromInput}
            userAddress={fromUserAddress}
            connectWallet={connectFromWallet}
            disConnectWallet={setFromUserAddress}
          />
          <FlexCenter>
            <ChangeBox $diabled={fromInfo && toInfo ? false : true} onClick={changeFromAndTo}>
              <Image src={img_change} $width='24px' />
            </ChangeBox>
          </FlexCenter>
          <Container
            openSelectChain={openShowSelectChain}
            isFrom={false}
            info={toInfo}
            showError={showToError}
            errorMessage={toErrorMsg}
            openSelectCoin={openShowSelectCoin}
            coin={toCoin}
            inputValue={toInput}
            userAddress={toUserAddress}
            connectWallet={connectToWallet}
            disConnectWallet={setToUserAddress}
          />
          <Submit
            isDisabled={isSubmitDisabled}
            showText={submitText}
            submit={submit}
            config={config}
            actualInfo={actualInfo}
            isLoading={isLoading}
          />
        </MainBox>
      </MainOutBox>
      {
        showSelectChain && <SelectChain
          chainList={canUseChainList}
          chooseChain={chooseChain}
          closeSelectChain={setShowSelectChain}
          isFrom={isFrom}
        />
      }
      {
        showSelectCoin && <SelectCoin
          coinList={canUseCoinList}
          chooseCoin={chooseCoin}
          closeSelectCoin={setShowSelectCoin}
        />
      }
    </>
  )
}
