import styled from 'styled-components'
import { SearchOutline, CloseCircleOutline } from 'antd-mobile-icons'
import { FlexBetween, TextWhite, HeightBox, TextLightGreen, FlexCenter, Image, FlexStart, FlexEnd } from '@/assets/style/common'
import { CoinListType } from '@/interface'
import { useMemo, useState } from 'react'

//#region 
const SelectOutBox = styled.div`
  width: 100vw;
  height: 100vh;
  position: fixed;
  left: 0;
  top: 0;
  background: var(--colors-02070F);
  padding: 40px 0;
  @media (max-width: 499.95px) {
    padding: 20px 0;
  }
`
const InfoBox = styled(FlexBetween)`
  width: 100%;
  height: 100%;
  max-width: 650px;
  margin: auto;
  padding: 24px;
  @media (max-width: 499.95px) {
    padding: 24px 12px;
  }
`
const InputBox = styled.div`
  height: 60px;
  width: 100%;
  border-radius: 8px;
  border: 1px solid rgba(255,255,255,0.8);
  padding: 16px;
`
const SearchInput = styled.input.attrs({
  placeholder: 'Search by name or contract address'
})`
  flex: auto;
  width: 100%;
  height: 100%;
  color: var(--colors-FFFFFF);
  background: transparent;
  border: none;
  outline: none;
  font-size: 16px;
`
const ListBox = styled.div`
  flex: auto;
  width: 100%;
  height: 100%;
  overflow-y: auto;
  &::-webkit-scrollbar{
    width: 10px;
  }
  &::-webkit-scrollbar-track{
    background-color: rgb(78,78,84);
  }
  &::-webkit-scrollbar-thumb{
    background: rgba(0,0,0,0.25);
    &:hover{
      background: rgba(0,0,0,0.5);
    }
  }
`
const CloseIcon = styled.div`
  position: absolute;
  right: 40px;
  top: 40px;
  cursor: pointer;
  @media (max-width: 499.95px) {
    right: 20px;
    top: 20px
  }
`
const ListItem = styled.div`
  display: grid;
  grid-template-columns: repeat(3,1fr);
  gap: 4px;
  padding: 10px;
  border-radius: 2px;
  &:hover{
    background-color: rgba(255, 255, 255, 0.098);
  }
`
//#endregion

interface PropsType {
  closeSelectCoin: (value: boolean) => void;
  chooseCoin: (item: CoinListType) => void,
  coinList: CoinListType[]
}

export default function SelectCoin({ closeSelectCoin, chooseCoin, coinList }: PropsType) {
  const stopEvent = (e: any) => {
    e.stopPropagation()
  }

  const [searchValue, setSearchValue] = useState('')

  const filterName = () => {
    const li = coinList.filter(ele => ele.symbol.toLowerCase().includes(searchValue.toLowerCase()))
    return li
  }

  const list = useMemo(() => {
    if (!searchValue) {
      return coinList
    } else {
      return filterName()
    }
  }, [coinList, searchValue])

  return (
    <SelectOutBox onClick={() => closeSelectCoin(false)}>
      <InfoBox $gap={16} $dir='column' onClick={(e: any) => stopEvent(e)}>
        <HeightBox>
          <TextWhite $pb={16} $align='center' $size={28}>Select asset</TextWhite>
          <label>
            <InputBox>
              <FlexBetween $gap={8} $height='100%'>
                <SearchInput
                  value={searchValue}
                  onChange={(e: any) => setSearchValue(e.target.value)}
                />
                <SearchOutline fontSize={24} />
              </FlexBetween>
            </InputBox>
          </label>
        </HeightBox>
        <ListBox>
          {
            list.map(item => (
              <ListItem key={item.contract} onClick={() => chooseCoin(item)}>
                <FlexStart $gap={8}>
                  <Image src={item.logo} $width='32px' />
                  <div>
                    <TextWhite $size={14} $weight={600}>{item.symbol}</TextWhite>
                    <TextLightGreen $size={12}>{item.chain}</TextLightGreen>
                  </div>
                </FlexStart>
                <FlexCenter>
                  <div>
                    <TextWhite $size={12}>Native</TextWhite>
                    <TextWhite $size={12}>{item.contract.slice(0, 6)}...{item.contract.slice(item.contract.length - 4)}</TextWhite>
                  </div>
                </FlexCenter>
                <FlexEnd>
                  <div>
                    <TextWhite $align='right' $size={12}>Balance</TextWhite>
                    <TextWhite $align='right' $size={12}>{item.balance ? item.balance : '--'}</TextWhite>
                  </div>
                </FlexEnd>
              </ListItem>
            ))
          }
        </ListBox>
      </InfoBox>
      <CloseIcon>
        <CloseCircleOutline fontSize={26} />
      </CloseIcon>
    </SelectOutBox>
  )
}
