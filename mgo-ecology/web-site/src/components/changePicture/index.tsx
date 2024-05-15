"use client"
import { useTranslation } from 'react-i18next';
import { changeUserInfo, changeIsLogin, selectShowChangePicture, changeShowChangePicture, selectUserInfo } from '@/lib/storemodules/appSlice'
import { useSelector, useDispatch } from "react-redux";
import { Modal, Input, Button, message, Radio, Upload } from 'antd';
import { useWindowSize } from '@/hooks/useWindowSize'
import { AppDispatch } from '@/lib/store'
import '@/styles/login.scss'
import '@/styles/personal.scss'
import { useState } from 'react';
import { getLangType } from '@/utils'
import { $clientPost, $clientGet } from '@/utils/request'
import { ConfigType } from '@/interface'
import type { UploadProps } from 'antd';

export default function index({ config }: { config: ConfigType }) {
  const { t, i18n } = useTranslation();
  const [messageApi, contextHolder] = message.useMessage();
  const showChangePicture = useSelector(selectShowChangePicture)
  const dispatch = useDispatch<AppDispatch>()
  const { screenWidth } = useWindowSize()
  const userInfo = useSelector(selectUserInfo)

  const getUser = async () => {
    const res: any = await $clientGet('/getInfo')
    if (res.code === 0 && res.data) {
      dispatch(changeUserInfo(res.data))
      localStorage.setItem('nextmango:tk', res.data.token)
      dispatch(changeIsLogin(true))
    }
  }

  const closeModal = () => {
    dispatch(changeShowChangePicture(false))
  }

  const [pictureValue, setPictureValue] = useState('1')
  const radioChange = (e: any) => {
    setPictureValue(e.target.value);
  }

  const [newImgUrl, setNewImgUrl] = useState('')

  const uploadProps: UploadProps = {
    name: 'file',
    action: process.env.NEXT_PUBLIC_API_BASE_URL + '/upload',
    headers: {
      'x-token': userInfo.token,
      // 'Content-Type': 'multipart/form-data',
      // "Accept": "application/json"
    },
    onChange(info) {
      if (info.file.status === 'done') {
        console.log(info.file.response.data.file.url);
        setNewImgUrl(info.file.response.data.file.url)
      } else if (info.file.status === 'error') {
        message.error(`${info.file.name} file upload failed.`);
      }
    },
  }

  const [avatarLoading, setAvatarLoading] = useState(false)
  const changeAvatar = async () => {
    if (pictureValue == '2' && !newImgUrl) {
      messageApi.warning(t('t188'));
      return
    }
    try {
      setAvatarLoading(true)
      const res = await $clientPost('/setInfo', {
        language: getLangType(i18n.language),
        headerImg: pictureValue == '2' ? newImgUrl : config.image
      })
      if (res.code === 0) {
        messageApi.success(res.msg);
        setNewImgUrl('')
        getUser()
        setTimeout(() => {
          closeModal()
          setAvatarLoading(false)
        }, 1000);
      } else {
        throw new Error(res.msg)
      }
    } catch (error: any) {
      messageApi.error(error.message);
      setAvatarLoading(false)
    }
  }

  return (
    <>
      {contextHolder}
      <Modal open={showChangePicture} width={screenWidth >= 768 ? 720 : '90%'} maskClosable={false} onCancel={closeModal}>
        <div className='pic-style login-style'>
          <div className='text-[18px] md:text-[32px] font-bold text-white pb-5 bb1'>{t('t199')}</div>
          <div className='pt-[15px] md:pt-10 bb1'>
            <Radio.Group style={{ width: '100%' }} onChange={radioChange} value={pictureValue}>
              <div className='start-center gap-[10px] md:gap-5 pb-[10px] md:pb-[30px]'>
                <Radio value='1'></Radio>
                <img src={config.image} className="w-[28px] md:w-[56px]" alt="" />
                <span className="text-sm md:text-2xl text-white">{t('t200')}</span>
              </div>
              <div className='between-center pb-5 md:pb-[50px]'>
                <div className='start-center gap-[10px] md:gap-5'>
                  <Radio value="2"></Radio>
                  {
                    newImgUrl ?
                      <img src={newImgUrl} className='w-[28px] md:w-[56px]' alt="" /> :
                      <img src={userInfo.headerImg} className='w-[28px] md:w-[56px]' alt="" />
                  }
                  <span className="text-sm md:text-2xl text-white">{t('t201')}</span>
                </div>
                <div className='center-center'>
                  <Upload {...uploadProps} maxCount={1}>
                    <div className="upboxbg py-1 md:py-2 px-3 md:px-5 center-center gap-[5px] md:gap-[10px] be_pointer">
                      <img src="/img/img104.png" className="w-4" alt="" />
                      <span className="text-xs md:text-base text-green81">{t('t202')}</span>
                    </div>
                  </Upload>
                </div>
              </div>
            </Radio.Group>
          </div>
          <div className="pt-[10px] md:pt-[30px] end-center gap-[10px] md:gap-5">
            <div className="text-green81 text-sm md:text-lg be_hover_underline" onClick={closeModal}>
              {t('t203')}
            </div>
            <Button loading={avatarLoading} onClick={changeAvatar}>{t('t194')}</Button>
          </div>
        </div>
      </Modal>
    </>
  )
}
