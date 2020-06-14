use std::collections::hash_map::Iter;
use std::collections::HashMap;
use hyper::Request;
use std::convert::From;

#[derive(Debug)]
pub struct UriPath {
    parts: Vec<String>,
    query_params: HashMap<String, String>,
}

impl From<&Request> for UriPath {
    fn from(req: &Request) -> Self {
        let parts = req.path().split('/').skip(1).map(|x| x.to_owned()).collect();
        let query_params = req.query().map(|query| {
            let mut query_params = HashMap::new();
            for pair in query.split('&') {
                let mut split = pair.split('=');
                query_params.insert(
                    split.next().expect("query key").to_owned(),
                    split.next().expect("query value").to_owned(),
                );
            }
            query_params
        });
        UriPath {
            parts,
            query_params: query_params.unwrap_or(HashMap::new())
        }
    }
}

impl UriPath {
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
