#[cfg(test)]
mod network {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use sys_metrics::network::*;

    #[test]
    #[allow(unused_comparisons)]
    fn test_ionets() {
        #[cfg(target_os = "linux")]
        let ionets = get_physical_ionets().unwrap();
        #[cfg(target_os = "macos")]
        let ionets = get_ionets().unwrap();

        assert!(ionets.len() > 0);
    }
}
