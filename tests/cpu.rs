#[cfg(test)]
#[allow(unused_comparisons)]
mod cpu {
    use sys_metrics::cpu::*;

    #[test]
    fn test_cpucorecount() {
        let logical_count = get_logical_count().unwrap();
        assert!(logical_count > 0);
        assert!(logical_count < 1024);

        let physical_count = get_physical_count().unwrap();
        assert!(physical_count > 0);
        assert!(physical_count < 1024);
    }

    #[test]
    fn test_cpufreq() {
        let cpufreq = get_cpufreq().unwrap();

        assert!(cpufreq > 0.0);
    }

    #[test]
    fn test_cputimes() {
        let cputimes = get_cputimes().unwrap();

        assert!(cputimes.total_time() >= 0);
        assert!(cputimes.busy_time() >= 0);
        assert!(cputimes.idle_time() >= 0);
    }

    #[test]
    fn test_cpustats() {
        let cpustats = get_cpustats().unwrap();

        assert!(cpustats.interrupts >= 0);
        assert!(cpustats.ctx_switches >= 0);
        assert!(cpustats.soft_interrupts >= 0);
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
