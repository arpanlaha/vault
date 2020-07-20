import axios from "axios";

const API_URL = process.env.GATSBY_API_URL;

interface Crate {
  categories: string[];
  created_at: string;
  description: string;
  downloads: number;
  features: Record<string, string[]>;
  keywords: string[];
  name: string;
  version: string;
}

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
