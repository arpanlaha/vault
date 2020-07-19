use std::collections::HashMap;

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

        return Ok(query_param_strs
            .map(|query_param_str| {
                let mut query_param_split = query_param_str.split('=');
                (
                    String::from(query_param_split.next().unwrap()),
                    String::from(query_param_split.next().unwrap()),
                )
            })
            .collect::<HashMap<String, String>>());
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

        return Ok(query_param_map);
    }
}
