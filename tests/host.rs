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

    #[test]
    fn test_hostname() {
        // TODO
        let _hostname = get_hostname().unwrap();
    }

    #[test]
    fn test_users() {
        // TODO
        let _users = get_users();
    }
}
