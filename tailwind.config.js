/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    files: ["*.html", "./src/**/*.rs"],
  },
  theme: {
    extend: {
      colors: {
        base: {
          100: 'oklch(25.33% 0.016 252.42)',
          200: 'oklch(23.26% 0.014 253.1)',
          300: 'oklch(21.15% 0.012 254.09)',
          content: 'oklch(97.807% 0.029 256.847)'
        },
        primary: {
          DEFAULT: 'oklch(58% 0.233 277.117)',
          content: 'oklch(96% 0.018 272.314)'
        },
        secondary: {
          DEFAULT: 'oklch(65% 0.241 354.308)',
          content: 'oklch(94% 0.028 342.258)'
        },
        accent: {
          DEFAULT: 'oklch(77% 0.152 181.912)',
          content: 'oklch(38% 0.063 188.416)'
        },
        neutral: {
          DEFAULT: 'oklch(14% 0.005 285.823)',
          content: 'oklch(92% 0.004 286.32)'
        },
        info: {
          DEFAULT: 'oklch(74% 0.16 232.661)',
          content: 'oklch(29% 0.066 243.157)'
        },
        success: {
          DEFAULT: 'oklch(76% 0.177 163.223)',
          content: 'oklch(37% 0.077 168.94)'
        },
        warning: {
          DEFAULT: 'oklch(82% 0.189 84.429)',
          content: 'oklch(41% 0.112 45.904)'
        },
        error: {
          DEFAULT: 'oklch(71% 0.194 13.428)',
          content: 'oklch(27% 0.105 12.094)'
        }
      },
      borderRadius: {
        'selector': '1rem',
        'field': '0.5rem',
        'box': '0.5rem'
      },
      spacing: {
        'selector': '0.25rem',
        'field': '0.25rem'
      },
      borderWidth: {
        DEFAULT: '1px'
      }
    }
  },
  plugins: [
    require('daisyui'),
    function({ addBase }) {
      addBase({
        ':root': {
          'color-scheme': 'dark',
          '--depth': '1',
          '--noise': '1'
        }
      });
    }
  ],
  daisyui: {
    themes: [
      {
        dark: {
          "color-scheme": "dark",
          "primary": "oklch(58% 0.233 277.117)",
          "primary-content": "oklch(96% 0.018 272.314)",
          "secondary": "oklch(65% 0.241 354.308)",
          "secondary-content": "oklch(94% 0.028 342.258)",
          "accent": "oklch(77% 0.152 181.912)",
          "accent-content": "oklch(38% 0.063 188.416)",
          "neutral": "oklch(14% 0.005 285.823)",
          "neutral-content": "oklch(92% 0.004 286.32)",
          "base-100": "oklch(25.33% 0.016 252.42)",
          "base-200": "oklch(23.26% 0.014 253.1)",
          "base-300": "oklch(21.15% 0.012 254.09)",
          "base-content": "oklch(97.807% 0.029 256.847)",
          "info": "oklch(74% 0.16 232.661)",
          "info-content": "oklch(29% 0.066 243.157)",
          "success": "oklch(76% 0.177 163.223)",
          "success-content": "oklch(37% 0.077 168.94)",
          "warning": "oklch(82% 0.189 84.429)",
          "warning-content": "oklch(41% 0.112 45.904)",
          "error": "oklch(71% 0.194 13.428)",
          "error-content": "oklch(27% 0.105 12.094)",
          "--rounded-box": "0.5rem",
          "--rounded-btn": "0.5rem",
          "--rounded-badge": "1rem",
          "--animation-btn": "0.25s",
          "--animation-input": "0.2s",
          "--btn-focus-scale": "0.95",
          "--border-btn": "1px",
          "--tab-border": "1px",
          "--tab-radius": "0.5rem",
        },
      },
    ],
  },
  darkMode: 'class'
}