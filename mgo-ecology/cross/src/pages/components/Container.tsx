import styled from 'styled-components';
import { GridCommon, FlexBetween, TextWhite, FlexCenter, Image, PaddingBox, TextLightGreen, FlexStart, Pointer, FullPointer } from '@/assets/style/common'
import img_wallet from '@/assets/image/wallet.png'
import img_help from '@/assets/image/help.png'
import img_warning from '@/assets/image/warning.png'
import { useWindowSize } from '@/hooks/useWindowSize'
import { memo } from 'react'
import { ChainListType, CoinListType } from '@/interface'
import { Popover, Toast } from 'antd-mobile'
import '@/assets/style/common.css'
import copyText from 'copy-to-clipboard'

//#region 
const ConnectWallet = styled.button.attrs<{
  $diabled?: boolean
}>(({ $diabled }) => ({
  disabled: $diabled ?? false
}))`
  border: 1px solid #979797;
  padding: 8px 16px;
  border-radius: 8px;
  background: transparent;
  cursor: pointer;
  &:disabled{
    opacity: 0.6;
  }
  &:active:not(:disabled){
    opacity: 0.85;
    transform: translateY(1px);
  }
`
const BorderBox = styled(PaddingBox)`
  border: 1px solid rgba(255,255,255,0.7);
  border-radius: 8px;
  .chaininfo{
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    gap: 12px;
    @media (max-width: 499.95px) {
      flex-direction: row;
      justify-content: flex-start;
      width: 100%;
      gap: 8px;
      img{
        width: 18px;
      }
    }
  }
  .networks{
    @media (max-width: 499.95px) {
      width: 100%;
    }
  }
`
const GridBox = styled.div`
  display: grid;
  grid-template-columns: 158px 1fr 120px;
  gap: 12px;
  grid-template-areas: 
    'item1 item2 item2'
    'item1 item3 item4'
  ;
  @media (max-width: 499.95px) {
    grid-template-columns: 40% 1fr 120px;
    grid-template-areas: 
    'item1 item2 item2'
    'item3 item3 item4'
  ;
  }
`
const GridItemArea = styled.div<{
  $area: string
}>`
  grid-area: ${({ $area }) => $area};
  border-radius: 8px;
  background: var(--colors-293241);
  padding: 12px;
`
const Input = styled.input.attrs<{
  $readonly?: boolean
}>(({ $readonly }) => ({
  readOnly: $readonly ?? false,
  type: 'number',
  step: '0.1',
  min: '0',
  placeholder: '0.00'
}))`
  border: none;
  outline: none;
  background: transparent;
  color: var(--colors-FFFFFF);
  &::-webkit-outer-spin-button,
  &::-webkit-inner-spin-button {
      -webkit-appearance: none;
  }
  font-size: 16px;
`
const WarningBox = styled.div`
  background: rgba(245, 0, 0, 0.22);
  border-radius: 4px;
  padding: 6px;
  width: 100%;
`
//#endregion

interface PropsType {
  openSelectChain: (value: boolean) => void;
  isFrom: boolean;
  info?: ChainListType;
  showError?: boolean;
  errorMessage?: string;
  openSelectCoin: (value: boolean) => void;
  coin?: CoinListType;
  inputValue: string;
  changeInputValue?: (num: string) => void;
  userAddress?: string;
  connectWallet: () => void;
  disConnectWallet: (value: string) => void;
}

const actionsList = [
  {
    key: 'copy',
    text: 'Copy address'
  },
  {
    key: 'transactions',
    text: 'Transactions'
  },
  {
    key: 'disconnect',
    text: 'Disconnect'
  }
]

export default memo(function Container({ openSelectChain, isFrom, info, showError, errorMessage, openSelectCoin, coin, inputValue, changeInputValue, userAddress, connectWallet, disConnectWallet }: PropsType) {
  const { screenWidth } = useWindowSize()

  const clickMenuItem = (e: any) => {
    if (e.key === 'disconnect') {
      disConnectWallet('')
    } else if (e.key === 'transactions') {
      if (!info?.blockUrl) return
      window.open(`${info.blockUrl}/address/${userAddress}`)
    } else if (e.key === 'copy') {
      if (copyText(userAddress ?? 'no Address')) {
        Toast.show('Copied successfully')
      } else {
        Toast.show('Copy failed')
      }
    }
  }

  return (
    <GridCommon $rowgap={12}>
      <FlexBetween>
        <TextWhite $weight={600}>{isFrom ? 'From' : 'To'}</TextWhite>
        {
          userAddress ?
            <Popover.Menu
              actions={actionsList}
              trigger='click'
              placement='bottom'
              onAction={clickMenuItem}
            >
              <ConnectWallet $diabled={info ? false : true}>
                <FlexCenter $gap={8}>
                  <Image src={img_wallet} $width='24px' />
                  <TextWhite>{userAddress.slice(0, 6)}...{userAddress.slice(userAddress.length - 4)}</TextWhite>
                </FlexCenter>
              </ConnectWallet>
            </Popover.Menu>
            :
            <ConnectWallet $diabled={info ? false : true} onClick={connectWallet}>
              <FlexCenter $gap={8}>
                <Image src={img_wallet} $width='24px' />
                <TextWhite>{screenWidth >= 500 ? 'Connect wallet' : 'Connect'}</TextWhite>
              </FlexCenter>
            </ConnectWallet>
        }
      </FlexBetween>
      <BorderBox $padding='12px'>
        <GridCommon $rowgap={12}>
          <GridBox>
            <GridItemArea $area='item1' onClick={() => openSelectChain(isFrom)}>
              <FullPointer $pointer={true}>
                {
                  info ? <FlexCenter $height='100%' $gap={12} $dir='column'>
                    <TextLightGreen className='networks'>Network</TextLightGreen>
                    <div className='chaininfo'>
                      <Image src={info.logo} $width='56px' />
                      <TextWhite>{info.chainName}</TextWhite>
                    </div>
                  </FlexCenter> : <FlexCenter $gap={12} $dir='column' $height='100%'>
                    <Image src={img_help} $width='56px' />
                    <TextWhite>{screenWidth >= 500 ? 'Select network' : 'Select'}</TextWhite>
                  </FlexCenter>
                }
              </FullPointer>
            </GridItemArea>
            <GridItemArea $area='item2'>
              <TextLightGreen $size={14} $pb={4}>Asset</TextLightGreen>

              <Pointer $pointer={(info && coin) || (info && !coin) ? true : false}>
                {
                  info && coin ?
                    <FlexStart $gap={8} onClick={() => openSelectCoin(isFrom)}>
                      <Image src={coin.logo} $width='24px' />
                      <TextWhite>{coin.symbol}</TextWhite>
                      {
                        screenWidth >= 500 && <TextLightGreen $size={12}>({coin.chain} network)</TextLightGreen>
                      }
                    </FlexStart> :
                    info && !coin ?
                      <FlexStart $gap={8} onClick={() => openSelectCoin(isFrom)}>
                        <Image src={img_help} $width='24px' />
                        <TextWhite>Select</TextWhite>
                      </FlexStart> :
                      <FlexStart $gap={8}>
                        <TextWhite>--</TextWhite>
                      </FlexStart>
                }
              </Pointer>
            </GridItemArea>
            <GridItemArea $area='item3'>
              <TextLightGreen $size={14} $pb={4}>Amount</TextLightGreen>
              {
                info && coin ?
                  <>
                    <Input $readonly={isFrom ? false : true} value={inputValue} onInput={(e: any) => changeInputValue ? changeInputValue(e.target.value) : inputValue} />
                    <TextWhite $size={12} $opacity={0.7}>($0.00)</TextWhite>
                  </> :
                  <TextWhite>--</TextWhite>
              }
            </GridItemArea>
            <GridItemArea $area='item4'>
              <TextLightGreen $size={14} $pb={4}>Balance</TextLightGreen>
              <TextWhite $break='break-all'>{coin?.balance ? coin.balance : '--'}</TextWhite>
            </GridItemArea>
          </GridBox>
          {
            showError && <WarningBox>
              <FlexCenter $gap={8}>
                <Image src={img_warning} $width='16px' />
                <TextWhite $size={12}>{errorMessage}</TextWhite>
              </FlexCenter>
            </WarningBox>
          }
        </GridCommon>
      </BorderBox>
    </GridCommon>
  )
})
