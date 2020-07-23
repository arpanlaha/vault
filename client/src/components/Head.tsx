import React, { ReactElement } from "react";
import { Helmet } from "react-helmet";

export default function Head(): ReactElement {
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
      <title>Vault</title>
    </Helmet>
  );
}
