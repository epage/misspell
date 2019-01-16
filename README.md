<p align="center">
  <h3 align="center">misspell</h3>
  <p align="center">Correct commonly misspelled English words in source files 📖</p>
</p>

--------

[![pipeline status](https://gitlab.com/bloom42/misspell/badges/master/pipeline.svg)](https://gitlab.com/bloom42/misspell/commits/master)
[![misspell crate](https://img.shields.io/crates/v/misspell.svg)](https://crates.io/crates/misspell)
[![misspell documentation](https://docs.rs/misspell/badge.svg)](https://docs.rs/misspell)


1. [Install](#install)
2. [Usage](#usage)
3. [Docker](#docker)
4. [Wordlists](#wordlists)
5. [License](#license)

-------------------

## Install

```bash
cargo install misspell
```


## Usage

```bash
misspell # same as "misspell ."
misspell .
misspell file1.go file2.go directory1
```

## Docker

### Image

[registry.gitlab.com/bloom42/misspell](https://gitlab.com/bloom42/misspell/container_registry)

### Usage

```
docker run --rm -ti -v $PWD:/misspell registry.gitlab.com/bloom42/misspell:latest
```

## Wordlists

From [https://github.com/client9/misspell](https://github.com/client9/misspell)


## License

See `LICENSE.txt` and [https://opensource.bloom.sh/licensing](https://opensource.bloom.sh/licensing)
