use super::*;

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let x = Hex { q: 1, r: 1, s: -2 };
        let y = Hex { q: 2, r: 3, s: -5 };

        let z = x + y;

        assert_eq!(z.q, 3)
    }
}
