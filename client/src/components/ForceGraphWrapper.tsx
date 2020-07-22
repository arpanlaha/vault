import React, { ReactElement } from "react";
import { Crate, Dependency } from "../utils/types";
import loadable from "@loadable/component";

const ForceGraph = loadable(() => import("./ForceGraph"));

interface ForceGraphWrapperProps {
  crates: Crate[];
  dependencies: Dependency[];
}

export default function ForceGraphWrapper(
  props: ForceGraphWrapperProps
): ReactElement {
  const { crates, dependencies } = props;
  return (
    <ForceGraph
      graphData={{
        nodes: crates,
        links: dependencies,
      }}
      nodeId="name"
      linkSource="from"
      linkTarget="to"
      warmupTicks={100}
      backgroundColor="#000000"
      cooldownTicks={0}
      enableNodeDrag={false}
      nodeAutoColorBy="name"
      linkAutoColorBy="to"
      linkLabel={(dependency: any) =>
        `${dependency.from} depends on ${dependency.to}`
      }
      linkWidth={1.5}
      linkDirectionalParticles={4}
      linkDirectionalParticleWidth={1}
    />
  );
}
