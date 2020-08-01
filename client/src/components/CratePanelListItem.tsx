import React, { ReactElement } from "react";
import { Tag } from "antd";

interface CratePanelItemProps {
  label: string;
  values: string[];
}

export default function CratePanelItem(
  props: CratePanelItemProps
): ReactElement {
  const { label, values } = props;

  return (
    <p className="crate-panel-list">
      <b>{label}: </b>
      <span>
        {values.length > 0
          ? values.map((value) => <Tag key={value}>{value}</Tag>)
          : "None"}
      </span>
    </p>
  );
}
