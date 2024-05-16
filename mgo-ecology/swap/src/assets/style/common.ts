import styled from 'styled-components';

export const FlexCenter = styled.div<{
  $gap?: number
}>`
  display: flex;
  justify-content: center;
  align-items: center;
  gap: ${({ $gap }) => $gap ? $gap + 'px' : '0px'};
`
export const FlexBetween = styled.div<{
  $items?: 'start' | 'end',
  $gap?: number
}>`
  display: flex;
  justify-content: space-between;
  align-items: ${({ $items }) => $items ?? 'center'};
  gap: ${({ $gap }) => $gap ? $gap + 'px' : '0px'};
`
export const FlexStart = styled.div<{
  $items?: 'start' | 'end',
  $gap?: number
}>`
  display: flex;
  justify-content: flex-start;
  align-items: ${({ $items }) => $items ?? 'center'};
  gap: ${({ $gap }) => $gap ? $gap + 'px' : '0px'};
`
export const FlexEnd = styled(FlexStart)`
  justify-content: flex-end;
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
  $pb?: number | string
}>`
  font-size: ${({ $size }) => $size ?? 16}px;
  font-weight: ${({ $weight }) => $weight ?? 400};
  line-height: ${({ $leading }) => $leading ?? 1.5};
  text-align: ${({ $align }) => $align ?? 'left'};
  padding-bottom: ${({ $pb }) => typeof $pb === 'string' ? $pb : typeof $pb === 'number' ? $pb + 'px' : '0px'};
`
export const TextGreen = styled(TextBase)`
  color: #81D8CF;
`
export const TextPurple = styled(TextBase)`
color: var(--colors-7645d9);
`
export const TextCommon = styled(TextBase)`
color: #fff;
`
export const TextLilac = styled(TextBase)`
color: var(--colors-7a6eaa);
`
export const TextWhite = styled(TextBase)`
color: var(--colors-ffffff);
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