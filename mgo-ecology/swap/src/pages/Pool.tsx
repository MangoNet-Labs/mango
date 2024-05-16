import SwapTabs from '@/components/SwapTabs';
import styled from 'styled-components';
import TransactionHeader from '@/components/TransactionHeader';
import { SwapOutBox, SwapTransationBox, SwapTransationBoxLess1Px } from './Swap';
import { SubmitButton } from '@/components/TransactionBody';
import img_add2 from '@/assets/image/add.png';
import img_help from '@/assets/image/help.png';
import { Image, TextWhite, FlexCenter, FlexBetween, FlexStart } from '@/assets/style/common';
import { useNavigate } from 'react-router-dom';
import { MgoClient } from '@mgonetwork/mango.js/client';
import { fullNode, lp_front, coinList } from '@/utils/info';
import { useWalletKit } from '@mgonetwork/wallet-kit';
import { useEffect, useState } from 'react';
import { getTwoCoins } from '@/utils/fc';

//#region 
const PoolStyle = styled.div`
  padding: 16px;
`
const PoolBg = styled.div`
  background: #212833;
`
const ListItem = styled.div`
  padding: 8px 0;
`
//#endregion

export default function Limit() {
  const client = new MgoClient({ url: fullNode })
  const { currentAccount } = useWalletKit();
  const router = useNavigate()

  const goAdd = () => {
    router('/add')
  }

  interface ListType {
    balance: string;
    icon1: string;
    icon2: string;
  }

  const [list, setList] = useState<ListType[]>([])
  const getList = async () => {
    if (!currentAccount?.address) return
    try {
      const list = await client.getAllBalances({
        owner: currentAccount.address
      })
      let li = []
      for (let i = 0; i < list.length; i++) {
        const element = list[i];
        if (element.coinType.indexOf(lp_front) !== -1) {
          const coinLi = getTwoCoins(element.coinType)
          const img1 = coinList.find(ele => ele.address === coinLi[0])?.icon
          const img2 = coinList.find(ele => ele.address === coinLi[1])?.icon
          li.push({
            balance: element.totalBalance,
            icon1: img1 ?? img_help,
            icon2: img2 ?? img_help
          })
        }
      }
      setList(li)
    } catch (error) {
      console.log(error);
    }
  }

  useEffect(() => {
    getList()
  }, [currentAccount])

  return (
    <>
      <SwapTabs active='pool' />
      <SwapOutBox>
        <SwapTransationBox>
          <SwapTransationBoxLess1Px>
            <TransactionHeader page='limit' />
            <PoolBg>
              <PoolStyle>
                {
                  list.length ? list.map((item, i) => (
                    <ListItem key={i}>
                      <FlexBetween>
                        <FlexStart $gap={5}>
                          <Image src={item.icon1} $width='24px' />
                          <Image src={img_add2} $width='14px' />
                          <Image src={item.icon2} $width='24px' />
                        </FlexStart>
                        <TextWhite $align='center'>{item.balance}</TextWhite>
                      </FlexBetween>
                    </ListItem>
                  )) : <TextWhite $align='center'>No liquidity found</TextWhite>
                }
              </PoolStyle>
            </PoolBg>
            <PoolStyle>
              <SubmitButton onClick={goAdd}>
                <FlexCenter $gap={8}>
                  {/* <Image src={img_add} $width='16px' alt="" /> */}
                  <span>+</span>
                  <span>Add Liquidity</span>
                </FlexCenter>
              </SubmitButton>
            </PoolStyle>
          </SwapTransationBoxLess1Px>
        </SwapTransationBox>
      </SwapOutBox>
    </>
  )
}
