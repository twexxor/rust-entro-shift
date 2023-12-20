pub mod entro_shift;

#[cfg(test)]
mod test {
    use entro_shift::entro_shift;

    #[test]
    fn test_entro_shift() {
        let mut entropy = entro_shift(1111111111);
        let mut i: usize = 0;

        while i != 10 {
            entropy = entro_shift(entropy);
            i += 1;
        }

        assert_eq!(3963250355, entropy);
    }
}
