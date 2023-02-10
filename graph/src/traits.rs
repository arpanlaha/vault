use super::schema::{Category, Crate, Keyword};
use ahash::AHashMap;
use rand::Rng;
use std::{
    collections::{BTreeSet, VecDeque},
    hash::BuildHasher,
    ops::Bound::{Excluded, Included},
};

/// The max search results length permitted for collection searches.
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

/// A trait to enable returning a random element from a collection.
pub trait Random<T> {
    /// Returns a random element from a collection.
    fn random(&self) -> &T;
}

impl<T, U, S: BuildHasher> Random<T> for AHashMap<U, T, S> {
    /// Returns a random element from a `AHashMap`.
    fn random(&self) -> &T {
        self.values()
            .nth(rand::thread_rng().gen_range(0..self.len()))
            .unwrap()
    }
}

/// A trait to enable searching for elements within a collection.
pub trait Search<T: Vertex> {
    /// Searches ro elements within a collection.
    ///
    /// # Arguments
    /// * `search_term` - the term being searched.
    /// * `collection` - the `AHashMap` containing the values of the collection.
    fn search<'a>(&self, search_term: &str, collection: &'a AHashMap<String, T>)
        -> VecDeque<&'a T>;
}

impl<T: Vertex> Search<T> for BTreeSet<String> {
    /// Searches ro elements within a collection.
    ///
    /// # Arguments
    /// * `search_term` - the term being searched.
    /// * `collection` - the `AHashMap` containing the values of the collection.
    fn search<'a>(
        &self,
        search_term: &str,
        collection: &'a AHashMap<String, T>,
    ) -> VecDeque<&'a T> {
        // Search results are sorted by popularity, with tiebreakers favoring shorter results.
        // As the results are traversed in alphabetical order, further ties will be broken by lexicographic order.
        let should_replace = |a: &T, b: &T| {
            a.popularity() > b.popularity()
                || (a.popularity() == b.popularity() && a.id().len() < b.id().len())
        };

        if search_term.is_empty() {
            VecDeque::new()
        } else {
            // The pool of potential results is found by looking for all terms in a range from the search term to the "next" string.
            // The "next" string is the search term with the last character incremented by a value of one.
            let mut range_end = String::from(search_term);
            let to_push = (range_end.pop().unwrap() as u8 + 1) as char;
            range_end.push(to_push);

            // Searching using this range ([start, end)) will result in all elements with the search term as a prefix.
            let prefixed_vertex_names = self
                .range::<String, _>((Included(&String::from(search_term)), Excluded(&range_end)));

            let mut search_results: VecDeque<&T> = VecDeque::with_capacity(MAX_SEARCH_LENGTH + 1);

            for prefixed_vertex_name in prefixed_vertex_names {
                if prefixed_vertex_name != search_term {
                    let prefixed_vertex = collection.get(prefixed_vertex_name).unwrap();

                    if search_results.is_empty() {
                        // add if first element found
                        search_results.push_back(prefixed_vertex);
                    } else if should_replace(prefixed_vertex, search_results.back().unwrap()) {
                        if let Some((index, _)) =
                            search_results
                                .iter()
                                .enumerate()
                                .find(|(_, results_vertex)| {
                                    should_replace(prefixed_vertex, results_vertex)
                                })
                        {
                            // insert before first element that has a lower priority than the current name
                            search_results.insert(index, prefixed_vertex);

                            // cap results length at `MAX_SEARCH_LENGTH`
                            if search_results.len() > MAX_SEARCH_LENGTH {
                                search_results.pop_back();
                            }
                        }
                    }
                }
            }

            // If the search term is a vertex, prepend tha to the results.
            if let Some(search_vertex) = collection.get(search_term) {
                search_results.push_front(search_vertex);
                if search_results.len() > MAX_SEARCH_LENGTH {
                    search_results.pop_back();
                }
            }

            search_results
        }
    }
}
