import styled from 'styled-components';

export const Flex = styled.div<{
  $gap?: number,
  $items?: 'start' | 'end',
  $height?: string,
  $dir?: 'row-reverse' | 'column' | 'column-reverse'
}>`
  display: flex;
  flex-direction: ${({ $dir }) => $dir ?? 'row'};
  align-items: ${({ $items }) => $items ?? 'center'};
  gap: ${({ $gap }) => $gap ? $gap + 'px' : '0px'};
  height: ${({ $height }) => typeof $height === 'string' ? $height : typeof $height === 'number' ? $height + 'px' : 'auto'};
`
export const FlexStart = styled(Flex)`
  justify-content: flex-start;
`
export const FlexCenter = styled(Flex)`
  justify-content: center;
`
export const FlexEnd = styled(Flex)`
  justify-content: flex-end;
`
export const FlexBetween = styled(Flex)`
  justify-content: space-between;
`
export const FlexAround = styled(Flex)`
  justify-content: space-around;
`
export const FlexEvenly = styled(Flex)`
  justify-content: space-evenly;
`

export const GridCommon = styled.div<{
  $rowgap?: number
}>`
  display: grid;
  row-gap: ${({ $rowgap }) => $rowgap ? $rowgap + 'px' : '10px'};
`

export const TextBase = styled.div<{
  $size?: number,
  $weight?: number,
  $leading?: number,
  $align?: 'center' | 'right',
  $pb?: number | string,
  $opacity?: number,
  $break?: 'break-all' | 'break-word',
  $width?: string
}>`
  font-size: ${({ $size }) => $size ?? 16}px;
  font-weight: ${({ $weight }) => $weight ?? 400};
  line-height: ${({ $leading }) => $leading ?? 1.5};
  text-align: ${({ $align }) => $align ?? 'left'};
  padding-bottom: ${({ $pb }) => typeof $pb === 'string' ? $pb : typeof $pb === 'number' ? $pb + 'px' : '0px'};
  opacity: ${({ $opacity }) => $opacity ?? 1};
  word-break: ${({ $break }) => $break ?? 'normal'};
  width: ${({ $width }) => $width ?? 'auto'};
`
export const TextWhite = styled(TextBase)`
  color: var(--colors-FFFFFF);
`
export const TextBlack = styled(TextBase)`
  color: var(--colors-02070F);
`
export const TextLightGreen = styled(TextBase)`
  color: var(--colors-DAFCF8);
`

export const Image = styled.img<{
  $width?: string,
  $height?: string
}>`
  width: ${({ $width }) => $width ?? 'auto'};
  height: ${({ $height }) => $height ?? 'auto'};
`

export const Pt = styled.div<{
  $pt?: number | string
}>`
  padding-top: ${({ $pt }) => typeof $pt === 'string' ? $pt : typeof $pt === 'number' ? $pt + 'px' : '0px'};
`
export const PaddingBox = styled.div<{
  $padding?: string
}>`
  padding: ${({ $padding }) => $padding ?? '0px'};
`

export const HeightBox = styled.div<{
  $height?: number | string,
  $padding?: string
}>`
  width: 100%;
  height: ${({ $height }) => typeof $height === 'string' ? $height : typeof $height === 'number' ? $height + 'px' : 'auto'};
  padding: ${({ $padding }) => $padding ?? '0px'};
`

export const BorderBottom = styled.div<{
  $color: string
}>`
  border-bottom: 1px solid ${({ $color }) => $color};
`

export const Pointer = styled.div<{
  $pointer?: boolean
}>`
  cursor: ${({ $pointer }) => $pointer ? 'pointer' : 'default'};
`
export const FullPointer = styled(Pointer)`
width: 100%;
height: 100%;`