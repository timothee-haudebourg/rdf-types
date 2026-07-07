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

This crate provides generic, purely semantic (as opposed to syntactic)
building blocks to work with RDF data, regardless of how resources are
represented:
- [`Triple`](https://docs.rs/rdf-types/latest/rdf_types/triple/struct.Triple.html) and [`Quad`](https://docs.rs/rdf-types/latest/rdf_types/quad/struct.Quad.html) are simple tuple types representing a
  triple/quad of resources (a quad additionally carries an optional named
  graph);
- [`Domain`](https://docs.rs/rdf-types/latest/rdf_types/domain/trait.Domain.html) and its [`FiniteDomain`](https://docs.rs/rdf-types/latest/rdf_types/domain/trait.FiniteDomain.html), [`GenDomain`](https://docs.rs/rdf-types/latest/rdf_types/domain/trait.GenDomain.html) and
  [`ConstGenDomain`](https://docs.rs/rdf-types/latest/rdf_types/domain/trait.ConstGenDomain.html) refinements abstract over a set of resources,
  independently of any graph or dataset;
- [`Graph`](https://docs.rs/rdf-types/latest/rdf_types/dataset/graph/trait.Graph.html) and [`Dataset`](https://docs.rs/rdf-types/latest/rdf_types/dataset/trait.Dataset.html) abstract over, respectively, a collection of
  triples and a collection of quads, with refinements to iterate over
  their triples/quads and resources, perform pattern matching, and
  mutate them. [`BTreeGraph`](https://docs.rs/rdf-types/latest/rdf_types/dataset/graph/impl/btree_graph/struct.BTreeGraph.html)/[`IndexedBTreeGraph`](https://docs.rs/rdf-types/latest/rdf_types/dataset/graph/impl/indexed_btree_graph/struct.IndexedBTreeGraph.html) and
  [`BTreeDataset`](https://docs.rs/rdf-types/latest/rdf_types/dataset/impl/btree_dataset/struct.BTreeDataset.html)/[`IndexedBTreeDataset`](https://docs.rs/rdf-types/latest/rdf_types/dataset/impl/indexed_btree_dataset/struct.IndexedBTreeDataset.html) are ready-to-use
  implementations backed by B-trees.

Every trait and type above is generic over the resource type, and comes
with fallible (`Try*`) and asynchronous (`Async*`) counterparts, so they
can be implemented on top of I/O-backed or otherwise fallible storage.

This crate does *not* provide any lexical or syntactic representation of
RDF resources (IRIs, blank node identifiers, literals, etc.), nor the
notion of *interpretation* mapping such lexical resources to semantic
ones. See the [`rdf-syntax`](https://crates.io/crates/rdf-syntax) crate
for that.

[rdf]: <https://w3c.github.io/rdf-primer/spec/>
[w3c]: <https://www.w3.org/>

<!-- cargo-rdme end -->

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Read the [CONTRIBUTING.md](CONTRIBUTING.md) file before submitting any contribution.
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
