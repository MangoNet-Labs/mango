import '@/styles/personal.scss'
import '@/styles/login.scss'
import { useTranslation } from 'react-i18next';
import { selectUserInfo, changeUserInfo, changeIsLogin, changeShowForget, changeShowChangePassword, changeShowChangeEmail, changeShowChangePicture } from '@/lib/storemodules/appSlice'
import { useSelector, useDispatch } from "react-redux";
import { AppDispatch } from '@/lib/store'
import { useState } from 'react';
import { Input, Button, message } from 'antd';
import { $clientPost, $clientGet } from '@/utils/request'
import { getLangType } from '@/utils'
import { useRouter } from 'next/navigation'

export default function personal() {
  const { t, i18n } = useTranslation();
  const [messageApi, contextHolder] = message.useMessage();
  const userInfo = useSelector(selectUserInfo)
  const dispatch = useDispatch<AppDispatch>()
  const router = useRouter()

  const getUser = async () => {
    const res: any = await $clientGet('/getInfo')
    if (res.code === 0 && res.data) {
      dispatch(changeUserInfo(res.data))
      localStorage.setItem('nextmango:tk', res.data.token)
      dispatch(changeIsLogin(true))
    }
  }

  const [showChangeUserName, setShowChangeUserName] = useState(false)
  const [userName, setUserName] = useState('')
  const [userNameLoading, setUserNameLoading] = useState(false)
  const changeUserName = async () => {
    if (!userName) {
      messageApi.open({ type: 'warning', content: t('t44') });
      return
    }
    try {
      setUserNameLoading(true)
      const res = await $clientPost('/setInfo', {
        language: getLangType(i18n.language),
        username: userName
      })
      if (res.code === 0) {
        messageApi.open({ type: 'success', content: res.msg });
        setUserName('')
        getUser()
        setTimeout(() => {
          setShowChangeUserName(false)
          setUserNameLoading(false)
        }, 1000);
      } else {
        throw new Error(res.msg)
      }
    } catch (error: any) {
      messageApi.open({ type: 'error', content: error.message });
      setUserNameLoading(false)
    }
  }

  const signOut = () => {
    localStorage.removeItem('nextmango:tk')
    dispatch(changeUserInfo({}))
    dispatch(changeIsLogin(false))
    router.replace('/')
  }

  return (
    <>
      {contextHolder}
      <div className='personal-style login-style'>
        <div className='text-sm md:text-lg font-semibold text-white mb-[10px]'>{t('t192')}</div>
        <div className="start-center gap-[10px] mb-1">
          <span className="text-xs md:text-base text-white">{userInfo.userName}</span>
          <img src="/img/img92.png" className="w-4 be_pointer" onClick={() => setShowChangeUserName(!showChangeUserName)} alt="" />
        </div>
        {
          showChangeUserName && <div className='w-full md:w-[60%] py-2'>
            <div className='add-inputbox'>
              <Input placeholder={t('t193')} value={userName} onChange={(e: any) => setUserName(e.target.value)} addonAfter={
                <Button loading={userNameLoading} onClick={changeUserName}>{t('t194')}</Button>
              } />
            </div>
          </div>
        }
        <div className='text-gray83 text-xs md:text-sm mb-[15px] md:mb-[30px]'>{t('t195')} @{userInfo.userName}</div>
        <div className="text-sm md:text-lg font-semibold text-white mb-[10px]">{t('t219')}</div>
        <div className="start-center gap-[10px] mb-[15px] md:mb-[30px]">
          <img src={userInfo.headerImg} className="h-[28px] md:h-[56px]" alt="" />
          <img src="/img/img92.png" className="w-4 be_pointer" onClick={() => dispatch(changeShowChangePicture(true))} alt="" />
        </div>
        <div className="text-sm md:text-lgtext-lg font-semibold text-white mb-[10px]">{t('t90')}</div>
        <div className="start-center gap-[10px] mb-[15px] md:mb-[30px]">
          <span className="text-xs md:text-base text-white">{userInfo.email}</span>
          <img src="/img/img92.png" className="w-4 be_pointer" onClick={() => dispatch(changeShowChangeEmail(true))} alt="" />
        </div>
        <div className="text-sm md:text-lgtext-lg font-semibold text-white mb-[10px]">{t('t40')}</div>
        <div className="start-center gap-[10px] mb-[10px]">
          <span className="text-xs md:text-base text-white">{t('t196')}</span>
          <img src="/img/img92.png" className="w-4 be_pointer" onClick={() => dispatch(changeShowChangePassword(true))} alt="" />
        </div>
        <div className="start-center gap-[10px] mb-[10px]">
          <span className="text-xs md:text-base text-white">{t('t197')}</span>
          <img src="/img/img92.png" className="w-4 be_pointer" onClick={() => dispatch(changeShowForget(true))} alt="" />
        </div>
        <div className="py-[25px] md:py-[120px] start-center">
          <div className="w-[200px] md:w-[35%]">
            <Button className='van-button' size='large' block onClick={signOut}>{t('t198')}</Button>
          </div>
        </div>
      </div>
    </>
  )
}
