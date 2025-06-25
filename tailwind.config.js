const { fontFamily } = require("tailwindcss/defaultTheme");
/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./templates/*.html"],
  theme: {
    extend: {
      dropShadow: {
        glow: [
          "0 0px 4px rgba(255, 255, 255, 0.15)",
          "0 0px 4px rgba(255, 255, 255, 0.15)",
        ],
      },
      fontFamily: {
        hypersystem: ["Hypersystem", ...fontFamily.sans],
      },
      //colors: {
      //  'dark-purple': '#1F0A1E',
      //  'cinnabar': '#D1603D',
      //  'powder-blue': '#98C1D9',
      //  'state-blue': '#6969B3',
      //  'sunset': '#FFCB77',
      //},
    },
  },
  plugins: [require("@tailwindcss/forms")],
};
