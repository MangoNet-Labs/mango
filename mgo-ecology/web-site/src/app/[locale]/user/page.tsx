import Index from './_components/index'
import { getMetaDataInfo } from '@/utils/metadata'
import type { Metadata } from "next";

export function generateMetadata({ params: { locale } }: { params: { locale: string } }): Metadata {
  return getMetaDataInfo('/user', locale)
}

export default async function page() {
  return (
    <Index />
  )
}
