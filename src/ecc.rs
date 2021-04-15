use std::fmt;

pub struct FieldElement {
    num: i32,
    prime: i32,
}

impl FieldElement {
    pub fn new(num: i32, prime: i32) -> Result<Self, String> {
        if num >= prime || num < 0 {
            Err(format!("Num {} not in field range 0 to {}", num, prime - 1))
        } else {
            Ok(FieldElement { num, prime })
        }
    }
}

impl fmt::Debug for FieldElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FieldElement_{}({})", self.prime, self.num)
    }
}
