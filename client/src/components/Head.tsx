import React, { ReactElement } from "react";
import { Helmet } from "react-helmet";

interface HeadProps {
  currentCrateName?: string;
}

export default function Head(props: HeadProps): ReactElement {
  const { currentCrateName } = props;

  return (
    <Helmet htmlAttributes={{ lang: "en" }} defer={false}>
      <meta charSet="UTF-8" />
      <meta
        name="description"
        content="Interactively visualize your crates.io dependencies."
      />
      <meta
        name="keywords"
        content="Rust, crates, crates.io, package, dependency, registry, graph, visualization."
      />
      <meta name="author" content="ARpan Laha" />
      <meta name="theme-color" content="#0d0f12" />
      <meta name="apple-mobile-web-app-status-bar-style" content="black" />
      <title>
        {currentCrateName !== undefined
          ? `${currentCrateName} | Vault`
          : "Vault"}
      </title>
    </Helmet>
  );
}
