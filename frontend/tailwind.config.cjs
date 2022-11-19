const plugin = require('tailwindcss/plugin');

/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ['./src/**/*.{html,js,svelte,ts}'],
  theme: {
    extend: {
      colors: {
        mask: 'rgba(0, 0, 0, 0.5)'
      }
    }
  },
  plugins: [
    plugin(function ({ addComponents, theme }) {
      addComponents({
        '.hover-underline': {
          display: 'inline-block',
          position: 'relative',
          '&::after': {
            display: 'block',
            content: "''",
            width: '100%',
            height: '1px',
            bottom: '0',
            left: '0',
            'background-color': 'rgb(203 213 225)',
            transition: 'transform 0.15s ease-out',
            transform: 'scaleX(0)',
            transformOrigin: 'bottom'
          },
          '&:hover::after': {
            transform: 'scaleX(1)'
          }
        }
      });
    })
  ]
};
