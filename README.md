# RDF Types

[![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/timothee-haudebourg/rdf-types/ci.yml?style=flat-square&logo=github)](https://github.com/timothee-haudebourg/rdf-types/actions)
[![Crate informations](https://img.shields.io/crates/v/rdf-types.svg?style=flat-square)](https://crates.io/crates/rdf-types)
[![Crates.io MSRV](https://img.shields.io/crates/msrv/rdf-types?style=flat-square)](https://crates.io/crates/rdf-types)
[![License](https://img.shields.io/crates/l/rdf-types.svg?style=flat-square)](https://github.com/timothee-haudebourg/rdf-types#license)
[![Documentation](https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square)](https://docs.rs/rdf-types)

<!-- cargo-rdme start -->

The [Resource Description Framework (RDF)][rdf] is a very simple graph data
model defined by the [World Wide Web Consortium (W3C)][w3c] to represent
arbitrary pieces of information, primarily intended for the web. Nodes of
the graph are called *resources*, and resources are connected together using
*relations*, which are resources themselves.

This is a utility library providing common types, data-structures, traits,
constants and macro definitions to deal with RDF data:
- IRIs (through the `iref` crate), blank node identifiers and literals to
  represent resources in their lexical form as *terms*;
- Triples and quads;
- Interpretations projecting resources from the lexical domain to the value
  domain;
- Graphs and datasets representing collections of interpreted triples/quads.

[rdf]: <https://w3c.github.io/rdf-primer/spec/>
[w3c]: <https://www.w3.org/>

<!-- cargo-rdme end -->

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
