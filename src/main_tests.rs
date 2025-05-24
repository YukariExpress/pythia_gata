// src/main_tests.rs
// Unit tests for pythia_gata core logic

use super::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_rng_determinism() {
        let rng1 = new_rng(12345, "test");
        let rng2 = new_rng(12345, "test");
        // The first value from both RNGs should be the same
        assert_eq!(rng1.clone().next_u64(), rng2.clone().next_u64());
    }

    #[test]
    fn test_pia_format() {
        let mut rng = new_rng(1, "hello");
        let result = pia("hello", &mut rng);
        assert!(result.starts_with("Pia!"));
        assert!(result.ends_with("hello"));
    }

    #[test]
    fn test_divine_output() {
        let mut rng = new_rng(2, "question");
        let result = divine("question", &mut rng);
        assert!(result.contains("所求事项: question"));
        assert!(result.contains("结果: "));
    }
}
