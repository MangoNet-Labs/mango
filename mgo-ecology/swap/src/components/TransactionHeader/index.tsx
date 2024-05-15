import styled from 'styled-components';

export const ThOuterPadding = styled.div`
  border-bottom: 1px solid rgba(255,255,255,0.2);
  padding: 24px;
`
export const ThText1 = styled.h2`
  color: #FFFFFF;
  text-align: center;
  line-height: 1.5;
`
export const ThText2 = styled.div`
  color: #DAFCF8;
  font-size: 14px;
  line-height: 1.5;
  text-align: center;
`

export default function TransactionHeader({ page }: { page: 'swap' | 'limit' }) {
  return (
    <ThOuterPadding>
      <ThText1>{page === 'swap' ? 'Swap' : 'Your Liquidity'}</ThText1>
      <ThText2>{page === 'swap' ? 'Trade tokens in an instant' : 'Remove liquidity to receive tokens back'}</ThText2>
    </ThOuterPadding>
  )
}
