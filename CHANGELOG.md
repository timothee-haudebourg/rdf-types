# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.18.4] - 2024-02-27

### Added

- [84b0c84] Add `Triple::as_ref` method.

## [0.18.3] - 2024-01-24

### Added

- [4838539] Impl `ExportFromVocabulary` for `LiteralIndex`.
- [844385c] Impl `Vocabulary` & `Interpretation` for `&T`/`&mutT`.

## [0.18.2] - 2023-12-04

### Fixed

- [baa36c3] Fix bounds on `IdInterpretation`

## [0.18.1] - 2023-10-20

### Added

- [45c1935] Add RDF schema terms as consts.

## [0.18.0] - 2023-10-20

### Build

- [23be2e3] Upgrade `locspan` to version 0.8

## [0.17.5] - 2023-10-18

### Added

- [6afc956] Add default parameter for `Generator`.

### Changed

- [6afc956] Change (swap) type parameters for `WithGenerator`.

## [0.17.4] - 2023-10-18

### Fixed

- [35f411f] Fix `Generator` trait.

## [0.17.3] - 2023-10-18

### Added

- [b80b915] Add some missing `RdfTypeIriWithContext` impls.
- [b80b915] Impl `RdfTypeIriWithContext`  for `Iri`.
- [b80b915] Impl `RdfTypeIriWithContext` for `IriBuf`.
- [b80b915] Impl `RdfTypeIriWithContext` for `&T`.
- [b80b915] Impl `RdfTypeIriWithContext` for `IriOrIndex`.

## [0.17.2] - 2023-10-18

### Added

- [5fb72db] Add missing `DisplayWithContext` implementations.
- [5fb72db] Impl `DisplayWithContext` for `BlankIdIndex`.
- [5fb72db] Impl `DisplayWithContext` for `IriIndex`.
- [5fb72db] Impl `DisplayWithContext` for `LanguageTagIndex`.
- [5fb72db] Impl `DisplayWithContext` for `LiteralIndex`.

## [0.17.1] - 2023-10-18

### Changed

- [96ef48a] Changed (relaxed) bounds for `Id: DisplayWithContext`.
- [96ef48a] Changed (relaxed) bounds for `Id: RdfDisplayWithContext`.
- [96ef48a] Changed (relaxed) bounds for `Id: AsRefWithContext<str>`.

## [0.17.0] - 2023-10-18

### Added

- [2e3aea4] Add `interpretation::WithGenerator`
- [2e3aea4] Add `vocabulary: V` type parameter to `InterpretationMut`.

### Changed

- [2e3aea4] Change return type of `Generator<V>::next` to `Id<V::Iri, V::BlankId>`.

### Removed

- [2e3aea4] Remove `Namespace` trait.

## [0.16.1] - 2023-08-29

### Added

- [4dfcd06] Impl `Copy` for `Literal` and `literal::Type`.

## [0.16.0] - 2023-08-23

### Build

- [5ce4b06] Upgrade `iref`to version 3.0

## [0.15.4] - 2023-06-14

### Added

- [74befc2] Impl `MapLiteral` for `locspan::Meta`.

## [0.15.3] - 2023-06-07

### Added

- [45dee5f] Add `LanguageTagVocabulary` to `Vocabulary` alias.

## [0.15.2] - 2023-06-06

### Added

- [86d68bb] Impl `RdfDisplayWithContext` for base types.

## [0.15.1] - 2023-06-06

### Added

- [3ae5ccb] Impl `RdfDisplay` for `LanguageTag`.

## [0.15.0] - 2023-06-06

### Added

- [cde1bba] Add `literal::Type` type.
- [cde1bba] Add `InsertIntoVocabulary` trait.
- [cde1bba] Add `InsertedIntoVocabulary` trait.
- [cde1bba] Add `LiteralVocabulary` trait.
- [cde1bba] Add `LanguageTagVocabulary` trait.
- [390d3f5] Add interpretation traits.

### Changed

- [cde1bba] Change `Literal` type.

## [0.14.9] - 2023-05-19

### Added

- [814da24] Add `serde` support.

## [0.14.8] - 2023-05-17

### Added

- [716f385] Impl `IntoLiteral` for `Literal<S, T, L>`.
- [19ebd6e] Impl `IntoTerm` for `Term<I, L>`.
- [a8a7237] Impl `From<Index>` for `usize`.

## [0.14.6] - 2023-04-20

### Added

- [1061e0b] Impl `IntoId` for `Id<I, B>`.

## [0.14.5] - 2023-04-18

### Added

- [344d6b5] Add `Export*` traits.
- [344d6b5] Add `ExportQuad` trait.
- [344d6b5] Add `ExportId` trait.
- [344d6b5] Add `ExportTerm` trait.
- [344d6b5] Add `ExportLiteral` trait.
- [344d6b5] Add `IriVocabulary::owned_iri` method.
- [344d6b5] Add `BlankIdVocabulary::owned_blank_id` method.

## [0.14.4] - 2023-04-13

### Added

- [3326c22] Add standard reference types.

## [0.14.3] - 2023-04-11

### Changed

- [ab2c615] Change `S` bound in `Literal` display with context
- [ab2c615] Changed `S: Display` to `S: RdfDisplay` in `Literal: DisplayWithContext`
- [ab2c615] Changed `S: Display` to `S: RdfDisplay` in `Literal: RdfDisplayWithContext`.

## [0.14.2] - 2023-02-28

### Added

- [74932dd] Add `try_into_blank` and `try_into_iri`.

## [0.14.1] - 2023-02-28

### Added

- [a6617aa] Add `MaybeBlankId` and `MaybeIri` traits.
- [a6617aa] Add `AsBlankId` and `AsIri` traits.
- [a6617aa] Add `IntoBlankId` and `IntoIri` traits.
- [a6617aa] Add `FromBlankId` and `FromIri` traits.
- [69e9cbe] Add `Term::blank` and `Term::iri` constructors.

## [0.14.0] - 2023-02-28

### Added

- [6f1d2b6] Added a `Namespace` trait.
- [6f1d2b6] Added `AsRdfTerm` trait.
- [27f0a5e] Add `CHANGELOG.md` file.

### Changed

- [6f1d2b6] Changed type parameters of `Term`.
- [6f1d2b6] Changed meaning of the `I` param in `Term`: it is now the node id type.
- [6f1d2b6] Changed the `Generator` parameter: `V: Vocabulary` -> 'N: Namespace'.

### Removed

- [6f1d2b6] Removed the `B` param in `Term`, now included in `I`.
- [6f1d2b6] Removed `*Ref` types.

## [0.13.0] - 2023-02-28

### Changed

- [f8b3571] Change: factorize `Subject` (now `Id`) type in `Term`.
- [f8b3571] Changed `Subject` type name for `Id` (a type alias remains).

### Removed

- [f8b3571] Removed `Term` variants `Iri` and `Blank` for a single `Id` variant.

## [0.12.19] - 2023-02-20

### Added

- [621caf6] Add `Triple::strip_all_but_predicate`.

## [0.12.18] - 2023-02-20

### Added

- [fdffc81] Add `Triple::into_quad` and `Quad::into_triple`.

## [0.12.17] - 2023-01-24

### Added

- [59cb892] Add `PartialEq<str>` impl for `BlankId`.
- [a902355] Add `vocabulary::Scoped` & `genarator::Unscoped`.

## [0.12.16] - 2023-01-20

### Added

- [82a66d2] Add `insert_into` function to `meta` types.
- [82a66d2] Add more type parameters to `meta::Term` & `meta::Object`.

## [0.12.15] - 2023-01-09

### Fixed

- [4955511] Fix RDF display of IRIs.

## [0.12.14] - 2023-01-05

### Added

- [c4eabdf] Add `Subject::into_iri` and `into_blank`.

## [0.12.13] - 2022-12-20

### Added

- [c63d3b2] Add `QuadRef` from `Quad`-of-refs conversion.

## [0.12.12] - 2022-12-20

### Fixed

- [5d58558] Fix `GrdfQuad::as_quad_ref` name into `as_grdf_quad_ref`.

## [0.12.11] - 2022-12-20

### Added

- [bc79911] Add mut accessors & owned/borrowed conversions.

## [0.12.10] - 2022-12-19

### Changed

- [26ac486] Change string literal display function to be compatible with URDNA2015.

## [0.12.9] - 2022-12-14

### Added

- [28e67fa] Add `insert_into` methods for `Quad` & `Triple`.

## [0.12.8] - 2022-12-14

### Added

- [4ddc0fd] Add `insert_into` method for `Subject`.

## [0.12.7] - 2022-12-14

### Added

- [c5d8e30] Add `Strip` impl for `Subject`.

## [0.12.6] - 2022-12-14

### Added

- [4357754] Add `strip_all_but_predicate` & map functions.

## [0.12.5] - 2022-12-14

### Added

- [7d9c1e2] Add `insert_into` method to insert into vocab.

## [0.12.2] - 2022-10-21

### Added

- [fc97f5e] impl `RdfDisplay` for `IriIndex`/`BlankIdIndex`.

## [0.12.1] - 2022-10-21

### Added

- [31c3d3b] Add `RdfDisplay` impl for `Quad`/`Triple`.

## [0.11.0] - 2022-10-20

### Added

- [e4b35a1] Add `RdfDisplay` trait.

### Changed

- [e4b35a1] Changes the way terms are displayed.

## [0.10.4] - 2022-10-20

### Added

- [7dcf841] Add `with_metadata` methods for generators.

## [0.10.3] - 2022-10-20

### Added

- [35fdd1d] Impl `MetaGenerator` for `&mut G`.

## [0.10.2] - 2022-10-20

### Added

- [d67fe91] Add `Subject::as_str`.

### Fixed

- [d67fe91] Fix generator tests.

## [0.10.1] - 2022-10-20

### Added

- [b71180e] Add README.md
- [cbba710] Add `new` constructors.
- [ac82e7d] Add blank id creation utility functions.
- [3174eaa] Add CI.
- [d35c668] Add more functions.
- [29bcfdf] Add derives.
- [b6d2f49] Add `Debug` impl on `InvalidBlankId`
- [2df77fc] Add `Display` impl for terms.
- [2d04c7a] Add `BlankId` `PartialEq` impls.
- [617148f] Add `locspan::Strip` impls.
- [8b0c3d8] Add `gRDF` related functions & types.
- [f5f597b] Add `Strip` impl for quads & triples.
- [2ae2f4e] Add convenient `Loc` type aliases.
- [c60d1a5] Add `StrippedPartialEq` impl in `loc` module.
- [1536170] Add parameters to `Term`.
- [db23eed] Add `Borrow` impl for `&BlankIdBuf`.
- [9ac6deb] Add `Term`/`Object` in `loc` module.
- [0224b0f] Add type parameters to the `Literal` type.
- [f7d8f29] Add `PartialEq<BlankIdBuf>` impl for `&BlankId`.
- [fcf4591] Add `Stripped*` impls for `Term` and `Subject`.
- [a7c373a] Add `Debug` impls.
- [5fe7505] Add `AsRef<str>` impls.
- [0d59ff1] Add more `AsRef` impls for `BlankId*`.
- [eb70cea] Add vocabulary types and traits.
- [49c89c4] Add `AsStrWithVocabulary` trait.
- [dd57479] Add `IntoStrWithVocabulary` trait.
- [ed344d8] Add subject id generators.

### Build

- [8f34716] Upgrade `locspan` to version 0.7
- [f4f908a] Upgrade `langtag` to 0.3, move to 0.8.

### Changed

- [9fc6cd6] Move to version 0.1.2
- [046c463] Move to 0.2.0
- [6ac8c9d] Move to version `0.7.1`

### Fixed

- [2216d99] Fix `loc` feature.
- [959ace4] Fix wront iri type.
- [5cfc14b] Fix the `loc` module.
- [72a9b46] Fix loc module.
- [313ed93] Fix `Subject::into_term` signature.
- [ff9661f] Fix type parameters on `loc::Literal`
- [62b6175] Fix `Stripped*` impls.
- [845ee33] Fix `Stripped*` impls for `meta::Literal`

### Removed

- [78aa012] Remove one keyword.

