# EBIotic

EBIotic is a sleek Rust library tailored for seamless integration with the European Bioinformatics Institute's (EBI)
services. This library simplifies RESTful queries, converting them into straightforward Rust functions and delivering
the results as Rust data structures for effortless use in further analysis. It doesn't stop there; EBIotic also taps
into the renowned BLAST tool from NCBI, broadening its utility. With EBIotic, navigating the bioinformatics landscape
becomes a smoother journey, enabling your applications to leverage EBI's rich datasets and NCBI's robust tools with the
performance and reliability of Rust.

## Installation

The library is currently not stable but is available on [crates.io](https://crates.io/crates/ebiotic). To install, add
the following to your `Cargo.toml` file:

```toml
[dependencies]
ebiotic = "0.0.1"
```