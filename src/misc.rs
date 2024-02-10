use std::{collections::HashMap, fmt::Display};

/// An extension to ``std::collections::HashMap<K, Vec<String>>``
pub trait HashMapExt<K> {
    /// Merges two ``Hashmap<K, Vec<String>>`` returning the merged hashmap.
    fn join(self, new_hashmap: HashMap<K, Vec<String>>) -> HashMap<K, Vec<String>>;
}

impl<K: std::cmp::Eq + std::hash::Hash + std::clone::Clone> HashMapExt<K>
    for HashMap<K, Vec<String>>
{
    fn join(mut self, other_hashmap: HashMap<K, Vec<String>>) -> HashMap<K, Vec<String>> {
        // I am well aware that this function is dogshit for performance; but tbh I don't give enough of a shit to do anything about it.

        for key in other_hashmap.keys() {
            if self.contains_key(key) {
                for string in &other_hashmap[key] {
                    if self[key].contains(string) {
                        continue;
                    } // Avoid repeating strings in the vectors.
                }

                let mut self_vector = self[key].clone();
                let mut other_vector = other_hashmap[key].clone();

                self_vector.append(&mut other_vector);

                self.insert(key.clone(), self_vector);
            } else {
                self.insert(key.clone(), other_hashmap[key].clone());
            }
        }

        return self;
    }
}

/// A list of Oses for error handling purposes.
#[derive(Debug)]
pub enum Oses {
    Windows,
    Linux,
    MacOs,
    OtherUnknownOs,
}

/// Implement display for Oses.
impl Display for Oses {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
