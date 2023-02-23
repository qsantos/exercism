pub struct CircularBuffer<T> {
    start: usize,
    n_elements: usize,
    elements: Vec<T>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T: Default> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        let mut elements = Vec::new();
        elements.resize_with(capacity, || T::default());
        CircularBuffer {
            start: 0,
            n_elements: 0,
            elements,
        }
    }

    pub fn write(&mut self, element: T) -> Result<(), Error> {
        if self.n_elements == self.elements.len() {
            return Err(Error::FullBuffer);
        }
        let pos = (self.start + self.n_elements) % self.elements.len();
        self.elements[pos] = element;
        self.n_elements += 1;
        Ok(())
    }

    pub fn read(&mut self) -> Result<T, Error> {
        if self.n_elements == 0 {
            return Err(Error::EmptyBuffer);
        }
        let ret = std::mem::take(&mut self.elements[self.start]);
        self.start = (self.start + 1) % self.elements.len();
        self.n_elements -= 1;
        Ok(ret)
    }

    pub fn clear(&mut self) {
        let capacity = self.elements.len();
        self.start = 0;
        self.n_elements = 0;
        self.elements.clear();
        self.elements.resize_with(capacity, || T::default());
    }

    pub fn overwrite(&mut self, element: T) {
        let pos = (self.start + self.n_elements) % self.elements.len();
        self.elements[pos] = element;
        if self.n_elements == self.elements.len() {
            self.start = (self.start + 1) % self.elements.len();
        } else {
            self.n_elements += 1;
        }
    }
}
