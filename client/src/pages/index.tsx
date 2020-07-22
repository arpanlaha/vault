import React, { ReactElement, useEffect, useState } from "react";
import { Head, ForceGraphWrapper } from "../components";
import { notification } from "antd";
import { getDependencyGraph } from "../utils/api";
import { Crate, Dependency } from "../utils/types";
import "antd/dist/antd.dark.css";
import "../styles/vault.scss";

export default function Home(): ReactElement {
  const [crates, setCrates] = useState<Crate[]>([]);
  const [dependencies, setDependencies] = useState<Dependency[]>([]);
  const [error, setError] = useState("");

  useEffect(() => {
    if (typeof window !== undefined) {
      const loadCrate = async (): Promise<void> => {
        const dependencyGraphRes = await getDependencyGraph("actix-web");
        if (dependencyGraphRes.success) {
          setCrates(dependencyGraphRes.result.crates);
          setDependencies(dependencyGraphRes.result.dependencies);
        } else {
          setError(dependencyGraphRes.error);
        }
      };

      loadCrate();
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
      <div className="dependency-graph">
        <ForceGraphWrapper crates={crates} dependencies={dependencies} />
      </div>
    </>
  );
}
