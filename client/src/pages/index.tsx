import React, { ReactElement, useCallback, useEffect, useState } from "react";
import {
  decodeDelimitedArray,
  encodeDelimitedArray,
  StringParam,
  useQueryParam,
} from "use-query-params";
import { ForceGraphWrapper, Head, Sidebar } from "../components";
import { notification, Layout } from "antd";
import {
  getCfgNames,
  getDependencyGraph,
  getRandomDependencyGraph,
  getTargets,
} from "../utils/api";
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

const DEFAULT_TARGET = "x86_64-unknown-linux-gnu";
const DEFAULT_CFG_NAME = "unix";

export default function Home(): ReactElement {
  const [portrait, setPortrait] = useState(false);
  const [targets, setTargets] = useState<string[]>([]);
  const [cfgNames, setCfgNames] = useState<string[]>([]);
  const [searchTerm, setSearchTerm] = useState("");
  const [currentCrate, setCurrentCrate] = useState<Crate | null>(null);
  const [selectedFeatures, setSelectedFeatures] = useState<string[]>([]);
  const [selectedTarget, setSelectedTarget] = useState(DEFAULT_TARGET);
  const [selectedCfgName, setSelectedCfgName] = useState(DEFAULT_CFG_NAME);
  const [targetSearchTerm, setTargetSearchTerm] = useState("");
  const [cfgNameSearchTerm, setCfgNameSearchTerm] = useState("");
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
  const [urlTarget, setUrlTarget] = useQueryParam("target", StringParam);
  const [urlCfgName, setUrlCfgName] = useQueryParam("cfg_name", StringParam);

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
        setUrlTarget(undefined);
        setUrlCfgName(undefined);
        setError("");
        setDependencyGraph(randomDependencyGraphRes.result);
      } else {
        setError(randomDependencyGraphRes.error);
      }
    };

    loadRandomCrate();
  }, [setUrlCrateName, setUrlFeatures, setUrlTarget, setUrlCfgName]);

  const loadDependencyGraph = async (
    crateId: string,
    features: string[] = [],
    target: string | undefined = undefined,
    cfgName: string | undefined = undefined
  ): Promise<void> => {
    setSearchTerm(crateId);

    const dependencyGraphRes = await getDependencyGraph(
      crateId,
      features,
      target,
      cfgName
    );
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
    getCfgNames().then((cfgNamesRes) => {
      if (cfgNamesRes.success) {
        setCfgNames(cfgNamesRes.result.cfg_names);
      } else {
        setError(cfgNamesRes.error);
      }
    });

    getTargets().then((targetsRes) => {
      if (targetsRes.success) {
        setTargets(targetsRes.result.targets);
      } else {
        setError(targetsRes.error);
      }
    });

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

      loadDependencyGraph(
        urlCrateName,
        features,
        urlTarget ?? undefined,
        urlCfgName ?? undefined
      );
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
      setUrlFeatures(
        selectedFeatures.length > 0 ? selectedFeatures : undefined
      ),
    [selectedFeatures, setUrlFeatures]
  );

  useEffect(
    () =>
      setUrlTarget(
        selectedTarget !== DEFAULT_TARGET ? selectedTarget : undefined
      ),
    [selectedTarget, setUrlTarget]
  );

  useEffect(
    () =>
      setUrlCfgName(
        selectedCfgName !== DEFAULT_CFG_NAME ? selectedCfgName : undefined
      ),
    [selectedCfgName, setUrlCfgName]
  );

  useEffect(() => {
    setCfgNameSearchTerm(selectedCfgName);
  }, [selectedCfgName]);

  useEffect(() => setTargetSearchTerm(selectedTarget), [selectedTarget]);

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
          targets={targets}
          cfgNames={cfgNames}
          setSelectedCfgName={setSelectedCfgName}
          setSelectedTarget={setSelectedTarget}
          selectedCfgName={selectedCfgName}
          selectedTarget={selectedTarget}
          targetSearchTerm={targetSearchTerm}
          setTargetSearchTerm={setTargetSearchTerm}
          cfgNameSearchTerm={cfgNameSearchTerm}
          setCfgNameSearchTerm={setCfgNameSearchTerm}
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
