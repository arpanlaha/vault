import React, { ReactElement } from "react";
import { Tag } from "antd";

interface CratePanelItemProps {
  dependencies?: boolean;
  label: string;
  onClick?: (value: string) => void;
  values: string[];
}

export default function CratePanelItem(
  props: CratePanelItemProps
): ReactElement {
  const { dependencies, label, onClick, values } = props;

  return (
    <p className="crate-panel-list">
      <b>{label}: </b>
      <span>
        {values.length > 0
          ? values.map((value) => (
              <Tag
                className={dependencies ? "dependency-tag" : undefined}
                key={value}
                onClick={
                  onClick !== undefined ? () => onClick(value) : undefined
                }
              >
                {value}
              </Tag>
            ))
          : "None"}
      </span>
    </p>
  );
}
