
#[derive(Debug)]
pub struct Triangle {
    sides: (u32, u32, u32),
    pub kind: TriangleKind
}

#[derive(Debug)]
pub enum Error {
    InvalidTriangle,
    // ..
}

#[derive(Debug, PartialEq, Eq)]
pub enum TriangleKind {
    Equilateral,
    Isosceles,
    Scalene
}

pub type Result<T> = std::result::Result<T, Error>;

impl Triangle {

    fn classify(sides: (u32, u32, u32)) -> TriangleKind {
        match sides {
            (a, b, c) if a == b && b == c => TriangleKind::Equilateral,
            (a, b, c) if a != b && b != c &&  a != c => TriangleKind::Scalene,
            _ => TriangleKind::Isosceles
        }
    }

    fn is_valid(a: u32, b: u32, c: u32) -> bool {
        a + b > c && a + c > b && b + c > a
    }

    pub fn new(a: u32, b: u32, c: u32) -> Result<Self> {
        let sides = (a, b, c);
        match Self::is_valid(a, b, c) {
            false => Err(Error::InvalidTriangle),
            true => Ok(Self {
                sides,
                kind: Self::classify(sides)
            })
        }
    }

    pub fn from_triplet(sides: (u32, u32, u32)) -> Result<Self> {
        let (a,b,c) = sides;
        Self::new(a, b, c)
    }
}
