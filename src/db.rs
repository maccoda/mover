use serde_json::Value;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Database(HashMap<String, Value>);

impl Database {
    pub fn get(&self, s: &str) -> Option<Value> {
        self.0.get(s).map(|x| x.to_owned())
    }

    /// Returns element with the given key, filtered for the provided ID if
    /// present both the key and ID are present.
    pub fn find_with_id(&self, key: &str, id: usize) -> Option<Value> {
        let value = self.get(key);
        if let Some(value) = value {
            value
                .as_array()
                .expect("Not an array")
                .into_iter()
                .find(|x| {
                    x.as_object()
                        .expect("Not an object")
                        .get("id")
                        .expect("No ID field found")
                        .as_u64()
                        .expect("ID not u64")
                        == id as u64
                }).map(|x| x.to_owned())
        } else {
            None
        }
    }
}
