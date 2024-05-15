import styled from 'styled-components';
import { useNavigate } from 'react-router-dom';

const SwapTabsOuterBox = styled.div`
  background: var(--colors-02070F);
  padding-left: 12px;
  width: 100%;
  display: flex;
  justify-content: flex-start;
  align-items: center;
`
const SwapTabsItem = styled.div<{
  $active: boolean
}>`
  margin-right: 20px;
  height: 42px;
  font-size: 16px;
  padding: 0 4px;
  display: flex;
  justify-content: center;
  align-items: center;
  border-radius: 2px;
  border-bottom: 4px solid ${({ $active }) => $active ? '#80D8CF' : 'rgba(255,255,255,0)'};
  color: ${({ $active }) => $active ? '#FFFFFF' : 'rgba(255,255,255,0.8)'};
  font-weight: ${({ $active }) => $active ? 600 : 400};
  &:hover{
    background: rgba(255,255,255,0.1);
  }
  cursor: pointer;
`

type TabsActive = 'swap' | 'pool'
export default function SwapTabs({ active }: { active: TabsActive }) {
  const router = useNavigate()
  const goPage = (to: string) => {
    router(to)
  }

  return (
    <SwapTabsOuterBox>
      <SwapTabsItem $active={active == 'swap' ? true : false} onClick={() => goPage('/swap')}>Swap</SwapTabsItem>
      <SwapTabsItem $active={active == 'pool' ? true : false} onClick={() => goPage('/pool')}>Pool</SwapTabsItem>
    </SwapTabsOuterBox>
  )
}
