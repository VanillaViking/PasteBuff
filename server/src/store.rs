
struct KVStore<T: Clone> {
    buffer: Vec<Option<Selection<T>>>,
    capacity: usize,
}

#[derive(Debug, Clone)]
struct Selection<T: Clone> {
    value: T,
    next: usize,
    prev: usize,
}

impl<T: Clone> KVStore<T> {
    fn new(capacity: usize) -> Self {
        KVStore { buffer: vec![None; capacity], capacity } 
    }
}
