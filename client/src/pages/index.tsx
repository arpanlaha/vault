import React, {
  ReactElement,
  useEffect,
  useState,
  lazy,
  FunctionComponent,
} from "react";
import { Head } from "../components";
import { notification } from "antd";
import { getDependencyGraph } from "../utils/api";
import { DependencyGraph } from "../utils/types";
import { ForceGraphProps } from "../components/ForceGraph";
import "antd/dist/antd.dark.css";
import "../styles/vault.scss";

export default function Home(): ReactElement {
  const [
    dependencyGraph,
    setDependencyGraph,
  ] = useState<DependencyGraph | null>(null);
  const [error, setError] = useState("");
  const [ForceGraph, setForceGraph] = useState<FunctionComponent<
    ForceGraphProps
  > | null>(null);

  // const ForceGraph =

  useEffect(() => {
    if (typeof window !== undefined) {
      const loadCrate = async (): Promise<void> => {
        const dependencyGraphRes = await getDependencyGraph("actix-web");
        if (dependencyGraphRes.success) {
          setDependencyGraph(dependencyGraphRes.result);
        } else {
          setError(dependencyGraphRes.error);
        }
      };

      loadCrate();
      setForceGraph(lazy(() => import("../components/ForceGraph")));
    }
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
      {dependencyGraph !== null && ForceGraph !== null && (
        <div className="dependency-graph">
          <ForceGraph dependencyGraph={dependencyGraph} />
        </div>
      )}
    </>
  );
}
