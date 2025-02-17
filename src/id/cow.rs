use std::borrow::Cow;

use iref::Iri;

use crate::BlankId;

pub enum CowId<'a> {
	BlankId(Cow<'a, BlankId>),
	Iri(Cow<'a, Iri>),
}
