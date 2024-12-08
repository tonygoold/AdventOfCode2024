pub fn remove_suffix(val: isize, suffix: isize) -> Option<isize> {
    if suffix <= 0 {
        return None;
    }
    let modulus = 10isize.pow(suffix.ilog10() + 1);
    if val % modulus == suffix {
        Some(val / modulus)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_suffix_good() {
        let result = remove_suffix(1299, 99);
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_suffix_bad() {
        let result = remove_suffix(1298, 99);
        assert!(result.is_none());
    }
}
