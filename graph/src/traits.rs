use super::schema::{Category, Crate, Keyword};
use rand::Rng;
use std::collections::{BTreeSet, HashMap, VecDeque};
use std::hash::BuildHasher;
use std::ops::Bound::{Excluded, Included};

const MAX_SEARCH_LENGTH: usize = 10;

/// A trait intended for vertices in the API graph.
pub trait Vertex {
    /// Returns a unique identifier for use by the API graph.
    fn id(&self) -> &str;

    /// Returns the unique identifier from the SQL representation.
    fn sql_id(&self) -> usize;

    /// Returns a number corresponding to the popularity of the Vertex.
    fn popularity(&self) -> usize;
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
    fn popularity(&self) -> usize {
        self.crates.len()
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
    fn popularity(&self) -> usize {
        self.downloads
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
    fn popularity(&self) -> usize {
        self.crates.len()
    }
}

pub trait Random<T> {
    fn random(&self) -> &T;
}

impl<T, U, S: BuildHasher> Random<T> for HashMap<U, T, S> {
    fn random(&self) -> &T {
        self.values()
            .nth(rand::thread_rng().gen_range(0, self.len()))
            .unwrap()
    }
}

pub trait Search<T: Vertex> {
    fn search<'a>(&self, search_term: &str, collection: &'a HashMap<String, T>) -> VecDeque<&'a T>;
}

impl<T: Vertex> Search<T> for BTreeSet<String> {
    fn search<'a>(&self, search_term: &str, collection: &'a HashMap<String, T>) -> VecDeque<&'a T> {
        let should_replace = |a: &T, b: &T| {
            a.popularity() > b.popularity()
                || (a.popularity() == b.popularity() && a.id().len() < b.id().len())
        };

        if search_term.is_empty() {
            VecDeque::new()
        } else {
            let mut range_end = String::from(search_term);
            let to_push = (range_end.pop().unwrap() as u8 + 1) as char;
            range_end.push(to_push);

            let prefixed_crate_names = self
                .range::<String, _>((Included(&String::from(search_term)), Excluded(&range_end)));

            let mut search_results: VecDeque<&T> = VecDeque::with_capacity(MAX_SEARCH_LENGTH + 1);

            for prefixed_crate_name in prefixed_crate_names {
                if prefixed_crate_name != search_term {
                    let prefixed_crate = collection.get(prefixed_crate_name).unwrap();
                    if search_results.is_empty() {
                        search_results.push_back(prefixed_crate);
                    } else if should_replace(prefixed_crate, *search_results.back().unwrap()) {
                        if let Some((index, _)) =
                            search_results
                                .iter()
                                .enumerate()
                                .find(|(_, results_crate)| {
                                    should_replace(prefixed_crate, results_crate)
                                })
                        {
                            search_results.insert(index, prefixed_crate);

                            if search_results.len() > MAX_SEARCH_LENGTH {
                                search_results.pop_back();
                            }
                        }
                    }
                }
            }

            if let Some(search_crate) = collection.get(search_term) {
                search_results.push_front(search_crate);
                if search_results.len() > MAX_SEARCH_LENGTH {
                    search_results.pop_back();
                }
            }

            search_results
        }
    }
}

// pub trait Search<T: Vertex> {
//     fn search<'a>(&'a self, search_term: &str) -> Vec<&'a T>;
// }

// impl<T: Vertex, S: BuildHasher> Search<T> for HashMap<String, T, S> {
//     fn search<'a>(&'a self, search_term: &str) -> Vec<&'a T> {
//         let mut results: VecDeque<(f64, &T)> = VecDeque::new();

//         for vertex in self.values() {
//             let name = vertex.id();

//             if name != search_term {
//                 let similarity = strsim::jaro_winkler(name, search_term);
//                 if similarity > 0.5 {
//                     let search_score = similarity * vertex.popularity();

//                     if results.is_empty() {
//                         results.push_back((search_score, self.get(name).unwrap()));
//                     } else if search_score >= results.back().unwrap().0 {
//                         if let Some((index, _)) = results
//                             .iter()
//                             .enumerate()
//                             .find(|(_, (other_score, _))| search_score > *other_score)
//                         {
//                             results.insert(index, (search_score, vertex))
//                         }

//                         if results.len() > MAX_SEARCH_LENGTH {
//                             results.pop_back();
//                         }
//                     }
//                 }
//             }
//         }

//         if let Some(search_vertex) = self.get(search_term) {
//             results.insert(0, (0_f64, search_vertex));
//             if results.len() > MAX_SEARCH_LENGTH {
//                 results.pop_back();
//             }
//         }

//         results.iter().map(|(_, vertex)| *vertex).collect()
//     }
// }
