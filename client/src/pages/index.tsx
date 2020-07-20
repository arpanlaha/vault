import React, { ReactElement, useEffect, useState } from "react";
import { Head } from "../components";
import { getDependencyGraph } from "../utils/api";
import { ForceGraph3D, ForceGraph2D } from "react-force-graph";
import { DependencyGraph } from "../utils/types";

export default function Home(): ReactElement {
  const [
    dependencyGraph,
    setDependencyGraph,
  ] = useState<DependencyGraph | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const loadCrate = async (): Promise<void> => {
      const dependencyGraphRes = await getDependencyGraph("actix-web");
      if (dependencyGraphRes.success) {
        setDependencyGraph(dependencyGraphRes.result);
        console.log(dependencyGraphRes.result);
      } else {
        setError(dependencyGraphRes.error);
      }
    };

    loadCrate();
  }, []);

  useEffect(() => {
    if (error !== null) {
      console.log(error);
    }
  }, [error]);

  return (
    <>
      <Head />
      {dependencyGraph !== null && (
        <ForceGraph3D
          graphData={{
            nodes: dependencyGraph.crates,
            links: dependencyGraph.dependencies,
          }}
          nodeId="name"
          linkSource="from"
          linkTarget="to"
        />
      )}
    </>
  );
}
