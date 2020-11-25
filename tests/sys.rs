#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use sys_metrics::sys::*;

    #[test]
    fn test_uname() {
        let _ = get_uname().unwrap();
    }

    #[test]
    fn test_os_version() {
        let uname = get_uname().unwrap();

        let os_version = get_os_version().unwrap();
        let os_version_uname = get_os_version_from_uname(&uname);

        assert_eq!(os_version, os_version_uname);
    }

    #[test]
    fn test_hostname() {
        let uname = get_uname().unwrap();

        let hostname = get_hostname().unwrap();
        let hostname_uname = get_hostname_from_uname(&uname);

        assert_eq!(hostname, hostname_uname);
    }
}
