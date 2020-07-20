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

export interface Dependency {
  from: string;
  // kind: 0 | 1 | 2;
  to: string;
}

export interface DependencyGraph {
  crates: Crate[];
  dependencies: Dependency[];
}
