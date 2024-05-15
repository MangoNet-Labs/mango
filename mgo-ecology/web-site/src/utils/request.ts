export async function $ssrPost(url: string, body: { [key: string]: any } | {} = {}) {
  const res = await fetch(`${process.env.NEXT_PUBLIC_API_BASE_URL}${url}`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json;charset=utf-8'
    },
    body: JSON.stringify(body),
    cache: 'no-cache',  //ssr(服务端动态)设定'no-cache'  ssg(服务端静态)设定'force-cache'
  })
  if (res.status !== 200) {
    // This will activate the closest `error.js` Error Boundary
    throw new Error('$ssrPost Failed to fetch data')
  }
  return res.json()
}

export async function $ssrGet(url: string) {
  const res = await fetch(`${process.env.NEXT_PUBLIC_API_BASE_URL}${url}`, {
    method: 'GET',
    headers: {
      'Content-Type': 'application/json'
    },
    cache: 'no-cache'
  })
  if (res.status !== 200) {
    throw new Error('$ssrGet Failed to fetch data')
  }
  return res.json()
}

export async function $ssgPost(url: string, body: { [key: string]: any } | {} = {}) {
  const res = await fetch(`${process.env.NEXT_PUBLIC_API_BASE_URL}${url}`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json;charset=utf-8'
    },
    body: JSON.stringify(body),
    cache: 'force-cache'
  })
  if (res.status !== 200) {
    throw new Error('$ssgPost Failed to fetch data')
  }
  return res.json()
}

export async function $ssgGet(url: string) {
  const res = await fetch(`${process.env.NEXT_PUBLIC_API_BASE_URL}${url}`, {
    method: 'GET',
    headers: {
      'Content-Type': 'application/json'
    },
    cache: 'force-cache'
  })
  if (res.status !== 200) {
    throw new Error('$ssgGet Failed to fetch data')
  }
  return res.json()
}

export async function $isrPost(url: string, body: { [key: string]: any } | {} = {}, revalidate: number = 300) {
  const res = await fetch(`${process.env.NEXT_PUBLIC_API_BASE_URL}${url}`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json;charset=utf-8'
    },
    body: JSON.stringify(body),
    next: {
      revalidate: revalidate
    }
  })
  if (res.status !== 200) {
    throw new Error('$isrPost Failed to fetch data')
  }
  return res.json()
}

export async function $isrGet(url: string, revalidate: number = 60) {
  const res = await fetch(`${process.env.NEXT_PUBLIC_API_BASE_URL}${url}`, {
    method: 'GET',
    headers: {
      'Content-Type': 'application/json'
    },
    next: {
      revalidate: revalidate
    }
  })
  if (res.status !== 200) {
    throw new Error('$isrGet Failed to fetch data')
  }
  return res.json()
}

export async function $clientPost(url: string, body: { [key: string]: any } | {} = {}) {
  const token = localStorage.getItem('nextmango:tk')
  const res = await fetch(`${process.env.NEXT_PUBLIC_API_BASE_URL}${url}`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json;charset=utf-8',
      // 'Content-Type': 'application/x-www-form-urlencoded;charset=UTF-8',
      'x-token': token ?? ''
    },
    body: JSON.stringify(body),
    cache: 'no-cache',  //ssr(服务端动态)设定'no-cache'  ssg(服务端静态)设定'force-cache'
  })
  if (res.status !== 200) {
    // This will activate the closest `error.js` Error Boundary
    throw new Error('$clientPost Failed to fetch data')
  }
  return res.json()
}

export async function $clientGet(url: string) {
  const token = localStorage.getItem('nextmango:tk')
  const res = await fetch(`${process.env.NEXT_PUBLIC_API_BASE_URL}${url}`, {
    method: 'GET',
    headers: {
      'Content-Type': 'application/json',
      'x-token': token ?? ''
    }
  })
  if (res.status !== 200) {
    throw new Error('$clientGet Failed to fetch data')
  }
  return res.json()
}