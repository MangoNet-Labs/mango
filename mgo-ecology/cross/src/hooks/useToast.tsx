import { Toast } from 'antd-mobile'
import img_success from '@/assets/image/success.png'
import { Image } from '@/assets/style/common'

export function useToast() {
  return Toast.show({
    content: 'Connection successful',
    icon: <Image src={img_success} $width='40%' />,
    duration: 5000
  })
}