#[cfg(test)]
mod tests {
    #[test]
    fn test_it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_multiplication() {
        assert_eq!(3 * 3, 9);
    }

    #[test]
    fn test_pifagor() {
        let a = 3u32;
        let b = 4u32;
        let c = 5u32;
        let d = 6u32;
        assert_eq!(a.pow(2) + b.pow(2), c.pow(2));
    }
}