"use client"
import LanguageChanger from '@/components/LanguageChanger'
import Image from 'next/image'
import { useTranslation } from 'react-i18next';
import { changeIsLogin, selectIsLogin, selectUserInfo, changeUserInfo, changeShowLogIn, changeShowSignUp } from '@/lib/storemodules/appSlice'
import { AppDispatch } from '@/lib/store'
import { useDispatch, useSelector } from "react-redux";
import { $clientGet } from '@/utils/request'
import { useEffect, useRef, useState } from 'react'
import '@/styles/headers.scss'
import { useRouter } from 'next/navigation'
import { DownOutlined } from '@ant-design/icons'
import type { CollapseProps } from 'antd';
import { Drawer, Collapse } from 'antd'

export default function index() {
  const router = useRouter()
  const { t, i18n } = useTranslation();
  const isLogin = useSelector(selectIsLogin)
  const userInfo = useSelector(selectUserInfo)
  const dispatch = useDispatch<AppDispatch>()

  const getUser = async () => {
    const res: any = await $clientGet('/getInfo')
    if (res.code === 0 && res.data) {
      dispatch(changeUserInfo(res.data))
      localStorage.setItem('nextmango:tk', res.data.token)
      dispatch(changeIsLogin(true))
    }
  }

  useEffect(() => {
    getUser()
  }, [])

  const windowOpen = (url: string) => {
    setOpen(false)
    if (url) {
      window.open(url)
    }
    hideItem1Ref()
    hideItem2Ref()
    hideItem3Ref()
  }
  const goPage = (url: string) => {
    setOpen(false)
    router.push(url)
    hideItem1Ref()
    hideItem2Ref()
    hideItem3Ref()
  }
  const getUrl = () => {
    const lang = i18n.language
    return `${process.env.NEXT_PUBLIC_INVITE_DOMAIN}/pdf/${lang == 'kr' ? 'kr/MangoWhitepaperKr' : 'en/MangoNetwork_whitepaper_v1.2'}.pdf`
  }

  const [open, setOpen] = useState(false);
  const onClose = () => {
    setOpen(false);
  };

  const openLogInModal = () => {
    dispatch(changeShowLogIn(true))
  }
  const openSignUpModal = () => {
    dispatch(changeShowSignUp(true))
  }

  const items: CollapseProps['items'] = [
    {
      key: '1',
      label: <span className="text-base text-white font-semibold">{t('t4')}</span>,
      children: <div>
        <div className='flex justify-start items-start gap-[10px] mb-8 be_pointer' onClick={() => windowOpen('https://mgoscan.com/')}>
          <Image src='/img/img3.png' alt='' width={24} height={24} />
          <div>
            <div className="text-sm text-white whitespace-nowrap">{t('t24')}</div>
            <div className="text-xs text-gray9e">{t('t220')}</div>
          </div>
        </div>
        <div className='flex justify-start items-start gap-[10px] mb-8 be_pointer' onClick={() => windowOpen('https://mgoscan.com/validators')}>
          <Image src='/img/img4.png' alt='' width={24} height={24} />
          <div>
            <div className="text-sm text-white whitespace-nowrap">{t('t27')}</div>
            <div className="text-xs text-gray9e">{t('t221')}</div>
          </div>
        </div>
        <div className='flex justify-start items-start gap-[10px] mb-8 be_pointer' onClick={() => goPage('/dapp')}>
          <Image src='/img/img5.png' alt='' width={24} height={24} />
          <div>
            <div className="text-sm text-white whitespace-nowrap">{t('t16')}</div>
            <div className="text-xs text-gray9e">{t('t222')}</div>
          </div>
        </div>
        <div className='flex justify-start items-start gap-[10px] mb-8 be_pointer' onClick={() => goPage('/chain')}>
          <Image src='/img/img6.png' alt='' width={24} height={24} />
          <div>
            <div className="text-sm text-white whitespace-nowrap">Mango Omni-chain</div>
            <div className="text-xs text-gray9e">{t('t223')}</div>
          </div>
        </div>
        <div className='flex justify-start items-start gap-[10px] mb-8 be_pointer' onClick={() => goPage('/blogList')}>
          <Image src='/img/img7.png' alt='' width={24} height={24} />
          <div>
            <div className="text-sm text-white whitespace-nowrap">{t('t8')}</div>
            <div className="text-xs text-gray9e">{t('t224')}</div>
          </div>
        </div>
        <div className='flex justify-start items-start gap-[10px] mb-8 be_pointer' onClick={() => goPage('/btcLayer')}>
          <Image src='/img/img8.png' alt='' width={24} height={24} />
          <div>
            <div className="text-sm text-white whitespace-nowrap">{t('t237')}</div>
            <div className="text-xs text-gray9e">{t('t238')}</div>
          </div>
        </div>
      </div>
    },
    {
      key: '2',
      label: <span className="text-base text-white font-semibold">{t('t29')}</span>,
      children: <div>
        <div className='flex justify-start items-start gap-[10px] mb-8 be_pointer' onClick={() => windowOpen('https://docs.mangonet.io/')}>
          <Image src='/img/img48.png' alt='' width={24} height={24} />
          <div>
            <div className="text-sm text-white whitespace-nowrap">{t('t12')}</div>
            <div className="text-xs text-gray9e">{t('t226')}</div>
          </div>
        </div>
        <div className='flex justify-start items-start gap-[10px] mb-8 be_pointer' onClick={() => goPage('/foundation')}>
          <Image src='/img/img49.png' alt='' width={24} height={24} />
          <div>
            <div className="text-sm text-white whitespace-nowrap">{t('t30')}</div>
            <div className="text-xs text-gray9e">{t('t227')}</div>
          </div>
        </div>
        <div className='flex justify-start items-start gap-[10px] mb-8 be_pointer' onClick={() => windowOpen('https://mgoscan.com/')}>
          <Image src='/img/img50.png' alt='' width={24} height={24} />
          <div>
            <div className="text-sm text-white whitespace-nowrap">{t('t18')}</div>
            <div className="text-xs text-gray9e">{t('t228')}</div>
          </div>
        </div>
        <div className='flex justify-start items-start gap-[10px] mb-8 be_pointer' onClick={() => windowOpen('https://docs.mangonet.io/docs/guide/operator/mango-full-node')}>
          <Image src='/img/img51.png' alt='' width={24} height={24} />
          <div>
            <div className="text-sm text-white whitespace-nowrap">{t('t21')}</div>
            <div className="text-xs text-gray9e">{t('t229')}</div>
          </div>
        </div>
      </div>
    },
    {
      key: '3',
      label: <span className="text-base text-white font-semibold">{t('t13')}</span>,
      children: <div>
        <div className='flex justify-start items-start gap-[10px] mb-8 be_pointer' onClick={() => goPage('/activity')}>
          <Image src='/img/img52.png' alt='' width={24} height={24} />
          <div>
            <div className="text-sm text-white whitespace-nowrap">{t('t14')}</div>
            <div className="text-xs text-gray9e">{t('t230')}</div>
          </div>
        </div>
        <div className='flex justify-start items-start gap-[10px] mb-8 be_pointer' onClick={() => windowOpen('https://discord.gg/8ngRTu2F96')}>
          <Image src='/img/img53.png' alt='' width={24} height={24} />
          <div>
            <div className="text-sm text-white whitespace-nowrap">Discord</div>
            <div className="text-xs text-gray9e">{t('t231')}</div>
          </div>
        </div>
        <div className='flex justify-start items-start gap-[10px] mb-8 be_pointer' onClick={() => goPage('/community')}>
          <Image src='/img/img54.png' alt='' width={24} height={24} />
          <div>
            <div className="text-sm text-white whitespace-nowrap">{t('t32')}</div>
            <div className="text-xs text-gray9e">{t('t232')}</div>
          </div>
        </div>
        <div className='flex justify-start items-start gap-[10px] mb-8 be_pointer' onClick={() => windowOpen('https://t.me/MangoNetwork')}>
          <Image src='/img/img55.png' alt='' width={24} height={24} />
          <div>
            <div className="text-sm text-white whitespace-nowrap">Telegram</div>
            <div className="text-xs text-gray9e">{t('t233')}</div>
          </div>
        </div>
      </div>
    }
  ]

  const goUser = () => {
    setOpen(false)
    router.push('/user')
  }

  const item1Ref: any = useRef(null)
  const item2Ref: any = useRef(null)
  const item3Ref: any = useRef(null)
  const showItem1Ref = () => {
    item1Ref.current.style.display = 'block'
  }
  const hideItem1Ref = () => {
    item1Ref.current.style.display = 'none'
  }
  const showItem2Ref = () => {
    item2Ref.current.style.display = 'block'
  }
  const hideItem2Ref = () => {
    item2Ref.current.style.display = 'none'
  }
  const showItem3Ref = () => {
    item3Ref.current.style.display = 'block'
  }
  const hideItem3Ref = () => {
    item3Ref.current.style.display = 'none'
  }

  return (
    <div className="h-[60px] md:h-[100px]">
      <div className="box-px w-full h-[60px] md:h-[100px] fixed top-0 between-center z-[999] gap-5 header-style bg-black06">
        <img className="w-[80px] md:w-[115px] be_pointer" src="/img/logo.png" alt="" onClick={() => goPage('/')} />
        <div className='hidden xl:block h-full'>
          <div className='text-white h-full'>
            <div className='center-center gap-[80px] h-full'>
              <div className='center-center gap-[10px] h-full drop-item be_pointer' onMouseOver={showItem1Ref} onMouseLeave={hideItem1Ref}>
                <span className='text-lg font-medium drop-item-span'>{t('t213')}</span>
                <DownOutlined className="right-icon" />
                <div className='open-box' ref={item1Ref}>
                  <div className='do-box-grid'>
                    <div className='dob-list'>
                      <div className='py-[14px] w-full start-center gap-5 dob-item be_pointer' onClick={() => windowOpen('https://mgoscan.com/')}>
                        <Image src='/img/img3.png' alt='' width={24} height={24} />
                        <div>
                          <div className='text-sm text-white whitespace-nowrap leading-[2]'>{t('t24')}</div>
                          <div className='w-[250px] text-xs text-gray9e leading-[2]'>{t('t220')}</div>
                        </div>
                      </div>
                      <div className='py-[14px] w-full start-center gap-5 dob-item be_pointer' onClick={() => windowOpen('https://mgoscan.com/validators')}>
                        <Image src='/img/img4.png' alt='' width={24} height={24} />
                        <div>
                          <div className='text-sm text-white whitespace-nowrap leading-[2]'>{t('t27')}</div>
                          <div className='w-[250px] text-xs text-gray9e leading-[2]'>{t('t221')}</div>
                        </div>
                      </div>
                      <div className='py-[14px] w-full start-center gap-5 dob-item be_pointer' onClick={() => goPage('/dapp')}>
                        <Image src='/img/img5.png' alt='' width={24} height={24} />
                        <div>
                          <div className='text-sm text-white whitespace-nowrap leading-[2]'>{t('t16')}</div>
                          <div className='w-[250px] text-xs text-gray9e leading-[2]'>{t('t222')}</div>
                        </div>
                      </div>
                    </div>
                    <div className='bg-black32 h-full w-full'></div>
                    <div className='dob-list'>
                      <div className='py-[14px] w-full start-center gap-5 dob-item be_pointer' onClick={() => goPage('/chain')}>
                        <Image src='/img/img6.png' alt='' width={24} height={24} />
                        <div>
                          <div className='text-sm text-white whitespace-nowrap leading-[2]'>Mango Omni-chain</div>
                          <div className='w-[250px] text-xs text-gray9e leading-[2]'>{t('t223')}</div>
                        </div>
                      </div>
                      <div className='py-[14px] w-full start-center gap-5 dob-item be_pointer' onClick={() => goPage('/blogList')}>
                        <Image src='/img/img7.png' alt='' width={24} height={24} />
                        <div>
                          <div className='text-sm text-white whitespace-nowrap leading-[2]'>{t('t8')}</div>
                          <div className='w-[250px] text-xs text-gray9e leading-[2]'>{t('t224')}</div>
                        </div>
                      </div>
                      <div className='py-[14px] w-full start-center gap-5 dob-item be_pointer' onClick={() => goPage('/btcLayer')}>
                        <Image src='/img/img8.png' alt='' width={24} height={24} />
                        <div>
                          <div className='text-sm text-white whitespace-nowrap leading-[2]'>{t('t237')}</div>
                          <div className='w-[250px] text-xs text-gray9e leading-[2]'>{t('t238')}</div>
                        </div>
                      </div>
                    </div>
                  </div>
                </div>
              </div>
              <div className='center-center gap-[10px] h-full drop-item be_pointer' onMouseOver={showItem2Ref} onMouseLeave={hideItem2Ref}>
                <span className='text-lg font-medium drop-item-span'>{t('t29')}</span>
                <DownOutlined className="right-icon" />
                <div className='open-box' ref={item2Ref}>
                  <div className='do-box-grid'>
                    <div className='dob-list'>
                      <div className='py-[14px] w-full start-center gap-5 dob-item be_pointer' onClick={() => windowOpen('https://docs.mangonet.io/')}>
                        <Image src='/img/img48.png' alt='' width={24} height={24} />
                        <div>
                          <div className='text-sm text-white whitespace-nowrap leading-[2]'>{t('t12')}</div>
                          <div className='w-[250px] text-xs text-gray9e leading-[2]'>{t('t226')}</div>
                        </div>
                      </div>
                      <div className='py-[14px] w-full start-center gap-5 dob-item be_pointer' onClick={() => goPage('/foundation')}>
                        <Image src='/img/img49.png' alt='' width={24} height={24} />
                        <div>
                          <div className='text-sm text-white whitespace-nowrap leading-[2]'>{t('t30')}</div>
                          <div className='w-[250px] text-xs text-gray9e leading-[2]'>{t('t227')}</div>
                        </div>
                      </div>
                      <div className='py-[14px] w-full start-center gap-5 dob-item be_pointer' onClick={() => windowOpen('https://mgoscan.com/')}>
                        <Image src='/img/img50.png' alt='' width={24} height={24} />
                        <div>
                          <div className='text-sm text-white whitespace-nowrap leading-[2]'>{t('t18')}</div>
                          <div className='w-[250px] text-xs text-gray9e leading-[2]'>{t('t228')}</div>
                        </div>
                      </div>
                    </div>
                    <div className='bg-black32 h-full w-full'></div>
                    <div className='dob-list'>
                      <div className='py-[14px] w-full start-center gap-5 dob-item be_pointer' onClick={() => windowOpen('https://docs.mangonet.io/docs/guide/operator/mango-full-node')}>
                        <Image src='/img/img51.png' alt='' width={24} height={24} />
                        <div>
                          <div className='text-sm text-white whitespace-nowrap leading-[2]'>{t('t21')}</div>
                          <div className='w-[250px] text-xs text-gray9e leading-[2]'>{t('t229')}</div>
                        </div>
                      </div>
                    </div>
                  </div>
                </div>
              </div>
              <div className='center-center gap-[10px] h-full drop-item be_pointer' onMouseOver={showItem3Ref} onMouseLeave={hideItem3Ref}>
                <span className='text-lg font-medium drop-item-span'>{t('t13')}</span>
                <DownOutlined className="right-icon" />
                <div className='open-box' ref={item3Ref}>
                  <div className='do-box-grid'>
                    <div className='dob-list'>
                      <div className='py-[14px] w-full start-center gap-5 dob-item be_pointer' onClick={() => goPage('/activity')}>
                        <Image src='/img/img52.png' alt='' width={24} height={24} />
                        <div>
                          <div className='text-sm text-white whitespace-nowrap leading-[2]'>{t('t14')}</div>
                          <div className='w-[250px] text-xs text-gray9e leading-[2]'>{t('t230')}</div>
                        </div>
                      </div>
                      <div className='py-[14px] w-full start-center gap-5 dob-item be_pointer' onClick={() => windowOpen('https://discord.gg/8ngRTu2F96')}>
                        <Image src='/img/img53.png' alt='' width={24} height={24} />
                        <div>
                          <div className='text-sm text-white whitespace-nowrap leading-[2]'>Discord</div>
                          <div className='w-[250px] text-xs text-gray9e leading-[2]'>{t('t231')}</div>
                        </div>
                      </div>
                      <div className='py-[14px] w-full start-center gap-5 dob-item be_pointer' onClick={() => goPage('/community')}>
                        <Image src='/img/img54.png' alt='' width={24} height={24} />
                        <div>
                          <div className='text-sm text-white whitespace-nowrap leading-[2]'>{t('t32')}</div>
                          <div className='w-[250px] text-xs text-gray9e leading-[2]'>{t('t232')}</div>
                        </div>
                      </div>
                    </div>
                    <div className='bg-black32 h-full w-full'></div>
                    <div className='dob-list'>
                      <div className='py-[14px] w-full start-center gap-5 dob-item be_pointer' onClick={() => windowOpen('https://t.me/MangoNetwork')}>
                        <Image src='/img/img55.png' alt='' width={24} height={24} />
                        <div>
                          <div className='text-sm text-white whitespace-nowrap leading-[2]'>Telegram</div>
                          <div className='w-[250px] text-xs text-gray9e leading-[2]'>{t('t233')}</div>
                        </div>
                      </div>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
        <div className='block xl:hidden'>
          <div className='end-center gap-[15px]'>
            <LanguageChanger />
            <Image src='/img/img107.png' alt='menu' className='be_pointer' width={24} height={24} onClick={() => setOpen(true)} />
            <Drawer
              title={<img className='w-[80px] md:w-[115px] be_pointer' src='/img/logo.png' />}
              placement='top'
              closable={true}
              onClose={onClose}
              open={open}
              height='90vh'
              style={{
                backgroundColor: '#06061A',
                color: '#fff'
              }}
            >
              <div className='dra-style'>
                <div className='box-px'>
                  <Collapse
                    expandIconPosition='end'
                    items={items}
                    ghost
                    accordion
                  />
                </div>
                <div className='bt1 bb1 py-[15px] end-center gap-5 bg-black0a box-px'>
                  {
                    isLogin ?
                      <img src={userInfo.headerImg} className='h-[56px] be_pointer' onClick={goUser} alt="" /> :
                      <>
                        <div className='py-2 px-6 bg-green81 text-lg text-black06 be_pointer' onClick={openLogInModal}>
                          {t('t33')}
                        </div>
                        <div className='py-2 px-6 bg-black06 text-lg text-white border border-white border-solid be_pointer' onClick={openSignUpModal}>
                          {t('t34')}
                        </div>
                      </>
                  }
                </div>
              </div>
            </Drawer>
          </div>
        </div>
        <div className='hidden xl:block'>
          <div className='end-center gap-5'>
            <LanguageChanger />
            {
              isLogin ?
                <img src={userInfo.headerImg} className='h-[56px] be_pointer' alt="" onClick={() => router.push('/user')} /> :
                <>
                  <div className='py-2 px-6 bg-green81 text-lg text-black06 be_pointer' onClick={openLogInModal}>
                    {t('t33')}
                  </div>
                  <div className='py-2 px-6 bg-black06 text-lg text-white border border-white border-solid be_pointer' onClick={openSignUpModal}>
                    {t('t34')}
                  </div>
                </>
            }
          </div>
        </div>
      </div>
    </div>
  )
}
