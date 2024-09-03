/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./static/**/*.{html, js}", "./templates/**/*.{html, js}"],
  theme: {
    extend: {
      backdropFilter: {
        "blur-30": "blur(30px)",
      },
      backgroundImage: {
        "mask-gradient":
          "linear-gradient(rgb(0, 0, 0) 50%, rgba(0, 0, 0, 0.8) 70%, rgba(0, 0, 0, 0) 100%)",
      },
      maskImage: {
        "mask-gradient":
          "linear-gradient(rgb(0, 0, 0) 50%, rgba(0, 0, 0, 0.8) 70%, rgba(0, 0, 0, 0) 100%)",
      },
      inset: {
        "-1": "-1rem",
      },
    },
  },
  plugins: [require("tailwind-filters")],
};
