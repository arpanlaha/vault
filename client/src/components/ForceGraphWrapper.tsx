import React, { ReactElement, useState, useEffect } from "react";
import { Crate, Dependency } from "../utils/types";
import loadable from "@loadable/component";

const ForceGraph = loadable(() => import("./ForceGraph"));
const DIMENSION_FACTOR = 0.1;

interface ForceGraphWrapperProps {
  crates: Crate[];
  dependencies: Dependency[];
}

export default function ForceGraphWrapper(
  props: ForceGraphWrapperProps
): ReactElement {
  const [height, setHeight] = useState(0);
  const [width, setWidth] = useState(0);

  const resize = (): void => {
    const containerHeight = window.innerHeight;
    const containerWidth =
      window.innerWidth - document.querySelector("aside")!.clientWidth;
    setHeight(containerHeight * (1 - DIMENSION_FACTOR));
    setWidth(containerWidth - containerHeight * DIMENSION_FACTOR);
  };

  useEffect(() => {
    resize();
    window.addEventListener("resize", resize);
  }, []);

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
      backgroundColor="#000000"
      enableNodeDrag={false}
      nodeAutoColorBy="name"
      linkAutoColorBy="from"
      linkLabel={(dependency: any) =>
        `${dependency.from} depends on ${dependency.to}`
      }
      linkWidth={1.5}
      linkDirectionalParticles={4}
      linkDirectionalParticleWidth={1}
      height={height}
      width={width}
    />
  );
}
