/** @type {import('tailwindcss').Config} */
const config = {
    content: [
        "./dist/**/*.{css,html,js}",
        "./src/**/*.{css,html,rs}", 
        "./static/**/*.{css,html}"
    ],
    darkMode: 'class', // or 'media' or 'class'
    mode: 'jit',
    plugins: [
        require('@tailwindcss/forms'),
        require('@tailwindcss/typography'),
        require('flowbite/plugin')
    ],
    theme: {
        extend: {},
    },
    variants: {
        extend: {},
    },
};

module.exports = config;