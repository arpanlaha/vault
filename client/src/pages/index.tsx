import React, { ReactElement, useCallback, useEffect, useState } from "react";
import {
  decodeDelimitedArray,
  encodeDelimitedArray,
  StringParam,
  useQueryParam,
} from "use-query-params";
import { ForceGraphWrapper, Head, Sidebar } from "../components";
import { notification, Layout } from "antd";
import { getDependencyGraph, getRandomDependencyGraph } from "../utils/api";
import { Crate, DependencyGraph } from "../utils/types";

import "../styles/antd.scss";
import "../styles/vault.scss";

const { Content } = Layout;

const CommaArrayParam = {
  encode: (array: string[] | null | undefined) =>
    encodeDelimitedArray(array, ","),

  decode: (arrayStr: string | (string | null)[] | null | undefined) =>
    decodeDelimitedArray(arrayStr, ","),
};

export default function Home(): ReactElement {
  const [portrait, setPortrait] = useState(false);
  const [searchTerm, setSearchTerm] = useState("");
  const [currentCrate, setCurrentCrate] = useState<Crate | null>(null);
  const [selectedFeatures, setSelectedFeatures] = useState<string[]>([]);
  const [featureNames, setFeatureNames] = useState<string[]>([]);
  const [
    dependencyGraph,
    setDependencyGraph,
  ] = useState<DependencyGraph | null>(null);
  const [clickedCrateName, setClickedCrateName] = useState<string | null>(null);
  const [error, setError] = useState("");

  const [urlCrateName, setUrlCrateName] = useQueryParam("crate", StringParam);
  const [urlFeatures, setUrlFeatures] = useQueryParam(
    "features",
    CommaArrayParam
  );

  const setRandomCrate = useCallback((): void => {
    const loadRandomCrate = async (): Promise<void> => {
      const randomDependencyGraphRes = await getRandomDependencyGraph();
      if (randomDependencyGraphRes.success) {
        const crate = randomDependencyGraphRes.result.crates[0];
        setCurrentCrate(crate);
        setSelectedFeatures([]);
        setSearchTerm(crate.name);
        setUrlCrateName(crate.name);
        setUrlFeatures(undefined);
        setError("");
        setDependencyGraph(randomDependencyGraphRes.result);
      } else {
        setError(randomDependencyGraphRes.error);
      }
    };

    loadRandomCrate();
  }, [setUrlCrateName, setUrlFeatures]);

  const loadDependencyGraph = async (
    crateId: string,
    features: string[]
  ): Promise<void> => {
    setSearchTerm(crateId);

    const dependencyGraphRes = await getDependencyGraph(crateId, features);
    if (dependencyGraphRes.success) {
      const crate = dependencyGraphRes.result.crates[0];
      setCurrentCrate(crate);

      setError("");
      setDependencyGraph(dependencyGraphRes.result);
    } else {
      setError(dependencyGraphRes.error);
    }
  };

  useEffect(() => {
    if (
      urlCrateName === null ||
      urlCrateName === undefined ||
      urlCrateName.length === 0
    ) {
      setRandomCrate();
    } else {
      const features = (urlFeatures !== null &&
      urlFeatures !== undefined &&
      urlFeatures.length > 0 &&
      urlFeatures.every((urlFeature) => urlFeature !== null)
        ? urlFeatures
        : []) as string[];

      loadDependencyGraph(urlCrateName, features);
      setSelectedFeatures(features);
    }

    const checkLayout = (): void =>
      setPortrait(window.innerHeight > window.innerWidth);

    checkLayout();

    window.addEventListener("resize", checkLayout);
  }, []); // eslint-disable-line react-hooks/exhaustive-deps

  useEffect(() => {
    if (currentCrate !== null) {
      setFeatureNames(
        Object.keys(currentCrate.features).filter(
          (featureName) => featureName !== "default"
        )
      );
    } else {
      setDependencyGraph(null);
    }
  }, [currentCrate]);

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
      <Head currentCrateName={currentCrate?.name} />
      <Layout>
        <Sidebar
          clickedCrateName={clickedCrateName}
          currentCrate={currentCrate}
          selectedFeatures={selectedFeatures}
          featureNames={featureNames}
          graphLinks={dependencyGraph?.dependencies ?? []}
          graphNodes={dependencyGraph?.crates ?? []}
          portrait={portrait}
          searchTerm={searchTerm}
          setClickedCrateName={setClickedCrateName}
          setSelectedFeatures={setSelectedFeatures}
          loadDependencyGraph={loadDependencyGraph}
          setError={setError}
          setRandomCrate={setRandomCrate}
          setSearchTerm={setSearchTerm}
          setUrlCrateName={setUrlCrateName}
          setUrlFeatures={setUrlFeatures}
        />
        <Content className="content">
          <div className="column dependency-graph-container">
            <ForceGraphWrapper
              clickedCrateName={clickedCrateName}
              dependencyGraph={dependencyGraph}
              setClickedCrateName={setClickedCrateName}
              portrait={portrait}
            />
          </div>
        </Content>
      </Layout>
    </>
  );
}
