#[derive(Clone)]
pub struct RingBuffer<T> {
    values: Vec<T>,
    cursor: usize,
}

pub struct RingBufferIter<'a, T: 'a> {
    buffer: &'a RingBuffer<T>,
    cursor: usize,
    iter_count: usize,
}

impl<T: Clone> RingBuffer<T> {
    pub fn new() -> Self {
        Self { values: Vec::new(), cursor: 0 }
    }

    pub fn from_vec(values: Vec<T>) -> Self {
        Self { cursor: 0, values }
    }

    pub fn insert(&mut self, value: T) {
        self.forward();
        self.values.insert(self.cursor, value);
    }

    pub fn forward(&mut self) {
        self.cursor += 1;
        if self.cursor >= self.values.len() {
            self.cursor = 0;
        }
    }

    pub fn back(&mut self) {
        if self.cursor as i64 - 1 < 0 {
            self.cursor = self.values.len() - 1;
        } else {
            self.cursor -= 1;
        }
    }

    pub fn clone_current(&self) -> Option<T> {
        self.current().map(|s| s.clone())
    }

    pub fn current<'a>(&'a self) -> Option<&'a T> {
        match self.values.len() {
            0 => None,
            _ => Some(&self.values[self.cursor]),
        }
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn iter<'a>(&'a self) -> RingBufferIter<'a, T> {
        RingBufferIter { buffer: self, cursor: self.cursor, iter_count: 0 }
    }
}


impl<'a, T: Clone> RingBufferIter<'a, T> {
    pub fn forward(&mut self) {
        self.iter_count += 1;
        self.cursor += 1;
        if self.cursor >= self.buffer.values.len() {
            self.cursor = 0;
        }
    }

    pub fn current(&'a self) -> Option<&'a T> {
        if self.cursor >= self.buffer.values.len() {
            None
        } else {
            Some(&self.buffer.values[self.cursor])
        }
    }
}

impl<'a, T: Clone> Iterator for RingBufferIter<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.iter_count == self.buffer.len() { return None }
        let value = self.current().map(|c| c.clone());
        self.forward();
        value
    }
}
