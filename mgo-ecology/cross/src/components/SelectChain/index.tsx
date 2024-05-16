import styled from 'styled-components'
import { SearchOutline, CloseCircleOutline } from 'antd-mobile-icons'
import { FlexBetween, TextWhite, HeightBox, TextLightGreen, FlexCenter, Image } from '@/assets/style/common'
import { ChainListType } from '@/interface'
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
  placeholder: 'Search networks'
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
const ListGrid = styled.div`
  width: 100%;
  display: grid;
  grid-template-columns: repeat(3,1fr);
`
const ListItem = styled.div`
  width: 116px;
  padding: 12px 0;
  margin: 12px 0;
  border-radius: 8px;
  transition: background-color 0.4s ease 0s;
  cursor: pointer;
  &:hover{
    background-color: rgba(255, 255, 255, 0.098);
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
//#endregion

interface PropsType {
  closeSelectChain: (value: boolean) => void;
  chooseChain: (item: ChainListType) => void;
  chainList: ChainListType[];
  isFrom: boolean;
}

export default function SelectChain({ closeSelectChain, chooseChain, chainList, isFrom }: PropsType) {
  const stopEvent = (e: any) => {
    e.stopPropagation()
  }

  const [searchValue, setSearchValue] = useState('')

  // function createMutilWordQueryReg () {
  //   let regStr = searchValue;
  //   if (searchValue.indexOf(' ') !== -1) {
  //       const str = searchValue.split(' ').map(q => `(?=.*${q})`).join('');
  //       regStr = `^${str}.*`;
  //   }
  //   return new RegExp(regStr);
  // }


  const filterName = () => {
    const li = chainList.filter(ele => ele.chainName.toLowerCase().includes(searchValue.toLowerCase()))
    return li
  }

  const list = useMemo(() => {
    if (!searchValue) {
      return chainList
    } else {
      return filterName()
    }
  }, [chainList, searchValue])

  return (
    <SelectOutBox onClick={() => closeSelectChain(false)}>
      <InfoBox $gap={16} $dir='column' onClick={(e: any) => stopEvent(e)}>
        <HeightBox>
          <TextWhite $align='center' $size={28}>Sending {isFrom ? 'from' : 'to'}</TextWhite>
          <TextLightGreen $pb={16} $align='center'>Select Network</TextLightGreen>
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
          <ListGrid>
            {
              list.map(item => (
                <FlexCenter key={item.chainName} onClick={() => chooseChain(item)}>
                  <ListItem>
                    <FlexCenter $dir='column' $gap={16}>
                      <Image src={item.logo} $width='48px' $height='48px' />
                      <TextWhite>
                        {item.chainName}
                      </TextWhite>
                    </FlexCenter>
                  </ListItem>
                </FlexCenter>
              ))
            }
          </ListGrid>
        </ListBox>
      </InfoBox>
      <CloseIcon>
        <CloseCircleOutline fontSize={26} />
      </CloseIcon>
    </SelectOutBox>
  )
}
