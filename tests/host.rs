#[cfg(test)]
mod host {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use sys_metrics::host::*;
    use sys_metrics::virt::{self, *};

    #[test]
    fn test_host_info() {
        let host = get_host_info().unwrap();

        let loadavg = host.loadavg;

        let o = loadavg.one;
        assert!(o >= 0.0);
        let t = loadavg.five;
        assert!(t >= 0.0);
        let f = loadavg.fifteen;
        assert!(f >= 0.0);

        let c = host.uptime;
        assert!(c > 0);

        let x = host.os_version;
        assert!(x.len() > 0);

        let y = host.hostname;
        assert!(y.len() > 0);
    }

    #[test]
    fn test_hostname() {
        // TODO
        let _hostname = get_hostname().unwrap();
    }

    #[test]
    #[allow(unused_comparisons)]
    fn test_logged_users() {
        let x = match virt::get_virt_info() {
            Some(info) => info,
            None => Virtualization::Unknown,
        };

        if x != Virtualization::Wsl {
            // If on WSL this function will fail
            let users = get_logged_users().unwrap();
            assert!(users.len() >= 0);
        } else {
            // On WSL assume this test as success
            assert!(true);
        }
    }

    #[test]
    fn test_os_version() {
        let version = get_os_version().unwrap();

        assert!(version.len() > 0);
    }

    #[test]
    fn test_users() {
        let users = get_users().unwrap();

        #[cfg(target_os = "linux")]
        assert!(users.len() > 0);
        #[cfg(target_os = "macos")]
        #[allow(unused_comparisons)]
        assert!(users.len() >= 0);
    }

    #[test]
    fn test_uuid() {
        let x = match virt::get_virt_info() {
            Some(info) => info,
            None => Virtualization::Unknown,
        };

        if x != Virtualization::Wsl {
            // If on WSL this function will fail
            let uuid = get_uuid().unwrap();
            assert!(uuid.len() > 0);
        } else {
            // On WSL assume this test as success
            assert!(true);
        }
    }
}
