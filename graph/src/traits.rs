use super::schema::{Category, Crate, Keyword};
use rand::Rng;
use std::collections::{HashMap, VecDeque};

const MAX_SEARCH_LENGTH: usize = 10;

/// A trait intended for vertices in the API graph.
pub trait Vertex {
    /// Returns a unique identifier for use by the API graph.
    fn id(&self) -> &str;

    /// Returns the unique identifier from the SQL representation.
    fn sql_id(&self) -> usize;

    /// Returns a number corresponding to the popularity of the Vertex.
    fn popularity(&self) -> f64;
}

impl Vertex for Category {
    /// Returns a unique identifier for use by the API graph.
    fn id(&self) -> &str {
        self.category.as_str()
    }

    /// Returns the unique identifier from the SQL representation.
    fn sql_id(&self) -> usize {
        self.id
    }

    /// Returns a number corresponding to the popularity of the Vertex.
    fn popularity(&self) -> f64 {
        (self.crates.len() as f64).log10().log10()
    }
}

impl Vertex for Crate {
    /// Returns a unique identifier for use by the API graph.
    fn id(&self) -> &str {
        self.name.as_str()
    }

    /// Returns the unique identifier from the SQL representation.
    fn sql_id(&self) -> usize {
        self.id
    }

    /// Returns a number corresponding to the popularity of the Vertex.
    fn popularity(&self) -> f64 {
        (self.downloads as f64).log10().sqrt()
    }
}

impl Vertex for Keyword {
    /// Returns a unique identifier for use by the API graph.
    fn id(&self) -> &str {
        self.keyword.as_str()
    }
    /// Returns the unique identifier from the SQL representation.
    fn sql_id(&self) -> usize {
        self.id
    }

    /// Returns a number corresponding to the popularity of the Vertex.
    fn popularity(&self) -> f64 {
        (self.crates.len() as f64).log10().sqrt()
    }
}

pub trait Random<T> {
    fn random(&self) -> &T;
}

impl<T, U> Random<T> for HashMap<U, T> {
    fn random(&self) -> &T {
        self.values()
            .nth(rand::thread_rng().gen_range(0, self.len()))
            .unwrap()
    }
}

pub trait Search<T: Vertex> {
    fn search<'a>(&'a self, search_term: &str) -> Vec<&'a T>;
}

impl<T: Vertex> Search<T> for HashMap<String, T> {
    fn search<'a>(&'a self, search_term: &str) -> Vec<&'a T> {
        let mut results: VecDeque<(f64, &T)> = VecDeque::new();

        for vertex in self.values() {
            let name = vertex.id();

            if name != search_term {
                let similarity = strsim::jaro_winkler(name, search_term);
                if similarity > 0.5 {
                    let search_score = similarity * vertex.popularity();

                    if results.is_empty() {
                        results.push_back((search_score, self.get(name).unwrap()));
                    } else if search_score >= results.back().unwrap().0 {
                        if let Some((index, _)) = results
                            .iter()
                            .enumerate()
                            .find(|(_, (other_score, _))| search_score > *other_score)
                        {
                            results.insert(index, (search_score, vertex))
                        }

                        if results.len() > MAX_SEARCH_LENGTH {
                            results.pop_back();
                        }
                    }
                }
            }
        }

        if let Some(search_vertex) = self.get(search_term) {
            results.insert(0, (0f64, search_vertex));
            if results.len() > MAX_SEARCH_LENGTH {
                results.pop_back();
            }
        }

        results.iter().map(|(_, vertex)| *vertex).collect()
    }
}
