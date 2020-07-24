import React, { ReactElement, useEffect, useState } from "react";
import { Head, ForceGraphWrapper } from "../components";
import {
  AutoComplete,
  Checkbox,
  Collapse,
  Input,
  notification,
  Layout,
} from "antd";
import { getDependencyGraph, getRandomCrate, searchCrate } from "../utils/api";
import { Crate, Dependency } from "../utils/types";

import "../styles/antd.scss";
import "../styles/vault.scss";
import { CheckboxChangeEvent } from "antd/lib/checkbox";

const { Search } = Input;
const { Content, Sider } = Layout;
const CheckboxGroup = Checkbox.Group;
const { Panel } = Collapse;

export default function Home(): ReactElement {
  const [currentCrate, setCurrentCrate] = useState<Crate | null>(null);
  const [searchTerm, setSearchTerm] = useState("");
  const [searchCrates, setSearchCrates] = useState<Crate[]>([]);
  const [crates, setCrates] = useState<Crate[]>([]);
  const [dependencies, setDependencies] = useState<Dependency[]>([]);
  const [indeterminate, setIndeterminate] = useState(true);
  const [selectedFeatures, setSelectedFeatures] = useState<string[]>([]);
  const [error, setError] = useState("");

  useEffect(() => {
    const loadRandomCrate = async (): Promise<void> => {
      const randomCrateRes = await getRandomCrate();
      if (randomCrateRes.success) {
        setCurrentCrate(randomCrateRes.result);
      } else {
        setError(randomCrateRes.error);
      }
    };

    loadRandomCrate();
  }, []);

  useEffect(() => {
    if (currentCrate !== null) {
      const loadCrateDependencies = async (): Promise<void> => {
        const dependencyGraphRes = await getDependencyGraph(
          currentCrate.name,
          selectedFeatures
        );
        if (dependencyGraphRes.success) {
          setCrates(dependencyGraphRes.result.crates);
          setDependencies(dependencyGraphRes.result.dependencies);
        } else {
          setError(dependencyGraphRes.error);
        }
      };

      loadCrateDependencies();
    }
  }, [currentCrate, selectedFeatures]);

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

  useEffect(() => {
    if (searchTerm.length > 0) {
      const loadSearch = async (): Promise<void> => {
        const searchCrateRes = await searchCrate(searchTerm);
        if (searchCrateRes.success) {
          setSearchCrates(searchCrateRes.result);
        } else {
          setError(searchCrateRes.error);
        }
      };

      loadSearch();
    } else {
      setSearchCrates([]);
    }
  }, [searchTerm]);

  useEffect(() => {
    if (currentCrate !== null) {
      setIndeterminate(
        selectedFeatures.length > 0 &&
          selectedFeatures.length < Object.keys(currentCrate.features).length
      );
    }
  }, [currentCrate, selectedFeatures]);

  const handleSearchSelect = (selectedCrateName: string): void => {
    setCurrentCrate(
      searchCrates.find(
        (searchCrate) => searchCrate.name === selectedCrateName
      )!
    );
  };

  const handleAllFeatureToggle = (e: CheckboxChangeEvent): void => {
    if (currentCrate !== null) {
      setSelectedFeatures(
        e.target.checked ? Object.keys(currentCrate.features) : []
      );
      setIndeterminate(false);
    }
  };

  return (
    <>
      <Head />
      <Layout>
        <Sider width="25%" theme="light">
          <div className="sider">
            <h1>Vault</h1>
            <h2>Current crate: {currentCrate?.name}</h2>
            <AutoComplete
              options={
                searchCrates.map((searchCrate) => ({
                  value: searchCrate.name,
                })) as any
              }
              onSelect={handleSearchSelect}
              onSearch={setSearchTerm}
            >
              <Search
                placeholder="Search for a crate..."
                onSearch={handleSearchSelect}
                disabled={searchTerm.length === 0}
              />
            </AutoComplete>
            {currentCrate !== null &&
              Object.keys(currentCrate.features).length > 0 && (
                <Collapse>
                  <Panel header="Features" key="">
                    <Checkbox
                      indeterminate={indeterminate}
                      onChange={handleAllFeatureToggle}
                      checked={
                        selectedFeatures.length ===
                        Object.keys(currentCrate.features).length
                      }
                    >
                      Toggle all features
                    </Checkbox>
                    <CheckboxGroup
                      options={Object.keys(currentCrate.features)}
                      value={selectedFeatures}
                      onChange={setSelectedFeatures as any}
                    />
                  </Panel>
                </Collapse>
              )}
          </div>
        </Sider>
        <Content className="content">
          <div className="dependency-graph">
            <ForceGraphWrapper crates={crates} dependencies={dependencies} />
          </div>
        </Content>
      </Layout>
    </>
  );
}
