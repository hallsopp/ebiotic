# Ebiotic

[![Crates.io](https://img.shields.io/crates/v/ebiotic.svg)](https://crates.io/crates/ebiotic)
[![Documentation](https://docs.rs/ebiotic/badge.svg)](https://docs.rs/ebiotic)

Ebiotic provides a light-weight asynchronous interface for some popular Bioinformatics web services. It is designed to
enable access to the rich data and tools provided by institutes like the European Bioinformatics
Institute's ([(EBI)](https://www.ebi.ac.uk/))
and the National Center for Biotechnology Information ([NCBI](https://www.ncbi.nlm.nih.gov/)). It's built to serialize
and
deserialze data using common formats like JSON and specialised bioinformatics formats like FASTA using the `Record` data
structure from
the [rust-bio](https://rust-bio.github.io/) library.

## Current APIs

- NCBI
  - Web-BLAST (currently only supports protein searches as it was used for another project)
- EBI
  - Job Dispatcher
    - Clustal Omega (needs tidying up)
  - Knowledge & Data
    - DBfetch (bare-bones)

# Installation

The library is currently not stable but is available on [crates.io](https://crates.io/crates/ebiotic). To install, add
the following to your `Cargo.toml` file:

```toml
[dependencies]
ebiotic = "0.0.11"
```

# Contributing

Contributions are more than welcome. To implement a new endpoint follow the structure of the current modules and utilise
the functionality provided by the `ebiotic::core` module.

# TODO

- Docs!