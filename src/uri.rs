use std::collections::hash_map::Iter;
use std::collections::HashMap;

#[derive(Debug)]
pub struct UriPath {
    parts: Vec<String>,
    query_params: HashMap<String, String>,
}

impl UriPath {
    pub fn new<T: AsRef<str>>(path: T) -> UriPath {
        let split_path: Vec<&str> = path.as_ref().split('?').collect();
        let mut query_params = HashMap::new();
        if split_path.len() > 1 && split_path[1] != "" {
            for pair in split_path[1].split('&') {
                let mut split = pair.split('=');
                query_params.insert(
                    split.next().expect("query key").to_owned(),
                    split.next().expect("query value").to_owned(),
                );
            }
        }
        let parts = split_path[0].split('/').map(|x| x.to_owned()).collect();
        UriPath {
            parts,
            query_params,
        }
    }

    pub fn root(&self) -> &str {
        &self.parts[0]
    }

    pub fn part(&self, index: usize) -> Option<&str> {
        if index < self.parts.len() {
            Some(&self.parts[index])
        } else {
            None
        }
    }

    /// Iterator over query parameters
    pub fn query_params(&self) -> Iter<String, String> {
        self.query_params.iter()
    }

    /// Returns the id segment of the path if present. The ID segment is the
    /// second segment.
    pub fn id_segment(&self) -> Option<usize> {
        self.part(1).map(|x| x.parse().expect("ID is not a number"))
    }
}
