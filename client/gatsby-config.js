/* eslint-disable */
const path = require("path");

module.exports = {
  plugins: [
    {
      resolve: "gatsby-plugin-manifest",
      options: {
        name: "App name",
        short_name: "App short name",
        description: "App description",
        start_url: "/",
        background_color: "#ffffff",
        theme_color: "#ffffff",
        display: "browser",
        icon: "static/favicon.ico",
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
