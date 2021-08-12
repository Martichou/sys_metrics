#[cfg(test)]
#[allow(unused_comparisons)]
mod disks {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use sys_metrics::disks::*;

    #[test]
    fn test_partitions_physical() {
        let partitions = get_partitions_physical().unwrap();

        assert!(partitions.len() > 0);
    }

    #[test]
    fn test_physical_ioblocks() {
        let stats = get_physical_ioblocks().unwrap();

        assert!(stats.len() > 0);
    }
}
