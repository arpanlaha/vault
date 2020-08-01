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
}

export interface Dependency {
  from: string;
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
