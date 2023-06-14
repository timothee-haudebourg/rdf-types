use crate::{
	literal, GraphLabel, Id, InsertIntoVocabulary, InsertedIntoVocabulary, MapLiteral, Quad,
	Subject, Triple,
};
use iref::IriBuf;
use locspan::{Meta, Strip};

/// gRDF term with literal with metadata.
pub type Term<M, I = Id, T = literal::Type, S = String> = crate::Term<I, Literal<M, T, S>>;

/// RDF object with literal with metadata.
pub type Object<M, I = Id, T = literal::Type, S = String> = crate::Object<I, Literal<M, T, S>>;

/// gRDF term with metadata.
pub type MetaTerm<M, I = Id, T = literal::Type, S = String> = Meta<Term<M, I, T, S>, M>;

/// RDF object with metadata.
pub type MetaObject<M, I = Id, T = literal::Type, S = String> = Meta<Object<M, I, T, S>, M>;

/// Quad with metadata.
pub type MetaQuad<S, P, O, G, M> = Meta<Quad<Meta<S, M>, Meta<P, M>, Meta<O, M>, Meta<G, M>>, M>;

/// RDF quad with metadata.
pub type MetaRdfQuad<M> =
	Meta<Quad<Meta<Subject, M>, Meta<IriBuf, M>, MetaObject<M>, Meta<GraphLabel, M>>, M>;

/// gRDF quad with metadata.
pub type MetaGrdfQuad<M> = Meta<Quad<MetaTerm<M>, MetaTerm<M>, MetaTerm<M>, MetaTerm<M>>, M>;

/// RDF Literal with metadata.
pub type Literal<M, T = literal::Type, S = String> = crate::Literal<Meta<T, M>, Meta<S, M>>;

impl<I, B> Strip for Id<I, B> {
	type Stripped = Self;

	fn strip(self) -> Self {
		self
	}
}

impl<V, T: InsertIntoVocabulary<V>, M> InsertIntoVocabulary<V> for Meta<T, M> {
	type Inserted = Meta<T::Inserted, M>;

	fn insert_into_vocabulary(self, vocabulary: &mut V) -> Self::Inserted {
		Meta(self.0.insert_into_vocabulary(vocabulary), self.1)
	}
}

impl<V, T: InsertedIntoVocabulary<V>, M: Clone> InsertedIntoVocabulary<V> for Meta<T, M> {
	type Inserted = Meta<T::Inserted, M>;

	fn inserted_into_vocabulary(&self, vocabulary: &mut V) -> Self::Inserted {
		Meta(self.0.inserted_into_vocabulary(vocabulary), self.1.clone())
	}
}

impl<T: MapLiteral<L, M>, L, M, K> MapLiteral<L, M> for Meta<T, K> {
	type Output = Meta<T::Output, K>;

	fn map_literal(self, f: impl FnMut(L) -> M) -> Self::Output {
		Meta(self.0.map_literal(f), self.1)
	}
}

// /// RDF Literal with metadata.
// #[derive(
// 	Clone,
// 	PartialEq,
// 	Eq,
// 	Hash,
// 	PartialOrd,
// 	Ord,
// 	Debug,
// 	StrippedPartialEq,
// 	StrippedEq,
// 	StrippedPartialOrd,
// 	StrippedOrd,
// 	StrippedHash,
// )]
// #[locspan(ignore(M))]
// pub struct Literal<M, T = LiteralType<M, IriBuf, LanguageTagBuf>, S = String> {

// }

// impl<M, S, T, L> Literal<M, T, S> {
// 	pub fn is_typed(&self) -> bool {
// 		matches!(self, Self::TypedString(_, _))
// 	}

// 	pub fn ty(&self) -> Option<&Meta<T, M>> {
// 		match self {
// 			Self::TypedString(_, ty) => Some(ty),
// 			_ => None,
// 		}
// 	}

// 	pub fn is_lang_string(&self) -> bool {
// 		matches!(self, Self::LangString(_, _))
// 	}

// 	pub fn lang_tag(&self) -> Option<&Meta<L, M>> {
// 		match self {
// 			Self::LangString(_, tag) => Some(tag),
// 			_ => None,
// 		}
// 	}

// 	pub fn string_literal(&self) -> &Meta<S, M> {
// 		match self {
// 			Self::String(s) => s,
// 			Self::TypedString(s, _) => s,
// 			Self::LangString(s, _) => s,
// 		}
// 	}

// 	pub fn into_string_literal(self) -> Meta<S, M> {
// 		match self {
// 			Self::String(s) => s,
// 			Self::TypedString(s, _) => s,
// 			Self::LangString(s, _) => s,
// 		}
// 	}

// 	pub fn strip(self) -> super::Literal<T, S> {
// 		match self {
// 			Self::String(Meta(lit, _)) => super::Literal::String(lit),
// 			Self::TypedString(Meta(lit, _), Meta(iri_ref, _)) => {
// 				super::Literal::TypedString(lit, iri_ref)
// 			}
// 			Self::LangString(Meta(lit, _), Meta(tag, _)) => super::Literal::LangString(lit, tag),
// 		}
// 	}
// }

impl<M, T, S> Strip for Literal<M, T, S> {
	type Stripped = crate::Literal<T, S>;

	fn strip(self) -> Self::Stripped {
		let (Meta(value, _), Meta(type_, _)) = self.into_parts();
		crate::Literal::new(value, type_)
	}
}

// impl<M, S: fmt::Display, T: fmt::Display, L: fmt::Display> fmt::Display for Literal<M, T, S> {
// 	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
// 		match self {
// 			Self::String(s) => s.value().fmt(f),
// 			Self::TypedString(s, ty) => write!(f, "{}^^<{}>", s.value(), ty.value()),
// 			Self::LangString(s, tag) => write!(f, "{}@{}", s.value(), tag.value()),
// 		}
// 	}
// }

// #[cfg(feature = "contextual")]
// impl<M, S: fmt::Display, T, L: fmt::Display, V: crate::IriVocabulary<Iri = T>> DisplayWithContext<V>
// 	for Literal<M, T, S>
// {
// 	fn fmt_with(&self, vocabulary: &V, f: &mut fmt::Formatter) -> fmt::Result {
// 		match self {
// 			Self::String(s) => s.value().fmt(f),
// 			Self::TypedString(s, ty) => write!(
// 				f,
// 				"{}^^<{}>",
// 				s.value(),
// 				vocabulary.iri(ty.value()).unwrap()
// 			),
// 			Self::LangString(s, tag) => write!(f, "{}@{}", s.value(), tag.value()),
// 		}
// 	}
// }

impl<I: Strip, L: Strip> super::Term<I, L> {
	pub fn strip(self) -> super::Term<I::Stripped, L::Stripped> {
		match self {
			Self::Id(id) => super::Term::Id(id.strip()),
			Self::Literal(lit) => super::Term::Literal(lit.strip()),
		}
	}
}

impl<I: Strip, L: Strip> Strip for super::Term<I, L> {
	type Stripped = super::Term<I::Stripped, L::Stripped>;

	fn strip(self) -> Self::Stripped {
		self.strip()
	}
}

impl<S: Strip, P: Strip, O: Strip> Strip for Triple<S, P, O> {
	type Stripped = Triple<S::Stripped, P::Stripped, O::Stripped>;

	fn strip(self) -> Self::Stripped {
		Triple(self.0.strip(), self.1.strip(), self.2.strip())
	}
}

impl<S: Strip, P, O: Strip, M> Triple<S, Meta<P, M>, O> {
	/// Utility function to strip metadata off a triple when the predicate type
	/// `P` does not implement the [`Strip`] trait.
	/// This often happens for RDF triples because the predicate is an IRI usually
	/// represented with the [`IriBuf`] which does not implement [`Strip`].
	pub fn strip_all_but_predicate(self) -> Triple<S::Stripped, P, O::Stripped> {
		Triple(self.0.strip(), self.1.into_value(), self.2.strip())
	}
}

impl<S: Strip, P: Strip, O: Strip, G: Strip> Strip for Quad<S, P, O, G> {
	type Stripped = Quad<S::Stripped, P::Stripped, O::Stripped, G::Stripped>;

	fn strip(self) -> Self::Stripped {
		Quad(
			self.0.strip(),
			self.1.strip(),
			self.2.strip(),
			self.3.strip(),
		)
	}
}

impl<S: Strip, P, O: Strip, G: Strip, M> Quad<S, Meta<P, M>, O, G> {
	/// Utility function to strip metadata off a quad when the predicate type
	/// `P` does not implement the [`Strip`] trait.
	/// This often happens for RDF quads because the predicate is an IRI usually
	/// represented with the [`IriBuf`] which does not implement [`Strip`].
	pub fn strip_all_but_predicate(self) -> Quad<S::Stripped, P, O::Stripped, G::Stripped> {
		Quad(
			self.0.strip(),
			self.1.into_value(),
			self.2.strip(),
			self.3.strip(),
		)
	}
}

// impl<S, L, M> Literal<M, S, IriBuf, L> {
// 	pub fn inserted_into<V: IriVocabularyMut>(&self, vocabulary: &mut V) -> Literal<M, S, V::Iri, L>
// 	where
// 		S: Clone,
// 		L: Clone,
// 		M: Clone,
// 	{
// 		match self {
// 			Self::String(s) => Literal::String(s.clone()),
// 			Self::TypedString(s, Meta(t, m)) => {
// 				Literal::TypedString(s.clone(), Meta(vocabulary.insert(t.as_iri()), m.clone()))
// 			}
// 			Self::LangString(s, l) => Literal::LangString(s.clone(), l.clone()),
// 		}
// 	}

// 	pub fn insert_into<V: IriVocabularyMut>(self, vocabulary: &mut V) -> Literal<M, S, V::Iri, L> {
// 		match self {
// 			Self::String(s) => Literal::String(s),
// 			Self::TypedString(s, Meta(t, m)) => {
// 				Literal::TypedString(s, Meta(vocabulary.insert(t.as_iri()), m))
// 			}
// 			Self::LangString(s, l) => Literal::LangString(s, l),
// 		}
// 	}
// }

// impl<S, L, M> Term<M, Id<IriBuf, BlankIdBuf>, S, IriBuf, L> {
// 	#[allow(clippy::type_complexity)]
// 	pub fn inserted_into<V: VocabularyMut>(
// 		&self,
// 		vocabulary: &mut V,
// 	) -> Term<M, Id<V::Iri, V::BlankId>, S, V::Iri, L>
// 	where
// 		S: Clone,
// 		L: Clone,
// 		M: Clone,
// 	{
// 		match self {
// 			Self::Id(id) => Term::Id(id.inserted_into(vocabulary)),
// 			Self::Literal(l) => Term::Literal(l.inserted_into(vocabulary)),
// 		}
// 	}

// 	#[allow(clippy::type_complexity)]
// 	pub fn insert_into<V: VocabularyMut>(
// 		self,
// 		vocabulary: &mut V,
// 	) -> Term<M, Id<V::Iri, V::BlankId>, S, V::Iri, L> {
// 		match self {
// 			Self::Id(id) => Term::Id(id.insert_into(vocabulary)),
// 			Self::Literal(l) => Term::Literal(l.insert_into(vocabulary)),
// 		}
// 	}
// }

// impl<S, L, M>
// 	Triple<Meta<Subject, M>, Meta<IriBuf, M>, Meta<Object<M, Id<IriBuf, BlankIdBuf>, S, IriBuf, L>, M>>
// {
// 	#[allow(clippy::type_complexity)]
// 	pub fn inserted_into<V: VocabularyMut>(
// 		&self,
// 		vocabulary: &mut V,
// 	) -> Triple<
// 		Meta<Subject<V::Iri, V::BlankId>, M>,
// 		Meta<V::Iri, M>,
// 		Meta<Object<M, Id<V::Iri, V::BlankId>, S, V::Iri, L>, M>,
// 	>
// 	where
// 		S: Clone,
// 		L: Clone,
// 		M: Clone,
// 	{
// 		Triple(
// 			Meta(self.0.inserted_into(vocabulary), self.0.metadata().clone()),
// 			Meta(
// 				vocabulary.insert(self.1.as_iri()),
// 				self.1.metadata().clone(),
// 			),
// 			Meta(self.2.inserted_into(vocabulary), self.2.metadata().clone()),
// 		)
// 	}

// 	#[allow(clippy::type_complexity)]
// 	pub fn insert_into<V: VocabularyMut>(
// 		self,
// 		vocabulary: &mut V,
// 	) -> Triple<
// 		Meta<Subject<V::Iri, V::BlankId>, M>,
// 		Meta<V::Iri, M>,
// 		Meta<Object<M, Id<V::Iri, V::BlankId>, S, V::Iri, L>, M>,
// 	> {
// 		Triple(
// 			self.0.map(|s| s.insert_into(vocabulary)),
// 			self.1.map(|p| vocabulary.insert(p.as_iri())),
// 			self.2.map(|o| o.insert_into(vocabulary)),
// 		)
// 	}
// }

// impl<S, L, M>
// 	Triple<
// 		Meta<Term<M, Id<IriBuf, BlankIdBuf>, S, IriBuf, L>, M>,
// 		Meta<Term<M, Id<IriBuf, BlankIdBuf>, S, IriBuf, L>, M>,
// 		Meta<Term<M, Id<IriBuf, BlankIdBuf>, S, IriBuf, L>, M>,
// 	>
// {
// 	#[allow(clippy::type_complexity)]
// 	pub fn inserted_into<V: VocabularyMut>(
// 		&self,
// 		vocabulary: &mut V,
// 	) -> Triple<
// 		Meta<Term<M, Id<V::Iri, V::BlankId>, S, V::Iri, L>, M>,
// 		Meta<Term<M, Id<V::Iri, V::BlankId>, S, V::Iri, L>, M>,
// 		Meta<Term<M, Id<V::Iri, V::BlankId>, S, V::Iri, L>, M>,
// 	>
// 	where
// 		S: Clone,
// 		L: Clone,
// 		M: Clone,
// 	{
// 		Triple(
// 			Meta(self.0.inserted_into(vocabulary), self.0.metadata().clone()),
// 			Meta(self.1.inserted_into(vocabulary), self.1.metadata().clone()),
// 			Meta(self.2.inserted_into(vocabulary), self.2.metadata().clone()),
// 		)
// 	}

// 	#[allow(clippy::type_complexity)]
// 	pub fn insert_into<V: VocabularyMut>(
// 		self,
// 		vocabulary: &mut V,
// 	) -> Triple<
// 		Meta<Term<M, Id<V::Iri, V::BlankId>, S, V::Iri, L>, M>,
// 		Meta<Term<M, Id<V::Iri, V::BlankId>, S, V::Iri, L>, M>,
// 		Meta<Term<M, Id<V::Iri, V::BlankId>, S, V::Iri, L>, M>,
// 	> {
// 		Triple(
// 			self.0.map(|s| s.insert_into(vocabulary)),
// 			self.1.map(|p| p.insert_into(vocabulary)),
// 			self.2.map(|o| o.insert_into(vocabulary)),
// 		)
// 	}
// }

// impl<S, L, M>
// 	Quad<
// 		Meta<Subject, M>,
// 		Meta<IriBuf, M>,
// 		Meta<Object<M, Id<IriBuf, BlankIdBuf>, S, IriBuf, L>, M>,
// 		Meta<GraphLabel, M>,
// 	>
// {
// 	#[allow(clippy::type_complexity)]
// 	pub fn inserted_into<V: VocabularyMut>(
// 		&self,
// 		vocabulary: &mut V,
// 	) -> Quad<
// 		Meta<Subject<V::Iri, V::BlankId>, M>,
// 		Meta<V::Iri, M>,
// 		Meta<Object<M, Id<V::Iri, V::BlankId>, S, V::Iri, L>, M>,
// 		Meta<GraphLabel<V::Iri, V::BlankId>, M>,
// 	>
// 	where
// 		S: Clone,
// 		L: Clone,
// 		M: Clone,
// 	{
// 		Quad(
// 			Meta(self.0.inserted_into(vocabulary), self.0.metadata().clone()),
// 			Meta(
// 				vocabulary.insert(self.1.as_iri()),
// 				self.1.metadata().clone(),
// 			),
// 			Meta(self.2.inserted_into(vocabulary), self.2.metadata().clone()),
// 			self.3
// 				.as_ref()
// 				.map(|Meta(g, m)| Meta(g.inserted_into(vocabulary), m.clone())),
// 		)
// 	}

// 	#[allow(clippy::type_complexity)]
// 	pub fn insert_into<V: VocabularyMut>(
// 		self,
// 		vocabulary: &mut V,
// 	) -> Quad<
// 		Meta<Subject<V::Iri, V::BlankId>, M>,
// 		Meta<V::Iri, M>,
// 		Meta<Object<M, Id<V::Iri, V::BlankId>, S, V::Iri, L>, M>,
// 		Meta<GraphLabel<V::Iri, V::BlankId>, M>,
// 	> {
// 		Quad(
// 			self.0.map(|s| s.insert_into(vocabulary)),
// 			self.1.map(|p| vocabulary.insert(p.as_iri())),
// 			self.2.map(|o| o.insert_into(vocabulary)),
// 			self.3.map(|Meta(g, m)| Meta(g.insert_into(vocabulary), m)),
// 		)
// 	}
// }

// impl<S, L, M>
// 	Quad<
// 		Meta<Term<M, Id<IriBuf, BlankIdBuf>, S, IriBuf, L>, M>,
// 		Meta<Term<M, Id<IriBuf, BlankIdBuf>, S, IriBuf, L>, M>,
// 		Meta<Term<M, Id<IriBuf, BlankIdBuf>, S, IriBuf, L>, M>,
// 		Meta<Term<M, Id<IriBuf, BlankIdBuf>, S, IriBuf, L>, M>,
// 	>
// {
// 	#[allow(clippy::type_complexity)]
// 	pub fn inserted_into<V: VocabularyMut>(
// 		&self,
// 		vocabulary: &mut V,
// 	) -> Quad<
// 		Meta<Term<M, Id<V::Iri, V::BlankId>, S, V::Iri, L>, M>,
// 		Meta<Term<M, Id<V::Iri, V::BlankId>, S, V::Iri, L>, M>,
// 		Meta<Term<M, Id<V::Iri, V::BlankId>, S, V::Iri, L>, M>,
// 		Meta<Term<M, Id<V::Iri, V::BlankId>, S, V::Iri, L>, M>,
// 	>
// 	where
// 		S: Clone,
// 		L: Clone,
// 		M: Clone,
// 	{
// 		Quad(
// 			Meta(self.0.inserted_into(vocabulary), self.0.metadata().clone()),
// 			Meta(self.1.inserted_into(vocabulary), self.1.metadata().clone()),
// 			Meta(self.2.inserted_into(vocabulary), self.2.metadata().clone()),
// 			self.3
// 				.as_ref()
// 				.map(|Meta(g, m)| Meta(g.inserted_into(vocabulary), m.clone())),
// 		)
// 	}

// 	#[allow(clippy::type_complexity)]
// 	pub fn insert_into<V: VocabularyMut>(
// 		self,
// 		vocabulary: &mut V,
// 	) -> Quad<
// 		Meta<Term<M, Id<V::Iri, V::BlankId>, S, V::Iri, L>, M>,
// 		Meta<Term<M, Id<V::Iri, V::BlankId>, S, V::Iri, L>, M>,
// 		Meta<Term<M, Id<V::Iri, V::BlankId>, S, V::Iri, L>, M>,
// 		Meta<Term<M, Id<V::Iri, V::BlankId>, S, V::Iri, L>, M>,
// 	> {
// 		Quad(
// 			self.0.map(|s| s.insert_into(vocabulary)),
// 			self.1.map(|p| p.insert_into(vocabulary)),
// 			self.2.map(|o| o.insert_into(vocabulary)),
// 			self.3.map(|Meta(g, m)| Meta(g.insert_into(vocabulary), m)),
// 		)
// 	}
// }
