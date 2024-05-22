import styled from 'styled-components';
import { PaddingBox, GridCommon, TextWhite, FlexBetween, TextBlack, FlexCenter } from '@/assets/style/common'
import { memo } from 'react'
import { ConfigType, ActualInfoType } from '@/interface'
import { bigNumberComputed, BcType } from '@/utils/web3'
import { DotLoading } from 'antd-mobile'

//#region 
const BorderBox = styled(PaddingBox)`
  border: 1px solid rgba(255,255,255,0.7);
  border-radius: 8px;
`
const InfoBox = styled.div`
  padding: 8px;
  border-radius: 4px;
  background: var(--colors-293241);
  margin-bottom: 20px;
`
const SubmitBtn = styled.button.attrs<{
  $diabled?: boolean
}>(({ $diabled }) => ({
  disabled: $diabled ?? false
}))`
  display: flex;
  justify-content: center;
  align-items: center;
  border: none;
  padding: 8px 20px;
  border-radius: 30px;
  background: var(--colors-FFFFFF);
  cursor: pointer;
  &:disabled{
    opacity: 0.6;
  }
  &:active:not(:disabled){
    opacity: 0.85;
    transform: translateY(1px);
  }
`
//#endregion

interface PropsType {
  isDisabled: boolean;
  showText: string;
  submit: () => void;
  config: ConfigType;
  actualInfo: ActualInfoType;
  isLoading: boolean;
}

export default memo(function Submit({ isDisabled, showText, submit, config, actualInfo, isLoading }: PropsType) {

  return (
    <BorderBox $padding='12px'>
      <InfoBox>
        <GridCommon $rowgap={4}>
          <FlexBetween $gap={8}>
            <TextWhite $size={12} $opacity={0.7}>fee</TextWhite>
            <TextWhite $break='break-all' $size={12} $opacity={0.7}>{config.feeType === '1' ? config.fee : `${bigNumberComputed(config.fee, 100, BcType.times)}%`}</TextWhite>
          </FlexBetween>
          <FlexBetween $gap={8}>
            <TextWhite $size={12} $opacity={0.7}>Minimum exchange</TextWhite>
            <TextWhite $break='break-all' $size={12} $opacity={0.7}>{config.minLimit}</TextWhite>
          </FlexBetween>
          <FlexBetween $gap={8}>
            <TextWhite $size={12} $opacity={0.7}>maximum exchange</TextWhite>
            <TextWhite $break='break-all' $size={12} $opacity={0.7}>{config.maxLimit == null ? '∞' : config.maxLimit}</TextWhite>
          </FlexBetween>
          <FlexBetween $gap={8}>
            <TextWhite $size={12} $opacity={0.7}>Actual deduction</TextWhite>
            <TextWhite $break='break-all' $size={12} $opacity={0.7}>{actualInfo.ActualDeduction}</TextWhite>
          </FlexBetween>
          <FlexBetween $gap={8}>
            <TextWhite $size={12} $opacity={0.7}>Actual gain</TextWhite>
            <TextWhite $break='break-all' $size={12} $opacity={0.7}>{actualInfo.ActualGain}</TextWhite>
          </FlexBetween>
        </GridCommon>
      </InfoBox>
      <FlexCenter>
        <SubmitBtn $diabled={isDisabled || isLoading} onClick={submit}>
          {isLoading ? <DotLoading color='#02070F' /> : <></>}
          <TextBlack>{showText}</TextBlack>
        </SubmitBtn>
      </FlexCenter>
    </BorderBox>
  )
})
