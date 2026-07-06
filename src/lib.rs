//! The [Resource Description Framework (RDF)][rdf] is a very simple graph data
//! model defined by the [World Wide Web Consortium (W3C)][w3c] to represent
//! arbitrary pieces of information, primarily intended for the web. Nodes of
//! the graph are called *resources*, and resources are connected together using
//! *relations*, which are resources themselves.
//!
//! This is a utility library providing common types, data-structures, traits,
//! constants and macro definitions to deal with RDF data:
//! - IRIs (through the `iref` crate), blank node identifiers and literals to
//!   represent resources in their lexical form as *terms*;
//! - Triples and quads;
//! - Interpretations projecting resources from the lexical domain to the value
//!   domain;
//! - Graphs and datasets representing collections of interpreted triples/quads.
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
