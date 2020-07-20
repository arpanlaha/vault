import axios from "axios";
import { Crate, DependencyGraph } from "./types";

const API_URL = process.env.GATSBY_API_URL;

type Response<T> =
  | {
      result: T;
      success: true;
    }
  | { error: string; success: false };

export const getCrate = (crateId: string): Promise<Response<Crate>> =>
  axios
    .get(`${API_URL}/crates/${crateId}`)
    .then((response) => ({
      success: true as true,
      result: response.data,
    }))
    .catch((error) => ({
      success: false as false,
      error:
        error ??
        "Server error - please post an issue at https://github.com/arpanlaha/vault/issues",
    }));

export const getDependencyGraph = (
  crateId: string,
  features: string[] = []
): Promise<Response<DependencyGraph>> =>
  axios
    .get(
      `${API_URL}/graph/${crateId}${
        features.length > 0 ? `?features=${features.join(",")}` : ""
      }`
    )
    .then((response) => ({
      success: true as true,
      result: response.data,
    }))
    .catch((error) => ({
      success: false as false,
      error:
        error ??
        "Server error - please post an issue at https://github.com/arpanlaha/vault/issues",
    }));
