/** @type {import('tailwindcss').Config} */
const config = {
    content: [
        "./src/**/*.{css,html,rs}", 
        "./dist/**/*.{css,html}"
    ],
    darkMode: 'class', // or 'media' or 'class'
    mode: 'jit',
    plugins: [],
    purge: [
        "src/**/*.rs"
    ],
    theme: {
        extend: {},
    },
    variants: {
        extend: {},
    },
};

module.exports = config;