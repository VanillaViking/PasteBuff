use thiserror::Error;

#[derive(Debug, Clone)]
pub struct KVStore<T: Clone> {
    buffer: Vec<Option<Clipboard<T>>>,
    capacity: usize,
    length: usize,
    head: usize,
    tail: usize,
}

#[derive(Debug, Clone)]
pub struct Clipboard<T: Clone> {
    value: T,
    next: Option<usize>,
    prev: Option<usize>,
}

#[derive(Debug, Error)]
pub enum StoreArrayError {
    #[error("value at {0} was not found")]
    NotFound(usize)  
}

impl<T: Clone> KVStore<T> {
    pub fn new(capacity: usize) -> Self {
        KVStore {
            buffer: vec![None; capacity],
            capacity,
            length: 0,
            head: 0,
            tail: 0,
        }
    }

    pub fn array_insert(&mut self, value: T) -> usize {
        if self.buffer[self.head].is_none() {
            self.buffer[self.head] = Some(Clipboard {
                value,
                next: None,
                prev: None,
            });
            self.length += 1;
            return self.head;
        }
        if self.length < self.capacity {
            self.buffer[self.tail]
                .as_mut()
                .expect("tail should not be None")
                .next = Some(self.tail + 1);
            self.buffer[self.tail + 1] = Some(Clipboard {
                value,
                next: None,
                prev: Some(self.tail),
            });
            self.tail += 1;
            self.length += 1;
        } else {
            let next_head = self.buffer[self.head]
                .as_ref()
                .expect("head exists")
                .next
                .expect("capacity must be greater than 1");
            self.buffer[self.head] = Some(Clipboard {
                value,
                next: None,
                prev: Some(self.tail),
            });
            self.tail = self.head;
            self.head = next_head;
        }
        self.tail
    }

    pub fn array_get(&self, idx: usize) -> Option<&T> {
        Some(&self.buffer.get(idx)?.as_ref()?.value)
        
    }

    pub fn touch(&mut self, idx: usize) -> Result<(), StoreArrayError> {
        if idx == self.tail {
            // nothing to do
            return Ok(())
        }

        let curr = self.buffer.get(idx).ok_or(StoreArrayError::NotFound(idx))?.as_ref().ok_or(StoreArrayError::NotFound(idx))?;
        let prev_opt = curr.prev.clone();
        let next_opt = curr.next.clone();
        if let Some(p) = prev_opt {
            self.buffer.get_mut(p).expect("checked").as_mut().expect("checked").next = next_opt;
        }

        if let Some(n) = next_opt {
            self.buffer.get_mut(n).expect("checked").as_mut().expect("checked").prev = prev_opt;
        }
        
        let curr = self.buffer.get_mut(idx).expect("checked").as_mut().expect("checked");
        curr.next = None;
        curr.prev = Some(self.tail);

        self.buffer.get_mut(self.tail).expect("tail should exist").as_mut().expect("tail should exist").next = Some(idx);
        self.tail = idx;

        Ok(())
        
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_get() {
        let mut store = KVStore::new(3);

        let idx1 = store.array_insert(10);
        let idx2 = store.array_insert(20);
        let idx3 = store.array_insert(30);

        assert_eq!(store.array_get(idx1), Some(&10));
        assert_eq!(store.array_get(idx2), Some(&20));
        assert_eq!(store.array_get(idx3), Some(&30));
    }

    #[test]
    fn test_insert_over_capacity() {
        let mut store = KVStore::new(2);

        let idx1 = store.array_insert(1);
        let idx2 = store.array_insert(2);

        // Inserting third value should overwrite the head
        let idx3 = store.array_insert(3);

        assert_eq!(idx3, idx1);
        assert_eq!(store.array_get(idx3), Some(&3));
    }

    #[test]
    fn test_touch_moves_to_tail() {
        let mut store = KVStore::new(3);

        let idx1 = store.array_insert("A");
        let idx2 = store.array_insert("B");
        let idx3 = store.array_insert("C");

        // Initial order: A (head) -> B -> C (tail)

        store.touch(idx1).unwrap();

        // After touch(idx1), idx1 becomes the new tail.
        assert_eq!(store.tail, idx1);

        // Data remains correct
        assert_eq!(store.array_get(idx1), Some(&"A"));
        assert_eq!(store.array_get(idx2), Some(&"B"));
        assert_eq!(store.array_get(idx3), Some(&"C"));
    }

    #[test]
    fn test_touch_tail_is_noop() {
        let mut store = KVStore::new(3);

        let idx1 = store.array_insert("A");
        let idx2 = store.array_insert("B");
        let idx3 = store.array_insert("C");

        // Current tail is idx3
        assert_eq!(store.tail, idx3);

        let res = store.touch(idx3);
        assert!(res.is_ok());
        // Tail should remain unchanged
        assert_eq!(store.tail, idx3);
    }

    #[test]
    fn test_touch_invalid_index() {
        let mut store = KVStore::new(3);
        store.array_insert(1);
        store.array_insert(2);

        let res = store.touch(999);
        assert!(matches!(res, Err(StoreArrayError::NotFound(999))));
    }

    #[test]
    fn test_get_invalid_index() {
        let store = KVStore::<i32>::new(2);

        assert_eq!(store.array_get(5), None);
    }
}
