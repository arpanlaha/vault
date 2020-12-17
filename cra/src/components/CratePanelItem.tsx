import React, { ReactElement, ReactNode } from "react";

interface CratePanelItemProps {
  label: string;
  value: ReactNode;
}

export default function CratePanelItem(
  props: CratePanelItemProps
): ReactElement {
  const { label, value } = props;

  return (
    <p>
      <span>
        <b>{label}: </b>
        <span>{value}</span>
      </span>
    </p>
  );
}
