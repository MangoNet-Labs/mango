"use client"
import { useTranslation } from 'react-i18next';
import { selectShowSignUp, changeShowLogIn, changeShowSignUp, changeUserInfo, changeIsLogin } from '@/lib/storemodules/appSlice'
import { useSelector, useDispatch } from "react-redux";
import { Modal, Input, Button, message } from 'antd';
import { useWindowSize } from '@/hooks/useWindowSize'
import { AppDispatch } from '@/lib/store'
import '@/styles/login.scss'
import { Chiq_Reduced_Bold } from '@/font/font'
import clsx from 'clsx'
import { useState, useRef, useEffect } from 'react';
import { getLangType } from '@/utils'
import { $clientPost, $clientGet } from '@/utils/request'

export default function index() {
  const { t, i18n } = useTranslation();
  const [messageApi, contextHolder] = message.useMessage();
  const showSignUp = useSelector(selectShowSignUp)
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
    dispatch(changeShowSignUp(false))
  }

  const showLogInModal = () => {
    dispatch(changeShowSignUp(false))
    dispatch(changeShowLogIn(true))
  }

  const [email, setEmail] = useState('')
  const [userName, setUserName] = useState('')
  const [code, setCode] = useState('')
  const [password, setPassword] = useState('')
  const [rePassword, setRePassword] = useState('')

  const [isloading, setIsloading] = useState(false)

  const register = async () => {
    if (!email) {
      messageApi.open({ type: 'warning', content: t('t43') });
      return
    }
    if (!userName) {
      messageApi.open({ type: 'warning', content: t('t44') });
      return
    }
    if (!code) {
      messageApi.open({ type: 'warning', content: t('t45') });
      return
    }
    if (!password) {
      messageApi.open({ type: 'warning', content: t('t36') });
      return
    }
    if (!rePassword) {
      messageApi.open({ type: 'warning', content: t('t46') });
      return
    }
    try {
      setIsloading(true)
      const res = await $clientPost('/registerUser', {
        language: getLangType(i18n.language),
        email: email,
        userName: userName,
        code: code,
        password: password,
        twoPassword: rePassword
      })
      if (res.code === 0) {
        messageApi.open({ type: 'success', content: res.msg });
        const { token } = res.data
        localStorage.setItem('nextmango:tk', token)
        setEmail('')
        setUserName('')
        setCode('')
        setPassword('')
        setRePassword('')
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

  const timer = useRef<any>(null)
  const [codeLoading, setCodeLoading] = useState(false)
  const [codeDown, setCodeDown] = useState(false)
  const [codeDownNum, setCodeDownNum] = useState(60)
  const sendCode = async () => {
    if (!email) {
      messageApi.open({ type: 'warning', content: t('t43') });
      return
    }
    if (codeLoading || codeDown) return
    try {
      setCodeLoading(true)
      const res = await $clientPost('/sendEmail', {
        email: email,
        event: 'register',
        language: getLangType(i18n.language)
      })
      if (res.code === 0) {
        setCodeDown(true)
        setCodeLoading(false)
        messageApi.open({ type: 'success', content: res.msg });
        timer.current = setInterval(() => {
          setCodeDownNum(prev => {
            if (prev <= 1) {
              clearInterval(timer.current);
              setCodeDown(false);
              return 60;
            }
            return prev - 1
          })
        }, 1000)
      } else {
        throw new Error(res.msg)
      }
    } catch (error: any) {
      messageApi.open({ type: 'error', content: error.message });
      setCodeLoading(false)
    }
  }

  useEffect(() => {
    return () => {
      clearInterval(timer.current)
    }
  }, [])


  return (
    <>
      {contextHolder}
      <Modal open={showSignUp} width={screenWidth >= 768 ? 720 : '90%'} maskClosable={false} onCancel={closeModal}>
        <div className='login-style'>
          <div className={
            clsx('login-text1', Chiq_Reduced_Bold.className)
          }>{t('t47')}</div>
          <div className="login-text2">{t('t48')}</div>
          <div className="mb-[10px] ">
            <div className='add-inputbox'>
              <Input placeholder={t('t90')} value={email} onChange={(e: any) => setEmail(e.target.value)} />
            </div>
            <div className="text-sm md:text-base text-gray67 pt-1">{t('t49')}</div>
          </div>
          <div className="mb-[10px] ">
            <div className='add-inputbox'>
              <Input placeholder={t('t192')} value={userName} onChange={(e: any) => setUserName(e.target.value)} />
            </div>
            <div className="text-sm md:text-base text-gray67 pt-1">{t('t50')}</div>
          </div>
          <div className='mb-5'>
            <div className='add-inputbox'>
              <Input placeholder={t('t51')} value={code} onChange={(e: any) => setCode(e.target.value)} addonAfter={
                <Button loading={codeLoading} onClick={sendCode}>{codeDown ? codeDownNum : t('t52')}</Button>
              } />
            </div>
          </div>
          <div className="mb-[10px] ">
            <div className='add-inputbox'>
              <Input.Password placeholder={t('t40')} value={password} onChange={(e: any) => setPassword(e.target.value)} />
            </div>
            <div className="text-sm md:text-base text-gray67 pt-1">{t('t53')}</div>
          </div>
          <div className="mb-[10px] ">
            <div className='add-inputbox'>
              <Input.Password placeholder={t('t55')} value={rePassword} onChange={(e: any) => setRePassword(e.target.value)} />
            </div>
            <div className="text-sm md:text-base text-gray67 pt-1">{t('t54')}</div>
          </div>
          <div className="start-center gap-[10px] md:gap-5">
            <Button loading={isloading} onClick={register}>{t('t42')}</Button>
            <div className="text-green81 text-sm md:text-lg be_hover_underline" onClick={showLogInModal}>
              {t('t33')}
            </div>
          </div>
        </div>
      </Modal>
    </>
  )
}
