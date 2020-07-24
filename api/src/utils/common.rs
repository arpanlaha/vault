use super::super::ingest::schema::Vertex;
use rand::Rng;
use std::collections::HashMap;

const MAX_SEARCH_LENGTH: usize = 10;

pub enum QueryParamError {
    InvalidQueryString,
}

pub fn get_query_params(query_str: &str) -> Result<HashMap<String, String>, QueryParamError> {
    if query_str.is_empty() {
        return Ok(HashMap::new());
    }

    if query_str.contains('&') {
        let mut query_param_strs = query_str.split('&');

        if query_param_strs.all(|query_param_str| !query_param_str.contains('=')) {
            return Err(QueryParamError::InvalidQueryString);
        }

        Ok(query_param_strs
            .map(|query_param_str| {
                let mut query_param_split = query_param_str.split('=');
                (
                    String::from(query_param_split.next().unwrap()),
                    String::from(query_param_split.next().unwrap()),
                )
            })
            .collect::<HashMap<String, String>>())
    } else {
        if !query_str.contains('=') {
            return Err(QueryParamError::InvalidQueryString);
        }
        let mut query_param_split = query_str.split('=');
        let mut query_param_map: HashMap<String, String> = HashMap::new();
        query_param_map.insert(
            String::from(query_param_split.next().unwrap()),
            String::from(query_param_split.next().unwrap()),
        );

        Ok(query_param_map)
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
        let mut results: Vec<(f64, &T)> = vec![];

        for vertex in self.values() {
            let name = vertex.id();

            if name != search_term {
                let search_score = strsim::jaro_winkler(name, search_term) * vertex.popularity();

                if results.is_empty() {
                    results.push((search_score, self.get(name).unwrap()));
                } else if search_score >= results.last().unwrap().0 {
                    if let Some((index, _)) = results
                        .iter()
                        .enumerate()
                        .find(|(_, (other_score, _))| search_score > *other_score)
                    {
                        results.insert(index, (search_score, vertex))
                    }

                    if results.len() > MAX_SEARCH_LENGTH {
                        results.pop();
                    }
                }
            }
        }

        if let Some(search_vertex) = self.get(search_term) {
            results.insert(0, (0f64, search_vertex));
            if results.len() > MAX_SEARCH_LENGTH {
                results.pop();
            }
        }

        results.iter().map(|(_, vertex)| *vertex).collect()
    }
}
