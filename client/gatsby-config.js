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
    "gatsby-plugin-offline",
    "gatsby-plugin-react-helmet",
    "gatsby-plugin-preload-fonts",
    "gatsby-plugin-sass",
    "gatsby-plugin-sharp",
    "gatsby-plugin-split-css",
    {
      resolve: "gatsby-plugin-typescript",
      options: {
        allExtensions: true,
        isTSX: true,
      },
    },
    {
      resolve: "gatsby-source-filesystem",
      options: {
        name: "images",
        path: path.join(__dirname, "src", "images"),
      },
    },
    "gatsby-transformer-sharp",
  ],
};
