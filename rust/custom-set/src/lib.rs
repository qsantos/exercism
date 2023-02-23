#[derive(Debug, Eq)]
pub struct CustomSet<T: Copy + Eq> {
    elements: Vec<T>,
}

impl<T: Copy + Eq> CustomSet<T> {
    pub fn new(input: &[T]) -> Self {
        CustomSet {
            elements: Vec::from(input),
        }
    }

    pub fn contains(&self, element: &T) -> bool {
        self.elements.contains(element)
    }

    pub fn add(&mut self, element: T) {
        if !self.contains(&element) {
            self.elements.push(element);
        }
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        for element in &self.elements {
            if !other.contains(element) {
                return false;
            }
        }
        true
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        for element in &self.elements {
            if other.contains(element) {
                return false;
            }
        }
        for element in &other.elements {
            if self.contains(element) {
                return false;
            }
        }
        true
    }

    #[must_use]
    pub fn intersection(&self, other: &Self) -> Self {
        let mut elements = Vec::new();
        for &element in &self.elements {
            if other.contains(&element) {
                elements.push(element);
            }
        }
        Self { elements }
    }

    #[must_use]
    pub fn difference(&self, other: &Self) -> Self {
        let mut elements = Vec::new();
        for &element in &self.elements {
            if !other.contains(&element) {
                elements.push(element);
            }
        }
        Self { elements }
    }

    #[must_use]
    pub fn union(&self, other: &Self) -> Self {
        let mut ret = Self::new(&self.elements);
        for &element in &other.elements {
            ret.add(element);
        }
        ret
    }
}

impl<T: Copy + Eq> PartialEq for CustomSet<T> {
    fn eq(&self, other: &Self) -> bool {
        self.is_subset(other) && other.is_subset(self)
    }
}
