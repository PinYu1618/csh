module.exports = {
    mode: 'jit',
    purge: [
      "src/**/*.rs"
    ],
    darkMode: false,
    theme: {
      container: {
        center: true,
        padding: "1em",
      },
      extend: {
        colors: {
          csh: "#ecf0f3",
          github: "#222222",
        },
        borderRadius: {
          card: "10px",
        },
        boxShadow: {
          "csh-card": "12px 12px 16px 0 rgba(0, 0, 0, 0.15), -8px -8px 12px 0 rgba(255, 255, 255, 0.75)",
          "csh-button": "6px 6px 4px 0 rgba(0, 0, 0, 0.15), -4px -4px 6px 0 rgba(255, 255, 255, 0.75)",
        },
        height: {
          card: "300px",
          button: "40px",
        }, 
        width: {
          card: "250px",
          button: "100px",
        },
      },
    },
    variants: {
      extend: {},
    },
    plugins: [],
}