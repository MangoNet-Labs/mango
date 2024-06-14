"use client"
import { $clientPost } from '@/utils/request'
import '@/styles/apply.scss'
import { useTranslation } from 'react-i18next';
import { useState, useEffect } from 'react'
import { getLangType } from '@/utils'
import type { RadioChangeEvent, GetProp } from 'antd';
import { Input, Radio, Checkbox, Button, message } from 'antd';
import { AmountListType } from '@/interface'

export default function index({ categorys, amountList }: { categorys: AmountListType[], amountList: AmountListType[] }) {
  const { t, i18n } = useTranslation();
  const [messageApi, contextHolder] = message.useMessage();

  const [isloading, setIsloading] = useState(false)
  const [projectName, setProjectName] = useState('')
  const [description, setDescription] = useState('')
  const [category, setCategory] = useState('')
  const radioChange = (e: RadioChangeEvent) => {
    setCategory(e.target.value);
  }

  const [secondCategory, setSecondCategory] = useState<any[]>([])
  const checkboxChange: GetProp<typeof Checkbox.Group, 'onChange'> = (checkedValues) => {
    if (checkedValues.length > 3) return
    setSecondCategory(checkedValues)
  };

  const [amount, setAmount] = useState('')
  const radioChange2 = (e: RadioChangeEvent) => {
    setAmount(e.target.value);
  }

  const [firstName, setFirstName] = useState('')
  const [lastName, setLastName] = useState('')
  const [email, setEmail] = useState('')

  const submit = async () => {
    if (!projectName) {
      messageApi.open({ type: 'warning', content: t('t64') });
      return
    }
    if (!description) {
      messageApi.open({ type: 'warning', content: t('t65') });
      return
    }
    if (!category) {
      messageApi.open({ type: 'warning', content: t('t66') });
      return
    }
    if (!secondCategory.length) {
      messageApi.open({ type: 'warning', content: t('t67') });
      return
    }
    if (!amount) {
      messageApi.open({ type: 'warning', content: t('t68') });
      return
    }
    if (!firstName) {
      messageApi.open({ type: 'warning', content: t('t69') });
      return
    }
    if (!lastName) {
      messageApi.open({ type: 'warning', content: t('t70') });
      return
    }
    if (!email) {
      messageApi.open({ type: 'warning', content: t('t71') });
      return
    }
    try {
      setIsloading(true)
      const res = await $clientPost('/grants', {
        language: getLangType(i18n.language),
        name: projectName,
        describe: description,
        flag: category,
        secondlist: secondCategory.join(','),
        money: amount,
        username: firstName,
        surname: lastName,
        email: email
      })
      if (res.code === 0) {
        messageApi.open({ type: 'success', content: res.msg });
        clearInfo()
      } else {
        throw new Error(res.msg)
      }
      setTimeout(() => {
        setIsloading(false)
      }, 1000);
    } catch (error: any) {
      messageApi.open({ type: 'error', content: error.message });
      setIsloading(false)
    }
  }

  const clearInfo = () => {
    setProjectName('')
    setDescription('')
    setCategory('')
    setSecondCategory([])
    setAmount('')
    setFirstName('')
    setLastName('')
    setEmail('')
  }

  return (
    <>
      {contextHolder}
      <div className='apply-style'>
        <div className='box1 px-[15px] md:px-[50px] pb-[15px] md:pb-[50px]'>
          <div className="pt-[15px] md:pt-[50px] bb32">
            <div className="text-[14px] md:text-2xl">
              <span className="text-white">1. {t('t75')}</span>
              <span className="text-redff">* </span>
            </div>
            <div className="pt-[10px] md:pt-5 pb-[10px]">
              <Input placeholder={t('t76')} value={projectName} onChange={(e: any) => setProjectName(e.target.value)} />
            </div>
          </div>
          <div className="pt-[15px] md:pt-[50px] bb32">
            <div className="text-[14px] md:text-2xl">
              <span className="text-white">2. {t('t77')}</span>
              <span className="text-redff">* </span>
            </div>
            <div className="text-grayb2 text-[12px] md:text-xl pt-[5px] md:pt-5">{t('t78')}</div>
            <div className="pt-[10px] md:pt-5 pb-[10px]">
              <Input placeholder={t('t76')} value={description} onChange={(e: any) => setDescription(e.target.value)} />
            </div>
          </div>
          <div className="pt-[15px] md:pt-[50px] bb32">
            <div className="text-[14px] md:text-2xl">
              <span className="text-white">3. {t('t79')}:</span>
              <span className="text-redff">* </span>
            </div>
            <div className="text-grayb2 text-[12px] md:text-xl pt-[5px] md:pt-5">{t('t80')}:</div>
            <div className="py-5">
              <Radio.Group style={{ width: '100%' }} onChange={radioChange} value={category}>
                <div className='b1-grid1'>
                  {
                    categorys.map(item => (

                      <Radio key={item.value} value={item.value}>{item.label}</Radio>
                    ))
                  }
                </div>
              </Radio.Group>
            </div>
          </div>
          <div className="pt-[15px] md:pt-[50px] bb32">
            <div className="text-[14px] md:text-2xl">
              <span className="text-white">4. {t('t81')}:</span>
              <span className="text-redff">* </span>
            </div>
            <div className="text-grayb2 text-[12px] md:text-xl pt-[5px] md:pt-5">
              {t('t82')}<br />
              {t('t83')} {!secondCategory ? 3 : secondCategory.length ? 3 - secondCategory.length : 3}:</div>
            <div className="py-5">
              <Checkbox.Group value={secondCategory} style={{ width: '100%' }} onChange={checkboxChange}>
                <div className='b1-grid1'>
                  {
                    categorys.map(item => (
                      <Checkbox key={item.value} value={item.value}>{item.label}</Checkbox>
                    ))
                  }
                </div>
              </Checkbox.Group>
            </div>
          </div>
          <div className="pt-[15px] md:pt-[50px] bb32">
            <div className="text-[14px] md:text-2xl">
              <span className="text-white">5. {t('t84')}:</span>
              <span className="text-redff">* </span>
            </div>
            <div className="text-grayb2 text-[12px] md:text-xl pt-[5px] md:pt-5">{t('t85')}</div>
            <div className="text-grayb2 text-[12px] md:text-xl pt-[5px] md:pt-5">{t('t86')}</div>
            <div className="py-5">
              <Radio.Group style={{ width: '100%' }} onChange={radioChange2} value={amount}>
                <div className='b1-grid1'>
                  {
                    amountList.map(item => (

                      <Radio key={item.value} value={item.value}>{item.label}</Radio>
                    ))
                  }
                </div>
              </Radio.Group>
            </div>
          </div>
          <div className="pt-[15px] md:pt-[50px] bb32">
            <div className="text-[14px] md:text-2xl">
              <span className="text-white">6. {t('t87')}</span>
              <span className="text-redff">* </span>
            </div>
            <div className="text-[14px] md:text-xl pt-[5px] md:pt-5">
              <span className="text-white">{t('t88')}</span>
              <span className="text-redff">* </span>
            </div>
            <div className="pt-[10px] md:pt-5 pb-[10px]">
              <Input placeholder={t('t76')} value={firstName} onChange={(e: any) => setFirstName(e.target.value)} />
            </div>
          </div>
          <div className="pt-[15px] md:pt-[50px] bb32">
            <div className="text-[14px] md:text-xl pt-[5px] md:pt-5">
              <span className="text-white">{t('t89')}</span>
              <span className="text-redff">* </span>
            </div>
            <div className="pt-[10px] md:pt-5 pb-[10px]">
              <Input placeholder={t('t76')} value={lastName} onChange={(e: any) => setLastName(e.target.value)} />
            </div>
          </div>
          <div className="pt-[15px] md:pt-[50px] bb32">
            <div className="text-[14px] md:text-xl pt-[5px] md:pt-5">
              <span className="text-white">{t('t90')}</span>
              <span className="text-redff">* </span>
            </div>
            <div className="pt-[10px] md:pt-5 pb-[10px]">
              <Input placeholder={t('t76')} value={email} onChange={(e: any) => setEmail(e.target.value)} />
            </div>
          </div>
          <div className="pt-[25px] md:pt-[100px]">
            <div className="b1-bth">
              <Button className='van-button' size='large' loading={isloading} block onClick={submit}>{t('t91')}</Button>
            </div>
            <div className="pt-[10px] md:pt-5 text-center">
              <span className="text-white text-[12px] md:text-2xl be_hover_underline" onClick={clearInfo}>{t('t92')}</span>
            </div>
          </div>
        </div>
      </div>
    </>
  )
}
