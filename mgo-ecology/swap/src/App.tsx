import { createGlobalStyle } from 'styled-components';
import { lazy, Suspense } from 'react';
import { Routes, Route, Navigate } from 'react-router-dom';
import Headers from '@/components/Headers';
import PageLoading from '@/components/PageLoading';
import img_bg1 from '@/assets/image/bg1.png'

const Swap = lazy(() => import('@/pages/Swap.tsx'))
const Pool = lazy(() => import('@/pages/Pool.tsx'))
const Add = lazy(() => import('@/pages/Add.tsx'))

const GlobalStyles = createGlobalStyle`
  html, body, ul, li, ol, dl, dd, dt, p, h1, h2, h3, h4, h5, h6, form, fieldset, legend, img { margin:0; padding:0; }
  ul, ol ,li{ list-style:none; }
  /* body { background: linear-gradient(180deg,#f7f7f8 22.88%,#d6f2eb 99.79%,#2ed8a7 99.87%,#00a380);} */
  body { 
    background: var(--colors-02070F);
    background-image: url(${img_bg1});
    background-repeat: no-repeat;
    background-size: 100% auto;
    background-position: center bottom;
  }
  *{box-sizing:border-box}
  body,html,#root{
      min-height: 100%;
      /* overflow: hidden; */
  }
  :root {
    --colors-7645d9: #7645d9;
    --colors-1fc7d4: #1fc7d4;
    --colors-280d5f: #280d5f;
    --colors-7a6eaa: #7a6eaa;
    --colors-ffffff: #FFFFFF;
    --colors-02070F: #02070F;
    --colors-80D8CF:#80D8CF;
  }
`

function App() {
  return (
    <>
      <GlobalStyles />
      <Headers />
      <Suspense fallback={<PageLoading />}>
        <Routes>
          {/* <Route index element={<Swap />} /> */}
          <Route path="/" element={<Navigate to="/swap" />} />
          <Route path="/swap" element={<Swap />}></Route>
          <Route path="/pool" element={<Pool />}></Route>
          <Route path="/add" element={<Add />}></Route>
        </Routes>
      </Suspense>
    </>
  )
}

export default App
