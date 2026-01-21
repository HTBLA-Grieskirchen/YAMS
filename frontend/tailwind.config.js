const { heroui } = require("@heroui/react");

/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./src/**/*.{js,jsx,ts,tsx}",
    "./node_modules/@heroui/theme/dist/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {},
  },
  darkMode: "class",
  plugins: [
    heroui({
      themes: {
        light: {
          colors: {
            primary: {
              DEFAULT: "#0ea5e9",
              foreground: "#ffffff",
            },
            secondary: {
              DEFAULT: "#5eead4",
              foreground: "#000000",
            },
            focus: "#0ea5e9",
          },
        },
        dark: {
          colors: {
            primary: {
              DEFAULT: "#3b82f6",
              foreground: "#ffffff",
            },
            secondary: {
              DEFAULT: "#4ade80",
              foreground: "#000000",
            },
            focus: "#3b82f6",
          },
        },
      },
    }),
  ],
};
