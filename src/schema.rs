use iref::Iri;
use static_iref::iri;

pub const RDFS_RESOURCE: &Iri = iri!("http://www.w3.org/2000/01/rdf-schema#Resource");
pub const RDFS_CLASS: &Iri = iri!("http://www.w3.org/2000/01/rdf-schema#Class");
pub const RDFS_LITERAL: &Iri = iri!("http://www.w3.org/2000/01/rdf-schema#Literal");
pub const RDFS_DATATYPE: &Iri = iri!("http://www.w3.org/2000/01/rdf-schema#Datatype");
pub const RDFS_CONTAINER: &Iri = iri!("http://www.w3.org/2000/01/rdf-schema#Container");
pub const RDFS_CONTAINER_MEMBERSHIP_PROPERTY: &Iri =
	iri!("http://www.w3.org/2000/01/rdf-schema#ContainerMembershipProperty");

pub const RDFS_RANGE: &Iri = iri!("http://www.w3.org/2000/01/rdf-schema#range");
pub const RDFS_DOMAIN: &Iri = iri!("http://www.w3.org/2000/01/rdf-schema#domain");
pub const RDFS_SUB_CLASS_OF: &Iri = iri!("http://www.w3.org/2000/01/rdf-schema#subClassOf");
pub const RDFS_SUB_PROPERTY_OF: &Iri = iri!("http://www.w3.org/2000/01/rdf-schema#subPropertyOf");
pub const RDFS_LABEL: &Iri = iri!("http://www.w3.org/2000/01/rdf-schema#label");
pub const RDFS_COMMENT: &Iri = iri!("http://www.w3.org/2000/01/rdf-schema#comment");
pub const RDFS_MEMBER: &Iri = iri!("http://www.w3.org/2000/01/rdf-schema#member");
pub const RDFS_SEE_ALSO: &Iri = iri!("http://www.w3.org/2000/01/rdf-schema#seeAlso");
pub const RDFS_IS_DEFINED_BY: &Iri = iri!("http://www.w3.org/2000/01/rdf-schema#isDefinedBy");

pub const RDF_LANG_STRING: &Iri = iri!("http://www.w3.org/1999/02/22-rdf-syntax-ns#langString");
pub const RDF_HTML: &Iri = iri!("http://www.w3.org/1999/02/22-rdf-syntax-ns#HTML");
pub const RDF_XML_LITERAL: &Iri = iri!("http://www.w3.org/1999/02/22-rdf-syntax-ns#XMLLiteral");
pub const RDF_JSON: &Iri = iri!("http://www.w3.org/1999/02/22-rdf-syntax-ns#JSON");
pub const RDF_PROPERTY: &Iri = iri!("http://www.w3.org/1999/02/22-rdf-syntax-ns#Property");
pub const RDF_BAG: &Iri = iri!("http://www.w3.org/1999/02/22-rdf-syntax-ns#Bag");
pub const RDF_SEQ: &Iri = iri!("http://www.w3.org/1999/02/22-rdf-syntax-ns#Seq");
pub const RDF_ALT: &Iri = iri!("http://www.w3.org/1999/02/22-rdf-syntax-ns#Alt");
pub const RDF_LIST: &Iri = iri!("http://www.w3.org/1999/02/22-rdf-syntax-ns#List");
pub const RDF_STATEMENT: &Iri = iri!("http://www.w3.org/1999/02/22-rdf-syntax-ns#Statement");

pub const RDF_TYPE: &Iri = iri!("http://www.w3.org/1999/02/22-rdf-syntax-ns#type");
pub const RDF_FIRST: &Iri = iri!("http://www.w3.org/1999/02/22-rdf-syntax-ns#first");
pub const RDF_REST: &Iri = iri!("http://www.w3.org/1999/02/22-rdf-syntax-ns#rest");
pub const RDF_SUBJECT: &Iri = iri!("http://www.w3.org/1999/02/22-rdf-syntax-ns#subject");
pub const RDF_PREDICATE: &Iri = iri!("http://www.w3.org/1999/02/22-rdf-syntax-ns#predicate");
pub const RDF_OBJECT: &Iri = iri!("http://www.w3.org/1999/02/22-rdf-syntax-ns#object");
pub const RDF_VALUE: &Iri = iri!("http://www.w3.org/1999/02/22-rdf-syntax-ns#value");

pub const RDF_NIL: &Iri = iri!("http://www.w3.org/1999/02/22-rdf-syntax-ns#nil");
