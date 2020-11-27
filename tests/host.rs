#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use sys_metrics::host::*;

    #[test]
    fn test_host_info() {
        let host = get_host_info().unwrap();

        {
            let loadavg = host.loadavg;

            let o = loadavg.one;
            assert!(o >= 0.0);
            let t = loadavg.five;
            assert!(t >= 0.0);
            let f = loadavg.fifteen;
            assert!(f >= 0.0);
        }

        {
            let mem = host.memory;

            let _ = mem.avail_swap;
            let _ = mem.avail_virt;
            let _ = mem.total_swap;
            let x = mem.total_virt;
            assert!(x > 0);
        }

        let c = host.uptime;
        assert!(c > 0);

        let _ = host.os_version;
        let _ = host.hostname;
    }

    #[test]
    fn test_uuid() {
        let uuid = get_uuid().unwrap();

        assert!(uuid.len() > 0);
    }
}
