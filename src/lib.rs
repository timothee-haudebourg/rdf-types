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

#[doc(hidden)]
pub use static_iref;

mod blankid;
mod display;
mod grdf;
mod literal;
mod r#macro;
mod quad;
mod schema;
mod term;
mod triple;

pub use blankid::*;
pub use display::*;
pub use grdf::*;
pub use literal::*;
pub use quad::*;
pub use schema::*;
pub use term::*;
pub use triple::*;

pub mod dataset;
pub mod generator;
pub mod interpretation;
pub mod pattern;
pub mod utils;
pub mod vocabulary;

pub use dataset::Dataset;
pub use generator::Generator;
pub use interpretation::{Interpretation, InterpretationMut};
pub use iref::{Iri, IriBuf};
pub use vocabulary::{Vocabulary, VocabularyMut};

pub const XSD_STRING: &Iri = static_iref::iri!("http://www.w3.org/2001/XMLSchema#string");

/// IRI type that may be <http://www.w3.org/2001/XMLSchema#string>.
///
/// This is used upon formatting RDF literals to omit the type when it is not
/// required (because it is implicitly `xsd:string`).
pub trait IsXsdStringIri {
	/// Checks if this IRI is <http://www.w3.org/2001/XMLSchema#string>.
	fn is_xsd_string_iri(&self) -> bool;
}

impl IsXsdStringIri for IriBuf {
	fn is_xsd_string_iri(&self) -> bool {
		self == XSD_STRING
	}
}

impl IsXsdStringIri for Iri {
	fn is_xsd_string_iri(&self) -> bool {
		self == XSD_STRING
	}
}
