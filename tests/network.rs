#[cfg(test)]
#[allow(unused_comparisons)]
mod network {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use sys_metrics::network::*;

    #[test]
    fn test_ionets() {
        #[cfg(target_os = "linux")]
        let ionets = get_physical_ionets().unwrap();
        #[cfg(target_os = "macos")]
        let ionets = get_ionets().unwrap();

        assert!(!ionets.is_empty());
    }
}
