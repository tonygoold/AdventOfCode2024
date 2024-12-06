use std::collections::{HashMap, HashSet};
use std::hash::Hash;

pub struct Lattice<T>
where
    T: Eq + Hash,
{
    rels: HashMap<T, HashSet<T>>,
}

impl<T: Hash + Eq> Default for Lattice<T> {
    fn default() -> Self {
        Lattice::new()
    }
}

impl<T: Hash + Eq> Lattice<T> {
    pub fn new() -> Self {
        Lattice {
            rels: HashMap::new(),
        }
    }

    pub fn insert(&mut self, a: T, b: T) {
        self.rels.entry(a).or_default().insert(b);
    }

    pub fn remove(&mut self, a: T, b: T) {
        if let Some(vals) = self.rels.get_mut(&a) {
            if vals.remove(&b) && vals.is_empty() {
                self.rels.remove(&a);
            }
        }
    }

    pub fn has(&self, a: &T, b: &T) -> bool {
        self.rels.get(a).map_or(false, |vals| vals.contains(b))
    }

    pub fn has_transitive(&self, a: &T, b: &T) -> bool {
        if let Some(vals) = self.rels.get(a) {
            if vals.contains(b) {
                return true;
            }
            vals.iter().any(|val| self.has(val, b))
        } else {
            false
        }
    }

    pub fn is_total_over(&self, v: &[T]) -> bool {
        v.iter().enumerate().all(|(index, a)| {
            v.iter()
                .skip(index + 1)
                .all(|b| self.has(a, b) || self.has(b, a))
        })
    }
}
