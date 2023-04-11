/** @type {import('tailwindcss').Config} */

module.exports = {
    content: [
        "./src/**/*.{js,jsx,ts,tsx}",
    ],

    theme: {
        extend: {},
    },

    plugins: [
        require("@tailwindcss/typography"),
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
                    "success": "#5fe3a3",
                    "warning": "#facc15",
                    "error": "#fc404d",
                },

                dark: {
                    "primary": "#3b82f6",
                    "secondary": "#4ade80",
                    "accent": "#e879f9",
                    "neutral": "#191d24",
                    "base-100": "#2a303c",
                    "info": "#06b6d4",
                    "success": "#32CD8A",
                    "warning": "#fba53c",
                    "error": "#f50a16",
                },
            },
        ],
    },
}
