#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use sys_metrics::cpu::*;

    #[test]
    fn test_cpufreq() {
        let cpufreq = get_cpufreq().unwrap();

        assert!(cpufreq > 0.0);
    }

    #[test]
    fn test_cpustats() {
        let cpustats = get_cpustats().unwrap();

        assert!(cpustats.user >= 0);
    }

    #[test]
    fn test_loadavg() {
        let loadavg = get_loadavg().unwrap();

        let o = loadavg.one;
        assert!(o >= 0.0);
        let t = loadavg.five;
        assert!(t >= 0.0);
        let f = loadavg.fifteen;
        assert!(f >= 0.0);
    }
}
