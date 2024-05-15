import type { Config } from "tailwindcss";

const config: Config = {
  content: [
    "./src/pages/**/*.{js,ts,jsx,tsx,mdx}",
    "./src/components/**/*.{js,ts,jsx,tsx,mdx}",
    "./src/app/**/*.{js,ts,jsx,tsx,mdx}",
  ],
  theme: {
    extend: {
      colors: {
        black06: '#06061A',
        black0a: '#0A0A1E',
        black32: '#323245',
        grayb2: '#B2B2B2',
        gray83: '#83838D',
        grayd8: '#D8D8D8',
        gray66: '#666666',
        grayce: '#CED5E3',
        gray67: '#676773',
        gray9e: '#9EA2AB',
        green81: '#81D8CF',
        greene5: '#E5F7EE',
        redff: '#FF0000'
      }
    },
  },
  plugins: [],
  corePlugins: {
    preflight: false
  }
};
export default config;
