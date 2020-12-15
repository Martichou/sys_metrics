sys_metrics
========
[![crates.io](https://img.shields.io/crates/v/sys_metrics.svg)](https://crates.io/crates/sys_metrics)
[![AGPL License](https://img.shields.io/badge/license-AGPL-blue.svg)](LICENSE)
[![CI](https://github.com/Martichou/sys_metrics/workflows/CI/badge.svg)](https://github.com/Martichou/sys_metrics/actions)

`sys_metrics` is a WIP project intended to give an alternative to others tools which can be slower or provide too many useless informations.

It's a synchronous library which try to be as fast as possible.

WIP Notes
--------------------------

`sys_metrics` in it's WIP stage will only support Linux and macOS.
The structure might not be perfect as of now and is subject to change. If you have a comment about it or anything else feel free to open an issue.

Benchmarks
--------------------------

```bash
➜ cargo bench
```

For reference you can check https://perf-ci.speculare.cloud/ for comparaison across commits.

_Note: starting at commit [c0a7005](https://github.com/Martichou/sys_metrics/commit/c0a700596c4366b65855083bdd2f8719e95ee0a1)_

Contributing
--------------------------

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.