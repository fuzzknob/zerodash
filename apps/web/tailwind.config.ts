import type { Config } from "tailwindcss";
import defaultTheme from "tailwindcss/defaultTheme";

const config: Config = {
  content: [
    "./src/pages/**/*.{js,ts,jsx,tsx,mdx}",
    "./src/components/**/*.{js,ts,jsx,tsx,mdx}",
    "./src/app/**/*.{js,ts,jsx,tsx,mdx}",
  ],
  theme: {
    extend: {
      fontFamily: {
        sans: ["var(--font-inter)", ...defaultTheme.fontFamily.sans],
      },
      colors: {
        background: {
          primary: "var(--color-background-primary)",
          secondary: "var(--color-background-secondary)",
          inverse: "var(--color-background-inverse)",
        },
        foreground: {
          primary: "var(--color-foreground-primary)",
          secondary: "var(--color-foreground-secondary)",
          inverse: "var(--color-foreground-inverse)",
        },
        primary: "var(--color-primary)",
        secondary: "var(--color-secondary)",
      },
    },
  },
  plugins: [],
};

export default config;
