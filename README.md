# Delta Substream Powered Subgraph

## Overview

This subgraph tracks the `Multicall` handler calls on the [`DeltaBrokerProxy`](https://polygonscan.com/address/0x74E95F3Ec71372756a01eB9317864e3fdde1AC53#code)

## Running this locally

1. Install [Substreams CLI](https://substreams.streamingfast.io/getting-started/installing-the-cli)
2. `substreams run -e polygon.streamingfast.io:443 delta-v0.1.0.spkg graph_out --start-block 50589017 --stop-block +1000`

## Deploying this Subgraph
1. `pnpm install`
2. `pnpm graph build`
3. `pnpm graph deploy`
