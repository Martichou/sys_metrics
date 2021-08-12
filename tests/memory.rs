#[cfg(test)]
#[allow(unused_comparisons)]
mod memory {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use sys_metrics::memory::*;

    #[test]
    fn test_memory() {
        let mem = get_memory().unwrap();

        assert!(mem.total > 0);
        assert!(mem.free >= 0);
        assert!(mem.used >= 0);
    }

    #[test]
    fn test_has_swap() {
        let _ = has_swap().unwrap();
    }

    #[test]
    fn test_swap() {
        if has_swap().unwrap() {
            let mem = get_swap().unwrap();

            assert!(mem.total >= 0);
            assert!(mem.free >= 0);
            assert!(mem.used >= 0);
        } else {
            assert!(true);
        }
    }
}
