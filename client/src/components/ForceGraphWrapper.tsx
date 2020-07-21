import React, { ReactElement } from "react";
import { DependencyGraph } from "../utils/types";
import loadable from "@loadable/component";

const ForceGraph = loadable(() => import("./ForceGraph"));

interface ForceGraphWrapperProps {
  dependencyGraph: DependencyGraph;
}

export default function ForceGraphWrapper(
  props: ForceGraphWrapperProps
): ReactElement {
  const { dependencyGraph } = props;
  return (
    <ForceGraph
      graphData={{
        nodes: dependencyGraph.crates,
        links: dependencyGraph.dependencies,
      }}
      nodeId="name"
      linkSource="from"
      linkTarget="to"
      warmupTicks={100}
      backgroundColor="#000000"
      cooldownTicks={0}
      enableNodeDrag={false}
    />
  );
}
