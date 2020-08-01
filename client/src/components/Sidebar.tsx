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
import { searchCrate } from "../utils/api";
import { Crate, CrateDistance, CrateInfo, Dependency } from "../utils/types";
import { CheckboxChangeEvent } from "antd/lib/checkbox";

import { RedoOutlined } from "@ant-design/icons";

const { Panel } = Collapse;
const { Search } = Input;
const { Sider } = Layout;
const CheckboxGroup = Checkbox.Group;
const ListItem = List.Item;
const ListItemMeta = List.Item.Meta;

interface SidebarProps {
  clickedCrateName: string | null;
  currentCrate: CrateInfo | null;
  featureNames: string[];
  graphLinks: Dependency[];
  graphNodes: CrateDistance[];
  portrait: boolean;
  searchTerm: string;
  setClickedCrateName: (clickedCrateName: string | null) => void;
  setCurrentCrate: (crate: CrateInfo | null) => void;
  setError: (error: string) => void;
  setRandomCrate: () => void;
  setSearchTerm: (searchTerm: string) => void;
  setUrlCrateName: (urlCrateName: string) => void;
  setUrlFeatures: (urlFeatures: string[] | undefined) => void;
}

export default function Sidebar(props: SidebarProps): ReactElement {
  const [searchCrates, setSearchCrates] = useState<Crate[]>([]);
  const [indeterminate, setIndeterminate] = useState(true);

  const {
    clickedCrateName,
    currentCrate,
    featureNames,
    graphLinks,
    graphNodes,
    portrait,
    searchTerm,
    setClickedCrateName,
    setCurrentCrate,
    setError,
    setRandomCrate,
    setSearchTerm,
    setUrlCrateName,
    setUrlFeatures,
  } = props;

  useEffect(() => {
    if (currentCrate !== null) {
      setIndeterminate(
        currentCrate.selectedFeatures.length > 0 &&
          currentCrate.selectedFeatures.length < featureNames.length
      );
    }
  }, [featureNames, currentCrate]);

  const handleSearch = async (searchInput: string): Promise<void> => {
    setSearchTerm(searchInput);
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
  };

  const handleSearchSelect = (selectedCrateName: string): void => {
    setSearchTerm(selectedCrateName);
    setUrlCrateName(selectedCrateName);
    setUrlFeatures(undefined);
    const selectedCrate = searchCrates.find(
      (searchCrate) => searchCrate.name === selectedCrateName
    );
    if (selectedCrate !== undefined) {
      setCurrentCrate(
        selectedCrateName.length > 0
          ? {
              crate: searchCrates.find(
                (searchCrate) => searchCrate.name === selectedCrateName
              )!,
              selectedFeatures: [],
            }
          : null
      );
    } else {
      setError(`Crate with id ${selectedCrateName} does not exist.`);
    }
  };

  const handleAllFeatureToggle = (e: CheckboxChangeEvent): void => {
    if (currentCrate !== null) {
      setCurrentCrate({
        ...currentCrate,
        selectedFeatures: e.target.checked ? featureNames : [],
      });
      setUrlFeatures(e.target.checked ? featureNames : undefined);
      setIndeterminate(false);
    }
  };

  const handleCheckboxGroup = (checked: string[]): void => {
    if (currentCrate !== null) {
      setCurrentCrate({
        ...currentCrate,
        selectedFeatures: checked,
      });
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
      <div className="column sider">
        <h1>Current crate: {currentCrate?.crate.name}</h1>
        <div className="row crate-picker">
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
              header={clickedCrateName ?? currentCrate.crate.name}
              key="crate"
            >
              {" "}
              <CratePanelBody
                crate={
                  clickedCrateName !== null
                    ? graphNodes.find(
                        (crate) => crate.name === clickedCrateName
                      )!
                    : currentCrate.crate
                }
                dependencies={graphLinks
                  .filter(
                    (dependency) =>
                      dependency.from === clickedCrateName ??
                      currentCrate.crate.name
                  )
                  .map((dependency) => dependency.to)}
              />
            </Panel>

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
                    currentCrate.selectedFeatures.length === featureNames.length
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
    </Sider>
  );
}
