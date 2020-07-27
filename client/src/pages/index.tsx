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
import { Crate, CrateDistance, Dependency } from "../utils/types";
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

interface CrateInfo {
  crate: Crate;
  selectedFeatures: string[];
}

export default function Home(): ReactElement {
  const [currentCrate, setCurrentCrate] = useState<CrateInfo | null>(null);
  const [featureNames, setFeatureNames] = useState<string[]>([]);
  const [searchTerm, setSearchTerm] = useState("");
  const [searchCrates, setSearchCrates] = useState<Crate[]>([]);
  const [graphNodes, setGraphNodes] = useState<CrateDistance[]>([]);
  const [graphLinks, setGraphLinks] = useState<Dependency[]>([]);
  const [indeterminate, setIndeterminate] = useState(true);
  const [clickedCrateName, setClickedCrateName] = useState<string | null>(null);
  const [error, setError] = useState("");

  const setRandomCrate = (): void => {
    const loadRandomCrate = async (): Promise<void> => {
      const randomCrateRes = await getRandomCrate();
      if (randomCrateRes.success) {
        setCurrentCrate({ crate: randomCrateRes.result, selectedFeatures: [] });
        setSearchTerm(randomCrateRes.result.name);
        setError("");
      } else {
        setError(randomCrateRes.error);
      }
    };

    loadRandomCrate();
  };

  useEffect(setRandomCrate, []);

  useEffect(() => {
    if (currentCrate !== null) {
      setFeatureNames(Object.keys(currentCrate.crate.features));
    }
  }, [currentCrate]);

  useEffect(() => {
    if (currentCrate !== null) {
      const loadCrateDependencies = async (): Promise<void> => {
        const dependencyGraphRes = await getDependencyGraph(
          currentCrate.crate.name,
          currentCrate.selectedFeatures
        );
        if (dependencyGraphRes.success) {
          setGraphNodes(dependencyGraphRes.result.crates);
          setGraphLinks(dependencyGraphRes.result.dependencies);
          setError("");
        } else {
          setError(dependencyGraphRes.error);
        }
      };

      loadCrateDependencies();
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

  useEffect(() => {
    if (currentCrate !== null) {
      setIndeterminate(
        currentCrate.selectedFeatures.length > 0 &&
          currentCrate.selectedFeatures.length < featureNames.length
      );
    }
  }, [featureNames, currentCrate]);

  const handleSearch = async (searchInput: string): Promise<void> => {
    if (searchInput.length > 0) {
      const searchCrateRes = await searchCrate(searchInput);
      if (searchCrateRes.success) {
        setSearchCrates(searchCrateRes.result);
      } else {
        setError(searchCrateRes.error);
      }
    } else {
      setSearchCrates([]);
    }
    setSearchTerm(searchInput);
  };

  const handleSearchSelect = (selectedCrateName: string): void => {
    setSearchTerm(selectedCrateName);
    setCurrentCrate({
      crate: searchCrates.find(
        (searchCrate) => searchCrate.name === selectedCrateName
      )!,
      selectedFeatures: [],
    });
  };

  const handleAllFeatureToggle = (e: CheckboxChangeEvent): void => {
    if (currentCrate !== null) {
      setCurrentCrate({
        ...currentCrate,
        selectedFeatures: e.target.checked ? featureNames : [],
      });
      setIndeterminate(false);
    }
  };

  const handleCheckboxGroup = (checked: string[]): void => {
    if (currentCrate !== null) {
      setCurrentCrate({ ...currentCrate, selectedFeatures: checked });
    }
  };

  const handleListClick = (crate: CrateDistance): void =>
    setClickedCrateName(crate.name !== clickedCrateName ? crate.name : null);

  return (
    <>
      <Head />
      <Layout>
        <Sider width="30%" theme="light">
          <div className="sider">
            <h1>Vault</h1>
            <h2>Current crate: {currentCrate?.crate.name}</h2>
            <div className="crate-picker">
              <AutoComplete
                options={
                  searchCrates.map((searchCrate) => ({
                    value: searchCrate.name,
                  })) as any
                }
                onSelect={handleSearchSelect}
                onSearch={handleSearch}
                value={searchTerm}
              >
                <Search
                  placeholder="Search for a crate..."
                  onSearch={handleSearchSelect}
                  disabled={searchTerm.length === 0}
                  allowClear
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
                    extra={`${currentCrate.selectedFeatures.length}/${featureNames.length}`}
                  >
                    <Checkbox
                      indeterminate={indeterminate}
                      onChange={handleAllFeatureToggle}
                      checked={
                        currentCrate.selectedFeatures.length ===
                        featureNames.length
                      }
                    >
                      Toggle all features
                    </Checkbox>
                    <CheckboxGroup
                      options={featureNames}
                      value={currentCrate.selectedFeatures}
                      onChange={handleCheckboxGroup as any}
                    />
                  </Panel>
                )}
                <Panel
                  header="Included crates"
                  key="crates"
                  extra={graphNodes.length}
                >
                  <List
                    dataSource={graphNodes}
                    renderItem={(crate: CrateDistance) => (
                      <Button
                        className="crate-list-button"
                        onClick={() => handleListClick(crate)}
                        block
                      >
                        <ListItem>
                          <ListItemMeta
                            title={
                              <div className="row">
                                <a
                                  href={`https://crates.io/crates/${crate.name}`}
                                  key="crates.io-link"
                                  target="_blank"
                                  rel="noopener noreferrer"
                                >
                                  {crate.name}
                                </a>
                                <div>Depth: {crate.distance}</div>
                              </div>
                            }
                            description={crate.description}
                          />
                        </ListItem>
                      </Button>
                    )}
                  />
                </Panel>
                {graphLinks.length > 0 && (
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
                )}
              </Collapse>
            )}
          </div>
        </Sider>
        <Content className="content">
          <div className="dependency-graph">
            <ForceGraphWrapper
              crates={graphNodes}
              dependencies={graphLinks}
              clickedCrateName={clickedCrateName}
              setClickedCrateName={setClickedCrateName}
            />
          </div>
        </Content>
      </Layout>
    </>
  );
}
