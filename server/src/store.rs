use std::{collections::HashMap, hash::Hash};

use thiserror::Error;

#[derive(Debug, Clone)]
pub struct KVStore<K, V>
where
    V: Clone,
    K: Eq + Hash
{
    buffer: Vec<Option<Clipboard<V>>>,
    map: HashMap<K, usize>,
    capacity: usize,
    length: usize,
    head: usize,
    tail: usize,
}

#[derive(Debug, Clone)]
pub struct Clipboard<T>
where
    T: Clone,
{
    value: T,
    next: Option<usize>,
    prev: Option<usize>,
}

#[derive(Debug, Error)]
pub enum StoreArrayError {
    #[error("value at {0} was not found")]
    NotFound(usize),
}

impl<K, V> KVStore<K, V>
where
    V: Clone,
    K: Eq + Hash,
{
    pub fn new(capacity: usize) -> Self {
        KVStore {
            buffer: vec![None; capacity],
            map: HashMap::with_capacity(capacity),
            capacity,
            length: 0,
            head: 0,
            tail: 0,
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        if let Some(idx) = self.map.get(&key) {
            self.buffer[*idx].as_mut().expect("should exist").value = value;
        } else {
            let val_idx = self.array_insert(value);
            self.map.insert(key, val_idx);
        }
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        let array_idx = self.map.get(key)?.clone();
        self.touch(array_idx).ok()?;
        self.array_get(array_idx)
    }

    fn array_insert(&mut self, value: V) -> usize {
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

    fn array_get(&self, idx: usize) -> Option<&V> {
        Some(&self.buffer.get(idx)?.as_ref()?.value)
    }

    fn touch(&mut self, idx: usize) -> Result<(), StoreArrayError> {
        if idx == self.tail {
            // nothing to do
            return Ok(());
        }

        let curr = self
            .buffer
            .get(idx)
            .ok_or(StoreArrayError::NotFound(idx))?
            .as_ref()
            .ok_or(StoreArrayError::NotFound(idx))?;
        let prev_opt = curr.prev.clone();
        let next_opt = curr.next.clone();
        if let Some(p) = prev_opt {
            self.buffer
                .get_mut(p)
                .expect("checked")
                .as_mut()
                .expect("checked")
                .next = next_opt;
        }

        if let Some(n) = next_opt {
            self.buffer
                .get_mut(n)
                .expect("checked")
                .as_mut()
                .expect("checked")
                .prev = prev_opt;
        }

        let curr = self
            .buffer
            .get_mut(idx)
            .expect("checked")
            .as_mut()
            .expect("checked");
        curr.next = None;
        curr.prev = Some(self.tail);

        self.buffer
            .get_mut(self.tail)
            .expect("tail should exist")
            .as_mut()
            .expect("tail should exist")
            .next = Some(idx);
        self.tail = idx;

        Ok(())
    }
}
