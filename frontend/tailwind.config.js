/** @type {import('tailwindcss').Config} */
const withMT = require("@material-tailwind/react/utils/withMT");

module.exports = withMT({
    content: [
        "./src/**/*.{js,jsx,ts,tsx}",
    ],

    theme: {
        extend: {},
    },

    plugins: [
        require("daisyui"),
        require('@tailwindcss/line-clamp')
    ],

    daisyui: {
        styled: true,

        themes: [
            {
                light: {
                    "primary": "#0ea5e9",
                    "secondary": "#5eead4",
                    "accent": "#d8b4fe",
                    "neutral": "#374151",
                    "base-100": "#f3f4f6",
                    "info": "#a5f3fc",
                    "success": "#5FE3A3",
                    "warning": "#facc15",
                    "error": "#FC404D",
                },

                dark: {
                    "primary": "#3b82f6",
                    "secondary": "#4ade80",
                    "accent": "#e879f9",
                    "neutral": "#1F1E33",
                    "base-100": "#373338",
                    "info": "#06b6d4",
                    "success": "#32CD8A",
                    "warning": "#FBA53C",
                    "error": "#F50A16",
                },
            },
        ],
    },
})
