import React, { ReactElement, useEffect, useState } from "react";
import {
  decodeDelimitedArray,
  encodeDelimitedArray,
  StringParam,
  useQueryParam,
} from "use-query-params";
import { ForceGraphWrapper, Head, Sidebar } from "../components";
import { notification, Layout } from "antd";
import { getCrate, getDependencyGraph, getRandomCrate } from "../utils/api";
import { CrateInfo, DependencyGraph } from "../utils/types";

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
  const [currentCrate, setCurrentCrate] = useState<CrateInfo | null>(null);
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

  const setRandomCrate = (): void => {
    const loadRandomCrate = async (): Promise<void> => {
      const randomCrateRes = await getRandomCrate();
      if (randomCrateRes.success) {
        setCurrentCrate({
          crate: randomCrateRes.result,
          selectedFeatures: [],
        });
        setSearchTerm(randomCrateRes.result.name);
        setUrlCrateName(randomCrateRes.result.name);
        setError("");
      } else {
        setError(randomCrateRes.error);
      }
    };

    loadRandomCrate();
  };

  useEffect(() => {
    if (
      urlCrateName === null ||
      urlCrateName === undefined ||
      urlCrateName.length === 0
    ) {
      setRandomCrate();
    } else {
      const loadUrlCrate = async (): Promise<void> => {
        const features = (urlFeatures !== null &&
        urlFeatures !== undefined &&
        urlFeatures.length > 0 &&
        urlFeatures.every((urlFeature) => urlFeature !== null)
          ? urlFeatures
          : []) as string[];

        const crateRes = await getCrate(urlCrateName);

        if (crateRes.success) {
          setCurrentCrate({
            crate: crateRes.result,
            selectedFeatures: features,
          });
        } else {
          setError(crateRes.error);
        }
      };

      loadUrlCrate();
    }

    const checkLayout = (): void =>
      setPortrait(window.innerHeight > window.innerWidth);

    checkLayout();

    window.addEventListener("resize", checkLayout);
  }, []); // eslint-disable-line react-hooks/exhaustive-deps

  useEffect(() => {
    if (currentCrate !== null) {
      setFeatureNames(
        Object.keys(currentCrate.crate.features).filter(
          (featureName) => featureName !== "default"
        )
      );
      const loadCrateDependencies = async (): Promise<void> => {
        const dependencyGraphRes = await getDependencyGraph(
          currentCrate.crate.name,
          currentCrate.selectedFeatures
        );
        if (dependencyGraphRes.success) {
          setDependencyGraph(dependencyGraphRes.result);
          setError("");
        } else {
          setError(dependencyGraphRes.error);
        }
      };

      loadCrateDependencies();
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
      <Head currentCrateName={currentCrate?.crate.name} />
      <Layout>
        <Sidebar
          clickedCrateName={clickedCrateName}
          currentCrate={currentCrate}
          featureNames={featureNames}
          graphLinks={dependencyGraph?.dependencies ?? []}
          graphNodes={dependencyGraph?.crates ?? []}
          portrait={portrait}
          searchTerm={searchTerm}
          setClickedCrateName={setClickedCrateName}
          setCurrentCrate={setCurrentCrate}
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
