#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use sys_metrics::memory::*;

    #[test]
    fn test_memory() {
        let mem = get_memory().unwrap();

        let _ = mem.avail_swap;
        let _ = mem.avail_virt;
        let _ = mem.total_swap;

        let x = mem.total_virt;
        assert!(x > 0);
    }
}
