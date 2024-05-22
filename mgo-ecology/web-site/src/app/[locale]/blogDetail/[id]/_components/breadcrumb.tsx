"use client"
import { Breadcrumb } from 'antd';
import { useRouter } from 'next/navigation'
import Link from 'next/link'

export default function breadcrumb({ pageTitle }: { pageTitle: string }) {
  const router = useRouter()


  return (
    <Breadcrumb
      items={[
        {
          title: <Link href={'/'}>Home</Link>,
        },
        {
          title: <Link href={'/blogList'}>Blog</Link>,
        },
        {
          title: pageTitle,
        },
      ]}
    />
  )
}
