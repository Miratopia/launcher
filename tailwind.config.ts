import colors from 'tailwindcss/colors'
import type { Config } from 'tailwindcss'

export default <Partial<Config>>{
  theme: {
    extend: {
      colors: {
        primary: colors.yellow,
      }
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
