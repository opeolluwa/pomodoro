/** @type {import('tailwindcss').Config} */
import color from "tailwindcss/colors";

export default {
  content: ["./index.html", "./src/**/*.{vue,ts}"],
  theme: {
    extend: {
      color: {
        text: {},
        app: {
          DEFAULT: "#75CE8E",
          50: "#FCFEFD",
          100: "#EDF9F1",
          200: "#CFEED8",
          300: "#B1E3BF",
          400: "#93D9A7",
          500: "#75CE8E",
          600: "#4CBF6C",
          700: "#379C53",
          800: "#29723D",
          900: "#1A4927",
          950: "#12341C",
        },
        neutral: "#f5f5f5",
        shadow: {},
        ...color,
      },
    },
  },
  plugins: [],
};
