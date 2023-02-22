pub struct Triangle {
    sides: [u64; 3],
}

impl Triangle {
    pub fn build(mut sides: [u64; 3]) -> Option<Triangle> {
        sides.sort();
        let [a, b, c] = sides;
        if a != 0 && a + b >= c {
            Some(Triangle { sides })
        } else {
            None
        }
    }

    pub fn is_equilateral(&self) -> bool {
        let [a, b, c] = self.sides;
        a == b && b == c
    }

    pub fn is_scalene(&self) -> bool {
        !self.is_isosceles()
    }

    pub fn is_isosceles(&self) -> bool {
        let [a, b, c] = self.sides;
        a == b || b == c || c == a
    }
}
