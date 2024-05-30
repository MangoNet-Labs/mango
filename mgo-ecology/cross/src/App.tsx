import { createGlobalStyle } from 'styled-components';
import { lazy, Suspense } from 'react';
import { Routes, Route } from 'react-router-dom';
import PageLoading from '@/components/PageLoading';
import img_bg1 from '@/assets/image/bg1.png'

const Home = lazy(() => import('@/pages/Home.tsx'))

const GlobalStyles = createGlobalStyle`
  html, body, ul, li, ol, dl, dd, dt, p, h1, h2, h3, h4, h5, h6, form, fieldset, legend, img { margin:0; padding:0; }
  ul, ol ,li{ list-style:none; }
  body { 
    background: var(--colors-02070F);
    color: var(--colors-FFFFFF);
    background-image: url(${img_bg1});
    background-repeat: no-repeat;
    background-size: 100% auto;
    background-position: center bottom;
  }
  *{
    box-sizing:border-box
  }
  body,html,#root{
      min-height: 100%;
      /* overflow: hidden; */
  }
  html{
    &::-webkit-scrollbar{
      width: 10px;
    }
    &::-webkit-scrollbar-track{
      background-color: rgb(78,78,84);
    }
    &::-webkit-scrollbar-thumb{
      background: rgba(0,0,0,0.25);
      &:hover{
        background: rgba(0,0,0,0.5);
      }
    }
  }
  :root {
    --colors-02070F: #02070F;
    --colors-FFFFFF: #FFFFFF;
    --colors-293241: #293241;
    --colors-DAFCF8: #DAFCF8;
    --colors-F50000: #F50000;
    --colors-555555: #555555;
  }
`

function App() {
  return (
    <>
      <GlobalStyles />
      <Suspense fallback={<PageLoading />}>
        <Routes>
          <Route path="/" element={<Home />}></Route>
        </Routes>
      </Suspense>
    </>
  )
}

export default App
