import { CopyToClipboard } from 'react-copy-to-clipboard';
import { type ReactNode } from 'react';
import { Toast } from 'antd-mobile'

interface CopyProps {
  value: string
  children: ReactNode
}

export default function Copy({ value, children }: CopyProps) {

  const onCopy = () => {
    Toast.show({
      content: 'Copied successfully'
    })
  }

  return (
    <>
      <CopyToClipboard text={value} onCopy={onCopy}>
        {children}
      </CopyToClipboard>
    </>
  )
}