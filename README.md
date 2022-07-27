# ninja

[![Build Status](https://github.com/distninja/ninja/workflows/ci/badge.svg?branch=main&event=push)](https://github.com/distninja/ninja/actions?query=workflow%3Aci)
[![codecov](https://codecov.io/gh/distninja/ninja/branch/main/graph/badge.svg?token=p6goS9Jj4g)](https://codecov.io/gh/distninja/ninja)
[![License](https://img.shields.io/github/license/distninja/ninja.svg)](https://github.com/distninja/ninja/blob/main/LICENSE)
[![Tag](https://img.shields.io/github/tag/distninja/ninja.svg)](https://github.com/distninja/ninja/tags)
[![Gitter chat](https://badges.gitter.im/craftslab/distninja.png)](https://gitter.im/craftslab/distninja)



## Introduction

*ninja* is a distributed build system of [distninja](https://github.com/distninja) written in Rust.



## Prerequisites

- Rust >= 1.60.0



## Run

```bash
make install
make build
./target/release/ninja
```



## Docker

```bash
make docker
docker run ghcr.io/distninja/ninja:latest
```



## Usage

```
USAGE:
    ninja [OPTIONS]

OPTIONS:
    -C <DIR>         change to DIR before doing anything else
    -d <MODE>        enable debugging (use '-d list' to list modes)
    -f <FILE>        specify input build file [default=build.ninja]
    -h, --help       Print help information
    -j <N>           run N jobs in parallel (0 means infinity) [default=18 on this system]
    -k <N>           keep going until N jobs fail (0 means infinity) [default=1]
    -l <N>           do not start new jobs if the load average is greater than N
    -n               dry run (don't run commands but act like they succeeded)
    -q, --quiet      don't show progress status, just command output
    -t <TOOL>        run a subtool (use '-t list' to list subtools)
    -v, --verbose    show all command lines while building
    -V, --version    Print version information
    -w <FLAG>        adjust warnings (use '-w list' to list warnings)

if targets are unspecified, builds the 'default' target (see manual).
```



## License

Project License can be found [here](LICENSE).



## Reference

- [ninja](https://github.com/ninja-build/ninja)
