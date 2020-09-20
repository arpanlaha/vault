import axios, { AxiosResponse } from "axios";
import { stringify } from "qs";
import {
  CfgNameLIst,
  Crate,
  DependencyGraph,
  LastUpdated,
  Response,
  TargetList,
} from "./types";

const API_URL = process.env.GATSBY_VAULT_API_URL;

const wrapResponse = <T>(
  response: Promise<AxiosResponse<T>>
): Promise<Response<T>> =>
  response
    .then((response) =>
      response.status === 200
        ? {
            success: true as const,
            result: response.data,
          }
        : {
            success: false as const,
            error: String(response.data),
          }
    )
    .catch((error) => ({
      success: false as const,
      error:
        error?.response?.data ??
        "Server error - please post an issue at https://github.com/arpanlaha/vault/issues",
    }));

export const getDependencyGraph = (
  crateId: string,
  features: string[] = [],
  target: string | undefined = undefined,
  cfgName: string | undefined = undefined
): Promise<Response<DependencyGraph>> =>
  wrapResponse(
    axios.get(`${API_URL}/graph/${crateId}`, {
      params: {
        features: features.length > 0 ? features : undefined,
        target,
        cfg_name: cfgName,
      },
      paramsSerializer: (param) => stringify(param, { arrayFormat: "comma" }),
    })
  );

export const searchCrates = (searchTerm: string): Promise<Response<Crate[]>> =>
  wrapResponse(axios.get(`${API_URL}/search/crates/${searchTerm}`));

export const getRandomDependencyGraph = (): Promise<
  Response<DependencyGraph>
> => wrapResponse(axios.get(`${API_URL}/random/graph`));

export const getLastUpdated = (): Promise<Response<LastUpdated>> =>
  wrapResponse(axios.get(`${API_URL}/state/last-updated`));

export const getTargets = (): Promise<Response<TargetList>> =>
  wrapResponse(axios.get(`${API_URL}/compiler/targets`));

export const getCfgNames = (): Promise<Response<CfgNameLIst>> =>
  wrapResponse(axios.get(`${API_URL}/compiler/cfg-names`));
