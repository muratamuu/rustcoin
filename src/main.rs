mod ecc;

fn main() {
    let element = ecc::FieldElement::new(13, 14);
    println!("{:?}", element);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
