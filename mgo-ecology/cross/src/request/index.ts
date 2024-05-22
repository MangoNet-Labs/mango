import axios from 'axios'
import qs from "qs";
import { Toast } from 'antd-mobile'

export const imgurl = import.meta.env.VITE_IMG_BASE_URL
const instance = axios.create({
  baseURL: import.meta.env.VITE_API_BASE_URL,
  timeout: 30000,
  headers: {
    get: {
      'Content-Type': 'application/json'
    },
    post: {
      'Content-Type': 'application/x-www-form-urlencoded;charset=UTF-8'
    }
  }
})

instance.interceptors.request.use(
  config => {
    
    const token = localStorage.getItem('mgoswap:token');
    token && (config.headers.token = token);
    return config;
  },
  error => {
    return Promise.reject(error);
  }
)


instance.interceptors.response.use(
  response => {
    if (response.status === 200 || response.status === 201) {
      return Promise.resolve(response.data);
    } else {
      Toast.show({ content: response.data.msg })
      return Promise.reject(response.data);
    }
  },
 
  error => {
    if (error.response.status) {
      switch (error.response.status) {
        case 401:
          Toast.show({ content: error.response.data.msg })
          localStorage.removeItem('mgoswap:token')
          break;
        default:
          Toast.show({ content: 'Abnormal network request' })
          break;
      }
      return Promise.reject(error.response.data);
    }
  }
);

// enum LangType {
//   zh = 'zh',
//   en = 'en'
// }
// const getLangType = () => {
//   const local_lang = localStorage.getItem('mgoswap:lang')
//   const language = local_lang == 'zh-CN' ? LangType.zh : LangType.en
//   return language
// }

interface Arguments {
  url: string,
  method: string,
  [key: string]: any
}
async function apiRequest(method: string, url: string, params = {}, isupload = false) {
  let arg: Arguments = {
    url: url,
    method: method
  }
  const addObj = {
    // lang: getLangType()
  }
  method == 'GET' ? arg['params'] = Object.assign(params, addObj) : method == 'POST' ? arg['data'] = qs.stringify(Object.assign(params, addObj)) : ''
  if (isupload) {
    arg['headers']['Content-Type'] = 'multipart/form-data'
    arg['data'] = params
  }
  return instance(arg)
}

export const $get = (url: string, params?: object) => apiRequest('GET', url, params)
export const $post = (url: string, params?: object) => apiRequest('POST', url, params)
export const $upload = (url: string, params?: object) => apiRequest('POST', url, params, true)