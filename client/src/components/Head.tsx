import React, { Component, Props, ReactElement } from "react";
import { Helmet } from "react-helmet";

interface HeadProps extends Props<Component> {
  title?: string;
  description?: string;
  keywords?: string;
}

export default function Head(props: HeadProps): ReactElement {
  const { description, keywords, title } = props;
  return (
    <Helmet htmlAttributes={{ lang: "en" }} defer={false}>
      <meta charSet="UTF-8" />
      <meta name="description" content={description ?? "App description"} />
      <meta name="keywords" content={keywords ?? "App keywords"} />
      <meta name="author" content="App author" />
      <title>{title ?? "App title"}</title>
    </Helmet>
  );
}
