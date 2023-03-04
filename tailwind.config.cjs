/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ['./src/**/*.{html,js,svelte,ts}'],
  theme: {
    extend: {
      colors: {
        'bg_blue': 'rgb(37 99 235)',
        'bg_red': 'rgb(220 38 38)',
        'bg_green': 'rgb(22 163 74)',
        'bg_yellow': 'rgb(202 138 4)',
        'text_blue': 'rgb(37 99 235)',
        'text_red': 'rgb(220 38 38)',
        'text_green': 'rgb(22 163 74)',
        'text_yellow': 'rgb(202 138 4)',

      }
    },
  },
  plugins: []
};