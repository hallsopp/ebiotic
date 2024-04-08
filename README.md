# Ebiotic

[![Crates.io](https://img.shields.io/crates/v/ebiotic.svg)](https://crates.io/crates/ebiotic)
[![Crates.io](https://img.shields.io/crates/l/ebiotic.svg)](https://crates.io/crates/ebiotic)
[![Documentation](https://docs.rs/ebiotic/badge.svg)](https://docs.rs/ebiotic)
[![Build Status](https://github.com/hallsopp/ebiotic/actions/workflows/rust.yml/badge.svg)]()

Ebiotic provides a light-weight asynchronous interface for some popular Bioinformatics web services. It is designed to
enable access to the rich data and tools provided by institutes like the European Bioinformatics
Institute ([EBI](https://www.ebi.ac.uk/)) and the National Center for Biotechnology
Information ([NCBI](https://www.ncbi.nlm.nih.gov/)). It's built to serialize
and deserialze data using common formats like JSON and specialised bioinformatics formats like FASTA, using the `Record`
data structure from the [rust-bio](https://rust-bio.github.io/) library.

**Disclaimer:**
The library does not safeguard against misuse of the endpoints (beyond hard-coded polling frequencies) and so it is
important to read and understand the terms of use for each API before using this library.

## Installation

The library is currently in the early stages of development and is therefore not stable, but remains available
on [crates.io](https://crates.io/crates/ebiotic). To install, add
the following to your `Cargo.toml` file:

```toml
[dependencies]
ebiotic = "0.0.25"
```

## Usage

The library is designed to be simple to use and to provide a consistent interface to the various APIs. The following
example demonstrates how to use the library to search the European Nucleotide
Archive ([ENA](https://www.ebi.ac.uk/ena/browser/home)) for a selection of entries.

```rust
use ebiotic::data::*;

#[tokio::main]
async fn main_fasta() {
    let dbfetch = Dbfetch::default();
    let ids = DbfetchIds::new(vec!["M10051".to_string(), "M10052".to_string()]);
    let result = dbfetch.run(ids).await.unwrap().into_records();
}
```

The `Dbfetch` struct is used to create a new instance of the `Dbfetch` API. The `DbfetchIds` struct is used to create a
new instance of the `DbfetchIds` request. The `run` method is then called on the `Dbfetch` instance with
the `DbfetchIds` instance as an argument. The `run` method returns a `Result` which is then unwrapped and converted into
a `Vec<Record>` using the `into_records` method.

In order to perform HTTP requests, a system client is required. The `EbioticClient` struct is used to create a new
client for this purpose. By default, the `EbioticClient` uses an asynchronous `reqwest` client under the hood. This
means it can be customised for platform specific requirements, such as using a proxy or customising the user agent. More
information on the `reqwest` client can be found in the [reqwest documentation](https://docs.rs/crate/reqwest/latest).

```rust
use ebiotic::data::*;
use std::time::Duration;
use reqwest;

#[tokio::main]
async fn main_blast() {
    let client = EbioticClient::new(
        reqwest::Client::builder()
            .timeout(Duration::from_secs(10))
            .proxy(reqwest::Proxy::all("http://my-proxy:8080").unwrap())
            .build()
            .unwrap()
    );

    let dbfetch = Dbfetch::new(client, DbfetchDbs::EnaSequence, DataReturnFormats::Fasta, DbfetchStyle::Raw);
    let ids = DbfetchIds::new(vec!["M10051".to_string(), "M10052".to_string()]);
    let result = dbfetch.run(ids).await.unwrap().into_records();
}
```

More examples can be found in the [documentation](https://docs.rs/ebiotic). Including how to run in synchronous
code-bases using thread blocking.

## Current APIs

### NCBI:

- Web-BLAST (currently only supports protein searches as it was used for another project)

### EBI:

**Job Dispatcher:**

- Clustal Omega

**Knowledge & Data:**

- DBfetch
- EBI Search

## Contributing

Contributions are more than welcome. To implement a new endpoint follow the structure of the current modules and utilise
the functionality provided by the `ebiotic::core` module. Alternatively, pick something from the TODO list and try to
implement or find something in the code that you can improve!
If you have any questions or need help, feel free to open an issue or reach out to me on via email.

## TODOs

- More tools and data sources
- Add supplementary methods to result types (beyond just returning the data)
- Add more tests
- Add more documentation
- Add more examples
- Add more error handling
- Add logging system beyond print statements (e.g. tracing or log crate)
- Add more configuration options
- Safety checks for API usage (?)
- Citations for tools and APIs
