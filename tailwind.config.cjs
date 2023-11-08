const daisyui = require("daisyui");
const forms = require("@tailwindcss/forms");
const typo = require("@tailwindcss/typography");

/** @type {import('tailwindcss').Config}*/
const config = {
  content: ["./src/**/*.{html,js,svelte,ts}"],

  theme: {
    extend: {
      "primary": "#641ae6",
      "secondary": "#d926a9",
      "accent": "#1fb2a6",
      "neutral": "#2a323c",
      "base-100": "#1d232a",
      "info": "#3abff8",
      "success": "#36d399",
      "warning": "#fbbd23",
      "error": "#f87272",
    },
  },

  plugins: [forms, typo, daisyui],
};

module.exports = config;
