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

// 请求拦截器
instance.interceptors.request.use(
  config => {
    // 每次发送请求之前判断是否存在token，如果存在，则统一在http请求的header都加上token，不用每次请求都手动添加了
    // 即使本地存在token，也有可能token是过期的，所以在响应拦截器中要对返回状态进行判断
    const token = localStorage.getItem('mgoswap:token');
    token && (config.headers.token = token);
    return config;
  },
  error => {
    return Promise.reject(error);
  }
)

// 响应拦截器
instance.interceptors.response.use(
  response => {
    if (response.status === 200) {
      return Promise.resolve(response.data);
    } else {
      Toast.show({ content: response.data.msg })
      return Promise.reject(response.data);
    }
  },
  // 服务器状态码不是200的情况    
  error => {
    if (error.response.status) {
      switch (error.response.status) {
        case 401:
          //未登录
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