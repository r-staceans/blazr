
<!-- README.md is generated from README.Rmd. Please edit that file -->

# blazr <a href="https://r-staceans.github.io/blazr/"><img src="man/figures/logo.png" align="right" height="138" /></a>

<!-- badges: start -->

[![R-CMD-check](https://github.com/r-staceans/blazr/actions/workflows/R-CMD-check.yaml/badge.svg)](https://github.com/r-staceans/blazr/actions/workflows/R-CMD-check.yaml)
[![Codecov test
coverage](https://codecov.io/gh/r-staceans/blazr/graph/badge.svg)](https://app.codecov.io/gh/r-staceans/blazr)
[![Lifecycle:
experimental](https://img.shields.io/badge/lifecycle-experimental-orange.svg)](https://lifecycle.r-lib.org/articles/stages.html#experimental)
<!-- badges: end -->

⚠️ **This package is a work in progress and is not yet ready for use.**

The goal of blazr is to provide R with a blazingly fast interface to
multi-threading in Rust.

## Installation

Install the development version of `blazr` from GitHub with:

``` r
# install.packages("pak")
pak::pak("r-staceans/blazr")
#> 
#> → Will install 1 package.
#> → Will download 1 package with unknown size.
#> + blazr   0.0.0.9000 [bld][cmp][dl] (GitHub: ac3d506) + ✔ rustc, ✔ cargo
#> ✔ All system requirements are already installed.
#> 
#> ℹ Getting 1 pkg with unknown size
#> ✔ Got blazr 0.0.0.9000 (source) (1.73 MB)
#> ℹ Installing system requirements
#> ℹ Executing `sudo sh -c apt-get -y update`
#> Get:1 file:/etc/apt/apt-mirrors.txt Mirrorlist [142 B]
#> Hit:2 http://azure.archive.ubuntu.com/ubuntu noble InRelease
#> Hit:3 http://azure.archive.ubuntu.com/ubuntu noble-updates InRelease
#> Hit:4 http://azure.archive.ubuntu.com/ubuntu noble-backports InRelease
#> Hit:5 http://azure.archive.ubuntu.com/ubuntu noble-security InRelease
#> Hit:6 https://packages.microsoft.com/repos/azure-cli noble InRelease
#> Hit:7 https://packages.microsoft.com/ubuntu/24.04/prod noble InRelease
#> Reading package lists...
#> ℹ Executing `sudo sh -c apt-get -y install rustc cargo`
#> Reading package lists...
#> Building dependency tree...
#> Reading state information...
#> rustc is already the newest version (1.75.0+dfsg0ubuntu1-0ubuntu7.1).
#> cargo is already the newest version (1.75.0+dfsg0ubuntu1-0ubuntu7.1).
#> 0 upgraded, 0 newly installed, 0 to remove and 79 not upgraded.
#> ℹ Packaging blazr 0.0.0.9000
#> ✔ Packaged blazr 0.0.0.9000 (590ms)
#> ℹ Building blazr 0.0.0.9000
#> ✔ Built blazr 0.0.0.9000 (7.8s)
#> ✔ Installed blazr 0.0.0.9000 (github::r-staceans/blazr@ac3d506) (65ms)
#> ✔ 1 pkg: added 1, dld 1 (NA B) [13.5s]
```

## Example

This is just a dummy example:

``` r
library(blazr)

blazr::to_upper("hello, world")
#> [1] "HELLO, WORLD"
```
