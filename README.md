# ninja

[![Build Status](https://github.com/distninja/ninja/workflows/CI/badge.svg?branch=main&event=push)](https://github.com/distninja/ninja/actions?query=workflow%3ACI)
[![codecov](https://codecov.io/gh/distninja/ninja/branch/main/graph/badge.svg?token=p6goS9Jj4g)](https://codecov.io/gh/distninja/ninja)
[![License](https://img.shields.io/github/license/distninja/ninja.svg)](https://github.com/distninja/ninja/blob/main/LICENSE)
[![Tag](https://img.shields.io/github/tag/distninja/ninja.svg)](https://github.com/distninja/ninja/tags)
[![Gitter chat](https://badges.gitter.im/craftslab/distninja.png)](https://gitter.im/craftslab/distninja)



## Introduction

*ninja* is a distributed build system of [distninja](https://github.com/distninja) written in Rust.



## Prerequisites

- Rust >= 1.59.0



## Run

```bash
make install
make build
./target/x86_64-unknown-linux-musl/release/ninja --config-file="src/config/config.yml"
```



## Docker

```bash
make docker
docker run -v src/config:/tmp ghcr.io/distninja/ninja:latest --config-file="/tmp/config.yml"
```



## Usage

```
USAGE:
    ninja [OPTIONS] --config-file <NAME>

OPTIONS:
    -c, --config-file <NAME>    Config file (.yml)
    -h, --help                  Print help information
    -s, --show-dashboard        Show dashboard
    -V, --version               Print version information
```



## Settings

*ninja* parameters can be set in the directory [config](https://github.com/distninja/ninja/blob/main/src/config).

An example of configuration in [config.yml](https://github.com/distninja/ninja/blob/main/src/config/config.yml):

```yaml
apiVersion: v1
kind: ninja
metadata:
  name: ninja
spec:
  foo: foo
```



## Dashboard

> `ninja --config-file=/path/to/config.yml --show-dashboard`



## License

Project License can be found [here](LICENSE).



## Reference

- [ninja](https://github.com/ninja-build/ninja)
- [tui-rs](https://github.com/fdehau/tui-rs)
