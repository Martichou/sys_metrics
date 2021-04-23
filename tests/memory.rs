#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use sys_metrics::memory::*;

    #[test]
    fn test_memory() {
        let _mem = get_memory().unwrap();
        let _swap = get_swap().unwrap();

        assert!(true);
    }
}
