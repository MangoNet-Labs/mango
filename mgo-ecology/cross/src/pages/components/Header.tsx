import { Image, FlexStart, HeightBox } from '@/assets/style/common'
import img_logo from '@/assets/image/logo.png'


export default function Header() {
  return (
    <HeightBox $height={60} $padding='0 0 0 20px'>
      <FlexStart $height='100%'>
        <Image $height='30px' src={img_logo} />
      </FlexStart>
    </HeightBox>
  )
}
