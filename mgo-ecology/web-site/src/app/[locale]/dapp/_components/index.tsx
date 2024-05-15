"use client"
import { useState, useEffect } from 'react'
import { useTranslation } from 'react-i18next';
import '@/styles/dapp.scss'
import { AutoComplete } from 'antd';
import { $ssrGet } from '@/utils/request'
import { getLangType, windowOpen } from '@/utils'
import { DappType } from '@/interface';
import type { MenuProps } from 'antd';
import { Menu, Dropdown } from 'antd';

export default function index({ items, categorys }: { items: MenuProps['items'], categorys: MenuProps['items'] }) {
  const { t, i18n } = useTranslation();

  const [options, setOptions] = useState<DappType[]>([])
  const [dappList, setDappList] = useState<DappType[]>([])
  const querySearchAsync = async (inputValue: string) => {
    const res = await $ssrGet(`/dApp?class_id=&classification_id=&language=${getLangType(i18n.language)}&page=1&pageSize=100&title=${inputValue}`)
    const list = res.data.list.map((ele: any) => ({
      ...ele,
      value: ele.title
    }))
    setOptions(list)
  }
  const handleSelect = (value: string) => {
    const item = options.find(ele => ele.value == value)!
    setDappList([item])
  }
  const getDapp = async () => {
    const res = await $ssrGet(`/dApp?class_id=${selectValue}&classification_id=${menuValue}&language=${getLangType(i18n.language)}&page=1&pageSize=100`)
    setDappList(res.data.list)
  }
  useEffect(() => {
    if (items?.length) {
      setMenuValue(items[0]?.key as string)
    }
  }, [])

  const [menuValue, setMenuValue] = useState('')
  const menuChange = (item: any) => {
    setMenuValue(item.key)
  }

  const [selectValue, setSelectValue] = useState('')
  const [selectInputValue, setSelectInputValue] = useState('')
  const selectChange: MenuProps['onClick'] = ({ key }) => {
    setSelectValue(key)
    const item: any = categorys?.find(ele => ele?.key == key)
    setSelectInputValue(item?.label)
  }

  useEffect(() => {
    if (menuValue) {
      getDapp()
    }
  }, [menuValue])

  useEffect(() => {
    if (selectValue) {
      getDapp()
    }
  }, [selectValue])


  return (
    <div className='dapp-style box-px'>
      <div className='box1 between-center gap-3'>
        <span className="b1-text1">{t('t132')}</span>
        <div className='b1-inp'>
          <AutoComplete
            placeholder={t('t217')}
            onSearch={querySearchAsync}
            style={{
              width: '100%',
              height: '50px'
            }}
            options={options}
            onSelect={handleSelect}
          />
        </div>
      </div>
      <div className='box2'>
        <div className='b2-left'>
          <Menu
            items={items}
            selectedKeys={[menuValue]}
            onSelect={menuChange}
          />
        </div>
        <div className='b2-top'>
          <Menu
            items={items}
            onSelect={menuChange}
            mode='horizontal'
            selectedKeys={[menuValue]}
          />
        </div>
        <div className='b2-bottom'>
          <div className='b2-select'>
            <Dropdown menu={{ items: categorys, onClick: selectChange }} placement="bottom">
              <input placeholder={t('t218')} className='add-input' type="text" value={selectInputValue} readOnly />
            </Dropdown>
          </div>
          <div className='b2-list'>
            {
              dappList.map(item => (
                <div onClick={() => windowOpen(item.url)} className='b2-items be_pointer' key={item.ID}>
                  <div className='b2i-row1 start-center'>
                    <img src={item.image} className="b2i-img1" alt="" />
                    <span className="b2i-text1">{item.title}</span>
                  </div>
                  {/* <div className='b2i-row2 ellipsis2'>{item.introduction}</div> */}
                  <div className='b2i-row2 ellipsis2' dangerouslySetInnerHTML={{ __html: item.introduction }}></div>
                  <div className='b2i-row3 end-center'>
                    <img src='/img/img45.png' className="b2i-img2 be_pointer" alt="" onClick={() => windowOpen(item.twitter)} />
                    <img src='/img/img47.png' className="b2i-img2 be_pointer" alt="" onClick={() => windowOpen(item.telegram)} />
                    <img src='/img/img46.png' className="b2i-img2 be_pointer" alt="" onClick={() => windowOpen(item.discord)} />
                  </div>
                </div>
              ))
            }
          </div>
          <div className="b2-tips">
            {t('t133')}:
            <span>{t('t134')}</span>
          </div>
        </div>
      </div>
    </div>
  )
}
