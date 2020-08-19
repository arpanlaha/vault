import React, { ReactElement, useEffect, useState } from "react";
import { CratePanelBody } from "../components";
import {
  AutoComplete,
  Button,
  Checkbox,
  Collapse,
  Input,
  Layout,
  List,
} from "antd";
import { getLastUpdated, searchCrates } from "../utils/api";
import { Crate, CrateDistance, Dependency } from "../utils/types";
import { CheckboxChangeEvent } from "antd/lib/checkbox";

import { RedoOutlined } from "@ant-design/icons";

const { Panel } = Collapse;
const { Search } = Input;
const { Content, Footer, Sider } = Layout;
const CheckboxGroup = Checkbox.Group;
const ListItem = List.Item;
const ListItemMeta = List.Item.Meta;

const MINUTE = 60;
const HOUR = MINUTE * 60;
const DAY = HOUR * 24; // eslint-disable-line @typescript-eslint/no-magic-numbers

interface SidebarProps {
  clickedCrateName: string | null;
  currentCrate: Crate | null;
  featureNames: string[];
  graphLinks: Dependency[];
  graphNodes: CrateDistance[];
  loadDependencyGraph: (crateId: string, features: string[]) => void;
  portrait: boolean;
  searchTerm: string;
  setClickedCrateName: (clickedCrateName: string | null) => void;
  setError: (error: string) => void;
  setRandomCrate: () => void;
  setSearchTerm: (searchTerm: string) => void;
  setSelectedFeatures: (selectedFeatures: string[]) => void;
  setUrlCrateName: (urlCrateName: string) => void;
  setUrlFeatures: (urlFeatures: string[] | undefined) => void;
  selectedFeatures: string[];
}

export default function Sidebar(props: SidebarProps): ReactElement {
  const [searchedCrates, setSearchedCrates] = useState<Crate[]>([]);
  const [indeterminate, setIndeterminate] = useState(true);
  const [lastUpdated, setLastUpdated] = useState<string | null>(null);

  const {
    clickedCrateName,
    currentCrate,
    featureNames,
    graphLinks,
    graphNodes,
    loadDependencyGraph,
    portrait,
    searchTerm,
    setClickedCrateName,
    setError,
    setRandomCrate,
    setSearchTerm,
    setSelectedFeatures,
    setUrlCrateName,
    setUrlFeatures,
    selectedFeatures,
  } = props;

  useEffect(() => {
    const loadLastUpdated = async (): Promise<void> => {
      const lastUpdatedRes = await getLastUpdated();
      if (lastUpdatedRes.success) {
        const { seconds } = lastUpdatedRes.result;
        if (seconds > DAY) {
          const days = Math.floor(seconds / DAY);
          setLastUpdated(`${days} day${days > 1 ? "s" : ""}`);
        } else if (seconds > HOUR) {
          const hours = Math.floor(seconds / HOUR);
          setLastUpdated(`${hours} hour${hours > 1 ? "s" : ""}`);
        } else if (seconds > MINUTE) {
          const minutes = Math.floor(seconds / MINUTE);
          setLastUpdated(`${minutes} minute${minutes > 1 ? "s" : ""}`);
        } else {
          setLastUpdated(`${seconds} second${seconds > 1 ? "s" : ""}`);
        }
      } else {
        setError(lastUpdatedRes.error);
      }
    };

    loadLastUpdated();
  }, [setError]);

  useEffect(() => {
    if (currentCrate !== null) {
      setIndeterminate(
        selectedFeatures.length > 0 &&
          selectedFeatures.length < featureNames.length
      );
    }
  }, [featureNames, currentCrate, selectedFeatures]);

  const handleSearch = async (searchInput: string): Promise<void> => {
    setSearchTerm(searchInput);
    if (searchInput.length > 0) {
      const searchCratesRes = await searchCrates(searchInput.toLowerCase());
      if (searchCratesRes.success) {
        setSearchedCrates(searchCratesRes.result);
      } else {
        setError(searchCratesRes.error);
      }
    } else {
      setSearchedCrates([]);
    }
  };

  const handleCrateSelect = (
    crates: Crate[]
  ): ((selectedCrateName: string) => void) => (selectedCrateName: string) => {
    handleSearch(selectedCrateName);

    if (selectedCrateName !== "" && selectedCrateName !== currentCrate?.name) {
      setUrlCrateName(selectedCrateName);
      setUrlFeatures(undefined);
      const selectedCrate = crates.find(
        (crate) => crate.name === selectedCrateName
      );
      if (selectedCrate !== undefined) {
        loadDependencyGraph(selectedCrateName, []);
      } else {
        setError(`Crate with id ${selectedCrateName} does not exist.`);
      }
    }
  };

  const handleSearchSelect = handleCrateSelect(searchedCrates);

  const handlePanelSelect = handleCrateSelect(graphNodes);

  const handleAllFeatureToggle = (e: CheckboxChangeEvent): void => {
    if (currentCrate !== null) {
      setSelectedFeatures(e.target.checked ? featureNames : []);
      loadDependencyGraph(
        currentCrate.name,
        e.target.checked ? featureNames : []
      );
      setUrlFeatures(e.target.checked ? featureNames : undefined);
      setIndeterminate(false);
    }
  };

  const handleCheckboxGroup = (checked: string[]): void => {
    if (currentCrate !== null) {
      setSelectedFeatures(checked);
      loadDependencyGraph(currentCrate.name, checked);
      setUrlFeatures(checked);
    }
  };

  const handleListClick = (crate: CrateDistance): void =>
    setClickedCrateName(crate.name !== clickedCrateName ? crate.name : null);

  return (
    <Sider
      width={portrait ? "80%" : "30%"}
      theme="light"
      collapsible={portrait}
      collapsedWidth={0}
    >
      <Layout>
        <Content>
          <div className="column sider">
            <h1>{currentCrate?.name ?? "loading..."}</h1>
            <div className="row crate-picker">
              <AutoComplete
                options={
                  searchedCrates.map((searchedCrate) => ({
                    value: searchedCrate.name,
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
                  enterButton
                />
              </AutoComplete>
              <Button onClick={setRandomCrate} icon={<RedoOutlined />}>
                Random
              </Button>
            </div>
            {currentCrate !== null && (
              <Collapse accordion>
                <Panel
                  header={clickedCrateName ?? currentCrate.name}
                  key="crate"
                  extra={
                    clickedCrateName !== null &&
                    clickedCrateName !== currentCrate.name ? (
                      <Button
                        type="link"
                        onClick={() => handlePanelSelect(clickedCrateName)}
                      >
                        View dependency graph
                      </Button>
                    ) : undefined
                  }
                >
                  <CratePanelBody
                    crate={graphNodes.find(
                      (crate) =>
                        crate.name === (clickedCrateName ?? currentCrate.name)
                    )}
                    dependencies={graphLinks
                      .filter(
                        (dependency) =>
                          dependency.from ===
                          (clickedCrateName ?? currentCrate.name)
                      )
                      .map((dependency) => dependency.to)}
                    setClickedCrateName={setClickedCrateName}
                  />
                </Panel>

                {featureNames.length > 0 && (
                  <Panel
                    header="Features"
                    key="features"
                    extra={`${selectedFeatures.length}/${featureNames.length}`}
                  >
                    <Checkbox
                      indeterminate={indeterminate}
                      onChange={handleAllFeatureToggle}
                      checked={selectedFeatures.length === featureNames.length}
                    >
                      Toggle all features
                    </Checkbox>
                    <CheckboxGroup
                      options={featureNames}
                      value={selectedFeatures}
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
                              <div className="row crate-row">
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
        </Content>
        <Footer className="row footer">
          {lastUpdated !== null && (
            <span>
              Last updated {lastUpdated} ago.{" "}
              <a
                href="https://github.com/arpanlaha/vault"
                target="_blank"
                rel="noopener noreferrer"
              >
                View on GitHub.
              </a>
            </span>
          )}
        </Footer>
      </Layout>
    </Sider>
  );
}
