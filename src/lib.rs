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
#![recursion_limit = "1024"]

#[doc(hidden)]
pub use iref;

mod blankid;
mod display;
mod id;
mod literal;
mod r#macro;
mod quad;
mod schema;
mod term;
mod triple;

pub use blankid::*;
pub use display::*;
pub use id::*;
pub use literal::*;
pub use quad::*;
pub use schema::*;
pub use term::*;
pub use triple::*;

pub mod dataset;
pub mod interpretation;
pub mod pattern;
pub mod utils;

pub use dataset::Dataset;
pub use interpretation::{Interpretation, InterpretationMut};
pub use iref::{iri, Iri, IriBuf};

pub const XSD_STRING: &Iri = iri!("http://www.w3.org/2001/XMLSchema#string");
