# Vault

Interactively visualize your crates.io dependencies.

## Overview

This web application allows you to view the dependency graph of any [crates.io](https://crates.io/) crate, clearing any uncertainties about what transitive dependencies you would pull in by depending on it. Selecting specific features is supported; however, only the latest stable version of each crate (or the latest version if no stable version exists) is tracked, which may result in dependency graphs of outdated crates being incorrect if they depend on an earlier version of a crate.

This application is a work in progress, and may break from time to time until it is finalized.

## Stack

### Client

This application uses React on the frontend, with [Ant Design](https://ant.design/) supplying UI components and [react-force-graph](https://github.com/vasturiano/react-force-graph) powering the graph visualization.

### API

The server is built on [warp](https://github.com/seanmonstar/warp), and pulls in the latest crates.io data daily from the [official database dump](https://static.crates.io/db-dump.tar.gz). This data is processed and held in memory, as I found no graph databases suitable for the functionality I desired.
