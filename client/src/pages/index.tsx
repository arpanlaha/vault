import React, { ReactElement, useEffect, useState } from "react";
import { Head, ForceGraphWrapper } from "../components";
import {
  AutoComplete,
  Button,
  Checkbox,
  Collapse,
  Input,
  notification,
  Layout,
  List,
} from "antd";
import { getDependencyGraph, getRandomCrate, searchCrate } from "../utils/api";
import { Crate, Dependency } from "../utils/types";
import { CheckboxChangeEvent } from "antd/lib/checkbox";

import { RedoOutlined } from "@ant-design/icons";

import "../styles/antd.scss";
import "../styles/vault.scss";

const { Panel } = Collapse;
const { Search } = Input;
const { Content, Sider } = Layout;
const CheckboxGroup = Checkbox.Group;
const ListItem = List.Item;
const ListItemMeta = List.Item.Meta;

export default function Home(): ReactElement {
  const [currentCrate, setCurrentCrate] = useState<Crate | null>(null);
  const [featureNames, setFeatureNames] = useState<string[]>([]);
  const [searchTerm, setSearchTerm] = useState("");
  const [searchCrates, setSearchCrates] = useState<Crate[]>([]);
  const [graphNodes, setGraphNodes] = useState<Crate[]>([]);
  const [graphLinks, setGraphLinks] = useState<Dependency[]>([]);
  const [indeterminate, setIndeterminate] = useState(true);
  const [selectedFeatureNames, setSelectedFeatureNames] = useState<string[]>(
    []
  );
  const [error, setError] = useState("");

  const setRandomCrate = (): void => {
    const loadRandomCrate = async (): Promise<void> => {
      const randomCrateRes = await getRandomCrate();
      if (randomCrateRes.success) {
        setCurrentCrate(randomCrateRes.result);
      } else {
        setError(randomCrateRes.error);
      }
    };

    loadRandomCrate();
  };

  useEffect(setRandomCrate, []);

  useEffect(() => {
    if (currentCrate !== null) {
      setSelectedFeatureNames([]);
      setFeatureNames(Object.keys(currentCrate.features));
    }
  }, [currentCrate]);

  useEffect(() => {
    if (currentCrate !== null) {
      const loadCrateDependencies = async (): Promise<void> => {
        const dependencyGraphRes = await getDependencyGraph(
          currentCrate.name,
          selectedFeatureNames
        );
        if (dependencyGraphRes.success) {
          setGraphNodes(dependencyGraphRes.result.crates);
          setGraphLinks(dependencyGraphRes.result.dependencies);
        } else {
          setError(dependencyGraphRes.error);
        }
      };

      loadCrateDependencies();
    }
  }, [currentCrate, selectedFeatureNames]);

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
    setIndeterminate(
      selectedFeatureNames.length > 0 &&
        selectedFeatureNames.length < featureNames.length
    );
  }, [featureNames, selectedFeatureNames]);

  const handleSearchSelect = (selectedCrateName: string): void => {
    setCurrentCrate(
      searchCrates.find(
        (searchCrate) => searchCrate.name === selectedCrateName
      )!
    );
  };

  const handleAllFeatureToggle = (e: CheckboxChangeEvent): void => {
    if (currentCrate !== null) {
      setSelectedFeatureNames(e.target.checked ? featureNames : []);
      setIndeterminate(false);
    }
  };

  return (
    <>
      <Head />
      <Layout>
        <Sider width="30%" theme="light">
          <div className="sider">
            <h1>Vault</h1>
            <h2>Current crate: {currentCrate?.name}</h2>
            <div className="crate-picker">
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
              <Button onClick={setRandomCrate} icon={<RedoOutlined />}>
                Random
              </Button>
            </div>
            {currentCrate !== null && (
              <Collapse accordion>
                {featureNames.length > 0 && (
                  <Panel
                    header="Features"
                    key="features"
                    extra={`${selectedFeatureNames.length}/${featureNames.length}`}
                  >
                    <Checkbox
                      indeterminate={indeterminate}
                      onChange={handleAllFeatureToggle}
                      checked={
                        selectedFeatureNames.length === featureNames.length
                      }
                    >
                      Toggle all features
                    </Checkbox>
                    <CheckboxGroup
                      options={featureNames}
                      value={selectedFeatureNames}
                      onChange={setSelectedFeatureNames as any}
                    />
                  </Panel>
                )}
                <Panel
                  header="Included crates"
                  key="crates"
                  extra={graphNodes.length}
                >
                  <List
                    bordered
                    dataSource={graphNodes}
                    renderItem={(crate: Crate) => (
                      <ListItem>
                        <ListItemMeta
                          title={
                            <a
                              href={`https://crates.io/crates/${crate.name}`}
                              key="crates.io-link"
                              target="_blank"
                              rel="noopener noreferrer"
                            >
                              {crate.name}
                            </a>
                          }
                          description={crate.description}
                        />
                      </ListItem>
                    )}
                  />
                </Panel>
                <Panel
                  header="Dependencies"
                  key="dependencies"
                  extra={graphLinks.length}
                >
                  <List
                    bordered
                    dataSource={graphLinks}
                    renderItem={(dependency: Dependency) => (
                      <ListItem>
                        {dependency.from} depends on {dependency.to}
                      </ListItem>
                    )}
                  />
                </Panel>
              </Collapse>
            )}
          </div>
        </Sider>
        <Content className="content">
          <div className="dependency-graph">
            <ForceGraphWrapper crates={graphNodes} dependencies={graphLinks} />
          </div>
        </Content>
      </Layout>
    </>
  );
}
