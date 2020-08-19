import axios, { AxiosResponse } from "axios";
import {
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
            success: true as true,
            result: response.data,
          }
        : {
            success: false as false,
            error: String(response.data),
          }
    )
    .catch((error) => ({
      success: false as false,
      error:
        error?.response?.data ??
        "Server error - please post an issue at https://github.com/arpanlaha/vault/issues",
    }));

export const getDependencyGraph = (
  crateId: string,
  features: string[] = []
): Promise<Response<DependencyGraph>> =>
  wrapResponse(
    axios.get(
      `${API_URL}/graph/${crateId}${
        features.length > 0 ? `?features=${features.join(",")}` : ""
      }`
    )
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

export const getCfgNames = (): Promise<Response<LastUpdated>> =>
  wrapResponse(axios.get(`${API_URL}/compiler/cfg-names`));
