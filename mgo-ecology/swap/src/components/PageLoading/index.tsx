import { DotLoading } from 'antd-mobile'
import styled from 'styled-components';
import { FlexCenter } from '@/assets/style/common';

const PageLoad = styled(FlexCenter)`
  height: calc(100vh - 56px);
  width: 100%;
`

export default function PageLoading() {
  return (
    <PageLoad>
      <DotLoading />
    </PageLoad>
  )
}
