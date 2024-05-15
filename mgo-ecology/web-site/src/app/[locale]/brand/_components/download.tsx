"use client"
import '@/styles/brand.scss'
import { useTranslation } from 'react-i18next';

export default function download({ imageList, zip }: { imageList: any, zip: string }) {
  const { t } = useTranslation();
  const toUrl = (url: string) => {
    window.open(url)
  }

  function download(link: string) {
    const xhr = new XMLHttpRequest();
    xhr.open('GET', link)
    xhr.responseType = 'blob'
    xhr.send()
    xhr.onload = function () {
      const fileBlob = xhr.response;
      console.log(fileBlob)
      const fileUrl = URL.createObjectURL(fileBlob)
      console.log(fileUrl)

      let a = document.createElement("a"); 
      let event = new MouseEvent("click"); 
      a.download = ""; 
      a.href = fileUrl; 
      a.dispatchEvent(event);
    }
  }

  return (
    <div className='brand-style'>
      <div className="mb-8">
        <div className="text-xl mb-5">{t('t235')}</div>
        <div className="bra-roun1" onClick={() => toUrl(zip)}>{t('t236')}</div>
      </div>
      <div>
        {
          Object.keys(imageList).map(key => (
            <div className='pt-5' key={key}>
              <div className="text-lg font-semibold pb-3 bb1">{key}</div>
              <div>
                {
                  imageList[key].map((item: any, i: any) => (
                    <div className='pt-5 pb-7 px-[10%] md:px-[15%] lg:px-[25%] between-start bb1' key={i}>
                      <div className="start-center gap-5 flex-wrap">
                        <div className="bra-roun2" onClick={() => download(item.pngImage)}>PNG</div>
                        <div className="bra-roun2" onClick={() => download(item.svgFile)}>SVG</div>
                      </div>
                      <div className="max-w-[130px]">
                        <img src={item.pngImage} className="mr-auto ml-auto max-w-full h-auto" alt="" />
                      </div>
                    </div>
                  ))
                }
              </div>
            </div>
          ))
        }
      </div>
    </div>
  )
}
