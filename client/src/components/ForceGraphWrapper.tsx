import React, {
  Dispatch,
  ReactElement,
  SetStateAction,
  useState,
  useEffect,
} from "react";
import ForceGraph3D from "react-force-graph-3d";
import { CrateDistance, Dependency, DependencyGraph } from "../utils/types";

const DIMENSION_FACTOR = 0.1;

interface ForceGraphWrapperProps {
  clickedCrateName: string | null;
  dependencyGraph: DependencyGraph | null;
  portrait: boolean;
  setClickedCrateName: Dispatch<SetStateAction<string | null>>;
}

export default function ForceGraphWrapper(
  props: ForceGraphWrapperProps
): ReactElement {
  const {
    clickedCrateName,
    dependencyGraph,
    portrait,
    setClickedCrateName,
  } = props;

  const crates = dependencyGraph?.crates ?? [];
  const dependencies = dependencyGraph?.dependencies ?? [];

  const [height, setHeight] = useState(0);
  const [width, setWidth] = useState(0);

  const RED = "hsl(0, 100%, 50%)";
  const GREEN = "hsl(120, 100%, 50%)";
  const BLUE = "hsl(216, 100%, 50%)";
  const GRAY = "hsl(0, 0%, 50%)";

  useEffect(() => {
    const resize = (): void => {
      if (portrait) {
        setHeight(window.innerHeight);
        setWidth(window.innerWidth);
      } else {
        const containerHeight = window.innerHeight;
        const containerWidth =
          window.innerWidth - document.querySelector("aside")!.clientWidth;
        setHeight(containerHeight * (1 - DIMENSION_FACTOR));
        setWidth(containerWidth - containerHeight * DIMENSION_FACTOR);
      }
    };

    resize();
    window.addEventListener("resize", resize);
  }, [portrait]);

  useEffect(() => setClickedCrateName(null), [
    dependencyGraph,
    setClickedCrateName,
  ]);

  const handleLinkLabel = (dependency: Dependency): string =>
    `${dependency.from} depends on ${dependency.to}`;

  const handleNodeClick = (crate: CrateDistance): void =>
    setClickedCrateName(crate.name !== clickedCrateName ? crate.name : null);

  const handleBackgroundClick = (): void => setClickedCrateName(null);

  const handleNodeColor = (crate: CrateDistance): string => {
    const { name } = crate;
    if (name === clickedCrateName) {
      return GREEN;
    } else if (
      dependencies.some(
        (dependency) =>
          dependency.from === name && dependency.to === clickedCrateName
      )
    ) {
      return RED;
    } else if (
      dependencies.some(
        (dependency) =>
          dependency.from === clickedCrateName && dependency.to === name
      )
    ) {
      return BLUE;
    }
    return GRAY;
  };

  const handleLinkColor = (dependency: Dependency): string => {
    if (dependency.to === clickedCrateName) {
      return RED;
    } else if (dependency.from === clickedCrateName) {
      return BLUE;
    }
    return GRAY;
  };

  return (
    <ForceGraph3D
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
      linkLabel={handleLinkLabel as any}
      linkWidth={1.5}
      linkDirectionalParticles={4}
      linkDirectionalParticleWidth={1}
      height={height}
      width={width}
      onNodeClick={handleNodeClick as any}
      onBackgroundClick={handleBackgroundClick}
      nodeColor={clickedCrateName === null ? "color" : (handleNodeColor as any)}
      linkColor={clickedCrateName === null ? "color" : (handleLinkColor as any)}
    />
  );
}
