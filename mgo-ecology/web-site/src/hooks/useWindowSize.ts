"use client"
import { useEffect, useState } from 'react'
import { useDebouncedCallback } from 'use-debounce';

export function useWindowSize() {
  const [screenWidth, setScreenWidth] = useState(0)
  const [screenHeight, setScreenHeight] = useState(0)

  const handleResize = () => {
    setScreenWidth(window.innerWidth);
    setScreenHeight(window.innerHeight);
  };

  const change = useDebouncedCallback(handleResize, 100)

  useEffect(() => {
    setScreenWidth(window.innerWidth);
    setScreenHeight(window.innerHeight);
    window.addEventListener('resize', change)
    return () => {
      window.removeEventListener('resize', change)
    }
  }, [])

  return {
    screenWidth,
    screenHeight
  }
}