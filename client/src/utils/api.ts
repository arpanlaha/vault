import axios, { AxiosResponse } from "axios";
import { Crate, DependencyGraph } from "./types";

const API_URL = process.env.GATSBY_API_URL;

type Response<T> =
  | {
      result: T;
      success: true;
    }
  | { error: string; success: false };

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
        : { success: false as false, error: String(response.data) }
    )
    .catch((error) => ({
      success: false as false,
      error:
        error ??
        "Server error - please post an issue at https://github.com/arpanlaha/vault/issues",
    }));

export const getCrate = (crateId: string): Promise<Response<Crate>> =>
  wrapResponse(axios.get(`${API_URL}/crates/${crateId}`));

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
