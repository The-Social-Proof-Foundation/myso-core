// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

const defaultTheme = require("tailwindcss/defaultTheme");

module.exports = {
  corePlugins: {
    preflight: false, // disable Tailwind's reset
  },
  content: ["./src/**/*.{js,jsx,ts,tsx}", "./docs/**/*.mdx"], // my markdown stuff is in ../docs, not /src
  safelist: ["text-myso-success-dark"],
  darkMode: ["class", '[data-theme="dark"]'], // hooks into docusaurus' dark mode settings
  theme: {
    extend: {
      fontFamily: {
        sans: ["Inter", ...defaultTheme.fontFamily.sans],
        twkeverett: ["Twkeverett"],
      },
      colors: {
        "myso-black": "var(--myso-black)",
        "myso-blue-primary": "rgb(var(--myso-blue-primary)/<alpha-value>)",
        "myso-blue": "var(--myso-blue)",
        "myso-blue-bright": "rgb(var(--myso-blue-bright)/<alpha-value>)",
        "myso-blue-light": "rgb(var(--myso-blue-light)/<alpha-value>)",
        "myso-blue-lighter": "var(--myso-blue-lighter)",
        "myso-blue-dark": "rgb(var(--myso-blue-dark)/<alpha-value>)",
        "myso-blue-darker": "var(--myso-blue-darker)",
        "myso-hero": "var(--myso-hero)",
        "myso-hero-dark": "var(--myso-hero-dark)",
        "myso-steel": "var(--myso-steel)",
        "myso-steel-dark": "var(--myso-steel-dark)",
        "myso-steel-darker": "var(--myso-steel-darker)",
        "myso-header-nav": "var(--myso-header-nav)",
        "myso-success": "var(--myso-success)",
        "myso-success-dark": "var(--myso-success-dark)",
        "myso-success-light": "var(--myso-success-light)",
        "myso-issue": "var(--myso-issue)",
        "myso-issue-dark": "var(--myso-issue-dark)",
        "myso-issue-light": "var(--myso-issue-light)",
        "myso-warning": "var(--myso-warning)",
        "myso-warning-dark": "var(--myso-warning-dark)",
        "myso-warning-light": "var(--myso-warning-light)",
        "myso-code": "var(--myso-code)",
        "myso-gray-3s": "rgb(var(--myso-gray-3s)/<alpha-value>)",
        "myso-gray-5s": "rgb(var(--myso-gray-5s)/<alpha-value>)",
        "myso-gray": {
          35: "rgb(var(--myso-gray-35)/<alpha-value>)",
          40: "rgb(var(--myso-gray-40)/<alpha-value>)",
          45: "rgb(var(--myso-gray-45)/<alpha-value>)",
          50: "var(--myso-gray-50)",
          55: "rgb(var(--myso-gray-55)/<alpha-value>)",
          60: "var(--myso-gray-60)",
          65: "var(--myso-gray-65)",
          70: "var(--myso-gray-70)",
          75: "var(--myso-gray-75)",
          80: "var(--myso-gray-80)",
          85: "var(--myso-gray-85)",
          90: "var(--myso-gray-90)",
          95: "var(--myso-gray-95)",
          100: "var(--myso-gray-100)",
        },
        "myso-grey": {
          35: "rgb(var(--myso-gray-35)/<alpha-value>)",
          40: "rgb(var(--myso-gray-40)/<alpha-value>)",
          45: "rgb(var(--myso-gray-45)/<alpha-value>)",
          50: "var(--myso-gray-50)",
          55: "rgb(var(--myso-gray-55)/<alpha-value>)",
          60: "var(--myso-gray-60)",
          65: "var(--myso-gray-65)",
          70: "var(--myso-gray-70)",
          75: "var(--myso-gray-75)",
          80: "var(--myso-gray-80)",
          85: "var(--myso-gray-85)",
          90: "var(--myso-gray-90)",
          95: "var(--myso-gray-95)",
          100: "var(--myso-gray-100)",
        },
        "myso-disabled": "rgb(var(--myso-disabled)/<alpha-value>)",
        "myso-link-color-dark": "var(--myso-link-color-dark)",
        "myso-link-color-light": "var(--myso-link-color-light)",
        "myso-ghost-white": "var(--myso-ghost-white)",
        "myso-ghost-dark": "var(--myso-ghost-dark)",
        "ifm-background-color-dark": "var(--ifm-background-color-dark)",
        "myso-white": "rgb(var(--myso-white)/<alpha-value>)",
        "myso-card-dark": "rgb(var(--myso-card-dark)/<alpha-value>)",
        "myso-card-darker": "rgb(var(--myso-card-darker)/<alpha-value>)",
      },
      borderRadius: {
        myso: "40px",
      },
      boxShadow: {
        myso: "0px 0px 4px rgba(0, 0, 0, 0.02)",
        "myso-button": "0px 1px 2px rgba(16, 24, 40, 0.05)",
        "myso-notification": "0px 0px 20px rgba(29, 55, 87, 0.11)",
      },
      gradientColorStopPositions: {
        36: "36%",
      },
    },
  },
  plugins: [
    function ({ addUtilities }) {
      const arrowMask = `url("data:image/svg+xml;utf8,<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24'><path d='M8.12 4.12a1 1 0 0 1 1.41 0l6.35 6.35a1 1 0 0 1 0 1.41l-6.35 6.35a1 1 0 1 1-1.41-1.41L13.59 12 8.12 6.53a1 1 0 0 1 0-1.41z'/></svg>") no-repeat center / contain`;

      addUtilities({
        ".mask-arrow": {
          transition: "transform 0.2s ease",
          background: "currentColor",
          WebkitMask: arrowMask,
          mask: arrowMask,
        },
      });
    },
  ],
};
