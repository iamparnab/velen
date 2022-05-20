pub mod server_utils {
    use std::collections::HashMap;

    use regex::Regex;
    pub fn get_method_path_query(content: &str) -> (String, String, HashMap<String, String>) {
        /* First extract method and path + query_params */
        let outer_re = Regex::new(r"^(GET|POST) (.+) HTTP").unwrap();
        let outer_caps = outer_re.captures(content).unwrap();

        let method = outer_caps.get(1).unwrap().as_str().to_string();
        let path_with_query = outer_caps.get(2).unwrap().as_str().to_string();

        /* Then extract path and query_param string*/
        let inner_re = Regex::new(r"(.*)\?(.*)").unwrap();
        let inner_caps = inner_re.captures(&path_with_query);

        let path: String;
        let query_string: String;

        if let Some(caps) = inner_caps {
            path = caps.get(1).unwrap().as_str().to_string();
            query_string = caps.get(2).unwrap().as_str().to_string();
        } else {
            /* No Query String is present */
            path = path_with_query;
            query_string = String::from("");
        }

        let query_param_pairs = query_string.split("&");

        /* Then extrac individual Key and Values */
        let pair_re = Regex::new(r"(.*)=(.*)").unwrap();

        let mut query_map = HashMap::new();

        for pair in query_param_pairs {
            let pair_caps = pair_re.captures(pair);

            if let Some(caps) = pair_caps {
                let key = caps.get(1).unwrap().as_str().to_string();
                let value = caps.get(2).unwrap().as_str().to_string();

                query_map.insert(key, value);
            }
        }
        return (method, path, query_map);
    }
    pub fn parse_body(content: &str) -> String {
        let mut itr = content.split("\r\n\r\n");
        itr.next();
        let val = itr
            .next()
            .unwrap()
            .trim_end_matches(char::from(0))
            .to_string();
        return val;
    }
}
