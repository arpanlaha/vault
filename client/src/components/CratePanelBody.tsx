import React, { ReactElement } from "react";
import CratePanelItem from "./CratePanelItem";
import CratePanelListItem from "./CratePanelListItem";
import { Crate } from "../utils/types";

interface CratePanelBodyProps {
  crate: Crate;
  dependencies: string[];
  setClickedCrateName: (clickedCrateName: string) => void;
}

export default function CratePanelBody(
  props: CratePanelBodyProps
): ReactElement {
  const { crate, dependencies, setClickedCrateName } = props;
  const { name, description, version, downloads, categories, keywords } = crate;

  return (
    <>
      <CratePanelItem label="Description" value={description} />
      <CratePanelItem label="Version" value={version} />
      <CratePanelItem label="Downloads" value={downloads} />
      <CratePanelListItem label="Categories" values={categories} />
      <CratePanelListItem label="Keywords" values={keywords} />
      <CratePanelListItem
        dependencies
        label="Dependencies"
        onClick={setClickedCrateName}
        values={dependencies}
      />
      <a
        href={`https://crates.io/crates/${name}`}
        target="_blank"
        rel="noopener noreferrer"
      >
        View on crates.io
      </a>
    </>
  );
}
