#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use sys_metrics::disks::*;

    #[test]
    fn test_partitions_physical() {
        let partitions = get_partitions_physical().unwrap();

        assert!(partitions.len() > 0);

        for p in partitions {
            let _ = p.name;
            let _ = p.mount_point;
            let _ = p.total_space;
            let _ = p.avail_space;
        }
    }

    #[test]
    fn test_physical_ioblocks() {
        let stats = get_physical_ioblocks().unwrap();

        assert!(stats.len() > 0);

        for s in stats {
            let _ = s.device_name;
            let _ = s.read_count;
            let _ = s.write_bytes;
        }
    }
}
