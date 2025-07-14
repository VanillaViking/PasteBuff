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

    fn array_insert(&mut self, value: T) -> usize {
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
        let curr = self.buffer.get(idx).ok_or(StoreArrayError::NotFound(idx))?.as_ref().ok_or(StoreArrayError::NotFound(idx))?;


        Ok(())
        
    }
}
