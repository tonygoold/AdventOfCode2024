use std::ops::Index;

pub struct RLArray<T>(Vec<(T, usize)>);

impl<T> RLArray<T> {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn reserve(&mut self, additional: usize) {
        self.0.reserve(additional);
    }
}

impl<T> Default for RLArray<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Index<usize> for RLArray<T> {
    type Output = (T, usize);

    fn index(&self, index: usize) -> &Self::Output {
        self.0.index(index)
    }
}

impl<T> IntoIterator for RLArray<T> {
    type Item = (T, usize);
    type IntoIter = <Vec<Self::Item> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<T: Eq> RLArray<T> {
    pub fn push(&mut self, item: (T, usize)) -> usize {
        self.insert(self.len(), item)
    }

    pub fn insert(&mut self, index: usize, item: (T, usize)) -> usize {
        // If same as previous, merge items
        if index > 0 && self.0[index - 1].0 == item.0 {
            self.0[index - 1].1 += item.1;
            return index - 1;
        }

        // If same as next, merge items
        if index < self.0.len() && self.0[index].0 == item.0 {
            self.0[index].1 += item.1;
        } else {
            self.0.insert(index, item);
        }
        index
    }

    pub fn replace(&mut self, index: usize, item: (T, usize)) -> usize {
        // If same as previous, merge items
        if index > 0 && self.0[index - 1].0 == item.0 {
            self.0[index - 1].1 += item.1;
            // If also same as next, merge items again
            if index + 1 < self.0.len() && self.0[index + 1].0 == item.0 {
                self.0[index - 1].1 += self.0[index + 1].1;
                // Suboptimal due to moving the tail twice
                self.0.remove(index + 1);
                self.0.remove(index);
            } else {
                self.0.remove(index);
            }
            return index - 1;
        }

        // If same as next, merge items
        if index + 1 < self.0.len() && self.0[index + 1].0 == item.0 {
            self.0[index + 1].1 += item.1;
            self.0.remove(index);
        } else {
            self.0[index] = item;
        }
        index
    }

    pub fn remove(&mut self, index: usize) {
        self.0.remove(index);
        // If same previous and next, merge items
        if index > 0 && index < self.0.len() && self.0[index - 1].0 == self.0[index].0 {
            self.0[index - 1].1 += self.0[index].1;
            self.0.remove(index);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_empty() {
        let mut arr = RLArray::<char>::new();
        arr.insert(0, ('a', 3));
        assert_eq!(arr.len(), 1);
        assert_eq!(arr[0], ('a', 3));
    }

    #[test]
    fn test_insert_different() {
        let mut arr = RLArray::<char>::new();
        arr.insert(0, ('a', 3));
        arr.insert(0, ('b', 5));
        assert_eq!(arr.len(), 2);
        assert_eq!(arr[0], ('b', 5));
        assert_eq!(arr[1], ('a', 3));
    }

    #[test]
    fn test_insert_next_same() {
        let mut arr = RLArray::<char>::new();
        arr.insert(0, ('a', 3));
        let index = arr.insert(1, ('a', 5));
        assert_eq!(index, 0);
        assert_eq!(arr.len(), 1);
        assert_eq!(arr[0], ('a', 8));
    }

    #[test]
    fn test_insert_prev_same() {
        let mut arr = RLArray::<char>::new();
        arr.insert(0, ('a', 3));
        arr.insert(0, ('a', 5));
        assert_eq!(arr.len(), 1);
        assert_eq!(arr[0], ('a', 8));
    }

    #[test]
    fn test_replace_different() {
        let mut arr = RLArray::<char>::new();
        arr.insert(0, ('a', 3));
        arr.insert(1, ('b', 5));
        arr.insert(2, ('c', 7));
        arr.replace(1, ('d', 9));
        assert_eq!(arr.len(), 3);
        assert_eq!(arr[0], ('a', 3));
        assert_eq!(arr[1], ('d', 9));
        assert_eq!(arr[2], ('c', 7));
    }

    #[test]
    fn test_replace_prev_same() {
        let mut arr = RLArray::<char>::new();
        arr.insert(0, ('a', 3));
        arr.insert(1, ('b', 5));
        arr.insert(2, ('c', 7));
        let index = arr.replace(1, ('a', 9));
        assert_eq!(index, 0);
        assert_eq!(arr.len(), 2);
        assert_eq!(arr[0], ('a', 12));
        assert_eq!(arr[1], ('c', 7));
    }

    #[test]
    fn test_replace_next_same() {
        let mut arr = RLArray::<char>::new();
        arr.insert(0, ('a', 3));
        arr.insert(1, ('b', 5));
        arr.insert(2, ('c', 7));
        let index = arr.replace(1, ('c', 9));
        assert_eq!(index, 1);
        assert_eq!(arr.len(), 2);
        assert_eq!(arr[0], ('a', 3));
        assert_eq!(arr[1], ('c', 16));
    }

    #[test]
    fn test_remove_different() {
        let mut arr = RLArray::<char>::new();
        arr.insert(0, ('a', 3));
        arr.insert(1, ('b', 5));
        arr.insert(2, ('c', 7));
        arr.remove(1);
        assert_eq!(arr.len(), 2);
        assert_eq!(arr[0], ('a', 3));
        assert_eq!(arr[1], ('c', 7));
    }

    #[test]
    fn test_remove_prev_next_same() {
        let mut arr = RLArray::<char>::new();
        arr.insert(0, ('a', 3));
        arr.insert(1, ('b', 5));
        arr.insert(2, ('a', 7));
        arr.remove(1);
        assert_eq!(arr.len(), 1);
        assert_eq!(arr[0], ('a', 10));
    }
}
