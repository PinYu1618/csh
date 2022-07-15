module.exports = {
    mode: 'jit',
    purge: [
      "src/**/*.rs"
    ],
    darkMode: false, // or 'media' or 'class'
    theme: {
      extend: {
        colors: {},
        borderRadius: {},
        boxShadow: {},
        height: {},
        width: {},
      },
    },
    variants: {
      extend: {},
    },
    plugins: [],
}