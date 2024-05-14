"use client"
import { useTranslation } from 'react-i18next';
import { changeUserInfo, changeIsLogin, selectShowChangePassword, changeShowChangePassword, changeShowForget } from '@/lib/storemodules/appSlice'
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
  const showChangePassword = useSelector(selectShowChangePassword)
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
    dispatch(changeShowChangePassword(false))
  }
  const [email, setEmail] = useState('')
  const [code, setCode] = useState('')
  const [rePassword, setRePassword] = useState('')

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
        event: 'resetPassword',
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

  const [forgetLoading, setForgetLoading] = useState(false)
  const changePassword = async () => {
    if (!email) {
      messageApi.open({ type: 'warning', content: t('t43') });
      return
    }
    if (!code) {
      messageApi.open({ type: 'warning', content: t('t190') });
      return
    }
    if (!rePassword) {
      messageApi.open({ type: 'warning', content: t('t191') });
      return
    }
    try {
      setForgetLoading(true)
      const res = await $clientPost('/changeUserPassword', {
        language: getLangType(i18n.language),
        email: email,
        code: code,
        newpassword: rePassword
      })
      if (res.code === 0) {
        messageApi.open({ type: 'success', content: res.msg });
        setEmail('')
        setCode('')
        setRePassword('')
        getUser()
        setTimeout(() => {
          closeModal()
          setForgetLoading(false)
        }, 1000);
      } else {
        throw new Error(res.msg)
      }
    } catch (error: any) {
      messageApi.open({ type: 'error', content: error.message });
      setForgetLoading(false)
    }
  }

  const showForgetModal = () => {
    dispatch(changeShowChangePassword(false))
    dispatch(changeShowForget(true))
  }

  return (
    <>
      {contextHolder}
      <Modal open={showChangePassword} width={screenWidth >= 768 ? 720 : '90%'} maskClosable={false} onCancel={closeModal}>
        <div className='login-style'>
          <div className={
            clsx('login-text1 mb-3', Chiq_Reduced_Bold.className)
          }>{t('t204')}</div>
          <div className='mb-4 md:mb-6'>
            <div className='add-inputbox'>
              <Input placeholder={t('t90')} value={email} onChange={(e: any) => setEmail(e.target.value)} />
            </div>
          </div>
          <div className='mb-4 md:mb-6'>
            <div className='add-inputbox'>
              <Input placeholder={t('t51')} value={code} onChange={(e: any) => setCode(e.target.value)} addonAfter={
                <Button loading={codeLoading} onClick={sendCode}>{codeDown ? codeDownNum : t('t52')}</Button>
              } />
            </div>
          </div>
          <div className='pb-5 md:pb-10 bb1'>
            <div className='add-inputbox'>
              <Input.Password placeholder={t('t206')} value={rePassword} onChange={(e: any) => setRePassword(e.target.value)} />
            </div>
          </div>
          <div className='pt-[15px] md:pt-[30px] between-center gap-[5px]'>
            <div className='text-xs md:text-sm text-grayb2 be_hover_underline' onClick={showForgetModal}>
              {t('t205')}
            </div>
            <div className='end-center gap-[10px] md:gap-5'>
              <div className='text-green81 text-sm md:text-lg be_hover_underline' onClick={closeModal}>{t('t203')}</div>
              <Button loading={forgetLoading} onClick={changePassword}>{t('t194')}</Button>
            </div>
          </div>
        </div>
      </Modal>
    </>
  )
}