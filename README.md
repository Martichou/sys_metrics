sys_metrics
========
[![crates.io](https://img.shields.io/crates/v/sys_metrics.svg)](https://crates.io/crates/sys_metrics)
[![Docs.rs](https://docs.rs/sys_metrics/badge.svg)](https://docs.rs/sys_metrics)
[![AGPL License](https://img.shields.io/badge/license-AGPL-blue.svg)](LICENSE)
[![CI](https://github.com/Martichou/sys_metrics/workflows/CI/badge.svg)](https://github.com/Martichou/sys_metrics/actions)

`sys_metrics` is a WIP project intended to give an alternative to others tools which can be slower or provide too many useless informations.
It will try to have at least the same functionality as [psutil](https://github.com/giampaolo/psutil) or [heim](https://github.com/heim-rs/heim).

It's a synchronous library which try to be freaking fast (maybe even the fastest) and as dependency-free as possible.

WIP Notes
--------------------------

`sys_metrics` in it's WIP stage will only support Linux and macOS.
The structure is subject to change from version to version. 
If you have a comment about it or anything else feel free to open an issue.

Usage
--------------------------

Add the dependency to your `Cargo.toml`
```toml
[dependencies]
sys_metrics = "0.2"
```
Example of **basic** usage:
```rust
use sys_metrics::{cpu::*};

// This is just a very basic example of the CPU part.
// Check the doc, this crate can do much more.
fn main() {
    let cpufreq = get_cpufreq().unwrap();
    println!("CPU Speed: {:13}MHz\n", cpufreq as u64);

    let cpu_logical = get_logical_count().unwrap();
    let cpu_physical = get_physical_count().unwrap();

    println!("CPU Core: {:12}\nLogical processors: {}", cpu_physical, cpu_logical);

    let loadavg = get_loadavg().unwrap();
    println!("Load average: {:10} {} {}", loadavg.one, loadavg.five, loadavg.fifteen);
}
```
For a more complex example, check this crate in use here: [speculare-client/src/harvest/data_harvest.rs](https://github.com/speculare-cloud/speculare-client/blob/master/src/harvest/data_harvest.rs).

Benchmarks
--------------------------

```bash
➜ cargo bench
```

Supported targets
--------------------------
| Target                               | `test` |
|--------------------------------------|:------:|
| `x86_64-apple-darwin`                |   ✓    |
| `x86_64-unknown-linux-gnu`           |   ✓    |

Contributing
--------------------------

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.
