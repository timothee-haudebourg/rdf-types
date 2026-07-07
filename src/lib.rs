//! The [Resource Description Framework (RDF)][rdf] is a very simple graph data
//! model defined by the [World Wide Web Consortium (W3C)][w3c] to represent
//! arbitrary pieces of information, primarily intended for the web. Nodes of
//! the graph are called *resources*, and resources are connected together using
//! *relations*, which are resources themselves.
//!
//! This crate provides generic, purely semantic (as opposed to syntactic)
//! building blocks to work with RDF data, regardless of how resources are
//! represented:
//! - [`Triple`] and [`Quad`] are simple tuple types representing a
//!   triple/quad of resources (a quad additionally carries an optional named
//!   graph);
//! - [`Domain`] and its [`FiniteDomain`], [`GenDomain`] and
//!   [`ConstGenDomain`] refinements abstract over a set of resources,
//!   independently of any graph or dataset;
//! - [`Graph`] and [`Dataset`] abstract over, respectively, a collection of
//!   triples and a collection of quads, with refinements to iterate over
//!   their triples/quads and resources, perform pattern matching, and
//!   mutate them. [`BTreeGraph`]/[`IndexedBTreeGraph`] and
//!   [`BTreeDataset`]/[`IndexedBTreeDataset`] are ready-to-use
//!   implementations backed by B-trees.
//!
//! Every trait and type above is generic over the resource type, and comes
//! with fallible (`Try*`) and asynchronous (`Async*`) counterparts, so they
//! can be implemented on top of I/O-backed or otherwise fallible storage.
//!
//! This crate does *not* provide any lexical or syntactic representation of
//! RDF resources (IRIs, blank node identifiers, literals, etc.), nor the
//! notion of *interpretation* mapping such lexical resources to semantic
//! ones. See the [`rdf-syntax`](https://crates.io/crates/rdf-syntax) crate
//! for that.
//!
//! [rdf]: <https://w3c.github.io/rdf-primer/spec/>
//! [w3c]: <https://www.w3.org/>
#![allow(async_fn_in_trait)]
mod dataset;
pub mod diff;
mod domain;
mod isomorphism;
pub mod pattern;
mod quad;
mod triple;
pub mod util;

pub use dataset::*;
pub use domain::*;
pub use isomorphism::*;
pub use quad::*;
pub use triple::*;
