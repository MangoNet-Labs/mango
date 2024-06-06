import { DotLoading } from 'antd-mobile'
import styled from 'styled-components';
import { FlexCenter } from '@/assets/style/common';

const PageLoad = styled(FlexCenter)`
  height: 100vh;
  width: 100%;
`

export default function PageLoading() {
  return (
    <PageLoad>
      <DotLoading color='white' />
    </PageLoad>
  )
}
