import React, { ReactElement, useEffect, useState } from "react";
import { Head, ForceGraphWrapper } from "../components";
import { AutoComplete, Input, notification, Layout } from "antd";
import { getDependencyGraph, searchCrate } from "../utils/api";
import { Crate, Dependency } from "../utils/types";

import "../styles/antd.scss";
import "../styles/vault.scss";

const { Search } = Input;
const { Content, Sider } = Layout;

export default function Home(): ReactElement {
  const [currentCrate, setCurrentCrate] = useState("actix-web");
  const [searchTerm, setSearchTerm] = useState("");
  const [searchCrates, setSearchCrates] = useState<Crate[]>([]);
  const [crates, setCrates] = useState<Crate[]>([]);
  const [dependencies, setDependencies] = useState<Dependency[]>([]);
  const [error, setError] = useState("");

  useEffect(() => {
    const loadCrate = async (): Promise<void> => {
      const dependencyGraphRes = await getDependencyGraph(currentCrate);
      if (dependencyGraphRes.success) {
        setCrates(dependencyGraphRes.result.crates);
        setDependencies(dependencyGraphRes.result.dependencies);
      } else {
        setError(dependencyGraphRes.error);
      }
    };

    loadCrate();
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
    const loadSearch = async (): Promise<void> => {
      const searchCrateRes = await searchCrate(searchTerm);
      if (searchCrateRes.success) {
        setSearchCrates(searchCrateRes.result);
      } else {
        setError(searchCrateRes.error);
      }
    };

    if (searchTerm.length > 0) {
      loadSearch();
    } else {
      setSearchCrates([]);
    }
  }, [searchTerm]);

  const handleSearchButton = (): void => {
    if (searchTerm.length > 0) {
      setCurrentCrate(searchTerm);
    }
  };

  return (
    <>
      <Head />
      <Layout>
        <Sider className="sider" width="20%" theme="light">
          <h1>Vault</h1>
          <h2>Current crate: {currentCrate}</h2>
          <AutoComplete
            options={
              searchCrates.map((searchCrate) => ({
                value: searchCrate.name,
              })) as any
            }
            onSelect={setCurrentCrate}
            onSearch={setSearchTerm}
          >
            <Search
              placeholder="Search for a crate..."
              enterButton
              onClick={handleSearchButton}
            />
          </AutoComplete>
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
