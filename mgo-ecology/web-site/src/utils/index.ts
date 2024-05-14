export const getLangType = (local_lang: string) => {
  const language = local_lang == 'zh' ? '0' : local_lang == 'zhTw' ? '1' : local_lang == 'en' ? '2' : local_lang == 'kr' ? '3' : ''
  return language
}

export const windowOpen = (url: string) => {
  if (url) {
    window.open(url)
  }
}

export const getLastPage = (total: number, pageSize: number) => {
  if (total % pageSize === 0) {
    return total / pageSize;
  } else {
    return Math.ceil(total / pageSize);
  }
}