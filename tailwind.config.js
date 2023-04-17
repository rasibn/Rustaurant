/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./index.html",
    "./src/**/*.{rs,html}"
  ],
  theme: {
    extend: {
      colors: {
      },
      fontFamily: {
        Poppins: ['Poppins, sans-serif'],
      },
    },
  },
  plugins: [],
}

