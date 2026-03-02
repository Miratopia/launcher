import colors from 'tailwindcss/colors'
import type { Config } from 'tailwindcss'

export default <Partial<Config>>{
  theme: {
    extend: {
      colors: {
        primary: colors.yellow,
      },
      keyframes: {
        indeterminate: {
          '0%': { transform: 'translateX(-100%)' },
          '100%': { transform: 'translateX(400%)' },
        },
      },
      animation: {
        indeterminate: 'indeterminate 1.5s ease-in-out infinite',
      },
    },
  },
  content: {
    files: [
      "srcDir/components/**/*.{vue,js,jsx,mjs,ts,tsx}",
      "srcDir/layouts/**/*.{vue,js,jsx,mjs,ts,tsx}",
      "srcDir/pages/**/*.{vue,js,jsx,mjs,ts,tsx}",
      "srcDir/plugins/**/*.{js,ts,mjs}",
      "srcDir/composables/**/*.{js,ts,mjs}",
      "srcDir/{A,a}pp.{vue,js,jsx,mjs,ts,tsx}",
      "srcDir/{E,e}rror.{vue,js,jsx,mjs,ts,tsx}",
    ]
  },
  plugins: [],
}
