import styled from 'styled-components';
import SwapTabs from '@/components/SwapTabs';
import TransactionHeader from '@/components/TransactionHeader';
import TransactionBody from '@/components/TransactionBody';
import { useEffect } from 'react';

export const SwapOutBox = styled.div`
  padding: 16px;
`
export const SwapTransationBox = styled.div`
  width: 328px;
  background: #05342F;
  border-radius: 24px;
  color: #280d5f;
  overflow: hidden;
  position: relative;
  padding: 1px 1px 3px;
  margin: auto;
`
export const SwapTransationBoxLess1Px = styled.div`
  width: 100%;
  height: 100%;
  overflow: inherit;
  background: #131821;
  border-radius: 24px;
`

export default function Swap() {

  useEffect(() => {
    const noRoad = localStorage.getItem('noRoad')
    if (noRoad != 'true') {
      localStorage.setItem('noRoad', 'true')
      setTimeout(() => {
        window.location.reload()
      }, 200);
    }
  }, [])

  return (
    <>
      <SwapTabs active='swap' />
      <SwapOutBox>
        <SwapTransationBox>
          <SwapTransationBoxLess1Px>
            <TransactionHeader page='swap' />
            <TransactionBody />
          </SwapTransationBoxLess1Px>
        </SwapTransationBox>
      </SwapOutBox>
    </>
  )
}
