'use client';

import { useRouter, usePathname } from 'next/navigation';
import { useTranslation } from 'react-i18next';
import i18nConfig from '@/i18nConfig';
import type { MenuProps } from 'antd';
import { Dropdown } from 'antd';
import Image from 'next/image'

const items: MenuProps['items'] = [
  {
    key: 'en',
    label: (<span>English</span>),
  },
  {
    key: 'kr',
    label: (<span>한국어</span>),
  },
  {
    key: 'zhTw',
    label: (<span>繁體中文</span>),
  },
];

export default function LanguageChanger() {
  const { i18n } = useTranslation();
  const currentLocale = i18n.language;
  const router = useRouter();
  const currentPathname = usePathname();

  const onClick: MenuProps['onClick'] = ({ key }) => {
    const newLocale = key

    // set cookie for next-i18n-router
    const days = 30;
    const date = new Date();
    date.setTime(date.getTime() + days * 24 * 60 * 60 * 1000);
    const expires = date.toUTCString();
    document.cookie = `NEXT_LOCALE=${newLocale};expires=${expires};path=/`;

    // redirect to the new locale path
    if (
      currentLocale === i18nConfig.defaultLocale
    ) {
      router.push('/' + newLocale + currentPathname);
    } else {
      router.push(
        currentPathname.replace(`/${currentLocale}`, `/${newLocale}`)
      );
    }

    router.refresh();
  };

  return (
    <Dropdown menu={{ items, onClick }} placement="bottom">
      <Image src="/img/img44.png" alt='change language' width={24} height={24} />
    </Dropdown>
  );
}