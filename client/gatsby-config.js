/* eslint-disable */
const path = require("path");

module.exports = {
  plugins: [
    {
      resolve: "gatsby-plugin-manifest",
      options: {
        name: "Vault",
        short_name: "Interactively visualize your crates.io dependencies.",
        description: "Interactively visualize your crates.io dependencies.",
        start_url: "/",
        background_color: "#ffffff",
        theme_color: "#ffffff",
        display: "browser",
        icon: "static/logo.svg",
      },
    },
    "gatsby-plugin-react-helmet",
    "gatsby-plugin-sass",
    {
      resolve: "gatsby-plugin-typescript",
      options: {
        allExtensions: true,
        isTSX: true,
      },
    },
  ],
};
