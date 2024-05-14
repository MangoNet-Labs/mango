"use client"
import { useTranslation } from 'react-i18next';
import { selectShowLogIn, changeShowLogIn, changeShowSignUp, changeShowForget, changeUserInfo, changeIsLogin } from '@/lib/storemodules/appSlice'
import { useSelector, useDispatch } from "react-redux";
import { Modal, Input, Button, message } from 'antd';
import { useWindowSize } from '@/hooks/useWindowSize'
import { AppDispatch } from '@/lib/store'
import '@/styles/login.scss'
import { Chiq_Reduced_Bold } from '@/font/font'
import clsx from 'clsx'
import { useState } from 'react';
import { getLangType } from '@/utils'
import { $clientGet, $clientPost } from '@/utils/request'

export default function index() {
  const { t, i18n } = useTranslation();
  const [messageApi, contextHolder] = message.useMessage();
  const showLogin = useSelector(selectShowLogIn)
  const dispatch = useDispatch<AppDispatch>()
  const { screenWidth } = useWindowSize()

  const getUser = async () => {
    const res: any = await $clientGet('/getInfo')
    if (res.code === 0 && res.data) {
      dispatch(changeUserInfo(res.data))
      localStorage.setItem('nextmango:tk', res.data.token)
      dispatch(changeIsLogin(true))
    }
  }

  const closeModal = () => {
    dispatch(changeShowLogIn(false))
  }

  const [userName, setUserName] = useState('')
  const [password, setPassword] = useState('')

  const showSignModal = () => {
    dispatch(changeShowLogIn(false))
    dispatch(changeShowSignUp(true))
  }
  const showForgetModal = () => {
    dispatch(changeShowLogIn(false))
    dispatch(changeShowForget(true))
  }

  const [isloading, setIsloading] = useState(false)
  const logIn = async () => {
    if (!userName) {
      messageApi.open({ type: 'warning', content: t('t35') });
      return
    }
    if (!password) {
      messageApi.open({ type: 'warning', content: t('t36') });
      return
    }
    try {
      setIsloading(true)
      const res = await $clientPost('/loginUser', {
        language: getLangType(i18n.language),
        username: userName,
        password: password
      })
      if (res.code === 0) {
        messageApi.open({ type: 'success', content: res.msg });
        const { token } = res.data
        localStorage.setItem('nextmango:tk', token)
        setUserName('')
        setPassword('')
        getUser()
        setTimeout(() => {
          closeModal()
          setIsloading(false)
        }, 1000);
      } else {
        throw new Error(res.msg)
      }
    } catch (error: any) {
      messageApi.open({ type: 'error', content: error.message });
      setIsloading(false)
    }
  }

  return (
    <>
      {contextHolder}
      <Modal open={showLogin} width={screenWidth >= 768 ? 720 : '90%'} maskClosable={false} onCancel={closeModal}>
        <div className='login-style'>
          <div className={
            clsx('login-text1', Chiq_Reduced_Bold.className)
          }>{t('t37')}</div>
          <div className="login-text2">{t('t38')}</div>
          <div className="mb-[15px] md:mb-10 add-inputbox">
            <Input placeholder={t('t39')} value={userName} onChange={(e: any) => setUserName(e.target.value)} />
          </div>
          <div className="mb-2 add-inputbox">
            <Input.Password placeholder={t('t40')} value={password} onChange={(e: any) => setPassword(e.target.value)} />
          </div>
          <div className="start-center mb-[15px] md:mb-[30px]">
            <div className='text-xs md:text-base text-green81 be_hover_underline' onClick={showForgetModal}>
              {t('t41')}
            </div>
          </div>
          <div className="start-center gap-[10px] md:gap-5">
            <Button loading={isloading} onClick={logIn}>{t('t33')}</Button>
            <div className="text-green81 text-sm md:text-lg be_hover_underline" onClick={showSignModal}>
              {t('t42')}
            </div>
          </div>
        </div>
      </Modal>
    </>
  )
}
