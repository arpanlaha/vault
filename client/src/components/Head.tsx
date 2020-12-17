import React, { ReactElement } from "react";
import { Helmet } from "react-helmet";

interface HeadProps {
  currentCrateName?: string;
}

export default function Head(props: HeadProps): ReactElement {
  const { currentCrateName } = props;

  return (
    <Helmet htmlAttributes={{ lang: "en" }} defer={false}>
      <title>
        {currentCrateName !== undefined
          ? `${currentCrateName} | Vault`
          : "Vault"}
      </title>
    </Helmet>
  );
}
