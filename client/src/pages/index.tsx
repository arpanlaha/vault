import React, { ReactElement, useEffect, useState } from "react";
import { Head } from "../components";
import { notification } from "antd";
import { ForceGraph3D } from "react-force-graph";
import { getDependencyGraph } from "../utils/api";
import { DependencyGraph } from "../utils/types";

import "antd/dist/antd.dark.css";
import "../styles/vault.scss";

export default function Home(): ReactElement {
  const [
    dependencyGraph,
    setDependencyGraph,
  ] = useState<DependencyGraph | null>(null);
  const [error, setError] = useState("");

  useEffect(() => {
    const loadCrate = async (): Promise<void> => {
      const dependencyGraphRes = await getDependencyGraph("actix-web");
      if (dependencyGraphRes.success) {
        setDependencyGraph(dependencyGraphRes.result);
      } else {
        setError(dependencyGraphRes.error);
      }
    };

    loadCrate();
  }, []);

  useEffect(
    () =>
      error !== ""
        ? notification.error({
            message: "Error",
            description: error,
            key: "error",
            duration: 0,
          })
        : notification.close("error"),
    [error]
  );

  return (
    <>
      <Head />
      {dependencyGraph !== null && (
        <div className="dependency-graph">
          <ForceGraph3D
            graphData={{
              nodes: dependencyGraph.crates,
              links: dependencyGraph.dependencies,
            }}
            nodeId="name"
            linkSource="from"
            linkTarget="to"
          />
        </div>
      )}
    </>
  );
}
