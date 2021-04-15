#[derive(PartialEq)]
pub struct FieldElement {
    num: i32,
    prime: i32,
}

impl FieldElement {
    pub fn new(num: i32, prime: i32) -> Self {
        assert!(num < prime && num >= 0, "Num {} not in field range 0 to {}", num, prime - 1);
        FieldElement { num, prime }
    }
}

impl std::fmt::Debug for FieldElement {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "FieldElement_{}({})", self.prime, self.num)
    }
}

impl std::ops::Add for FieldElement {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        assert!(self.prime == other.prime, "Cannot add two numbers in different Fields");
        let num = (self.num + other.num) % self.prime;
        FieldElement { num, prime: self.prime }
    }
}

impl std::ops::Sub for FieldElement {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        assert!(self.prime == other.prime, "Cannot subtraction two numbers in different Fields");
        let mut num = (self.num - other.num) % self.prime;
        num = if num < 0 { num + self.prime } else { num };
        FieldElement { num, prime: self.prime }
    }
}

impl std::ops::Mul for FieldElement {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        assert!(self.prime == other.prime, "Cannot multiply two numbers in different Fields");
        let num = (self.num * other.num) % self.prime;
        FieldElement { num, prime: self.prime }
    }
}

impl std::ops::Div for FieldElement {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        assert!(self.prime == other.prime, "Cannot divide two numbers in different Fields");
        let mut num = Self::powmod(other.num, (self.prime - 2) as u32, self.prime);
        num = (self.num * num) % self.prime;
        FieldElement { num, prime: self.prime }
    }
}

impl FieldElement {
    pub fn pow(&self, exponent: i32) -> Self {
        let mut n = exponent % (self.prime - 1);
        n = if n < 0 { n + (self.prime - 1) } else { n };
        let num = Self::powmod(self.num, n as u32, self.prime);
        FieldElement { num, prime: self.prime }
    }

    pub fn powmod(n: i32, exponent: u32, modulo: i32) -> i32 {
        let mut ret = 1;
        for _ in 0..exponent {
            ret = ret * n % modulo;
        }
        return ret;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn equality() {
        let a = super::FieldElement::new(7, 13);
        let b = super::FieldElement::new(7, 13);
        assert_eq!(a, b);
    }

    #[test]
    fn not_equality() {
        let a = super::FieldElement::new(7, 13);
        let b = super::FieldElement::new(8, 13);
        assert_ne!(a, b);
    }

    #[test]
    fn add() {
        let a = super::FieldElement::new( 7, 13);
        let b = super::FieldElement::new(12, 13);
        let c = super::FieldElement::new( 6, 13);
        assert!(a + b == c);
    }

    #[test]
    fn sub() {
        let a = super::FieldElement::new( 9, 57);
        let b = super::FieldElement::new(29, 57);
        let c = super::FieldElement::new(37, 57);
        assert!(a - b == c);
    }

    #[test]
    fn mul() {
        let a = super::FieldElement::new(95, 97);
        let b = super::FieldElement::new(45, 97);
        let c = super::FieldElement::new( 7, 97);
        assert!(a * b == c);
    }

    #[test]
    fn pow_1() {
        let a = super::FieldElement::new(3, 13);
        let b = super::FieldElement::new(1, 13);
        assert!(a.pow(3) == b);
    }

    #[test]
    fn pow_2() {
        let a = super::FieldElement::new(12, 97);
        let b = super::FieldElement::new(77, 97);
        let c = super::FieldElement::new(63, 97);
        assert!(a.pow(7) * b.pow(49) == c);
    }

    #[test]
    fn neg_pow_1() {
        let a = super::FieldElement::new(7, 13);
        let b = super::FieldElement::new(8, 13);
        assert!(a.pow(-3) == b);
    }

    #[test]
    fn neg_pow_2() {
        let a = super::FieldElement::new(17, 31);
        let b = super::FieldElement::new(29, 31);
        assert!(a.pow(-3) == b);
    }

    #[test]
    fn neg_pow_3() {
        let a = super::FieldElement::new( 4, 31);
        let b = super::FieldElement::new(11, 31);
        let c = super::FieldElement::new(13, 31);
        assert!(a.pow(-4) * b == c);
    }

    #[test]
    fn truediv() {
        let a = super::FieldElement::new( 3, 31);
        let b = super::FieldElement::new(24, 31);
        let c = super::FieldElement::new( 4, 31);
        assert!(a / b == c);
    }

}
