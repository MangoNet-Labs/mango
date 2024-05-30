import styled from 'styled-components';
import { PaddingBox, FlexStart, FlexCenter, BorderBottom } from '@/assets/style/common'

const TabsItem = styled(FlexCenter) <{ $active: boolean }>`
  margin-right: 20px;
  height: 40px;
  font-size: 16px;
  padding: 0 4px;
  border-bottom: 3px solid ${({ $active }) => $active ? '#80D8CF' : 'rgba(255,255,255,0)'};
  color: ${({ $active }) => $active ? '#FFFFFF' : 'rgba(255,255,255,0.8)'};
  font-weight: ${({ $active }) => $active ? 600 : 400};
  &:hover{
    background: rgba(255,255,255,0.1);
  }
  cursor: pointer;
`

export default function Tabs() {
  return (
    <BorderBottom $color='#979797'>
      <PaddingBox $padding='0 0 0 20px'>
        <FlexStart>
          <TabsItem $active>BeingBridge</TabsItem>
        </FlexStart>
      </PaddingBox>
    </BorderBottom>
  )
}
