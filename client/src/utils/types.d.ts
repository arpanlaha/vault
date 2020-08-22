export type Response<T> =
  | {
      result: T;
      success: true;
    }
  | { error: string; success: false };

export interface Crate {
  categories: string[];
  created_at: string;
  description: string;
  downloads: number;
  features: Record<string, string[]>;
  keywords: string[];
  name: string;
  version: string;
}

export interface CrateDistance extends Crate {
  distance: number;
  enabled_features: string[];
}

export interface Dependency {
  from: string;
  target?: string;
  to: string;
}

export interface DependencyGraph {
  crates: CrateDistance[];
  dependencies: Dependency[];
}

export interface CrateInfo {
  crate: Crate;
  selectedFeatures: string[];
}

export interface LastUpdated {
  seconds: number;
}

export interface TargetList {
  targets: string[];
}

export interface CfgNameLIst {
  cfg_names: string[];
}
