import type { Metadata } from "next";
import "../globals.css";
import "../common.scss";
import '@/styles/changeAntd.scss'
import i18nConfig from '@/i18nConfig';
import { dir } from 'i18next';
import { AntdRegistry } from '@ant-design/nextjs-registry';
import Headers from '@/components/headers'
import Footers from '@/components/footers'
import initTranslations from '@/app/i18n'
import TranslationsProvider from '@/components/TranslationsProvider';
import { ReduxProvider } from '@/lib/store'
import { $ssrGet } from '@/utils/request'
import LoginIn from '@/components/loginIn'
import SignUp from '@/components/signUp'
import Forget from '@/components/forget'
import ChangePassword from '@/components/changePassword'
import ChangeEmail from '@/components/changeEmail'
import ChangePicture from '@/components/changePicture'
import { getMetaDataInfo } from '@/utils/metadata'

export function generateMetadata({ params: { locale } }: { params: { locale: string } }): Metadata {
  return getMetaDataInfo('/', locale)
}

export function generateStaticParams() {
  return i18nConfig.locales.map(locale => ({ locale }));
}

const i18nNamespaces = ['common'];

export default async function RootLayout({
  children,
  params: { locale }
}: Readonly<{
  children: React.ReactNode;
  params: { locale: string }
}>) {
  const { resources } = await initTranslations(locale, i18nNamespaces);
  const config = await $ssrGet('/configList')

  return (
    <html lang={locale} dir={dir(locale)}>
      <body id="mango">
        <AntdRegistry>
          <TranslationsProvider
            namespaces={i18nNamespaces}
            locale={locale}
            resources={resources}>
            <ReduxProvider>
              <LoginIn />
              <SignUp />
              <Forget />
              <ChangePassword />
              <ChangeEmail />
              <ChangePicture config={config.data} />
              <Headers />
              <main className="app-out-box ">{children}</main>
              <Footers config={config.data} />
            </ReduxProvider>
          </TranslationsProvider>
        </AntdRegistry>
      </body>
    </html>
  );
}
