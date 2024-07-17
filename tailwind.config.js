/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "all",
  content: ["./src/**/*.{rs,html,css}", "./dist/**/*.html"],
  theme: {
    extend: {
      colors: {
        primary: "rgb(250, 250, 250)",
        secondary: "rgb(240, 240, 240)",
        tertiary: "rgb(232, 232, 232)",
        accent: {
          DEFAULT: "rgb(05, 05, 05)",
          hover: "rgb(25, 25, 25)",
        },
        disabled: "rgb(70, 70, 70)",
      },
      gridTemplateColumns: {
        cards: "repeat(auto-fill, minmax(384px, 1fr))",
      },
    },
  },
  plugins: [],
};
