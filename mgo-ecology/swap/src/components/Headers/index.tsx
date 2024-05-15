import styled from 'styled-components';
import img_logo from '@/assets/image/logo.png';
import { ConnectButton } from '@mgonetwork/wallet-kit';
import '@/assets/style/common.css';
import { useNavigate } from 'react-router-dom';

const Header = styled.div`
  height: 56px;
  padding: 0 16px;
  background-color: var(--colors-02070F);
  border-bottom: 1px solid rgba(151, 151, 151, 0.5);
  display: flex;
  justify-content: space-between;
  align-items: center;
  .logo-img{
    /* border-radius: 50%; */
    /* overflow: hidden; */
    width: 32px;
    height: auto;
  }
  .connet-btn{
    height: 32px;
    border: 0px;
    outline: 0px;
    cursor: pointer;
    box-shadow: rgba(14, 14, 44, 0.4) 0px -1px 0px 0px inset;
    background: var(--colors-1fc7d4);
    color: #fff;
    font-size: 16px;
    font-weight: bold;
    border-radius: 16px;
    padding-left: 16px;
    padding-right: 16px;
  }
`

export default function Headers() {
  const router = useNavigate()
  const goIndex = () => {
    router('/')
  }

  return (
    <Header>
      <img src={img_logo} alt="" className='logo-img' onClick={goIndex} />
      {/* <button className='connet-btn'>Connect</button> */}
      <ConnectButton connectText='Connect' />
    </Header>
  )
}
