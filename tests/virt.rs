#[cfg(test)]
mod virt {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use sys_metrics::virt::*;

    #[test]
    #[allow(unused_comparisons)]
    fn test_virt() {
        let _ = get_virt_info();
    }
}
