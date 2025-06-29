/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        'cyber-black': '#0a0a0a',
        'cyber-dark': '#1a1a1a',
        'cyber-gray': '#333333',
        'cyber-green': '#00ff00',
        'cyber-blue': '#00ffff',
        'cyber-red': '#ff0000',
        'cyber-yellow': '#ffff00',
      },
      fontFamily: {
        'mono': ['Courier New', 'monospace'],
        'cyber': ['Courier New', 'monospace'],
      },
      animation: {
        'glow': 'glow 2s ease-in-out infinite alternate',
        'pulse-slow': 'pulse 3s cubic-bezier(0.4, 0, 0.6, 1) infinite',
      },
      keyframes: {
        glow: {
          '0%': { boxShadow: '0 0 5px #00ff00, 0 0 10px #00ff00, 0 0 15px #00ff00' },
          '100%': { boxShadow: '0 0 10px #00ff00, 0 0 20px #00ff00, 0 0 30px #00ff00' },
        }
      }
    },
  },
  plugins: [],
} 