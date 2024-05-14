import Link from 'next/link'
import initTranslations from '@/app/i18n'
import TranslationsProvider from '@/components/TranslationsProvider';
import { ParamsLocale } from '@/interface'

const i18nNamespaces = ['notfound'];

export default async function NotFound({ params: { locale } }: ParamsLocale) {
  const { t, resources } = await initTranslations(locale, i18nNamespaces);

  return (
    <TranslationsProvider
      namespaces={i18nNamespaces}
      locale={locale}
      resources={resources}>
      <div className='text-white'>
        <h2>{t('title')}</h2>
        <p>{t('content')}</p>
        <Link href="/">{t('back')}</Link>
      </div>
    </TranslationsProvider>
  )
}