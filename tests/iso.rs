//! This file is auto generated using the
//! `tests/utils/generate-iso-test.rb` script.
use rdf_types::{BTreeDataset, Quad, pattern::Pattern};
use std::collections::BTreeMap;

type Term = Pattern<u32, u32>;

fn test(a: BTreeDataset<Term>, b: BTreeDataset<Term>) {
	match rdf_types::find_bijection(&a, &b) {
		Some(bijection) => {
			let substitution: BTreeMap<u32, u32> = bijection
				.forward
				.into_iter()
				.map(|(a, b)| (*a, *b))
				.collect();
			let c: BTreeDataset<Term> = a
				.into_iter()
				.map(|q| {
					q.map(|t| match t {
						Term::Var(x) => Term::Var(*substitution.get(&x).unwrap()),
						Term::Ground(_) => t,
					})
				})
				.collect();
			assert_eq!(c, b)
		}
		None => panic!("no substitution found"),
	}
}
#[test]
fn iso_001() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Ground(0), Term::Var(0), Term::Ground(0), None));
	a.insert(Quad(Term::Ground(1), Term::Var(0), Term::Ground(3), None));
	a.insert(Quad(
		Term::Ground(2),
		Term::Ground(1),
		Term::Ground(3),
		None,
	));
	a.insert(Quad(Term::Ground(2), Term::Ground(0), Term::Var(0), None));
	a.insert(Quad(Term::Ground(2), Term::Var(0), Term::Ground(1), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(0),
		Term::Var(1000),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Var(1000),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(1),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(0),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Var(1000),
		Term::Ground(1),
		None,
	));
	test(a, b)
}
#[test]
fn iso_002() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(
		Term::Ground(0),
		Term::Ground(3),
		Term::Ground(3),
		None,
	));
	a.insert(Quad(
		Term::Ground(3),
		Term::Ground(3),
		Term::Ground(1),
		None,
	));
	a.insert(Quad(
		Term::Ground(1),
		Term::Ground(1),
		Term::Ground(3),
		None,
	));
	a.insert(Quad(
		Term::Ground(2),
		Term::Ground(1),
		Term::Ground(1),
		None,
	));
	a.insert(Quad(Term::Ground(3), Term::Var(0), Term::Ground(2), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(3),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(3),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(1),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(1),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Var(1000),
		Term::Ground(2),
		None,
	));
	test(a, b)
}
#[test]
fn iso_003() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Ground(1), Term::Var(0), Term::Ground(1), None));
	a.insert(Quad(Term::Ground(1), Term::Ground(0), Term::Var(0), None));
	a.insert(Quad(
		Term::Ground(0),
		Term::Ground(1),
		Term::Ground(1),
		None,
	));
	a.insert(Quad(Term::Ground(1), Term::Ground(3), Term::Var(0), None));
	a.insert(Quad(Term::Ground(3), Term::Var(0), Term::Ground(1), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(1),
		Term::Var(1000),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(0),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(1),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(3),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Var(1000),
		Term::Ground(1),
		None,
	));
	test(a, b)
}
#[test]
fn iso_004() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(
		Term::Ground(3),
		Term::Ground(3),
		Term::Ground(1),
		None,
	));
	a.insert(Quad(Term::Ground(2), Term::Var(0), Term::Ground(0), None));
	a.insert(Quad(Term::Ground(0), Term::Var(0), Term::Ground(1), None));
	a.insert(Quad(Term::Ground(3), Term::Var(0), Term::Ground(3), None));
	a.insert(Quad(Term::Ground(3), Term::Var(0), Term::Ground(2), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(3),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Var(1000),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Var(1000),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Var(1000),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Var(1000),
		Term::Ground(2),
		None,
	));
	test(a, b)
}
#[test]
fn iso_005() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Ground(3), Term::Var(0), Term::Ground(0), None));
	a.insert(Quad(Term::Ground(2), Term::Var(0), Term::Var(0), None));
	a.insert(Quad(Term::Ground(1), Term::Ground(2), Term::Var(0), None));
	a.insert(Quad(Term::Var(0), Term::Var(0), Term::Ground(1), None));
	a.insert(Quad(Term::Var(0), Term::Ground(3), Term::Ground(0), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(3),
		Term::Var(1000),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Var(1000),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(2),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Var(1000),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(3),
		Term::Ground(0),
		None,
	));
	test(a, b)
}
#[test]
fn iso_006() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Ground(2), Term::Var(0), Term::Ground(3), None));
	a.insert(Quad(Term::Ground(0), Term::Var(0), Term::Ground(3), None));
	a.insert(Quad(Term::Var(0), Term::Ground(2), Term::Var(0), None));
	a.insert(Quad(Term::Var(0), Term::Ground(2), Term::Ground(1), None));
	a.insert(Quad(
		Term::Ground(1),
		Term::Ground(3),
		Term::Ground(1),
		None,
	));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(2),
		Term::Var(1000),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Var(1000),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(2),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(2),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(3),
		Term::Ground(1),
		None,
	));
	test(a, b)
}
#[test]
fn iso_007() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Ground(1), Term::Var(0), Term::Ground(0), None));
	a.insert(Quad(
		Term::Ground(0),
		Term::Ground(3),
		Term::Ground(0),
		None,
	));
	a.insert(Quad(Term::Ground(2), Term::Var(0), Term::Ground(3), None));
	a.insert(Quad(
		Term::Ground(3),
		Term::Ground(3),
		Term::Ground(3),
		None,
	));
	a.insert(Quad(Term::Var(0), Term::Ground(3), Term::Ground(0), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(1),
		Term::Var(1000),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(3),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Var(1000),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(3),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(3),
		Term::Ground(0),
		None,
	));
	test(a, b)
}
#[test]
fn iso_008() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(
		Term::Ground(3),
		Term::Ground(0),
		Term::Ground(0),
		None,
	));
	a.insert(Quad(
		Term::Ground(1),
		Term::Ground(0),
		Term::Ground(0),
		None,
	));
	a.insert(Quad(
		Term::Ground(0),
		Term::Ground(0),
		Term::Ground(0),
		None,
	));
	a.insert(Quad(Term::Ground(2), Term::Ground(1), Term::Var(0), None));
	a.insert(Quad(
		Term::Ground(0),
		Term::Ground(1),
		Term::Ground(2),
		None,
	));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(0),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(0),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(0),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(1),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(1),
		Term::Ground(2),
		None,
	));
	test(a, b)
}
#[test]
fn iso_009() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Ground(0), Term::Ground(0), Term::Var(0), None));
	a.insert(Quad(Term::Var(0), Term::Ground(0), Term::Ground(2), None));
	a.insert(Quad(Term::Ground(0), Term::Ground(3), Term::Var(0), None));
	a.insert(Quad(Term::Ground(3), Term::Ground(1), Term::Var(0), None));
	a.insert(Quad(Term::Var(0), Term::Var(0), Term::Ground(0), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(0),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(0),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(3),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(1),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Var(1000),
		Term::Ground(0),
		None,
	));
	test(a, b)
}
#[test]
fn iso_010() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(
		Term::Ground(2),
		Term::Ground(1),
		Term::Ground(0),
		None,
	));
	a.insert(Quad(
		Term::Ground(3),
		Term::Ground(3),
		Term::Ground(2),
		None,
	));
	a.insert(Quad(Term::Ground(2), Term::Ground(0), Term::Var(0), None));
	a.insert(Quad(Term::Ground(0), Term::Ground(3), Term::Var(0), None));
	a.insert(Quad(Term::Var(0), Term::Var(0), Term::Ground(1), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(1),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(3),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(0),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(3),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Var(1000),
		Term::Ground(1),
		None,
	));
	test(a, b)
}
#[test]
fn iso_011() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Ground(2), Term::Var(0), Term::Ground(2), None));
	a.insert(Quad(
		Term::Ground(2),
		Term::Ground(0),
		Term::Ground(0),
		None,
	));
	a.insert(Quad(Term::Ground(2), Term::Ground(0), Term::Var(1), None));
	a.insert(Quad(Term::Var(0), Term::Ground(1), Term::Ground(2), None));
	a.insert(Quad(Term::Var(0), Term::Ground(2), Term::Var(0), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(2),
		Term::Var(1000),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(0),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(0),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(1),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(2),
		Term::Var(1000),
		None,
	));
	test(a, b)
}
#[test]
fn iso_012() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Ground(2), Term::Var(1), Term::Var(1), None));
	a.insert(Quad(Term::Var(1), Term::Ground(1), Term::Ground(2), None));
	a.insert(Quad(Term::Var(1), Term::Ground(2), Term::Ground(1), None));
	a.insert(Quad(Term::Ground(0), Term::Var(1), Term::Ground(1), None));
	a.insert(Quad(Term::Ground(0), Term::Var(0), Term::Var(1), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(2),
		Term::Var(1001),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(1),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(2),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Var(1001),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Var(1000),
		Term::Var(1001),
		None,
	));
	test(a, b)
}
#[test]
fn iso_013() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(
		Term::Ground(1),
		Term::Ground(1),
		Term::Ground(0),
		None,
	));
	a.insert(Quad(Term::Var(0), Term::Ground(2), Term::Var(0), None));
	a.insert(Quad(Term::Ground(0), Term::Var(0), Term::Var(0), None));
	a.insert(Quad(Term::Ground(2), Term::Ground(2), Term::Var(1), None));
	a.insert(Quad(
		Term::Ground(1),
		Term::Ground(1),
		Term::Ground(2),
		None,
	));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(1),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(2),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Var(1000),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(2),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(1),
		Term::Ground(2),
		None,
	));
	test(a, b)
}
#[test]
fn iso_014() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(
		Term::Ground(2),
		Term::Ground(2),
		Term::Ground(0),
		None,
	));
	a.insert(Quad(Term::Ground(0), Term::Var(1), Term::Ground(1), None));
	a.insert(Quad(Term::Ground(1), Term::Var(0), Term::Ground(2), None));
	a.insert(Quad(Term::Var(1), Term::Var(0), Term::Ground(1), None));
	a.insert(Quad(Term::Var(0), Term::Ground(1), Term::Ground(1), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(2),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Var(1001),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Var(1000),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Var(1000),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(1),
		Term::Ground(1),
		None,
	));
	test(a, b)
}
#[test]
fn iso_015() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(
		Term::Ground(0),
		Term::Ground(2),
		Term::Ground(2),
		None,
	));
	a.insert(Quad(
		Term::Ground(1),
		Term::Ground(2),
		Term::Ground(0),
		None,
	));
	a.insert(Quad(Term::Ground(1), Term::Var(1), Term::Ground(2), None));
	a.insert(Quad(Term::Ground(1), Term::Var(1), Term::Var(0), None));
	a.insert(Quad(Term::Ground(0), Term::Var(0), Term::Ground(2), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(2),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(2),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Var(1001),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Var(1001),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Var(1000),
		Term::Ground(2),
		None,
	));
	test(a, b)
}
#[test]
fn iso_016() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Var(1), Term::Ground(0), Term::Ground(0), None));
	a.insert(Quad(
		Term::Ground(1),
		Term::Ground(0),
		Term::Ground(1),
		None,
	));
	a.insert(Quad(Term::Ground(1), Term::Var(1), Term::Ground(2), None));
	a.insert(Quad(Term::Ground(2), Term::Var(1), Term::Ground(2), None));
	a.insert(Quad(Term::Ground(1), Term::Var(0), Term::Ground(1), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(0),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(0),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Var(1001),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Var(1001),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Var(1000),
		Term::Ground(1),
		None,
	));
	test(a, b)
}
#[test]
fn iso_017() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Ground(0), Term::Ground(1), Term::Var(1), None));
	a.insert(Quad(Term::Var(1), Term::Ground(1), Term::Ground(1), None));
	a.insert(Quad(Term::Ground(2), Term::Var(0), Term::Ground(0), None));
	a.insert(Quad(Term::Ground(2), Term::Var(1), Term::Ground(2), None));
	a.insert(Quad(Term::Ground(0), Term::Var(1), Term::Var(1), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(1),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(1),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Var(1000),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Var(1001),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Var(1001),
		Term::Var(1001),
		None,
	));
	test(a, b)
}
#[test]
fn iso_018() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(
		Term::Ground(1),
		Term::Ground(2),
		Term::Ground(1),
		None,
	));
	a.insert(Quad(Term::Ground(0), Term::Ground(1), Term::Var(1), None));
	a.insert(Quad(Term::Var(1), Term::Ground(1), Term::Var(0), None));
	a.insert(Quad(Term::Ground(0), Term::Ground(2), Term::Var(0), None));
	a.insert(Quad(Term::Ground(1), Term::Var(1), Term::Ground(1), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(2),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(1),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(1),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(2),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Var(1001),
		Term::Ground(1),
		None,
	));
	test(a, b)
}
#[test]
fn iso_019() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Var(0), Term::Var(1), Term::Var(1), None));
	a.insert(Quad(Term::Ground(1), Term::Ground(0), Term::Var(0), None));
	a.insert(Quad(Term::Var(1), Term::Var(0), Term::Var(0), None));
	a.insert(Quad(Term::Var(0), Term::Ground(0), Term::Ground(1), None));
	a.insert(Quad(Term::Var(0), Term::Var(0), Term::Ground(0), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Var(1000),
		Term::Var(1001),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(0),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Var(1000),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(0),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Var(1000),
		Term::Ground(0),
		None,
	));
	test(a, b)
}
#[test]
fn iso_020() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(
		Term::Ground(1),
		Term::Ground(1),
		Term::Ground(0),
		None,
	));
	a.insert(Quad(
		Term::Ground(0),
		Term::Ground(2),
		Term::Ground(1),
		None,
	));
	a.insert(Quad(Term::Var(1), Term::Ground(0), Term::Ground(2), None));
	a.insert(Quad(Term::Ground(1), Term::Var(0), Term::Var(0), None));
	a.insert(Quad(Term::Ground(0), Term::Ground(1), Term::Var(1), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(1),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(2),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(0),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Var(1000),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(1),
		Term::Var(1001),
		None,
	));
	test(a, b)
}
#[test]
fn iso_021() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Var(1), Term::Var(0), Term::Var(0), None));
	a.insert(Quad(Term::Var(0), Term::Ground(1), Term::Var(1), None));
	a.insert(Quad(Term::Ground(2), Term::Var(1), Term::Ground(1), None));
	a.insert(Quad(
		Term::Ground(1),
		Term::Ground(0),
		Term::Ground(2),
		None,
	));
	a.insert(Quad(Term::Var(1), Term::Ground(1), Term::Ground(2), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Var(1001),
		Term::Var(1000),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(1),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Var(1001),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(0),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(1),
		Term::Ground(2),
		None,
	));
	test(a, b)
}
#[test]
fn iso_022() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Ground(0), Term::Var(1), Term::Ground(1), None));
	a.insert(Quad(Term::Ground(1), Term::Ground(0), Term::Var(0), None));
	a.insert(Quad(
		Term::Ground(2),
		Term::Ground(2),
		Term::Ground(0),
		None,
	));
	a.insert(Quad(
		Term::Ground(1),
		Term::Ground(0),
		Term::Ground(1),
		None,
	));
	a.insert(Quad(Term::Var(0), Term::Var(1), Term::Ground(2), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(0),
		Term::Var(1001),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(0),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(2),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(0),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Var(1001),
		Term::Ground(2),
		None,
	));
	test(a, b)
}
#[test]
fn iso_023() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Var(0), Term::Ground(0), Term::Ground(2), None));
	a.insert(Quad(
		Term::Ground(1),
		Term::Ground(2),
		Term::Ground(0),
		None,
	));
	a.insert(Quad(
		Term::Ground(2),
		Term::Ground(2),
		Term::Ground(2),
		None,
	));
	a.insert(Quad(Term::Var(0), Term::Ground(1), Term::Var(0), None));
	a.insert(Quad(Term::Var(0), Term::Var(1), Term::Var(0), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(0),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(2),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(2),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(1),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Var(1001),
		Term::Var(1000),
		None,
	));
	test(a, b)
}
#[test]
fn iso_024() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Ground(2), Term::Var(1), Term::Ground(1), None));
	a.insert(Quad(Term::Ground(0), Term::Ground(0), Term::Var(1), None));
	a.insert(Quad(Term::Ground(2), Term::Var(0), Term::Var(0), None));
	a.insert(Quad(Term::Ground(1), Term::Var(0), Term::Ground(0), None));
	a.insert(Quad(Term::Ground(2), Term::Ground(2), Term::Var(0), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(2),
		Term::Var(1001),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(0),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Var(1000),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Var(1000),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(2),
		Term::Var(1000),
		None,
	));
	test(a, b)
}
#[test]
fn iso_025() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(
		Term::Ground(0),
		Term::Ground(0),
		Term::Ground(2),
		None,
	));
	a.insert(Quad(Term::Ground(1), Term::Var(0), Term::Var(0), None));
	a.insert(Quad(Term::Var(0), Term::Ground(2), Term::Ground(0), None));
	a.insert(Quad(Term::Ground(1), Term::Var(1), Term::Ground(0), None));
	a.insert(Quad(Term::Ground(1), Term::Ground(1), Term::Var(1), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(0),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Var(1000),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(2),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Var(1001),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(1),
		Term::Var(1001),
		None,
	));
	test(a, b)
}
#[test]
fn iso_026() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Var(0), Term::Var(1), Term::Ground(0), None));
	a.insert(Quad(
		Term::Ground(0),
		Term::Ground(2),
		Term::Ground(2),
		None,
	));
	a.insert(Quad(Term::Ground(1), Term::Ground(0), Term::Var(1), None));
	a.insert(Quad(Term::Ground(2), Term::Var(1), Term::Var(0), None));
	a.insert(Quad(Term::Var(0), Term::Var(1), Term::Var(0), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Var(1000),
		Term::Var(1001),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(2),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(0),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Var(1001),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Var(1001),
		Term::Var(1000),
		None,
	));
	test(a, b)
}
#[test]
fn iso_027() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Var(0), Term::Ground(1), Term::Ground(1), None));
	a.insert(Quad(Term::Ground(0), Term::Ground(2), Term::Var(1), None));
	a.insert(Quad(Term::Var(0), Term::Var(1), Term::Var(0), None));
	a.insert(Quad(Term::Ground(2), Term::Var(0), Term::Ground(2), None));
	a.insert(Quad(Term::Var(0), Term::Var(0), Term::Ground(0), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(1),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(2),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Var(1001),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Var(1000),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Var(1000),
		Term::Ground(0),
		None,
	));
	test(a, b)
}
#[test]
fn iso_028() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(
		Term::Ground(1),
		Term::Ground(1),
		Term::Ground(1),
		None,
	));
	a.insert(Quad(Term::Var(0), Term::Var(1), Term::Ground(0), None));
	a.insert(Quad(Term::Ground(2), Term::Var(0), Term::Var(1), None));
	a.insert(Quad(Term::Var(1), Term::Var(1), Term::Ground(1), None));
	a.insert(Quad(Term::Var(0), Term::Var(1), Term::Var(1), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(1),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Var(1001),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Var(1000),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Var(1001),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Var(1001),
		Term::Var(1001),
		None,
	));
	test(a, b)
}
#[test]
fn iso_029() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Var(0), Term::Ground(1), Term::Ground(0), None));
	a.insert(Quad(Term::Var(0), Term::Ground(1), Term::Ground(1), None));
	a.insert(Quad(Term::Var(0), Term::Ground(2), Term::Var(0), None));
	a.insert(Quad(Term::Ground(0), Term::Var(0), Term::Ground(1), None));
	a.insert(Quad(
		Term::Ground(0),
		Term::Ground(2),
		Term::Ground(0),
		None,
	));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(1),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(1),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(2),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Var(1000),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(2),
		Term::Ground(0),
		None,
	));
	test(a, b)
}
#[test]
fn iso_030() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(
		Term::Ground(0),
		Term::Ground(1),
		Term::Ground(2),
		None,
	));
	a.insert(Quad(Term::Var(1), Term::Var(0), Term::Var(0), None));
	a.insert(Quad(Term::Var(0), Term::Ground(0), Term::Ground(2), None));
	a.insert(Quad(Term::Ground(2), Term::Var(0), Term::Var(0), None));
	a.insert(Quad(Term::Var(0), Term::Ground(1), Term::Ground(1), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(1),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Var(1000),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(0),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Var(1000),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(1),
		Term::Ground(1),
		None,
	));
	test(a, b)
}
#[test]
fn iso_031() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(
		Term::Ground(0),
		Term::Ground(1),
		Term::Ground(2),
		None,
	));
	a.insert(Quad(Term::Ground(5), Term::Ground(3), Term::Var(2), None));
	a.insert(Quad(Term::Ground(6), Term::Var(1), Term::Ground(0), None));
	a.insert(Quad(Term::Var(1), Term::Ground(4), Term::Ground(6), None));
	a.insert(Quad(Term::Var(1), Term::Ground(2), Term::Var(0), None));
	a.insert(Quad(
		Term::Ground(2),
		Term::Ground(6),
		Term::Ground(2),
		None,
	));
	a.insert(Quad(Term::Var(2), Term::Ground(3), Term::Ground(6), None));
	a.insert(Quad(
		Term::Ground(6),
		Term::Ground(3),
		Term::Ground(4),
		None,
	));
	a.insert(Quad(Term::Var(2), Term::Ground(5), Term::Ground(2), None));
	a.insert(Quad(
		Term::Ground(6),
		Term::Ground(1),
		Term::Ground(0),
		None,
	));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(1),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Ground(3),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Var(1001),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(4),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(2),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(6),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Ground(3),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Ground(3),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Ground(5),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Ground(1),
		Term::Ground(0),
		None,
	));
	test(a, b)
}
#[test]
fn iso_032() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(
		Term::Ground(6),
		Term::Ground(3),
		Term::Ground(2),
		None,
	));
	a.insert(Quad(Term::Ground(4), Term::Var(0), Term::Ground(4), None));
	a.insert(Quad(Term::Var(2), Term::Ground(3), Term::Var(2), None));
	a.insert(Quad(Term::Ground(5), Term::Ground(1), Term::Var(2), None));
	a.insert(Quad(Term::Ground(5), Term::Var(2), Term::Ground(1), None));
	a.insert(Quad(Term::Ground(3), Term::Ground(0), Term::Var(0), None));
	a.insert(Quad(Term::Var(0), Term::Var(1), Term::Ground(4), None));
	a.insert(Quad(Term::Ground(0), Term::Var(2), Term::Ground(1), None));
	a.insert(Quad(Term::Ground(2), Term::Var(0), Term::Ground(4), None));
	a.insert(Quad(Term::Var(0), Term::Ground(1), Term::Ground(2), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(6),
		Term::Ground(3),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Var(1000),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Ground(3),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Ground(1),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Var(1002),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(0),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Var(1001),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Var(1002),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Var(1000),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(1),
		Term::Ground(2),
		None,
	));
	test(a, b)
}
#[test]
fn iso_033() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(
		Term::Ground(4),
		Term::Ground(4),
		Term::Ground(4),
		None,
	));
	a.insert(Quad(Term::Var(0), Term::Ground(1), Term::Ground(1), None));
	a.insert(Quad(
		Term::Ground(3),
		Term::Ground(0),
		Term::Ground(4),
		None,
	));
	a.insert(Quad(Term::Ground(0), Term::Var(1), Term::Var(0), None));
	a.insert(Quad(
		Term::Ground(2),
		Term::Ground(1),
		Term::Ground(0),
		None,
	));
	a.insert(Quad(Term::Var(1), Term::Ground(1), Term::Ground(0), None));
	a.insert(Quad(Term::Var(1), Term::Ground(3), Term::Ground(2), None));
	a.insert(Quad(Term::Ground(5), Term::Ground(1), Term::Var(0), None));
	a.insert(Quad(Term::Var(2), Term::Var(1), Term::Ground(4), None));
	a.insert(Quad(Term::Var(0), Term::Ground(6), Term::Ground(1), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(4),
		Term::Ground(4),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(1),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(0),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Var(1001),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(1),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(1),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(3),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Ground(1),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Var(1001),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(6),
		Term::Ground(1),
		None,
	));
	test(a, b)
}
#[test]
fn iso_034() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Var(1), Term::Ground(5), Term::Var(0), None));
	a.insert(Quad(Term::Ground(2), Term::Var(0), Term::Ground(3), None));
	a.insert(Quad(Term::Ground(3), Term::Ground(2), Term::Var(2), None));
	a.insert(Quad(Term::Var(1), Term::Ground(5), Term::Ground(4), None));
	a.insert(Quad(Term::Ground(6), Term::Var(0), Term::Ground(6), None));
	a.insert(Quad(Term::Ground(0), Term::Var(0), Term::Ground(4), None));
	a.insert(Quad(Term::Var(0), Term::Ground(5), Term::Var(2), None));
	a.insert(Quad(
		Term::Ground(5),
		Term::Ground(1),
		Term::Ground(0),
		None,
	));
	a.insert(Quad(Term::Var(1), Term::Ground(1), Term::Ground(5), None));
	a.insert(Quad(Term::Var(0), Term::Var(2), Term::Ground(0), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(5),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Var(1000),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(2),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(5),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Var(1000),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Var(1000),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(5),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Ground(1),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(1),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Var(1002),
		Term::Ground(0),
		None,
	));
	test(a, b)
}
#[test]
fn iso_035() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Ground(4), Term::Ground(2), Term::Var(1), None));
	a.insert(Quad(Term::Var(0), Term::Ground(6), Term::Ground(1), None));
	a.insert(Quad(Term::Var(1), Term::Var(0), Term::Ground(4), None));
	a.insert(Quad(Term::Var(0), Term::Ground(4), Term::Ground(3), None));
	a.insert(Quad(
		Term::Ground(0),
		Term::Ground(5),
		Term::Ground(0),
		None,
	));
	a.insert(Quad(Term::Var(0), Term::Ground(0), Term::Ground(4), None));
	a.insert(Quad(Term::Ground(1), Term::Var(0), Term::Ground(3), None));
	a.insert(Quad(
		Term::Ground(2),
		Term::Ground(0),
		Term::Ground(4),
		None,
	));
	a.insert(Quad(
		Term::Ground(6),
		Term::Ground(3),
		Term::Ground(6),
		None,
	));
	a.insert(Quad(
		Term::Ground(0),
		Term::Ground(0),
		Term::Ground(3),
		None,
	));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(4),
		Term::Ground(2),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(6),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Var(1000),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(4),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(5),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(0),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Var(1000),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(0),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Ground(3),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(0),
		Term::Ground(3),
		None,
	));
	test(a, b)
}
#[test]
fn iso_036() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Var(0), Term::Var(1), Term::Ground(4), None));
	a.insert(Quad(
		Term::Ground(4),
		Term::Ground(6),
		Term::Ground(6),
		None,
	));
	a.insert(Quad(Term::Ground(2), Term::Ground(3), Term::Var(1), None));
	a.insert(Quad(
		Term::Ground(0),
		Term::Ground(4),
		Term::Ground(2),
		None,
	));
	a.insert(Quad(
		Term::Ground(3),
		Term::Ground(1),
		Term::Ground(1),
		None,
	));
	a.insert(Quad(Term::Ground(6), Term::Ground(3), Term::Var(0), None));
	a.insert(Quad(
		Term::Ground(3),
		Term::Ground(1),
		Term::Ground(4),
		None,
	));
	a.insert(Quad(
		Term::Ground(3),
		Term::Ground(4),
		Term::Ground(6),
		None,
	));
	a.insert(Quad(Term::Ground(1), Term::Var(0), Term::Ground(0), None));
	a.insert(Quad(Term::Ground(5), Term::Ground(0), Term::Var(0), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Var(1000),
		Term::Var(1001),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Ground(6),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(3),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(4),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(1),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Ground(3),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(1),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(4),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Var(1000),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Ground(0),
		Term::Var(1000),
		None,
	));
	test(a, b)
}
#[test]
fn iso_037() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(
		Term::Ground(1),
		Term::Ground(2),
		Term::Ground(4),
		None,
	));
	a.insert(Quad(
		Term::Ground(4),
		Term::Ground(6),
		Term::Ground(5),
		None,
	));
	a.insert(Quad(Term::Var(1), Term::Ground(0), Term::Var(1), None));
	a.insert(Quad(
		Term::Ground(4),
		Term::Ground(1),
		Term::Ground(5),
		None,
	));
	a.insert(Quad(Term::Ground(4), Term::Ground(3), Term::Var(1), None));
	a.insert(Quad(Term::Var(2), Term::Ground(1), Term::Ground(5), None));
	a.insert(Quad(
		Term::Ground(6),
		Term::Ground(2),
		Term::Ground(6),
		None,
	));
	a.insert(Quad(Term::Var(2), Term::Var(1), Term::Ground(2), None));
	a.insert(Quad(Term::Ground(3), Term::Var(1), Term::Ground(3), None));
	a.insert(Quad(
		Term::Ground(5),
		Term::Ground(6),
		Term::Ground(2),
		None,
	));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(2),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Ground(6),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(0),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Ground(1),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Ground(3),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Ground(1),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Ground(2),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Var(1001),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Var(1001),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Ground(6),
		Term::Ground(2),
		None,
	));
	test(a, b)
}
#[test]
fn iso_038() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Var(0), Term::Var(0), Term::Ground(4), None));
	a.insert(Quad(
		Term::Ground(0),
		Term::Ground(5),
		Term::Ground(3),
		None,
	));
	a.insert(Quad(
		Term::Ground(0),
		Term::Ground(5),
		Term::Ground(2),
		None,
	));
	a.insert(Quad(Term::Ground(5), Term::Ground(1), Term::Var(2), None));
	a.insert(Quad(Term::Var(0), Term::Ground(5), Term::Ground(1), None));
	a.insert(Quad(
		Term::Ground(6),
		Term::Ground(4),
		Term::Ground(1),
		None,
	));
	a.insert(Quad(Term::Ground(2), Term::Var(2), Term::Var(1), None));
	a.insert(Quad(Term::Var(2), Term::Var(0), Term::Ground(6), None));
	a.insert(Quad(
		Term::Ground(2),
		Term::Ground(5),
		Term::Ground(3),
		None,
	));
	a.insert(Quad(Term::Ground(6), Term::Ground(2), Term::Var(1), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Var(1000),
		Term::Var(1000),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(5),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(5),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Ground(1),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(5),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Ground(4),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Var(1002),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Var(1000),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(5),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Ground(2),
		Term::Var(1001),
		None,
	));
	test(a, b)
}
#[test]
fn iso_039() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Ground(1), Term::Var(0), Term::Ground(3), None));
	a.insert(Quad(
		Term::Ground(4),
		Term::Ground(3),
		Term::Ground(5),
		None,
	));
	a.insert(Quad(Term::Ground(4), Term::Var(0), Term::Var(0), None));
	a.insert(Quad(
		Term::Ground(2),
		Term::Ground(2),
		Term::Ground(0),
		None,
	));
	a.insert(Quad(Term::Ground(2), Term::Ground(6), Term::Var(2), None));
	a.insert(Quad(Term::Ground(4), Term::Var(1), Term::Ground(0), None));
	a.insert(Quad(Term::Ground(1), Term::Var(1), Term::Ground(3), None));
	a.insert(Quad(Term::Ground(1), Term::Var(0), Term::Ground(0), None));
	a.insert(Quad(Term::Ground(5), Term::Var(2), Term::Ground(2), None));
	a.insert(Quad(
		Term::Ground(4),
		Term::Ground(0),
		Term::Ground(4),
		None,
	));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(1),
		Term::Var(1000),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Ground(3),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Var(1000),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(2),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(6),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Var(1001),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Var(1001),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Var(1000),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Var(1002),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Ground(0),
		Term::Ground(4),
		None,
	));
	test(a, b)
}
#[test]
fn iso_040() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(
		Term::Ground(5),
		Term::Ground(4),
		Term::Ground(3),
		None,
	));
	a.insert(Quad(
		Term::Ground(3),
		Term::Ground(4),
		Term::Ground(3),
		None,
	));
	a.insert(Quad(Term::Ground(5), Term::Ground(6), Term::Var(2), None));
	a.insert(Quad(
		Term::Ground(1),
		Term::Ground(6),
		Term::Ground(4),
		None,
	));
	a.insert(Quad(Term::Ground(6), Term::Var(1), Term::Var(1), None));
	a.insert(Quad(Term::Var(0), Term::Ground(2), Term::Ground(2), None));
	a.insert(Quad(Term::Ground(0), Term::Var(2), Term::Ground(3), None));
	a.insert(Quad(Term::Var(0), Term::Ground(2), Term::Ground(1), None));
	a.insert(Quad(
		Term::Ground(0),
		Term::Ground(2),
		Term::Ground(1),
		None,
	));
	a.insert(Quad(
		Term::Ground(5),
		Term::Ground(3),
		Term::Ground(3),
		None,
	));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(5),
		Term::Ground(4),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(4),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Ground(6),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(6),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Var(1001),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(2),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Var(1002),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(2),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(2),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Ground(3),
		Term::Ground(3),
		None,
	));
	test(a, b)
}
#[test]
fn iso_041() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(
		Term::Ground(0),
		Term::Ground(6),
		Term::Ground(6),
		None,
	));
	a.insert(Quad(Term::Var(0), Term::Ground(3), Term::Ground(2), None));
	a.insert(Quad(
		Term::Ground(4),
		Term::Ground(2),
		Term::Ground(1),
		None,
	));
	a.insert(Quad(
		Term::Ground(6),
		Term::Ground(5),
		Term::Ground(6),
		None,
	));
	a.insert(Quad(Term::Ground(2), Term::Var(1), Term::Ground(1), None));
	a.insert(Quad(
		Term::Ground(3),
		Term::Ground(0),
		Term::Ground(5),
		None,
	));
	a.insert(Quad(Term::Ground(5), Term::Var(1), Term::Var(1), None));
	a.insert(Quad(Term::Var(1), Term::Ground(6), Term::Var(0), None));
	a.insert(Quad(
		Term::Ground(1),
		Term::Ground(6),
		Term::Ground(1),
		None,
	));
	a.insert(Quad(Term::Ground(2), Term::Ground(3), Term::Var(2), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(6),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(3),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Ground(2),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Ground(5),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Var(1001),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(0),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Var(1001),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(6),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(6),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(3),
		Term::Var(1002),
		None,
	));
	test(a, b)
}
#[test]
fn iso_042() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Var(1), Term::Var(2), Term::Ground(1), None));
	a.insert(Quad(Term::Ground(1), Term::Var(2), Term::Ground(1), None));
	a.insert(Quad(Term::Ground(4), Term::Ground(4), Term::Var(2), None));
	a.insert(Quad(Term::Ground(5), Term::Var(0), Term::Ground(6), None));
	a.insert(Quad(Term::Ground(0), Term::Var(2), Term::Ground(4), None));
	a.insert(Quad(Term::Ground(5), Term::Var(2), Term::Var(1), None));
	a.insert(Quad(Term::Ground(0), Term::Ground(0), Term::Var(0), None));
	a.insert(Quad(Term::Var(2), Term::Var(0), Term::Ground(5), None));
	a.insert(Quad(Term::Var(1), Term::Ground(2), Term::Var(0), None));
	a.insert(Quad(
		Term::Ground(3),
		Term::Ground(0),
		Term::Ground(6),
		None,
	));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Var(1001),
		Term::Var(1002),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Var(1002),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Ground(4),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Var(1000),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Var(1002),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Var(1002),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(0),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Var(1000),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(2),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(0),
		Term::Ground(6),
		None,
	));
	test(a, b)
}
#[test]
fn iso_043() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(
		Term::Ground(1),
		Term::Ground(4),
		Term::Ground(2),
		None,
	));
	a.insert(Quad(Term::Var(1), Term::Ground(0), Term::Var(0), None));
	a.insert(Quad(Term::Var(1), Term::Ground(4), Term::Ground(4), None));
	a.insert(Quad(
		Term::Ground(6),
		Term::Ground(3),
		Term::Ground(1),
		None,
	));
	a.insert(Quad(Term::Var(2), Term::Ground(1), Term::Ground(4), None));
	a.insert(Quad(Term::Var(2), Term::Ground(6), Term::Ground(0), None));
	a.insert(Quad(Term::Var(2), Term::Ground(1), Term::Ground(2), None));
	a.insert(Quad(Term::Ground(3), Term::Var(1), Term::Var(1), None));
	a.insert(Quad(
		Term::Ground(6),
		Term::Ground(1),
		Term::Ground(0),
		None,
	));
	a.insert(Quad(Term::Var(2), Term::Ground(5), Term::Var(2), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(4),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(0),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(4),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Ground(3),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Ground(1),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Ground(6),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Ground(1),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Var(1001),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Ground(1),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Ground(5),
		Term::Var(1002),
		None,
	));
	test(a, b)
}
#[test]
fn iso_044() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(
		Term::Ground(5),
		Term::Ground(3),
		Term::Ground(0),
		None,
	));
	a.insert(Quad(Term::Ground(6), Term::Ground(3), Term::Var(0), None));
	a.insert(Quad(Term::Ground(4), Term::Var(1), Term::Ground(2), None));
	a.insert(Quad(
		Term::Ground(6),
		Term::Ground(4),
		Term::Ground(1),
		None,
	));
	a.insert(Quad(Term::Ground(0), Term::Var(1), Term::Ground(2), None));
	a.insert(Quad(Term::Var(2), Term::Ground(3), Term::Var(1), None));
	a.insert(Quad(Term::Var(1), Term::Ground(5), Term::Ground(5), None));
	a.insert(Quad(
		Term::Ground(1),
		Term::Ground(0),
		Term::Ground(3),
		None,
	));
	a.insert(Quad(Term::Ground(4), Term::Var(1), Term::Ground(4), None));
	a.insert(Quad(Term::Ground(4), Term::Ground(0), Term::Var(1), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(5),
		Term::Ground(3),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Ground(3),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Var(1001),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Ground(4),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Var(1001),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Ground(3),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(5),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(0),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Var(1001),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Ground(0),
		Term::Var(1001),
		None,
	));
	test(a, b)
}
#[test]
fn iso_045() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Var(1), Term::Ground(2), Term::Ground(3), None));
	a.insert(Quad(
		Term::Ground(0),
		Term::Ground(1),
		Term::Ground(4),
		None,
	));
	a.insert(Quad(
		Term::Ground(1),
		Term::Ground(6),
		Term::Ground(0),
		None,
	));
	a.insert(Quad(
		Term::Ground(6),
		Term::Ground(5),
		Term::Ground(6),
		None,
	));
	a.insert(Quad(Term::Var(2), Term::Ground(2), Term::Ground(0), None));
	a.insert(Quad(Term::Var(0), Term::Ground(5), Term::Ground(5), None));
	a.insert(Quad(
		Term::Ground(1),
		Term::Ground(2),
		Term::Ground(3),
		None,
	));
	a.insert(Quad(Term::Ground(4), Term::Ground(2), Term::Var(2), None));
	a.insert(Quad(Term::Ground(5), Term::Var(0), Term::Ground(0), None));
	a.insert(Quad(Term::Ground(0), Term::Ground(2), Term::Var(2), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(2),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(1),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(6),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Ground(5),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Ground(2),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(5),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(2),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Ground(2),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Var(1000),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(2),
		Term::Var(1002),
		None,
	));
	test(a, b)
}
#[test]
fn iso_046() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Ground(0), Term::Var(1), Term::Var(2), None));
	a.insert(Quad(Term::Ground(3), Term::Var(1), Term::Ground(2), None));
	a.insert(Quad(Term::Ground(0), Term::Ground(2), Term::Var(2), None));
	a.insert(Quad(Term::Var(2), Term::Var(1), Term::Ground(2), None));
	a.insert(Quad(Term::Ground(3), Term::Var(2), Term::Var(0), None));
	a.insert(Quad(Term::Var(1), Term::Ground(4), Term::Ground(2), None));
	a.insert(Quad(
		Term::Ground(6),
		Term::Ground(4),
		Term::Ground(3),
		None,
	));
	a.insert(Quad(Term::Var(2), Term::Ground(6), Term::Ground(3), None));
	a.insert(Quad(
		Term::Ground(4),
		Term::Ground(3),
		Term::Ground(1),
		None,
	));
	a.insert(Quad(
		Term::Ground(5),
		Term::Ground(0),
		Term::Ground(6),
		None,
	));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(0),
		Term::Var(1001),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Var(1001),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(2),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Var(1001),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Var(1002),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(4),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Ground(4),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Ground(6),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Ground(3),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Ground(0),
		Term::Ground(6),
		None,
	));
	test(a, b)
}
#[test]
fn iso_047() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Ground(5), Term::Ground(1), Term::Var(2), None));
	a.insert(Quad(Term::Var(2), Term::Ground(0), Term::Ground(4), None));
	a.insert(Quad(Term::Ground(5), Term::Ground(3), Term::Var(1), None));
	a.insert(Quad(Term::Ground(6), Term::Var(2), Term::Ground(2), None));
	a.insert(Quad(Term::Ground(1), Term::Var(1), Term::Ground(0), None));
	a.insert(Quad(Term::Ground(3), Term::Var(0), Term::Ground(2), None));
	a.insert(Quad(Term::Ground(6), Term::Ground(0), Term::Var(2), None));
	a.insert(Quad(Term::Var(2), Term::Ground(4), Term::Ground(6), None));
	a.insert(Quad(Term::Ground(5), Term::Var(0), Term::Ground(2), None));
	a.insert(Quad(Term::Ground(5), Term::Var(1), Term::Ground(1), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(5),
		Term::Ground(1),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Ground(0),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Ground(3),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Var(1002),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Var(1001),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Var(1000),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Ground(0),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Ground(4),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Var(1000),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Var(1001),
		Term::Ground(1),
		None,
	));
	test(a, b)
}
#[test]
fn iso_048() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Var(1), Term::Ground(4), Term::Ground(2), None));
	a.insert(Quad(
		Term::Ground(4),
		Term::Ground(1),
		Term::Ground(6),
		None,
	));
	a.insert(Quad(Term::Var(1), Term::Ground(6), Term::Ground(4), None));
	a.insert(Quad(
		Term::Ground(3),
		Term::Ground(0),
		Term::Ground(2),
		None,
	));
	a.insert(Quad(Term::Var(0), Term::Ground(6), Term::Var(1), None));
	a.insert(Quad(Term::Ground(0), Term::Ground(2), Term::Var(2), None));
	a.insert(Quad(Term::Ground(3), Term::Ground(6), Term::Var(0), None));
	a.insert(Quad(Term::Ground(1), Term::Var(2), Term::Ground(5), None));
	a.insert(Quad(Term::Var(1), Term::Ground(4), Term::Ground(4), None));
	a.insert(Quad(Term::Ground(4), Term::Ground(6), Term::Var(0), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(4),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Ground(1),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(6),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(0),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(6),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(2),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(6),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Var(1002),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(4),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Ground(6),
		Term::Var(1000),
		None,
	));
	test(a, b)
}
#[test]
fn iso_049() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Var(0), Term::Ground(2), Term::Ground(6), None));
	a.insert(Quad(Term::Var(2), Term::Ground(2), Term::Ground(0), None));
	a.insert(Quad(Term::Ground(2), Term::Var(1), Term::Var(1), None));
	a.insert(Quad(
		Term::Ground(0),
		Term::Ground(1),
		Term::Ground(2),
		None,
	));
	a.insert(Quad(
		Term::Ground(3),
		Term::Ground(1),
		Term::Ground(5),
		None,
	));
	a.insert(Quad(Term::Ground(3), Term::Var(1), Term::Var(0), None));
	a.insert(Quad(Term::Ground(4), Term::Ground(4), Term::Var(0), None));
	a.insert(Quad(Term::Ground(1), Term::Ground(3), Term::Var(2), None));
	a.insert(Quad(
		Term::Ground(6),
		Term::Ground(6),
		Term::Ground(1),
		None,
	));
	a.insert(Quad(
		Term::Ground(3),
		Term::Ground(4),
		Term::Ground(2),
		None,
	));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(2),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Ground(2),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Var(1001),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(1),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(1),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Var(1001),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Ground(4),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(3),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Ground(6),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(4),
		Term::Ground(2),
		None,
	));
	test(a, b)
}
#[test]
fn iso_050() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Ground(6), Term::Ground(5), Term::Var(1), None));
	a.insert(Quad(
		Term::Ground(1),
		Term::Ground(0),
		Term::Ground(0),
		None,
	));
	a.insert(Quad(Term::Var(1), Term::Var(2), Term::Ground(2), None));
	a.insert(Quad(Term::Var(1), Term::Ground(2), Term::Var(0), None));
	a.insert(Quad(
		Term::Ground(5),
		Term::Ground(0),
		Term::Ground(6),
		None,
	));
	a.insert(Quad(Term::Var(1), Term::Ground(4), Term::Ground(6), None));
	a.insert(Quad(Term::Ground(3), Term::Var(0), Term::Ground(0), None));
	a.insert(Quad(
		Term::Ground(5),
		Term::Ground(5),
		Term::Ground(4),
		None,
	));
	a.insert(Quad(
		Term::Ground(3),
		Term::Ground(2),
		Term::Ground(0),
		None,
	));
	a.insert(Quad(
		Term::Ground(6),
		Term::Ground(5),
		Term::Ground(0),
		None,
	));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(6),
		Term::Ground(5),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(0),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Var(1002),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(2),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Ground(0),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(4),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Var(1000),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Ground(5),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(2),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Ground(5),
		Term::Ground(0),
		None,
	));
	test(a, b)
}
#[test]
fn iso_051() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Ground(3), Term::Var(0), Term::Var(1), None));
	a.insert(Quad(Term::Ground(5), Term::Ground(6), Term::Var(0), None));
	a.insert(Quad(
		Term::Ground(4),
		Term::Ground(0),
		Term::Ground(0),
		None,
	));
	a.insert(Quad(Term::Ground(4), Term::Var(2), Term::Ground(0), None));
	a.insert(Quad(Term::Ground(6), Term::Var(0), Term::Ground(3), None));
	a.insert(Quad(Term::Ground(0), Term::Var(1), Term::Ground(0), None));
	a.insert(Quad(
		Term::Ground(3),
		Term::Ground(1),
		Term::Ground(0),
		None,
	));
	a.insert(Quad(Term::Ground(2), Term::Ground(6), Term::Var(1), None));
	a.insert(Quad(Term::Var(1), Term::Ground(4), Term::Ground(6), None));
	a.insert(Quad(Term::Ground(2), Term::Var(1), Term::Ground(4), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(3),
		Term::Var(1000),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Ground(6),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Ground(0),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Var(1002),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Var(1000),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Var(1001),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(1),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(6),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(4),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Var(1001),
		Term::Ground(4),
		None,
	));
	test(a, b)
}
#[test]
fn iso_052() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Var(1), Term::Var(2), Term::Ground(4), None));
	a.insert(Quad(Term::Var(1), Term::Var(1), Term::Ground(2), None));
	a.insert(Quad(
		Term::Ground(5),
		Term::Ground(6),
		Term::Ground(3),
		None,
	));
	a.insert(Quad(Term::Var(2), Term::Ground(2), Term::Ground(2), None));
	a.insert(Quad(Term::Ground(4), Term::Ground(1), Term::Var(0), None));
	a.insert(Quad(Term::Ground(5), Term::Var(1), Term::Ground(4), None));
	a.insert(Quad(
		Term::Ground(3),
		Term::Ground(4),
		Term::Ground(3),
		None,
	));
	a.insert(Quad(Term::Ground(6), Term::Var(1), Term::Var(2), None));
	a.insert(Quad(Term::Ground(1), Term::Var(1), Term::Ground(5), None));
	a.insert(Quad(Term::Ground(5), Term::Ground(1), Term::Var(2), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Var(1001),
		Term::Var(1002),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Var(1001),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Ground(6),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Ground(2),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Ground(1),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Var(1001),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(4),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Var(1001),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Var(1001),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Ground(1),
		Term::Var(1002),
		None,
	));
	test(a, b)
}
#[test]
fn iso_053() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(
		Term::Ground(5),
		Term::Ground(5),
		Term::Ground(1),
		None,
	));
	a.insert(Quad(Term::Ground(4), Term::Var(1), Term::Ground(1), None));
	a.insert(Quad(
		Term::Ground(2),
		Term::Ground(3),
		Term::Ground(5),
		None,
	));
	a.insert(Quad(Term::Var(1), Term::Ground(5), Term::Var(1), None));
	a.insert(Quad(
		Term::Ground(2),
		Term::Ground(1),
		Term::Ground(2),
		None,
	));
	a.insert(Quad(Term::Var(2), Term::Ground(2), Term::Ground(5), None));
	a.insert(Quad(Term::Ground(5), Term::Var(2), Term::Ground(5), None));
	a.insert(Quad(
		Term::Ground(0),
		Term::Ground(1),
		Term::Ground(0),
		None,
	));
	a.insert(Quad(Term::Ground(4), Term::Ground(4), Term::Var(2), None));
	a.insert(Quad(Term::Ground(4), Term::Ground(4), Term::Var(1), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(5),
		Term::Ground(5),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Var(1001),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(3),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(5),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(1),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Ground(2),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Var(1002),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(1),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Ground(4),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Ground(4),
		Term::Var(1001),
		None,
	));
	test(a, b)
}
#[test]
fn iso_054() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Ground(5), Term::Var(2), Term::Var(2), None));
	a.insert(Quad(Term::Var(2), Term::Var(0), Term::Var(0), None));
	a.insert(Quad(Term::Ground(4), Term::Var(0), Term::Var(2), None));
	a.insert(Quad(Term::Var(2), Term::Ground(2), Term::Var(1), None));
	a.insert(Quad(
		Term::Ground(5),
		Term::Ground(0),
		Term::Ground(5),
		None,
	));
	a.insert(Quad(
		Term::Ground(3),
		Term::Ground(5),
		Term::Ground(6),
		None,
	));
	a.insert(Quad(Term::Ground(6), Term::Var(0), Term::Var(1), None));
	a.insert(Quad(
		Term::Ground(2),
		Term::Ground(5),
		Term::Ground(2),
		None,
	));
	a.insert(Quad(Term::Ground(6), Term::Var(1), Term::Ground(5), None));
	a.insert(Quad(
		Term::Ground(3),
		Term::Ground(6),
		Term::Ground(6),
		None,
	));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(5),
		Term::Var(1002),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Var(1000),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Var(1000),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Ground(2),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Ground(0),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(5),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Var(1000),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(5),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Var(1001),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(6),
		Term::Ground(6),
		None,
	));
	test(a, b)
}
#[test]
fn iso_055() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Var(1), Term::Ground(2), Term::Ground(1), None));
	a.insert(Quad(Term::Ground(6), Term::Var(2), Term::Ground(0), None));
	a.insert(Quad(
		Term::Ground(2),
		Term::Ground(5),
		Term::Ground(5),
		None,
	));
	a.insert(Quad(Term::Ground(0), Term::Ground(2), Term::Var(1), None));
	a.insert(Quad(Term::Ground(0), Term::Ground(1), Term::Var(1), None));
	a.insert(Quad(Term::Var(0), Term::Ground(5), Term::Ground(4), None));
	a.insert(Quad(Term::Var(2), Term::Var(0), Term::Var(2), None));
	a.insert(Quad(Term::Ground(0), Term::Var(1), Term::Ground(2), None));
	a.insert(Quad(Term::Var(0), Term::Ground(1), Term::Ground(5), None));
	a.insert(Quad(Term::Var(1), Term::Ground(0), Term::Var(1), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(2),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Var(1002),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(5),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(2),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(1),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(5),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Var(1000),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Var(1001),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(1),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(0),
		Term::Var(1001),
		None,
	));
	test(a, b)
}
#[test]
fn iso_056() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Var(0), Term::Ground(1), Term::Ground(3), None));
	a.insert(Quad(Term::Var(2), Term::Ground(6), Term::Ground(3), None));
	a.insert(Quad(
		Term::Ground(2),
		Term::Ground(0),
		Term::Ground(1),
		None,
	));
	a.insert(Quad(Term::Ground(4), Term::Ground(6), Term::Var(2), None));
	a.insert(Quad(Term::Var(1), Term::Ground(3), Term::Ground(6), None));
	a.insert(Quad(Term::Ground(6), Term::Ground(6), Term::Var(0), None));
	a.insert(Quad(Term::Var(2), Term::Var(1), Term::Ground(3), None));
	a.insert(Quad(
		Term::Ground(4),
		Term::Ground(0),
		Term::Ground(5),
		None,
	));
	a.insert(Quad(
		Term::Ground(0),
		Term::Ground(2),
		Term::Ground(5),
		None,
	));
	a.insert(Quad(
		Term::Ground(4),
		Term::Ground(6),
		Term::Ground(5),
		None,
	));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(1),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Ground(6),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(0),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Ground(6),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(3),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Ground(6),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Var(1001),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Ground(0),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(2),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Ground(6),
		Term::Ground(5),
		None,
	));
	test(a, b)
}
#[test]
fn iso_057() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Ground(4), Term::Var(0), Term::Var(1), None));
	a.insert(Quad(Term::Ground(2), Term::Var(2), Term::Ground(2), None));
	a.insert(Quad(
		Term::Ground(1),
		Term::Ground(0),
		Term::Ground(2),
		None,
	));
	a.insert(Quad(Term::Var(0), Term::Ground(6), Term::Var(2), None));
	a.insert(Quad(Term::Ground(6), Term::Ground(0), Term::Var(0), None));
	a.insert(Quad(
		Term::Ground(3),
		Term::Ground(2),
		Term::Ground(5),
		None,
	));
	a.insert(Quad(Term::Ground(3), Term::Var(1), Term::Ground(3), None));
	a.insert(Quad(Term::Var(1), Term::Ground(0), Term::Ground(2), None));
	a.insert(Quad(Term::Var(0), Term::Ground(4), Term::Var(1), None));
	a.insert(Quad(Term::Var(1), Term::Ground(5), Term::Var(1), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(4),
		Term::Var(1000),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Var(1002),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(0),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(6),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Ground(0),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(2),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Var(1001),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(0),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(4),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(5),
		Term::Var(1001),
		None,
	));
	test(a, b)
}
#[test]
fn iso_058() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(
		Term::Ground(6),
		Term::Ground(3),
		Term::Ground(2),
		None,
	));
	a.insert(Quad(Term::Var(0), Term::Var(0), Term::Ground(0), None));
	a.insert(Quad(
		Term::Ground(5),
		Term::Ground(6),
		Term::Ground(3),
		None,
	));
	a.insert(Quad(Term::Var(0), Term::Ground(5), Term::Ground(0), None));
	a.insert(Quad(
		Term::Ground(0),
		Term::Ground(4),
		Term::Ground(2),
		None,
	));
	a.insert(Quad(Term::Ground(2), Term::Var(2), Term::Ground(4), None));
	a.insert(Quad(Term::Var(1), Term::Ground(1), Term::Var(1), None));
	a.insert(Quad(Term::Ground(4), Term::Var(1), Term::Var(2), None));
	a.insert(Quad(Term::Ground(3), Term::Ground(3), Term::Var(2), None));
	a.insert(Quad(Term::Ground(5), Term::Var(0), Term::Ground(2), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(6),
		Term::Ground(3),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Var(1000),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Ground(6),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(5),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(4),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Var(1002),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(1),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Var(1001),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(3),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Var(1000),
		Term::Ground(2),
		None,
	));
	test(a, b)
}
#[test]
fn iso_059() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Ground(3), Term::Ground(6), Term::Var(2), None));
	a.insert(Quad(Term::Var(2), Term::Ground(3), Term::Ground(2), None));
	a.insert(Quad(Term::Var(1), Term::Ground(0), Term::Var(1), None));
	a.insert(Quad(
		Term::Ground(6),
		Term::Ground(6),
		Term::Ground(4),
		None,
	));
	a.insert(Quad(
		Term::Ground(4),
		Term::Ground(4),
		Term::Ground(5),
		None,
	));
	a.insert(Quad(Term::Ground(0), Term::Var(2), Term::Ground(0), None));
	a.insert(Quad(
		Term::Ground(2),
		Term::Ground(1),
		Term::Ground(0),
		None,
	));
	a.insert(Quad(Term::Var(2), Term::Ground(0), Term::Ground(6), None));
	a.insert(Quad(Term::Var(1), Term::Var(1), Term::Ground(5), None));
	a.insert(Quad(Term::Var(0), Term::Ground(1), Term::Var(2), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(6),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Ground(3),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(0),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Ground(6),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Ground(4),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Var(1002),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(1),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Ground(0),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Var(1001),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(1),
		Term::Var(1002),
		None,
	));
	test(a, b)
}
#[test]
fn iso_060() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Ground(2), Term::Ground(0), Term::Var(2), None));
	a.insert(Quad(
		Term::Ground(1),
		Term::Ground(6),
		Term::Ground(2),
		None,
	));
	a.insert(Quad(Term::Var(2), Term::Ground(3), Term::Ground(0), None));
	a.insert(Quad(Term::Var(2), Term::Ground(5), Term::Ground(5), None));
	a.insert(Quad(Term::Var(1), Term::Ground(3), Term::Var(2), None));
	a.insert(Quad(Term::Var(2), Term::Ground(6), Term::Ground(5), None));
	a.insert(Quad(Term::Ground(6), Term::Var(2), Term::Var(2), None));
	a.insert(Quad(Term::Ground(4), Term::Ground(2), Term::Var(0), None));
	a.insert(Quad(Term::Var(1), Term::Ground(4), Term::Ground(5), None));
	a.insert(Quad(Term::Var(0), Term::Ground(4), Term::Var(1), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(0),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(6),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Ground(3),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Ground(5),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(3),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Ground(6),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Var(1002),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Ground(2),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(4),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(4),
		Term::Var(1001),
		None,
	));
	test(a, b)
}
#[test]
fn iso_061() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(
		Term::Ground(3),
		Term::Ground(2),
		Term::Ground(3),
		None,
	));
	a.insert(Quad(Term::Var(2), Term::Ground(4), Term::Ground(1), None));
	a.insert(Quad(
		Term::Ground(4),
		Term::Ground(2),
		Term::Ground(5),
		None,
	));
	a.insert(Quad(Term::Ground(0), Term::Var(2), Term::Ground(0), None));
	a.insert(Quad(Term::Var(2), Term::Ground(4), Term::Ground(5), None));
	a.insert(Quad(
		Term::Ground(6),
		Term::Ground(0),
		Term::Ground(4),
		None,
	));
	a.insert(Quad(Term::Var(0), Term::Var(0), Term::Var(2), None));
	a.insert(Quad(
		Term::Ground(5),
		Term::Ground(3),
		Term::Ground(3),
		None,
	));
	a.insert(Quad(Term::Var(0), Term::Var(0), Term::Ground(0), None));
	a.insert(Quad(Term::Ground(6), Term::Ground(2), Term::Var(2), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(2),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Ground(4),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Ground(2),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Var(1002),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Ground(4),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Ground(0),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Var(1000),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Ground(3),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Var(1000),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Ground(2),
		Term::Var(1002),
		None,
	));
	test(a, b)
}
#[test]
fn iso_062() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Var(1), Term::Var(1), Term::Ground(0), None));
	a.insert(Quad(Term::Var(1), Term::Var(0), Term::Ground(4), None));
	a.insert(Quad(Term::Var(1), Term::Var(1), Term::Var(1), None));
	a.insert(Quad(Term::Ground(5), Term::Var(1), Term::Var(1), None));
	a.insert(Quad(
		Term::Ground(6),
		Term::Ground(2),
		Term::Ground(0),
		None,
	));
	a.insert(Quad(Term::Var(2), Term::Ground(0), Term::Ground(0), None));
	a.insert(Quad(Term::Var(2), Term::Ground(1), Term::Var(0), None));
	a.insert(Quad(
		Term::Ground(2),
		Term::Ground(0),
		Term::Ground(0),
		None,
	));
	a.insert(Quad(
		Term::Ground(0),
		Term::Ground(2),
		Term::Ground(6),
		None,
	));
	a.insert(Quad(
		Term::Ground(1),
		Term::Ground(5),
		Term::Ground(5),
		None,
	));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Var(1001),
		Term::Var(1001),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Var(1000),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Var(1001),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Var(1001),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Ground(2),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Ground(0),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Ground(1),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(0),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(2),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(5),
		Term::Ground(5),
		None,
	));
	test(a, b)
}
#[test]
fn iso_063() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Var(1), Term::Var(0), Term::Ground(3), None));
	a.insert(Quad(
		Term::Ground(4),
		Term::Ground(0),
		Term::Ground(1),
		None,
	));
	a.insert(Quad(Term::Var(1), Term::Ground(5), Term::Ground(0), None));
	a.insert(Quad(Term::Var(1), Term::Var(2), Term::Ground(0), None));
	a.insert(Quad(Term::Var(1), Term::Ground(5), Term::Ground(2), None));
	a.insert(Quad(Term::Var(2), Term::Ground(3), Term::Ground(6), None));
	a.insert(Quad(Term::Ground(5), Term::Var(2), Term::Ground(2), None));
	a.insert(Quad(Term::Ground(6), Term::Var(0), Term::Ground(0), None));
	a.insert(Quad(Term::Ground(1), Term::Var(1), Term::Ground(5), None));
	a.insert(Quad(Term::Var(2), Term::Ground(3), Term::Var(2), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Var(1001),
		Term::Var(1000),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Ground(0),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(5),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Var(1002),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(5),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Ground(3),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Var(1002),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Var(1000),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Var(1001),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Ground(3),
		Term::Var(1002),
		None,
	));
	test(a, b)
}
#[test]
fn iso_064() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Var(0), Term::Ground(2), Term::Ground(2), None));
	a.insert(Quad(Term::Var(1), Term::Var(0), Term::Var(0), None));
	a.insert(Quad(
		Term::Ground(5),
		Term::Ground(6),
		Term::Ground(1),
		None,
	));
	a.insert(Quad(Term::Ground(3), Term::Ground(0), Term::Var(2), None));
	a.insert(Quad(Term::Ground(4), Term::Var(0), Term::Ground(0), None));
	a.insert(Quad(Term::Var(2), Term::Var(2), Term::Ground(4), None));
	a.insert(Quad(Term::Var(1), Term::Var(2), Term::Ground(1), None));
	a.insert(Quad(Term::Ground(3), Term::Ground(2), Term::Var(0), None));
	a.insert(Quad(Term::Var(0), Term::Ground(4), Term::Var(1), None));
	a.insert(Quad(
		Term::Ground(1),
		Term::Ground(3),
		Term::Ground(2),
		None,
	));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(2),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Var(1000),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Ground(6),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(0),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Var(1000),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Var(1002),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Var(1002),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(2),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(4),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(3),
		Term::Ground(2),
		None,
	));
	test(a, b)
}
#[test]
fn iso_065() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(
		Term::Ground(1),
		Term::Ground(0),
		Term::Ground(1),
		None,
	));
	a.insert(Quad(Term::Var(1), Term::Ground(0), Term::Ground(3), None));
	a.insert(Quad(
		Term::Ground(1),
		Term::Ground(0),
		Term::Ground(4),
		None,
	));
	a.insert(Quad(
		Term::Ground(3),
		Term::Ground(1),
		Term::Ground(6),
		None,
	));
	a.insert(Quad(Term::Var(0), Term::Ground(5), Term::Ground(6), None));
	a.insert(Quad(Term::Var(2), Term::Var(0), Term::Var(1), None));
	a.insert(Quad(Term::Ground(2), Term::Ground(3), Term::Var(0), None));
	a.insert(Quad(Term::Var(1), Term::Var(1), Term::Ground(3), None));
	a.insert(Quad(Term::Ground(0), Term::Var(2), Term::Var(1), None));
	a.insert(Quad(
		Term::Ground(6),
		Term::Ground(0),
		Term::Ground(3),
		None,
	));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(0),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(0),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(0),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(1),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(5),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Var(1000),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(3),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Var(1001),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Var(1002),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Ground(0),
		Term::Ground(3),
		None,
	));
	test(a, b)
}
#[test]
fn iso_066() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(
		Term::Ground(6),
		Term::Ground(1),
		Term::Ground(0),
		None,
	));
	a.insert(Quad(Term::Var(2), Term::Ground(6), Term::Ground(1), None));
	a.insert(Quad(Term::Var(2), Term::Ground(1), Term::Var(2), None));
	a.insert(Quad(Term::Ground(6), Term::Var(0), Term::Var(1), None));
	a.insert(Quad(Term::Var(2), Term::Ground(6), Term::Var(1), None));
	a.insert(Quad(Term::Ground(2), Term::Var(0), Term::Ground(6), None));
	a.insert(Quad(Term::Ground(1), Term::Var(1), Term::Ground(6), None));
	a.insert(Quad(Term::Ground(0), Term::Var(1), Term::Ground(0), None));
	a.insert(Quad(
		Term::Ground(0),
		Term::Ground(6),
		Term::Ground(6),
		None,
	));
	a.insert(Quad(Term::Var(0), Term::Ground(6), Term::Ground(0), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(6),
		Term::Ground(1),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Ground(6),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Ground(1),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Var(1000),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Ground(6),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Var(1000),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Var(1001),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Var(1001),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(6),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(6),
		Term::Ground(0),
		None,
	));
	test(a, b)
}
#[test]
fn iso_067() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Var(0), Term::Ground(2), Term::Var(1), None));
	a.insert(Quad(
		Term::Ground(1),
		Term::Ground(0),
		Term::Ground(3),
		None,
	));
	a.insert(Quad(Term::Ground(4), Term::Var(0), Term::Ground(2), None));
	a.insert(Quad(
		Term::Ground(0),
		Term::Ground(3),
		Term::Ground(2),
		None,
	));
	a.insert(Quad(Term::Var(1), Term::Ground(1), Term::Var(1), None));
	a.insert(Quad(Term::Ground(5), Term::Ground(3), Term::Var(1), None));
	a.insert(Quad(Term::Ground(1), Term::Ground(3), Term::Var(1), None));
	a.insert(Quad(Term::Var(0), Term::Ground(6), Term::Ground(2), None));
	a.insert(Quad(
		Term::Ground(3),
		Term::Ground(2),
		Term::Ground(2),
		None,
	));
	a.insert(Quad(Term::Ground(3), Term::Ground(3), Term::Var(0), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(2),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(0),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Var(1000),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(3),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(1),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Ground(3),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(3),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(6),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(2),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(3),
		Term::Var(1000),
		None,
	));
	test(a, b)
}
#[test]
fn iso_068() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Ground(1), Term::Ground(3), Term::Var(1), None));
	a.insert(Quad(Term::Var(2), Term::Ground(0), Term::Ground(4), None));
	a.insert(Quad(
		Term::Ground(0),
		Term::Ground(1),
		Term::Ground(1),
		None,
	));
	a.insert(Quad(Term::Var(1), Term::Ground(0), Term::Ground(0), None));
	a.insert(Quad(Term::Var(2), Term::Var(2), Term::Ground(2), None));
	a.insert(Quad(Term::Var(2), Term::Ground(0), Term::Ground(6), None));
	a.insert(Quad(Term::Var(0), Term::Ground(0), Term::Ground(0), None));
	a.insert(Quad(Term::Ground(4), Term::Var(1), Term::Ground(0), None));
	a.insert(Quad(Term::Ground(1), Term::Var(0), Term::Var(1), None));
	a.insert(Quad(Term::Ground(6), Term::Ground(3), Term::Var(0), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(3),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Ground(0),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(1),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(0),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Var(1002),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Ground(0),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(0),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Var(1001),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Var(1000),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Ground(3),
		Term::Var(1000),
		None,
	));
	test(a, b)
}
#[test]
fn iso_069() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Ground(1), Term::Var(2), Term::Ground(4), None));
	a.insert(Quad(Term::Var(0), Term::Ground(6), Term::Var(2), None));
	a.insert(Quad(
		Term::Ground(2),
		Term::Ground(0),
		Term::Ground(6),
		None,
	));
	a.insert(Quad(Term::Ground(6), Term::Var(1), Term::Ground(4), None));
	a.insert(Quad(Term::Ground(6), Term::Var(2), Term::Var(1), None));
	a.insert(Quad(Term::Ground(2), Term::Var(0), Term::Ground(4), None));
	a.insert(Quad(Term::Ground(2), Term::Ground(5), Term::Var(0), None));
	a.insert(Quad(Term::Var(1), Term::Var(1), Term::Var(0), None));
	a.insert(Quad(Term::Var(1), Term::Ground(1), Term::Ground(5), None));
	a.insert(Quad(Term::Ground(6), Term::Ground(4), Term::Var(2), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(1),
		Term::Var(1002),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(6),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(0),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Var(1001),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Var(1002),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Var(1000),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(5),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Var(1001),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(1),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Ground(4),
		Term::Var(1002),
		None,
	));
	test(a, b)
}
#[test]
fn iso_070() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Ground(0), Term::Ground(0), Term::Var(0), None));
	a.insert(Quad(Term::Var(2), Term::Ground(5), Term::Ground(3), None));
	a.insert(Quad(
		Term::Ground(3),
		Term::Ground(4),
		Term::Ground(3),
		None,
	));
	a.insert(Quad(Term::Var(1), Term::Ground(5), Term::Ground(6), None));
	a.insert(Quad(Term::Ground(2), Term::Ground(4), Term::Var(1), None));
	a.insert(Quad(Term::Var(0), Term::Ground(1), Term::Ground(4), None));
	a.insert(Quad(Term::Ground(2), Term::Var(1), Term::Ground(6), None));
	a.insert(Quad(
		Term::Ground(2),
		Term::Ground(3),
		Term::Ground(0),
		None,
	));
	a.insert(Quad(Term::Var(0), Term::Var(2), Term::Var(0), None));
	a.insert(Quad(Term::Var(2), Term::Ground(5), Term::Var(1), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(0),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Ground(5),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(4),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(5),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(4),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(1),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Var(1001),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(3),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Var(1002),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Ground(5),
		Term::Var(1001),
		None,
	));
	test(a, b)
}
#[test]
fn iso_071() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Ground(0), Term::Var(1), Term::Ground(3), None));
	a.insert(Quad(Term::Ground(4), Term::Ground(5), Term::Var(2), None));
	a.insert(Quad(Term::Var(1), Term::Ground(6), Term::Ground(3), None));
	a.insert(Quad(
		Term::Ground(0),
		Term::Ground(6),
		Term::Ground(0),
		None,
	));
	a.insert(Quad(
		Term::Ground(3),
		Term::Ground(6),
		Term::Ground(2),
		None,
	));
	a.insert(Quad(
		Term::Ground(6),
		Term::Ground(6),
		Term::Ground(3),
		None,
	));
	a.insert(Quad(Term::Var(0), Term::Var(2), Term::Var(1), None));
	a.insert(Quad(Term::Ground(5), Term::Var(0), Term::Ground(6), None));
	a.insert(Quad(Term::Ground(2), Term::Ground(5), Term::Var(1), None));
	a.insert(Quad(Term::Ground(5), Term::Var(0), Term::Ground(1), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(0),
		Term::Var(1001),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Ground(5),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(6),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(6),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(6),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Ground(6),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Var(1002),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Var(1000),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(5),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Var(1000),
		Term::Ground(1),
		None,
	));
	test(a, b)
}
#[test]
fn iso_072() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(
		Term::Ground(0),
		Term::Ground(3),
		Term::Ground(0),
		None,
	));
	a.insert(Quad(Term::Var(2), Term::Ground(1), Term::Ground(2), None));
	a.insert(Quad(Term::Ground(3), Term::Ground(1), Term::Var(1), None));
	a.insert(Quad(Term::Ground(2), Term::Var(0), Term::Ground(5), None));
	a.insert(Quad(Term::Ground(4), Term::Var(1), Term::Var(2), None));
	a.insert(Quad(
		Term::Ground(0),
		Term::Ground(0),
		Term::Ground(3),
		None,
	));
	a.insert(Quad(Term::Ground(3), Term::Ground(4), Term::Var(0), None));
	a.insert(Quad(
		Term::Ground(3),
		Term::Ground(1),
		Term::Ground(5),
		None,
	));
	a.insert(Quad(Term::Var(2), Term::Ground(5), Term::Ground(1), None));
	a.insert(Quad(Term::Ground(0), Term::Ground(0), Term::Var(0), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(3),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Ground(1),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(1),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Var(1000),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Var(1001),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(0),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(4),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(1),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Ground(5),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(0),
		Term::Var(1000),
		None,
	));
	test(a, b)
}
#[test]
fn iso_073() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(
		Term::Ground(3),
		Term::Ground(2),
		Term::Ground(3),
		None,
	));
	a.insert(Quad(
		Term::Ground(1),
		Term::Ground(2),
		Term::Ground(3),
		None,
	));
	a.insert(Quad(
		Term::Ground(1),
		Term::Ground(1),
		Term::Ground(3),
		None,
	));
	a.insert(Quad(Term::Var(1), Term::Ground(6), Term::Ground(6), None));
	a.insert(Quad(Term::Var(0), Term::Ground(6), Term::Ground(1), None));
	a.insert(Quad(
		Term::Ground(0),
		Term::Ground(1),
		Term::Ground(4),
		None,
	));
	a.insert(Quad(
		Term::Ground(2),
		Term::Ground(5),
		Term::Ground(5),
		None,
	));
	a.insert(Quad(Term::Ground(6), Term::Var(2), Term::Var(0), None));
	a.insert(Quad(Term::Var(0), Term::Ground(3), Term::Var(2), None));
	a.insert(Quad(Term::Var(1), Term::Ground(6), Term::Var(2), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(2),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(2),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(1),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(6),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(6),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(1),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(5),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Var(1002),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(3),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(6),
		Term::Var(1002),
		None,
	));
	test(a, b)
}
#[test]
fn iso_074() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Var(0), Term::Ground(0), Term::Ground(4), None));
	a.insert(Quad(Term::Ground(4), Term::Var(0), Term::Ground(0), None));
	a.insert(Quad(Term::Ground(3), Term::Var(2), Term::Var(0), None));
	a.insert(Quad(Term::Var(2), Term::Ground(2), Term::Var(1), None));
	a.insert(Quad(Term::Ground(1), Term::Var(0), Term::Ground(2), None));
	a.insert(Quad(Term::Var(2), Term::Ground(0), Term::Ground(5), None));
	a.insert(Quad(Term::Ground(0), Term::Var(1), Term::Ground(5), None));
	a.insert(Quad(Term::Ground(6), Term::Ground(0), Term::Var(0), None));
	a.insert(Quad(Term::Var(2), Term::Ground(1), Term::Var(1), None));
	a.insert(Quad(Term::Ground(6), Term::Var(1), Term::Ground(6), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(0),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Var(1000),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Var(1002),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Ground(2),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Var(1000),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Ground(0),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Var(1001),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Ground(0),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Ground(1),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Var(1001),
		Term::Ground(6),
		None,
	));
	test(a, b)
}
#[test]
fn iso_075() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Var(1), Term::Ground(6), Term::Var(1), None));
	a.insert(Quad(Term::Ground(3), Term::Ground(1), Term::Var(1), None));
	a.insert(Quad(Term::Var(0), Term::Var(1), Term::Ground(0), None));
	a.insert(Quad(Term::Var(0), Term::Ground(0), Term::Var(1), None));
	a.insert(Quad(Term::Ground(1), Term::Ground(6), Term::Var(1), None));
	a.insert(Quad(Term::Ground(4), Term::Var(0), Term::Var(1), None));
	a.insert(Quad(Term::Ground(4), Term::Var(2), Term::Ground(0), None));
	a.insert(Quad(Term::Var(1), Term::Var(2), Term::Ground(0), None));
	a.insert(Quad(
		Term::Ground(0),
		Term::Ground(6),
		Term::Ground(3),
		None,
	));
	a.insert(Quad(Term::Ground(5), Term::Var(0), Term::Var(2), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(6),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(1),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Var(1001),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(0),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(6),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Var(1000),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Var(1002),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Var(1002),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(6),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Var(1000),
		Term::Var(1002),
		None,
	));
	test(a, b)
}
#[test]
fn iso_076() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Var(2), Term::Ground(3), Term::Ground(3), None));
	a.insert(Quad(Term::Ground(0), Term::Ground(0), Term::Var(0), None));
	a.insert(Quad(Term::Var(2), Term::Ground(1), Term::Ground(6), None));
	a.insert(Quad(Term::Var(0), Term::Var(0), Term::Ground(2), None));
	a.insert(Quad(Term::Ground(3), Term::Var(1), Term::Ground(4), None));
	a.insert(Quad(
		Term::Ground(3),
		Term::Ground(1),
		Term::Ground(6),
		None,
	));
	a.insert(Quad(Term::Var(0), Term::Ground(6), Term::Var(0), None));
	a.insert(Quad(Term::Var(1), Term::Ground(2), Term::Ground(5), None));
	a.insert(Quad(Term::Var(0), Term::Ground(6), Term::Ground(1), None));
	a.insert(Quad(Term::Var(2), Term::Ground(0), Term::Var(1), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Var(1002),
		Term::Ground(3),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(0),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Ground(1),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Var(1000),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Var(1001),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(1),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(6),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(2),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(6),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Ground(0),
		Term::Var(1001),
		None,
	));
	test(a, b)
}
#[test]
fn iso_077() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Ground(2), Term::Var(1), Term::Ground(3), None));
	a.insert(Quad(
		Term::Ground(6),
		Term::Ground(2),
		Term::Ground(5),
		None,
	));
	a.insert(Quad(Term::Ground(3), Term::Ground(4), Term::Var(2), None));
	a.insert(Quad(Term::Ground(6), Term::Var(1), Term::Ground(2), None));
	a.insert(Quad(Term::Ground(5), Term::Ground(5), Term::Var(2), None));
	a.insert(Quad(
		Term::Ground(4),
		Term::Ground(6),
		Term::Ground(0),
		None,
	));
	a.insert(Quad(
		Term::Ground(1),
		Term::Ground(2),
		Term::Ground(6),
		None,
	));
	a.insert(Quad(Term::Ground(4), Term::Var(0), Term::Var(1), None));
	a.insert(Quad(
		Term::Ground(5),
		Term::Ground(6),
		Term::Ground(1),
		None,
	));
	a.insert(Quad(Term::Var(1), Term::Ground(0), Term::Var(1), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(2),
		Term::Var(1001),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Ground(2),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(4),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Var(1001),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Ground(5),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Ground(6),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(2),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Var(1000),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Ground(6),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(0),
		Term::Var(1001),
		None,
	));
	test(a, b)
}
#[test]
fn iso_078() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Ground(6), Term::Var(2), Term::Ground(6), None));
	a.insert(Quad(Term::Ground(5), Term::Ground(2), Term::Var(0), None));
	a.insert(Quad(Term::Var(0), Term::Var(1), Term::Ground(1), None));
	a.insert(Quad(
		Term::Ground(3),
		Term::Ground(3),
		Term::Ground(3),
		None,
	));
	a.insert(Quad(
		Term::Ground(6),
		Term::Ground(5),
		Term::Ground(3),
		None,
	));
	a.insert(Quad(
		Term::Ground(0),
		Term::Ground(2),
		Term::Ground(1),
		None,
	));
	a.insert(Quad(Term::Ground(6), Term::Var(2), Term::Var(1), None));
	a.insert(Quad(
		Term::Ground(2),
		Term::Ground(0),
		Term::Ground(4),
		None,
	));
	a.insert(Quad(Term::Ground(5), Term::Ground(0), Term::Var(2), None));
	a.insert(Quad(Term::Ground(5), Term::Var(1), Term::Var(2), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(6),
		Term::Var(1002),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Ground(2),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Var(1001),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(3),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Ground(5),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(2),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Var(1002),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(0),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Ground(0),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Var(1001),
		Term::Var(1002),
		None,
	));
	test(a, b)
}
#[test]
fn iso_079() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Var(1), Term::Var(2), Term::Ground(0), None));
	a.insert(Quad(Term::Var(0), Term::Var(0), Term::Ground(2), None));
	a.insert(Quad(Term::Var(1), Term::Var(0), Term::Ground(0), None));
	a.insert(Quad(
		Term::Ground(5),
		Term::Ground(2),
		Term::Ground(4),
		None,
	));
	a.insert(Quad(Term::Var(0), Term::Ground(0), Term::Var(1), None));
	a.insert(Quad(
		Term::Ground(1),
		Term::Ground(5),
		Term::Ground(4),
		None,
	));
	a.insert(Quad(Term::Var(1), Term::Ground(5), Term::Var(1), None));
	a.insert(Quad(
		Term::Ground(1),
		Term::Ground(6),
		Term::Ground(1),
		None,
	));
	a.insert(Quad(Term::Var(2), Term::Ground(1), Term::Var(0), None));
	a.insert(Quad(Term::Var(1), Term::Ground(1), Term::Var(0), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Var(1001),
		Term::Var(1002),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Var(1000),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Var(1000),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Ground(2),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(0),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(5),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(5),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(6),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Ground(1),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(1),
		Term::Var(1000),
		None,
	));
	test(a, b)
}
#[test]
fn iso_080() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Ground(5), Term::Var(1), Term::Var(2), None));
	a.insert(Quad(Term::Ground(5), Term::Var(0), Term::Ground(2), None));
	a.insert(Quad(
		Term::Ground(0),
		Term::Ground(5),
		Term::Ground(4),
		None,
	));
	a.insert(Quad(
		Term::Ground(6),
		Term::Ground(1),
		Term::Ground(1),
		None,
	));
	a.insert(Quad(Term::Var(2), Term::Var(2), Term::Var(0), None));
	a.insert(Quad(
		Term::Ground(0),
		Term::Ground(3),
		Term::Ground(1),
		None,
	));
	a.insert(Quad(Term::Ground(3), Term::Ground(1), Term::Var(1), None));
	a.insert(Quad(Term::Ground(0), Term::Ground(3), Term::Var(1), None));
	a.insert(Quad(Term::Ground(0), Term::Ground(3), Term::Var(0), None));
	a.insert(Quad(Term::Ground(3), Term::Ground(0), Term::Var(2), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(5),
		Term::Var(1001),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Var(1000),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(5),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Ground(1),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Var(1002),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(3),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(1),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(3),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(3),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(0),
		Term::Var(1002),
		None,
	));
	test(a, b)
}
#[test]
fn iso_081() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Var(1), Term::Ground(3), Term::Var(0), None));
	a.insert(Quad(Term::Var(1), Term::Var(8), Term::Ground(2), None));
	a.insert(Quad(Term::Var(2), Term::Ground(4), Term::Var(2), None));
	a.insert(Quad(Term::Ground(7), Term::Var(0), Term::Var(4), None));
	a.insert(Quad(Term::Ground(2), Term::Var(4), Term::Ground(7), None));
	a.insert(Quad(Term::Var(2), Term::Var(0), Term::Var(3), None));
	a.insert(Quad(Term::Var(0), Term::Ground(8), Term::Var(1), None));
	a.insert(Quad(Term::Var(0), Term::Var(4), Term::Var(1), None));
	a.insert(Quad(Term::Var(7), Term::Var(5), Term::Var(0), None));
	a.insert(Quad(
		Term::Ground(9),
		Term::Ground(5),
		Term::Ground(7),
		None,
	));
	a.insert(Quad(Term::Ground(2), Term::Ground(1), Term::Var(0), None));
	a.insert(Quad(Term::Ground(0), Term::Var(0), Term::Var(3), None));
	a.insert(Quad(Term::Ground(9), Term::Var(0), Term::Var(7), None));
	a.insert(Quad(Term::Ground(2), Term::Var(6), Term::Var(7), None));
	a.insert(Quad(Term::Ground(4), Term::Var(0), Term::Var(6), None));
	a.insert(Quad(Term::Ground(7), Term::Var(8), Term::Ground(6), None));
	a.insert(Quad(Term::Var(3), Term::Ground(4), Term::Ground(3), None));
	a.insert(Quad(
		Term::Ground(0),
		Term::Ground(2),
		Term::Ground(9),
		None,
	));
	a.insert(Quad(Term::Var(4), Term::Ground(3), Term::Ground(9), None));
	a.insert(Quad(Term::Ground(2), Term::Ground(2), Term::Var(7), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(3),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Var(1008),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Ground(4),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Ground(7),
		Term::Var(1000),
		Term::Var(1004),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Var(1004),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Var(1000),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(8),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Var(1004),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Var(1005),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Ground(5),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(1),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Var(1000),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Var(1000),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Var(1006),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Var(1000),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Ground(7),
		Term::Var(1008),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Ground(4),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(2),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Ground(3),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(2),
		Term::Var(1007),
		None,
	));
	test(a, b)
}
#[test]
fn iso_082() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Var(9), Term::Var(9), Term::Ground(4), None));
	a.insert(Quad(Term::Var(2), Term::Var(4), Term::Ground(2), None));
	a.insert(Quad(Term::Var(0), Term::Var(0), Term::Ground(6), None));
	a.insert(Quad(Term::Var(2), Term::Var(1), Term::Var(0), None));
	a.insert(Quad(
		Term::Ground(7),
		Term::Ground(4),
		Term::Ground(9),
		None,
	));
	a.insert(Quad(Term::Var(1), Term::Var(9), Term::Ground(2), None));
	a.insert(Quad(Term::Ground(9), Term::Var(4), Term::Var(5), None));
	a.insert(Quad(Term::Var(1), Term::Ground(4), Term::Var(9), None));
	a.insert(Quad(Term::Var(1), Term::Var(9), Term::Var(5), None));
	a.insert(Quad(Term::Ground(3), Term::Var(0), Term::Ground(0), None));
	a.insert(Quad(Term::Var(2), Term::Var(0), Term::Ground(2), None));
	a.insert(Quad(Term::Var(1), Term::Ground(6), Term::Ground(8), None));
	a.insert(Quad(Term::Ground(5), Term::Ground(0), Term::Var(1), None));
	a.insert(Quad(
		Term::Ground(9),
		Term::Ground(1),
		Term::Ground(6),
		None,
	));
	a.insert(Quad(Term::Ground(3), Term::Var(3), Term::Ground(0), None));
	a.insert(Quad(Term::Ground(3), Term::Var(4), Term::Var(1), None));
	a.insert(Quad(
		Term::Ground(8),
		Term::Ground(7),
		Term::Ground(3),
		None,
	));
	a.insert(Quad(
		Term::Ground(3),
		Term::Ground(3),
		Term::Ground(8),
		None,
	));
	a.insert(Quad(
		Term::Ground(8),
		Term::Ground(7),
		Term::Ground(4),
		None,
	));
	a.insert(Quad(Term::Ground(1), Term::Ground(8), Term::Var(2), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Var(1009),
		Term::Var(1009),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Var(1004),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Var(1000),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Var(1001),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Ground(7),
		Term::Ground(4),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Var(1009),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Var(1004),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(4),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Var(1009),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Var(1000),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Var(1000),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(6),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Ground(0),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Ground(1),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Var(1003),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Var(1004),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Ground(7),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(3),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Ground(7),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(8),
		Term::Var(1002),
		None,
	));
	test(a, b)
}
#[test]
fn iso_083() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Var(1), Term::Var(8), Term::Var(6), None));
	a.insert(Quad(Term::Ground(9), Term::Var(3), Term::Var(6), None));
	a.insert(Quad(Term::Var(8), Term::Ground(2), Term::Ground(9), None));
	a.insert(Quad(Term::Ground(8), Term::Ground(4), Term::Var(6), None));
	a.insert(Quad(
		Term::Ground(3),
		Term::Ground(5),
		Term::Ground(4),
		None,
	));
	a.insert(Quad(Term::Var(7), Term::Var(8), Term::Var(7), None));
	a.insert(Quad(
		Term::Ground(3),
		Term::Ground(3),
		Term::Ground(1),
		None,
	));
	a.insert(Quad(Term::Var(6), Term::Var(8), Term::Var(9), None));
	a.insert(Quad(
		Term::Ground(3),
		Term::Ground(9),
		Term::Ground(7),
		None,
	));
	a.insert(Quad(Term::Var(2), Term::Ground(9), Term::Ground(0), None));
	a.insert(Quad(Term::Var(1), Term::Ground(4), Term::Ground(1), None));
	a.insert(Quad(Term::Var(1), Term::Ground(8), Term::Ground(6), None));
	a.insert(Quad(Term::Var(4), Term::Var(8), Term::Var(6), None));
	a.insert(Quad(Term::Ground(4), Term::Ground(5), Term::Var(5), None));
	a.insert(Quad(Term::Var(3), Term::Ground(1), Term::Var(8), None));
	a.insert(Quad(Term::Var(6), Term::Var(8), Term::Ground(3), None));
	a.insert(Quad(Term::Ground(8), Term::Ground(5), Term::Var(0), None));
	a.insert(Quad(Term::Var(3), Term::Var(6), Term::Ground(8), None));
	a.insert(Quad(Term::Ground(3), Term::Ground(9), Term::Var(4), None));
	a.insert(Quad(
		Term::Ground(1),
		Term::Ground(6),
		Term::Ground(6),
		None,
	));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Var(1001),
		Term::Var(1008),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Var(1003),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Ground(2),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Ground(4),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(5),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Var(1008),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(3),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Var(1006),
		Term::Var(1008),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(9),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Ground(9),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(4),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(8),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Var(1008),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Ground(5),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Ground(1),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Var(1006),
		Term::Var(1008),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Ground(5),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Var(1006),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(9),
		Term::Var(1004),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(6),
		Term::Ground(6),
		None,
	));
	test(a, b)
}
#[test]
fn iso_084() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Var(9), Term::Ground(6), Term::Var(8), None));
	a.insert(Quad(
		Term::Ground(2),
		Term::Ground(5),
		Term::Ground(1),
		None,
	));
	a.insert(Quad(Term::Var(8), Term::Ground(0), Term::Ground(6), None));
	a.insert(Quad(Term::Ground(1), Term::Ground(3), Term::Var(7), None));
	a.insert(Quad(Term::Ground(0), Term::Var(3), Term::Ground(8), None));
	a.insert(Quad(Term::Ground(6), Term::Var(1), Term::Var(0), None));
	a.insert(Quad(Term::Var(2), Term::Ground(7), Term::Ground(3), None));
	a.insert(Quad(Term::Ground(5), Term::Var(6), Term::Var(9), None));
	a.insert(Quad(Term::Ground(5), Term::Var(9), Term::Var(1), None));
	a.insert(Quad(Term::Ground(7), Term::Ground(7), Term::Var(1), None));
	a.insert(Quad(Term::Var(3), Term::Var(8), Term::Var(0), None));
	a.insert(Quad(Term::Ground(7), Term::Ground(5), Term::Var(9), None));
	a.insert(Quad(Term::Ground(8), Term::Ground(8), Term::Var(5), None));
	a.insert(Quad(Term::Var(1), Term::Var(7), Term::Var(7), None));
	a.insert(Quad(
		Term::Ground(4),
		Term::Ground(0),
		Term::Ground(8),
		None,
	));
	a.insert(Quad(Term::Ground(0), Term::Ground(8), Term::Var(9), None));
	a.insert(Quad(
		Term::Ground(5),
		Term::Ground(3),
		Term::Ground(5),
		None,
	));
	a.insert(Quad(Term::Var(7), Term::Ground(4), Term::Var(9), None));
	a.insert(Quad(Term::Var(4), Term::Var(7), Term::Ground(8), None));
	a.insert(Quad(Term::Var(0), Term::Ground(6), Term::Var(4), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Var(1009),
		Term::Ground(6),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(5),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Ground(0),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(3),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Var(1003),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Var(1001),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Ground(7),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Var(1006),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Var(1009),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(7),
		Term::Ground(7),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Var(1008),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Ground(7),
		Term::Ground(5),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Ground(8),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Var(1007),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Ground(0),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(8),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Ground(3),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Ground(4),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Var(1007),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(6),
		Term::Var(1004),
		None,
	));
	test(a, b)
}
#[test]
fn iso_085() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Ground(0), Term::Ground(7), Term::Var(7), None));
	a.insert(Quad(Term::Var(1), Term::Ground(7), Term::Var(8), None));
	a.insert(Quad(Term::Ground(1), Term::Ground(8), Term::Var(2), None));
	a.insert(Quad(Term::Var(7), Term::Ground(1), Term::Ground(1), None));
	a.insert(Quad(Term::Var(8), Term::Ground(1), Term::Var(1), None));
	a.insert(Quad(
		Term::Ground(4),
		Term::Ground(6),
		Term::Ground(9),
		None,
	));
	a.insert(Quad(Term::Ground(3), Term::Var(1), Term::Var(4), None));
	a.insert(Quad(Term::Var(0), Term::Var(2), Term::Ground(3), None));
	a.insert(Quad(Term::Ground(3), Term::Var(9), Term::Ground(6), None));
	a.insert(Quad(Term::Var(3), Term::Ground(7), Term::Var(3), None));
	a.insert(Quad(Term::Ground(8), Term::Var(5), Term::Ground(5), None));
	a.insert(Quad(Term::Ground(6), Term::Var(1), Term::Var(4), None));
	a.insert(Quad(Term::Var(7), Term::Var(5), Term::Var(1), None));
	a.insert(Quad(Term::Var(9), Term::Var(8), Term::Var(4), None));
	a.insert(Quad(Term::Var(8), Term::Ground(5), Term::Ground(9), None));
	a.insert(Quad(Term::Ground(0), Term::Var(0), Term::Var(5), None));
	a.insert(Quad(Term::Ground(0), Term::Ground(5), Term::Var(3), None));
	a.insert(Quad(Term::Var(8), Term::Ground(3), Term::Var(1), None));
	a.insert(Quad(Term::Ground(9), Term::Var(0), Term::Var(0), None));
	a.insert(Quad(Term::Ground(5), Term::Var(0), Term::Ground(2), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(7),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(7),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(8),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Ground(1),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Ground(1),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Ground(6),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Var(1001),
		Term::Var(1004),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Var(1002),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Var(1009),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Ground(7),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Var(1005),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Var(1001),
		Term::Var(1004),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Var(1005),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Var(1008),
		Term::Var(1004),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Ground(5),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Var(1000),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(5),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Ground(3),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Var(1000),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Var(1000),
		Term::Ground(2),
		None,
	));
	test(a, b)
}
#[test]
fn iso_086() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Var(9), Term::Ground(1), Term::Ground(0), None));
	a.insert(Quad(Term::Var(2), Term::Ground(9), Term::Ground(5), None));
	a.insert(Quad(Term::Ground(2), Term::Var(5), Term::Ground(9), None));
	a.insert(Quad(Term::Var(4), Term::Ground(6), Term::Var(1), None));
	a.insert(Quad(Term::Var(5), Term::Var(7), Term::Var(3), None));
	a.insert(Quad(Term::Ground(1), Term::Ground(7), Term::Var(4), None));
	a.insert(Quad(
		Term::Ground(8),
		Term::Ground(5),
		Term::Ground(5),
		None,
	));
	a.insert(Quad(Term::Var(2), Term::Ground(8), Term::Ground(1), None));
	a.insert(Quad(Term::Ground(4), Term::Var(4), Term::Var(3), None));
	a.insert(Quad(Term::Ground(1), Term::Var(0), Term::Var(9), None));
	a.insert(Quad(Term::Ground(4), Term::Var(0), Term::Ground(2), None));
	a.insert(Quad(Term::Var(3), Term::Ground(1), Term::Var(9), None));
	a.insert(Quad(Term::Ground(6), Term::Var(4), Term::Ground(1), None));
	a.insert(Quad(Term::Ground(9), Term::Var(6), Term::Ground(8), None));
	a.insert(Quad(Term::Var(2), Term::Ground(2), Term::Ground(8), None));
	a.insert(Quad(Term::Var(9), Term::Var(9), Term::Ground(5), None));
	a.insert(Quad(Term::Ground(6), Term::Var(2), Term::Var(3), None));
	a.insert(Quad(Term::Ground(5), Term::Var(0), Term::Ground(1), None));
	a.insert(Quad(Term::Ground(3), Term::Var(0), Term::Var(1), None));
	a.insert(Quad(Term::Var(1), Term::Ground(1), Term::Var(7), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Var(1009),
		Term::Ground(1),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Ground(9),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Var(1005),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Ground(6),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Var(1007),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(7),
		Term::Var(1004),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Ground(5),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Ground(8),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Var(1004),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Var(1000),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Var(1000),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Ground(1),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Var(1004),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Var(1006),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Ground(2),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Var(1009),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Var(1002),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Var(1000),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Var(1000),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(1),
		Term::Var(1007),
		None,
	));
	test(a, b)
}
#[test]
fn iso_087() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Var(8), Term::Var(4), Term::Ground(4), None));
	a.insert(Quad(Term::Ground(4), Term::Var(4), Term::Ground(0), None));
	a.insert(Quad(Term::Ground(9), Term::Var(0), Term::Var(3), None));
	a.insert(Quad(Term::Var(6), Term::Ground(1), Term::Ground(7), None));
	a.insert(Quad(Term::Var(8), Term::Var(7), Term::Ground(8), None));
	a.insert(Quad(Term::Ground(0), Term::Var(2), Term::Ground(7), None));
	a.insert(Quad(Term::Var(4), Term::Var(5), Term::Ground(8), None));
	a.insert(Quad(Term::Ground(4), Term::Var(7), Term::Var(3), None));
	a.insert(Quad(Term::Var(9), Term::Var(8), Term::Ground(1), None));
	a.insert(Quad(
		Term::Ground(6),
		Term::Ground(5),
		Term::Ground(5),
		None,
	));
	a.insert(Quad(Term::Ground(4), Term::Var(8), Term::Var(0), None));
	a.insert(Quad(
		Term::Ground(7),
		Term::Ground(4),
		Term::Ground(3),
		None,
	));
	a.insert(Quad(Term::Ground(9), Term::Ground(3), Term::Var(7), None));
	a.insert(Quad(Term::Ground(6), Term::Var(0), Term::Ground(1), None));
	a.insert(Quad(Term::Ground(3), Term::Var(2), Term::Ground(9), None));
	a.insert(Quad(Term::Ground(1), Term::Var(7), Term::Var(3), None));
	a.insert(Quad(
		Term::Ground(8),
		Term::Ground(8),
		Term::Ground(1),
		None,
	));
	a.insert(Quad(
		Term::Ground(2),
		Term::Ground(4),
		Term::Ground(6),
		None,
	));
	a.insert(Quad(Term::Ground(2), Term::Var(0), Term::Var(2), None));
	a.insert(Quad(Term::Ground(2), Term::Var(3), Term::Ground(6), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Var(1008),
		Term::Var(1004),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Var(1004),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Var(1000),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Var(1006),
		Term::Ground(1),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Var(1007),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Var(1002),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Var(1005),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Var(1007),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Var(1008),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Ground(5),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Var(1008),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Ground(7),
		Term::Ground(4),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Ground(3),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Var(1000),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Var(1002),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Var(1007),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Ground(8),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(4),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Var(1000),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Var(1003),
		Term::Ground(6),
		None,
	));
	test(a, b)
}
#[test]
fn iso_088() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Ground(1), Term::Ground(5), Term::Var(5), None));
	a.insert(Quad(Term::Var(6), Term::Var(2), Term::Var(3), None));
	a.insert(Quad(Term::Var(5), Term::Var(9), Term::Ground(0), None));
	a.insert(Quad(Term::Var(4), Term::Var(8), Term::Var(2), None));
	a.insert(Quad(Term::Ground(1), Term::Var(4), Term::Var(8), None));
	a.insert(Quad(Term::Var(4), Term::Var(9), Term::Ground(6), None));
	a.insert(Quad(Term::Var(0), Term::Var(0), Term::Var(6), None));
	a.insert(Quad(Term::Var(8), Term::Var(4), Term::Ground(4), None));
	a.insert(Quad(
		Term::Ground(0),
		Term::Ground(3),
		Term::Ground(6),
		None,
	));
	a.insert(Quad(Term::Ground(1), Term::Var(4), Term::Var(1), None));
	a.insert(Quad(Term::Var(5), Term::Ground(9), Term::Ground(0), None));
	a.insert(Quad(Term::Var(0), Term::Var(0), Term::Var(5), None));
	a.insert(Quad(
		Term::Ground(2),
		Term::Ground(9),
		Term::Ground(0),
		None,
	));
	a.insert(Quad(Term::Ground(9), Term::Var(6), Term::Ground(1), None));
	a.insert(Quad(Term::Ground(5), Term::Var(6), Term::Ground(6), None));
	a.insert(Quad(
		Term::Ground(1),
		Term::Ground(8),
		Term::Ground(7),
		None,
	));
	a.insert(Quad(Term::Ground(9), Term::Var(2), Term::Var(9), None));
	a.insert(Quad(
		Term::Ground(2),
		Term::Ground(1),
		Term::Ground(8),
		None,
	));
	a.insert(Quad(
		Term::Ground(8),
		Term::Ground(4),
		Term::Ground(5),
		None,
	));
	a.insert(Quad(Term::Ground(6), Term::Ground(4), Term::Var(3), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(5),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Var(1006),
		Term::Var(1002),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Var(1009),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Var(1008),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Var(1004),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Var(1009),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Var(1000),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Var(1004),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(3),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Var(1004),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Ground(9),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Var(1000),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(9),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Var(1006),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Var(1006),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(8),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Var(1002),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(1),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Ground(4),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Ground(4),
		Term::Var(1003),
		None,
	));
	test(a, b)
}
#[test]
fn iso_089() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Var(9), Term::Ground(6), Term::Ground(5), None));
	a.insert(Quad(Term::Var(7), Term::Ground(4), Term::Var(3), None));
	a.insert(Quad(Term::Ground(9), Term::Var(7), Term::Var(2), None));
	a.insert(Quad(Term::Var(6), Term::Var(5), Term::Var(8), None));
	a.insert(Quad(Term::Var(7), Term::Ground(3), Term::Var(7), None));
	a.insert(Quad(Term::Ground(5), Term::Var(2), Term::Ground(5), None));
	a.insert(Quad(
		Term::Ground(7),
		Term::Ground(8),
		Term::Ground(0),
		None,
	));
	a.insert(Quad(Term::Ground(6), Term::Var(1), Term::Var(0), None));
	a.insert(Quad(Term::Ground(1), Term::Var(3), Term::Var(7), None));
	a.insert(Quad(Term::Ground(0), Term::Ground(9), Term::Var(9), None));
	a.insert(Quad(Term::Var(0), Term::Var(6), Term::Ground(2), None));
	a.insert(Quad(Term::Var(7), Term::Ground(3), Term::Var(6), None));
	a.insert(Quad(Term::Var(1), Term::Var(0), Term::Ground(0), None));
	a.insert(Quad(Term::Var(4), Term::Var(6), Term::Var(5), None));
	a.insert(Quad(Term::Var(0), Term::Var(1), Term::Ground(3), None));
	a.insert(Quad(
		Term::Ground(9),
		Term::Ground(5),
		Term::Ground(0),
		None,
	));
	a.insert(Quad(Term::Var(0), Term::Ground(9), Term::Var(6), None));
	a.insert(Quad(Term::Ground(6), Term::Ground(8), Term::Var(0), None));
	a.insert(Quad(Term::Ground(1), Term::Var(3), Term::Var(9), None));
	a.insert(Quad(Term::Var(5), Term::Var(1), Term::Var(7), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Var(1009),
		Term::Ground(6),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Ground(4),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Var(1007),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Var(1006),
		Term::Var(1005),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Ground(3),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Var(1002),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Ground(7),
		Term::Ground(8),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Var(1001),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Var(1003),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(9),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Var(1006),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Ground(3),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Var(1000),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Var(1006),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Var(1001),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Ground(5),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(9),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Ground(8),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Var(1003),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Var(1001),
		Term::Var(1007),
		None,
	));
	test(a, b)
}
#[test]
fn iso_090() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Var(0), Term::Ground(6), Term::Ground(4), None));
	a.insert(Quad(Term::Var(3), Term::Ground(4), Term::Var(3), None));
	a.insert(Quad(
		Term::Ground(3),
		Term::Ground(2),
		Term::Ground(5),
		None,
	));
	a.insert(Quad(Term::Var(1), Term::Var(6), Term::Var(8), None));
	a.insert(Quad(Term::Ground(0), Term::Var(1), Term::Var(8), None));
	a.insert(Quad(Term::Var(7), Term::Ground(8), Term::Var(4), None));
	a.insert(Quad(Term::Var(5), Term::Var(9), Term::Ground(5), None));
	a.insert(Quad(Term::Var(5), Term::Ground(5), Term::Ground(3), None));
	a.insert(Quad(Term::Var(4), Term::Var(8), Term::Var(5), None));
	a.insert(Quad(Term::Ground(8), Term::Var(7), Term::Ground(1), None));
	a.insert(Quad(Term::Var(2), Term::Var(5), Term::Ground(6), None));
	a.insert(Quad(Term::Var(3), Term::Var(3), Term::Var(5), None));
	a.insert(Quad(
		Term::Ground(8),
		Term::Ground(3),
		Term::Ground(8),
		None,
	));
	a.insert(Quad(Term::Var(0), Term::Ground(6), Term::Var(2), None));
	a.insert(Quad(Term::Var(5), Term::Ground(0), Term::Var(9), None));
	a.insert(Quad(Term::Var(4), Term::Var(3), Term::Ground(1), None));
	a.insert(Quad(Term::Var(9), Term::Var(1), Term::Var(5), None));
	a.insert(Quad(Term::Var(4), Term::Var(9), Term::Var(0), None));
	a.insert(Quad(Term::Var(3), Term::Var(6), Term::Ground(7), None));
	a.insert(Quad(Term::Var(9), Term::Var(2), Term::Var(5), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(6),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Ground(4),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(2),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Var(1006),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Var(1001),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Ground(8),
		Term::Var(1004),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Var(1009),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Ground(5),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Var(1008),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Var(1007),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Var(1005),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Var(1003),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Ground(3),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(6),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Ground(0),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Var(1003),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Var(1001),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Var(1009),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Var(1006),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Var(1002),
		Term::Var(1005),
		None,
	));
	test(a, b)
}
#[test]
fn iso_091() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Var(6), Term::Ground(2), Term::Ground(5), None));
	a.insert(Quad(Term::Var(5), Term::Var(5), Term::Var(5), None));
	a.insert(Quad(Term::Var(1), Term::Ground(5), Term::Var(0), None));
	a.insert(Quad(Term::Var(2), Term::Var(6), Term::Ground(4), None));
	a.insert(Quad(Term::Ground(8), Term::Var(6), Term::Ground(7), None));
	a.insert(Quad(Term::Var(1), Term::Ground(2), Term::Var(2), None));
	a.insert(Quad(Term::Var(4), Term::Ground(8), Term::Ground(0), None));
	a.insert(Quad(Term::Var(4), Term::Var(1), Term::Var(9), None));
	a.insert(Quad(Term::Ground(5), Term::Var(7), Term::Ground(3), None));
	a.insert(Quad(Term::Var(6), Term::Ground(4), Term::Var(7), None));
	a.insert(Quad(Term::Ground(7), Term::Ground(5), Term::Var(6), None));
	a.insert(Quad(Term::Ground(2), Term::Ground(7), Term::Var(1), None));
	a.insert(Quad(Term::Ground(6), Term::Var(1), Term::Ground(0), None));
	a.insert(Quad(Term::Ground(6), Term::Ground(6), Term::Var(3), None));
	a.insert(Quad(Term::Var(2), Term::Ground(2), Term::Ground(0), None));
	a.insert(Quad(Term::Var(7), Term::Var(7), Term::Var(4), None));
	a.insert(Quad(Term::Var(1), Term::Var(9), Term::Ground(6), None));
	a.insert(Quad(Term::Var(1), Term::Var(7), Term::Ground(9), None));
	a.insert(Quad(
		Term::Ground(3),
		Term::Ground(6),
		Term::Ground(4),
		None,
	));
	a.insert(Quad(Term::Ground(3), Term::Var(5), Term::Var(7), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Var(1006),
		Term::Ground(2),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Var(1005),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(5),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Var(1006),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Var(1006),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(2),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Ground(8),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Var(1001),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Var(1007),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Var(1006),
		Term::Ground(4),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Ground(7),
		Term::Ground(5),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(7),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Var(1001),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Ground(6),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Ground(2),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Var(1007),
		Term::Var(1004),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Var(1009),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Var(1007),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(6),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Var(1005),
		Term::Var(1007),
		None,
	));
	test(a, b)
}
#[test]
fn iso_092() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Var(2), Term::Ground(4), Term::Ground(4), None));
	a.insert(Quad(Term::Var(1), Term::Ground(7), Term::Ground(8), None));
	a.insert(Quad(Term::Var(5), Term::Ground(8), Term::Var(5), None));
	a.insert(Quad(Term::Ground(7), Term::Var(4), Term::Var(9), None));
	a.insert(Quad(
		Term::Ground(7),
		Term::Ground(9),
		Term::Ground(9),
		None,
	));
	a.insert(Quad(Term::Var(8), Term::Ground(6), Term::Var(2), None));
	a.insert(Quad(Term::Ground(6), Term::Var(4), Term::Ground(3), None));
	a.insert(Quad(Term::Var(3), Term::Var(1), Term::Var(7), None));
	a.insert(Quad(Term::Var(4), Term::Var(1), Term::Ground(8), None));
	a.insert(Quad(Term::Var(6), Term::Var(0), Term::Var(8), None));
	a.insert(Quad(
		Term::Ground(2),
		Term::Ground(3),
		Term::Ground(4),
		None,
	));
	a.insert(Quad(Term::Var(8), Term::Ground(1), Term::Ground(3), None));
	a.insert(Quad(Term::Ground(6), Term::Ground(4), Term::Var(7), None));
	a.insert(Quad(Term::Var(4), Term::Ground(0), Term::Var(8), None));
	a.insert(Quad(Term::Ground(6), Term::Var(1), Term::Var(7), None));
	a.insert(Quad(Term::Var(0), Term::Ground(3), Term::Var(8), None));
	a.insert(Quad(Term::Ground(0), Term::Ground(3), Term::Var(9), None));
	a.insert(Quad(Term::Var(1), Term::Var(9), Term::Ground(0), None));
	a.insert(Quad(Term::Var(7), Term::Var(2), Term::Var(8), None));
	a.insert(Quad(Term::Var(0), Term::Var(3), Term::Var(3), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Var(1002),
		Term::Ground(4),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(7),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Ground(8),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Ground(7),
		Term::Var(1004),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Ground(7),
		Term::Ground(9),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Ground(6),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Var(1004),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Var(1001),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Var(1001),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Var(1006),
		Term::Var(1000),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(3),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Ground(1),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Ground(4),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Ground(0),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Var(1001),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(3),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(3),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Var(1009),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Var(1002),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Var(1003),
		Term::Var(1003),
		None,
	));
	test(a, b)
}
#[test]
fn iso_093() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Var(7), Term::Ground(3), Term::Ground(1), None));
	a.insert(Quad(Term::Ground(5), Term::Ground(9), Term::Var(3), None));
	a.insert(Quad(Term::Var(4), Term::Ground(5), Term::Var(4), None));
	a.insert(Quad(Term::Ground(7), Term::Var(1), Term::Ground(3), None));
	a.insert(Quad(Term::Var(7), Term::Var(3), Term::Ground(3), None));
	a.insert(Quad(
		Term::Ground(1),
		Term::Ground(6),
		Term::Ground(2),
		None,
	));
	a.insert(Quad(Term::Var(6), Term::Ground(9), Term::Ground(7), None));
	a.insert(Quad(Term::Ground(8), Term::Var(9), Term::Var(7), None));
	a.insert(Quad(Term::Ground(0), Term::Var(9), Term::Ground(2), None));
	a.insert(Quad(Term::Ground(9), Term::Var(4), Term::Var(4), None));
	a.insert(Quad(Term::Ground(5), Term::Ground(0), Term::Var(6), None));
	a.insert(Quad(Term::Ground(0), Term::Var(9), Term::Var(8), None));
	a.insert(Quad(Term::Var(6), Term::Ground(5), Term::Var(0), None));
	a.insert(Quad(Term::Ground(4), Term::Var(7), Term::Ground(5), None));
	a.insert(Quad(Term::Ground(6), Term::Var(4), Term::Ground(7), None));
	a.insert(Quad(Term::Var(1), Term::Var(7), Term::Var(3), None));
	a.insert(Quad(Term::Ground(6), Term::Var(6), Term::Ground(8), None));
	a.insert(Quad(Term::Var(1), Term::Ground(8), Term::Ground(6), None));
	a.insert(Quad(
		Term::Ground(5),
		Term::Ground(8),
		Term::Ground(7),
		None,
	));
	a.insert(Quad(Term::Var(2), Term::Var(0), Term::Var(1), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Var(1007),
		Term::Ground(3),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Ground(9),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Ground(5),
		Term::Var(1004),
		None,
	));
	b.insert(Quad(
		Term::Ground(7),
		Term::Var(1001),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Var(1003),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(6),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Var(1006),
		Term::Ground(9),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Var(1009),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Var(1009),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Var(1004),
		Term::Var(1004),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Ground(0),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Var(1009),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Var(1006),
		Term::Ground(5),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Var(1007),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Var(1004),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Var(1007),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Var(1006),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(8),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Ground(8),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Var(1000),
		Term::Var(1001),
		None,
	));
	test(a, b)
}
#[test]
fn iso_094() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Var(0), Term::Ground(8), Term::Ground(4), None));
	a.insert(Quad(Term::Ground(2), Term::Ground(1), Term::Var(5), None));
	a.insert(Quad(Term::Var(5), Term::Ground(6), Term::Var(3), None));
	a.insert(Quad(Term::Ground(0), Term::Var(2), Term::Var(0), None));
	a.insert(Quad(
		Term::Ground(8),
		Term::Ground(4),
		Term::Ground(1),
		None,
	));
	a.insert(Quad(Term::Var(4), Term::Var(6), Term::Var(5), None));
	a.insert(Quad(Term::Var(7), Term::Ground(3), Term::Var(6), None));
	a.insert(Quad(Term::Ground(8), Term::Ground(4), Term::Var(1), None));
	a.insert(Quad(Term::Var(3), Term::Ground(5), Term::Var(0), None));
	a.insert(Quad(Term::Ground(1), Term::Ground(0), Term::Var(4), None));
	a.insert(Quad(Term::Var(8), Term::Var(5), Term::Ground(3), None));
	a.insert(Quad(Term::Ground(2), Term::Var(8), Term::Var(6), None));
	a.insert(Quad(Term::Ground(9), Term::Var(8), Term::Ground(0), None));
	a.insert(Quad(Term::Var(8), Term::Ground(1), Term::Var(6), None));
	a.insert(Quad(Term::Ground(1), Term::Var(5), Term::Ground(9), None));
	a.insert(Quad(Term::Ground(0), Term::Ground(6), Term::Var(2), None));
	a.insert(Quad(
		Term::Ground(8),
		Term::Ground(2),
		Term::Ground(6),
		None,
	));
	a.insert(Quad(Term::Var(4), Term::Var(3), Term::Ground(7), None));
	a.insert(Quad(Term::Var(7), Term::Var(8), Term::Ground(9), None));
	a.insert(Quad(Term::Var(7), Term::Ground(0), Term::Ground(1), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(8),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(1),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Ground(6),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Var(1002),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Ground(4),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Var(1006),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Ground(3),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Ground(4),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Ground(5),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(0),
		Term::Var(1004),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Var(1005),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Var(1008),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Var(1008),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Ground(1),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Var(1005),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(6),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Ground(2),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Var(1003),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Var(1008),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Ground(0),
		Term::Ground(1),
		None,
	));
	test(a, b)
}
#[test]
fn iso_095() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Var(6), Term::Var(9), Term::Ground(6), None));
	a.insert(Quad(
		Term::Ground(6),
		Term::Ground(3),
		Term::Ground(0),
		None,
	));
	a.insert(Quad(Term::Var(9), Term::Var(3), Term::Ground(1), None));
	a.insert(Quad(Term::Ground(9), Term::Ground(4), Term::Var(5), None));
	a.insert(Quad(Term::Ground(4), Term::Var(9), Term::Ground(1), None));
	a.insert(Quad(Term::Var(2), Term::Ground(2), Term::Var(9), None));
	a.insert(Quad(Term::Var(9), Term::Ground(9), Term::Var(5), None));
	a.insert(Quad(Term::Var(1), Term::Ground(2), Term::Ground(8), None));
	a.insert(Quad(Term::Ground(4), Term::Ground(0), Term::Var(0), None));
	a.insert(Quad(Term::Var(6), Term::Var(3), Term::Var(0), None));
	a.insert(Quad(Term::Var(5), Term::Ground(8), Term::Ground(9), None));
	a.insert(Quad(
		Term::Ground(7),
		Term::Ground(6),
		Term::Ground(9),
		None,
	));
	a.insert(Quad(Term::Ground(5), Term::Var(4), Term::Ground(9), None));
	a.insert(Quad(Term::Var(1), Term::Var(7), Term::Var(9), None));
	a.insert(Quad(Term::Ground(8), Term::Var(5), Term::Var(3), None));
	a.insert(Quad(Term::Var(2), Term::Var(4), Term::Ground(3), None));
	a.insert(Quad(Term::Var(9), Term::Ground(5), Term::Ground(5), None));
	a.insert(Quad(Term::Ground(7), Term::Ground(0), Term::Var(1), None));
	a.insert(Quad(Term::Var(1), Term::Var(7), Term::Ground(8), None));
	a.insert(Quad(Term::Var(9), Term::Ground(7), Term::Ground(4), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Var(1006),
		Term::Var(1009),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Ground(3),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Var(1003),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Ground(4),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Var(1009),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Ground(2),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Ground(9),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(2),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Ground(0),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Var(1006),
		Term::Var(1003),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Ground(8),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Ground(7),
		Term::Ground(6),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Var(1004),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Var(1007),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Var(1005),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Var(1004),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Ground(5),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Ground(7),
		Term::Ground(0),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Var(1007),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Ground(7),
		Term::Ground(4),
		None,
	));
	test(a, b)
}
#[test]
fn iso_096() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(
		Term::Ground(2),
		Term::Ground(3),
		Term::Ground(6),
		None,
	));
	a.insert(Quad(Term::Ground(0), Term::Var(1), Term::Var(3), None));
	a.insert(Quad(Term::Var(9), Term::Var(6), Term::Var(5), None));
	a.insert(Quad(Term::Var(9), Term::Var(4), Term::Var(8), None));
	a.insert(Quad(Term::Ground(0), Term::Var(8), Term::Ground(0), None));
	a.insert(Quad(Term::Ground(2), Term::Var(8), Term::Ground(1), None));
	a.insert(Quad(Term::Var(5), Term::Ground(7), Term::Ground(6), None));
	a.insert(Quad(Term::Ground(7), Term::Var(5), Term::Var(0), None));
	a.insert(Quad(Term::Var(5), Term::Ground(3), Term::Ground(1), None));
	a.insert(Quad(Term::Ground(1), Term::Ground(4), Term::Var(3), None));
	a.insert(Quad(Term::Ground(1), Term::Var(5), Term::Ground(0), None));
	a.insert(Quad(Term::Ground(0), Term::Var(0), Term::Var(8), None));
	a.insert(Quad(
		Term::Ground(1),
		Term::Ground(3),
		Term::Ground(1),
		None,
	));
	a.insert(Quad(Term::Var(8), Term::Var(3), Term::Var(2), None));
	a.insert(Quad(Term::Var(5), Term::Var(2), Term::Var(9), None));
	a.insert(Quad(Term::Ground(1), Term::Ground(8), Term::Var(9), None));
	a.insert(Quad(Term::Var(5), Term::Var(0), Term::Var(9), None));
	a.insert(Quad(Term::Var(6), Term::Ground(3), Term::Var(6), None));
	a.insert(Quad(
		Term::Ground(9),
		Term::Ground(5),
		Term::Ground(1),
		None,
	));
	a.insert(Quad(Term::Ground(1), Term::Var(0), Term::Ground(5), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(3),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Var(1001),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Var(1006),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Var(1004),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Var(1008),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Var(1008),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Ground(7),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Ground(7),
		Term::Var(1005),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Ground(3),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(4),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Var(1005),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Var(1000),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(3),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Var(1003),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Var(1002),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(8),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Var(1000),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Var(1006),
		Term::Ground(3),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Ground(5),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Var(1000),
		Term::Ground(5),
		None,
	));
	test(a, b)
}
#[test]
fn iso_097() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Var(3), Term::Var(0), Term::Ground(0), None));
	a.insert(Quad(
		Term::Ground(7),
		Term::Ground(6),
		Term::Ground(2),
		None,
	));
	a.insert(Quad(Term::Ground(0), Term::Var(3), Term::Var(0), None));
	a.insert(Quad(Term::Var(3), Term::Ground(6), Term::Var(7), None));
	a.insert(Quad(Term::Var(8), Term::Ground(3), Term::Ground(1), None));
	a.insert(Quad(Term::Ground(7), Term::Var(6), Term::Var(2), None));
	a.insert(Quad(Term::Ground(5), Term::Var(5), Term::Var(2), None));
	a.insert(Quad(Term::Var(1), Term::Var(7), Term::Ground(6), None));
	a.insert(Quad(
		Term::Ground(8),
		Term::Ground(9),
		Term::Ground(2),
		None,
	));
	a.insert(Quad(Term::Var(2), Term::Ground(7), Term::Var(7), None));
	a.insert(Quad(Term::Ground(5), Term::Var(5), Term::Ground(5), None));
	a.insert(Quad(Term::Ground(7), Term::Ground(8), Term::Var(1), None));
	a.insert(Quad(Term::Var(8), Term::Var(5), Term::Ground(3), None));
	a.insert(Quad(Term::Ground(9), Term::Ground(7), Term::Var(7), None));
	a.insert(Quad(Term::Ground(1), Term::Var(1), Term::Ground(4), None));
	a.insert(Quad(Term::Var(0), Term::Ground(5), Term::Ground(1), None));
	a.insert(Quad(Term::Ground(4), Term::Var(9), Term::Var(4), None));
	a.insert(Quad(Term::Var(0), Term::Ground(7), Term::Ground(0), None));
	a.insert(Quad(
		Term::Ground(3),
		Term::Ground(6),
		Term::Ground(9),
		None,
	));
	a.insert(Quad(Term::Var(8), Term::Var(9), Term::Var(8), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Var(1003),
		Term::Var(1000),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(7),
		Term::Ground(6),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Var(1003),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Ground(6),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Ground(3),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Ground(7),
		Term::Var(1006),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Var(1005),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Var(1007),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Ground(9),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Ground(7),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Var(1005),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Ground(7),
		Term::Ground(8),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Var(1005),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Ground(7),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Var(1001),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(5),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Var(1009),
		Term::Var(1004),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(7),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(6),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Var(1009),
		Term::Var(1008),
		None,
	));
	test(a, b)
}
#[test]
fn iso_098() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Var(2), Term::Var(3), Term::Ground(7), None));
	a.insert(Quad(Term::Var(3), Term::Ground(9), Term::Var(6), None));
	a.insert(Quad(Term::Var(2), Term::Var(9), Term::Var(4), None));
	a.insert(Quad(Term::Var(0), Term::Ground(9), Term::Ground(6), None));
	a.insert(Quad(Term::Var(4), Term::Ground(4), Term::Var(7), None));
	a.insert(Quad(Term::Ground(5), Term::Var(9), Term::Ground(8), None));
	a.insert(Quad(
		Term::Ground(1),
		Term::Ground(0),
		Term::Ground(5),
		None,
	));
	a.insert(Quad(Term::Var(4), Term::Var(1), Term::Ground(1), None));
	a.insert(Quad(Term::Var(5), Term::Ground(9), Term::Ground(9), None));
	a.insert(Quad(Term::Ground(0), Term::Ground(3), Term::Var(9), None));
	a.insert(Quad(Term::Ground(3), Term::Var(0), Term::Var(3), None));
	a.insert(Quad(Term::Ground(4), Term::Var(5), Term::Var(4), None));
	a.insert(Quad(Term::Ground(9), Term::Ground(7), Term::Var(9), None));
	a.insert(Quad(Term::Ground(3), Term::Var(7), Term::Var(6), None));
	a.insert(Quad(Term::Ground(4), Term::Var(2), Term::Var(5), None));
	a.insert(Quad(Term::Var(7), Term::Var(9), Term::Ground(0), None));
	a.insert(Quad(Term::Var(6), Term::Var(5), Term::Var(3), None));
	a.insert(Quad(Term::Ground(1), Term::Ground(5), Term::Var(2), None));
	a.insert(Quad(Term::Ground(5), Term::Var(8), Term::Ground(5), None));
	a.insert(Quad(Term::Ground(0), Term::Var(7), Term::Ground(0), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Var(1002),
		Term::Var(1003),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Ground(9),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Var(1009),
		Term::Var(1004),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(9),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Ground(4),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Var(1009),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(0),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Var(1001),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Ground(9),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(3),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Var(1000),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Var(1005),
		Term::Var(1004),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Ground(7),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Var(1007),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Var(1002),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Var(1009),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Var(1006),
		Term::Var(1005),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(5),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Var(1008),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Var(1007),
		Term::Ground(0),
		None,
	));
	test(a, b)
}
#[test]
fn iso_099() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Var(8), Term::Var(1), Term::Ground(3), None));
	a.insert(Quad(Term::Ground(9), Term::Ground(7), Term::Var(4), None));
	a.insert(Quad(Term::Ground(8), Term::Var(0), Term::Ground(7), None));
	a.insert(Quad(Term::Var(2), Term::Ground(7), Term::Var(1), None));
	a.insert(Quad(Term::Ground(8), Term::Var(7), Term::Ground(5), None));
	a.insert(Quad(Term::Var(3), Term::Ground(2), Term::Ground(9), None));
	a.insert(Quad(Term::Ground(1), Term::Var(6), Term::Ground(5), None));
	a.insert(Quad(Term::Ground(6), Term::Var(1), Term::Ground(9), None));
	a.insert(Quad(Term::Var(1), Term::Ground(7), Term::Var(6), None));
	a.insert(Quad(Term::Ground(2), Term::Var(1), Term::Var(2), None));
	a.insert(Quad(Term::Ground(7), Term::Var(9), Term::Ground(9), None));
	a.insert(Quad(Term::Var(4), Term::Var(6), Term::Ground(3), None));
	a.insert(Quad(Term::Var(1), Term::Var(7), Term::Ground(8), None));
	a.insert(Quad(Term::Var(6), Term::Ground(8), Term::Ground(8), None));
	a.insert(Quad(Term::Ground(9), Term::Var(2), Term::Ground(8), None));
	a.insert(Quad(Term::Ground(2), Term::Ground(6), Term::Var(0), None));
	a.insert(Quad(Term::Var(8), Term::Var(3), Term::Ground(7), None));
	a.insert(Quad(Term::Var(8), Term::Var(2), Term::Ground(6), None));
	a.insert(Quad(Term::Var(9), Term::Var(9), Term::Var(0), None));
	a.insert(Quad(Term::Var(4), Term::Var(9), Term::Ground(8), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Var(1008),
		Term::Var(1001),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Ground(7),
		Term::Var(1004),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Var(1000),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Ground(7),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Var(1007),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Ground(2),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Var(1006),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Var(1001),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(7),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Var(1001),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Ground(7),
		Term::Var(1009),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Var(1006),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Var(1007),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Var(1006),
		Term::Ground(8),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Var(1002),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(6),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Var(1003),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Var(1002),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Var(1009),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Var(1009),
		Term::Ground(8),
		None,
	));
	test(a, b)
}
#[test]
fn iso_100() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Ground(6), Term::Var(4), Term::Var(6), None));
	a.insert(Quad(Term::Var(7), Term::Ground(0), Term::Ground(2), None));
	a.insert(Quad(Term::Var(8), Term::Var(0), Term::Ground(7), None));
	a.insert(Quad(
		Term::Ground(4),
		Term::Ground(4),
		Term::Ground(7),
		None,
	));
	a.insert(Quad(Term::Var(3), Term::Var(1), Term::Var(8), None));
	a.insert(Quad(Term::Ground(1), Term::Var(6), Term::Ground(2), None));
	a.insert(Quad(Term::Ground(3), Term::Ground(3), Term::Var(4), None));
	a.insert(Quad(Term::Var(7), Term::Ground(2), Term::Var(1), None));
	a.insert(Quad(Term::Var(7), Term::Var(5), Term::Var(0), None));
	a.insert(Quad(Term::Var(1), Term::Var(6), Term::Ground(6), None));
	a.insert(Quad(Term::Var(0), Term::Ground(3), Term::Ground(9), None));
	a.insert(Quad(
		Term::Ground(7),
		Term::Ground(4),
		Term::Ground(3),
		None,
	));
	a.insert(Quad(Term::Var(5), Term::Var(7), Term::Var(9), None));
	a.insert(Quad(Term::Var(1), Term::Var(7), Term::Var(7), None));
	a.insert(Quad(Term::Ground(5), Term::Var(3), Term::Ground(8), None));
	a.insert(Quad(
		Term::Ground(4),
		Term::Ground(1),
		Term::Ground(0),
		None,
	));
	a.insert(Quad(Term::Var(4), Term::Var(3), Term::Ground(5), None));
	a.insert(Quad(
		Term::Ground(1),
		Term::Ground(8),
		Term::Ground(4),
		None,
	));
	a.insert(Quad(Term::Var(6), Term::Ground(7), Term::Var(8), None));
	a.insert(Quad(Term::Var(6), Term::Ground(7), Term::Ground(3), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(6),
		Term::Var(1004),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Ground(0),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Var(1000),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Ground(4),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Var(1001),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Var(1006),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(3),
		Term::Var(1004),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Ground(2),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Var(1005),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Var(1006),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(3),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Ground(7),
		Term::Ground(4),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Var(1007),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Var(1007),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Var(1003),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Ground(1),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Var(1003),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(8),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Var(1006),
		Term::Ground(7),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Var(1006),
		Term::Ground(7),
		Term::Ground(3),
		None,
	));
	test(a, b)
}
#[test]
fn iso_101() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Var(1), Term::Var(7), Term::Ground(2), None));
	a.insert(Quad(Term::Ground(2), Term::Ground(2), Term::Var(4), None));
	a.insert(Quad(Term::Ground(7), Term::Var(2), Term::Ground(6), None));
	a.insert(Quad(Term::Var(8), Term::Var(8), Term::Var(0), None));
	a.insert(Quad(
		Term::Ground(1),
		Term::Ground(6),
		Term::Ground(3),
		None,
	));
	a.insert(Quad(Term::Ground(4), Term::Ground(0), Term::Var(7), None));
	a.insert(Quad(Term::Var(0), Term::Var(2), Term::Var(6), None));
	a.insert(Quad(Term::Ground(6), Term::Ground(6), Term::Var(7), None));
	a.insert(Quad(Term::Var(9), Term::Ground(9), Term::Var(3), None));
	a.insert(Quad(Term::Var(4), Term::Ground(3), Term::Ground(8), None));
	a.insert(Quad(Term::Var(9), Term::Ground(1), Term::Ground(2), None));
	a.insert(Quad(Term::Ground(5), Term::Var(8), Term::Var(5), None));
	a.insert(Quad(
		Term::Ground(4),
		Term::Ground(9),
		Term::Ground(6),
		None,
	));
	a.insert(Quad(Term::Var(1), Term::Ground(5), Term::Var(6), None));
	a.insert(Quad(Term::Var(7), Term::Var(2), Term::Ground(4), None));
	a.insert(Quad(Term::Ground(6), Term::Var(7), Term::Ground(8), None));
	a.insert(Quad(
		Term::Ground(3),
		Term::Ground(9),
		Term::Ground(7),
		None,
	));
	a.insert(Quad(
		Term::Ground(2),
		Term::Ground(7),
		Term::Ground(1),
		None,
	));
	a.insert(Quad(Term::Var(5), Term::Ground(7), Term::Var(1), None));
	a.insert(Quad(Term::Var(2), Term::Ground(0), Term::Ground(2), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Var(1001),
		Term::Var(1007),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(2),
		Term::Var(1004),
		None,
	));
	b.insert(Quad(
		Term::Ground(7),
		Term::Var(1002),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Var(1008),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(6),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Ground(0),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Var(1002),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Ground(6),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Ground(9),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Ground(3),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Ground(1),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Var(1008),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Ground(9),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(5),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Var(1002),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Var(1007),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(9),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(7),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Ground(7),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Ground(0),
		Term::Ground(2),
		None,
	));
	test(a, b)
}
#[test]
fn iso_102() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Ground(5), Term::Var(1), Term::Var(6), None));
	a.insert(Quad(
		Term::Ground(9),
		Term::Ground(9),
		Term::Ground(3),
		None,
	));
	a.insert(Quad(Term::Var(4), Term::Ground(5), Term::Var(5), None));
	a.insert(Quad(Term::Var(8), Term::Var(2), Term::Ground(7), None));
	a.insert(Quad(Term::Var(4), Term::Ground(9), Term::Ground(2), None));
	a.insert(Quad(Term::Ground(3), Term::Var(2), Term::Var(3), None));
	a.insert(Quad(Term::Var(2), Term::Var(5), Term::Var(0), None));
	a.insert(Quad(Term::Ground(8), Term::Ground(7), Term::Var(4), None));
	a.insert(Quad(Term::Var(0), Term::Var(5), Term::Ground(6), None));
	a.insert(Quad(Term::Var(3), Term::Var(8), Term::Var(9), None));
	a.insert(Quad(
		Term::Ground(7),
		Term::Ground(0),
		Term::Ground(5),
		None,
	));
	a.insert(Quad(Term::Ground(1), Term::Var(8), Term::Ground(0), None));
	a.insert(Quad(Term::Ground(4), Term::Var(3), Term::Ground(5), None));
	a.insert(Quad(Term::Var(7), Term::Ground(5), Term::Var(3), None));
	a.insert(Quad(Term::Var(5), Term::Var(2), Term::Var(9), None));
	a.insert(Quad(Term::Ground(7), Term::Var(8), Term::Ground(5), None));
	a.insert(Quad(Term::Var(3), Term::Ground(7), Term::Ground(8), None));
	a.insert(Quad(
		Term::Ground(2),
		Term::Ground(2),
		Term::Ground(2),
		None,
	));
	a.insert(Quad(Term::Var(7), Term::Ground(3), Term::Ground(9), None));
	a.insert(Quad(
		Term::Ground(9),
		Term::Ground(8),
		Term::Ground(0),
		None,
	));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(5),
		Term::Var(1001),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Ground(9),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Ground(5),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Var(1002),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Ground(9),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Var(1002),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Var(1005),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Ground(7),
		Term::Var(1004),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Var(1005),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Var(1008),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Ground(7),
		Term::Ground(0),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Var(1008),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Var(1003),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Ground(5),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Var(1002),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Ground(7),
		Term::Var(1008),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Ground(7),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(2),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Ground(3),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Ground(8),
		Term::Ground(0),
		None,
	));
	test(a, b)
}
#[test]
fn iso_103() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Var(3), Term::Ground(9), Term::Var(2), None));
	a.insert(Quad(Term::Var(6), Term::Ground(4), Term::Ground(1), None));
	a.insert(Quad(Term::Var(3), Term::Ground(2), Term::Var(6), None));
	a.insert(Quad(Term::Ground(6), Term::Var(6), Term::Var(4), None));
	a.insert(Quad(Term::Var(6), Term::Var(2), Term::Ground(4), None));
	a.insert(Quad(Term::Var(8), Term::Var(6), Term::Ground(4), None));
	a.insert(Quad(Term::Ground(4), Term::Var(2), Term::Ground(1), None));
	a.insert(Quad(Term::Ground(6), Term::Var(1), Term::Var(7), None));
	a.insert(Quad(Term::Var(1), Term::Ground(6), Term::Var(2), None));
	a.insert(Quad(Term::Ground(2), Term::Var(0), Term::Ground(4), None));
	a.insert(Quad(Term::Ground(9), Term::Ground(0), Term::Var(0), None));
	a.insert(Quad(Term::Var(4), Term::Ground(6), Term::Ground(4), None));
	a.insert(Quad(Term::Ground(0), Term::Ground(2), Term::Var(5), None));
	a.insert(Quad(
		Term::Ground(3),
		Term::Ground(9),
		Term::Ground(1),
		None,
	));
	a.insert(Quad(Term::Ground(0), Term::Ground(2), Term::Var(7), None));
	a.insert(Quad(Term::Ground(1), Term::Ground(5), Term::Var(4), None));
	a.insert(Quad(
		Term::Ground(4),
		Term::Ground(5),
		Term::Ground(0),
		None,
	));
	a.insert(Quad(
		Term::Ground(0),
		Term::Ground(2),
		Term::Ground(0),
		None,
	));
	a.insert(Quad(Term::Var(7), Term::Ground(9), Term::Ground(0), None));
	a.insert(Quad(Term::Var(4), Term::Ground(5), Term::Var(7), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Var(1003),
		Term::Ground(9),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Var(1006),
		Term::Ground(4),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Ground(2),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Var(1006),
		Term::Var(1004),
		None,
	));
	b.insert(Quad(
		Term::Var(1006),
		Term::Var(1002),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Var(1006),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Var(1002),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Var(1001),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(6),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Var(1000),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Ground(0),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Ground(6),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(2),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(9),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(2),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(5),
		Term::Var(1004),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Ground(5),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(2),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Ground(9),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Ground(5),
		Term::Var(1007),
		None,
	));
	test(a, b)
}
#[test]
fn iso_104() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Var(6), Term::Var(9), Term::Var(1), None));
	a.insert(Quad(
		Term::Ground(8),
		Term::Ground(9),
		Term::Ground(1),
		None,
	));
	a.insert(Quad(Term::Var(4), Term::Var(7), Term::Var(7), None));
	a.insert(Quad(Term::Var(0), Term::Var(5), Term::Ground(1), None));
	a.insert(Quad(Term::Var(4), Term::Var(4), Term::Ground(9), None));
	a.insert(Quad(
		Term::Ground(6),
		Term::Ground(0),
		Term::Ground(3),
		None,
	));
	a.insert(Quad(Term::Var(2), Term::Var(7), Term::Ground(7), None));
	a.insert(Quad(Term::Var(0), Term::Var(6), Term::Var(1), None));
	a.insert(Quad(Term::Ground(9), Term::Var(6), Term::Ground(0), None));
	a.insert(Quad(Term::Ground(9), Term::Ground(1), Term::Var(6), None));
	a.insert(Quad(Term::Ground(6), Term::Var(0), Term::Var(9), None));
	a.insert(Quad(Term::Var(6), Term::Ground(3), Term::Var(9), None));
	a.insert(Quad(Term::Ground(6), Term::Var(9), Term::Var(6), None));
	a.insert(Quad(Term::Var(3), Term::Ground(4), Term::Ground(4), None));
	a.insert(Quad(Term::Ground(2), Term::Ground(8), Term::Var(8), None));
	a.insert(Quad(Term::Ground(8), Term::Ground(7), Term::Var(0), None));
	a.insert(Quad(Term::Ground(7), Term::Ground(7), Term::Var(2), None));
	a.insert(Quad(Term::Var(9), Term::Ground(0), Term::Ground(2), None));
	a.insert(Quad(Term::Ground(8), Term::Var(4), Term::Ground(3), None));
	a.insert(Quad(Term::Var(7), Term::Var(4), Term::Ground(4), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Var(1006),
		Term::Var(1009),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Ground(9),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Var(1007),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Var(1005),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Var(1004),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Ground(0),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Var(1007),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Var(1006),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Var(1006),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Ground(1),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Var(1000),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Var(1006),
		Term::Ground(3),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Var(1009),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Ground(4),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(8),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Ground(7),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Ground(7),
		Term::Ground(7),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Ground(0),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Var(1004),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Var(1004),
		Term::Ground(4),
		None,
	));
	test(a, b)
}
#[test]
fn iso_105() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Ground(7), Term::Var(2), Term::Ground(3), None));
	a.insert(Quad(Term::Var(9), Term::Var(4), Term::Ground(6), None));
	a.insert(Quad(Term::Var(7), Term::Var(4), Term::Ground(7), None));
	a.insert(Quad(Term::Var(5), Term::Ground(5), Term::Var(2), None));
	a.insert(Quad(Term::Ground(3), Term::Ground(2), Term::Var(7), None));
	a.insert(Quad(Term::Var(8), Term::Ground(4), Term::Var(3), None));
	a.insert(Quad(Term::Var(6), Term::Var(1), Term::Ground(5), None));
	a.insert(Quad(
		Term::Ground(4),
		Term::Ground(9),
		Term::Ground(5),
		None,
	));
	a.insert(Quad(Term::Var(2), Term::Var(3), Term::Ground(3), None));
	a.insert(Quad(Term::Var(5), Term::Ground(4), Term::Var(9), None));
	a.insert(Quad(Term::Var(5), Term::Var(7), Term::Ground(3), None));
	a.insert(Quad(
		Term::Ground(4),
		Term::Ground(2),
		Term::Ground(5),
		None,
	));
	a.insert(Quad(Term::Ground(0), Term::Var(2), Term::Var(9), None));
	a.insert(Quad(Term::Var(9), Term::Var(1), Term::Var(5), None));
	a.insert(Quad(Term::Ground(9), Term::Ground(2), Term::Var(9), None));
	a.insert(Quad(Term::Var(4), Term::Var(1), Term::Var(8), None));
	a.insert(Quad(Term::Var(3), Term::Var(7), Term::Var(8), None));
	a.insert(Quad(Term::Var(8), Term::Ground(2), Term::Var(8), None));
	a.insert(Quad(Term::Var(0), Term::Ground(7), Term::Var(8), None));
	a.insert(Quad(Term::Var(2), Term::Var(9), Term::Var(2), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(7),
		Term::Var(1002),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Var(1004),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Var(1004),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Ground(5),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(2),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Ground(4),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Var(1006),
		Term::Var(1001),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Ground(9),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Var(1003),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Ground(4),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Var(1007),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Ground(2),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Var(1002),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Var(1001),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Ground(2),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Var(1001),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Var(1007),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Ground(2),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(7),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Var(1009),
		Term::Var(1002),
		None,
	));
	test(a, b)
}
#[test]
fn iso_106() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Ground(7), Term::Var(2), Term::Var(3), None));
	a.insert(Quad(Term::Var(8), Term::Var(9), Term::Var(2), None));
	a.insert(Quad(Term::Var(0), Term::Var(8), Term::Var(2), None));
	a.insert(Quad(Term::Ground(2), Term::Var(2), Term::Ground(4), None));
	a.insert(Quad(
		Term::Ground(9),
		Term::Ground(4),
		Term::Ground(9),
		None,
	));
	a.insert(Quad(Term::Ground(4), Term::Var(6), Term::Var(3), None));
	a.insert(Quad(Term::Var(6), Term::Var(5), Term::Ground(6), None));
	a.insert(Quad(Term::Var(4), Term::Ground(2), Term::Var(3), None));
	a.insert(Quad(Term::Var(0), Term::Ground(1), Term::Ground(8), None));
	a.insert(Quad(Term::Ground(5), Term::Ground(3), Term::Var(9), None));
	a.insert(Quad(Term::Ground(5), Term::Var(3), Term::Var(2), None));
	a.insert(Quad(Term::Var(8), Term::Var(5), Term::Ground(6), None));
	a.insert(Quad(Term::Ground(5), Term::Var(6), Term::Var(2), None));
	a.insert(Quad(Term::Var(5), Term::Ground(5), Term::Ground(1), None));
	a.insert(Quad(Term::Var(2), Term::Ground(7), Term::Var(0), None));
	a.insert(Quad(Term::Ground(9), Term::Var(4), Term::Var(9), None));
	a.insert(Quad(
		Term::Ground(4),
		Term::Ground(7),
		Term::Ground(7),
		None,
	));
	a.insert(Quad(Term::Ground(3), Term::Ground(7), Term::Var(8), None));
	a.insert(Quad(Term::Ground(2), Term::Ground(0), Term::Var(7), None));
	a.insert(Quad(Term::Ground(8), Term::Ground(7), Term::Var(1), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(7),
		Term::Var(1002),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Var(1009),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Var(1008),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Var(1002),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Ground(4),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Var(1006),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Var(1006),
		Term::Var(1005),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Ground(2),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(1),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Ground(3),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Var(1003),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Var(1005),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Var(1006),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Ground(5),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Ground(7),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Var(1004),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Ground(7),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(7),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(0),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Ground(7),
		Term::Var(1001),
		None,
	));
	test(a, b)
}
#[test]
fn iso_107() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Ground(1), Term::Ground(2), Term::Var(1), None));
	a.insert(Quad(Term::Var(6), Term::Ground(5), Term::Var(6), None));
	a.insert(Quad(Term::Var(3), Term::Var(5), Term::Var(4), None));
	a.insert(Quad(
		Term::Ground(8),
		Term::Ground(9),
		Term::Ground(4),
		None,
	));
	a.insert(Quad(Term::Var(5), Term::Ground(1), Term::Var(4), None));
	a.insert(Quad(Term::Var(6), Term::Var(0), Term::Ground(2), None));
	a.insert(Quad(Term::Var(8), Term::Var(2), Term::Ground(3), None));
	a.insert(Quad(Term::Ground(2), Term::Var(4), Term::Var(0), None));
	a.insert(Quad(Term::Var(7), Term::Var(3), Term::Var(2), None));
	a.insert(Quad(Term::Var(3), Term::Ground(8), Term::Ground(4), None));
	a.insert(Quad(Term::Ground(7), Term::Var(2), Term::Var(0), None));
	a.insert(Quad(Term::Var(3), Term::Var(0), Term::Ground(9), None));
	a.insert(Quad(Term::Var(9), Term::Var(2), Term::Ground(5), None));
	a.insert(Quad(
		Term::Ground(7),
		Term::Ground(0),
		Term::Ground(5),
		None,
	));
	a.insert(Quad(Term::Var(9), Term::Var(6), Term::Var(9), None));
	a.insert(Quad(Term::Var(6), Term::Var(3), Term::Ground(1), None));
	a.insert(Quad(
		Term::Ground(8),
		Term::Ground(2),
		Term::Ground(4),
		None,
	));
	a.insert(Quad(Term::Ground(1), Term::Ground(1), Term::Var(8), None));
	a.insert(Quad(Term::Var(7), Term::Ground(2), Term::Ground(3), None));
	a.insert(Quad(Term::Var(9), Term::Var(1), Term::Var(4), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(2),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Var(1006),
		Term::Ground(5),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Var(1005),
		Term::Var(1004),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Ground(9),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Ground(1),
		Term::Var(1004),
		None,
	));
	b.insert(Quad(
		Term::Var(1006),
		Term::Var(1000),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Var(1002),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Var(1004),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Var(1003),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Ground(8),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Ground(7),
		Term::Var(1002),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Var(1000),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Var(1002),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Ground(7),
		Term::Ground(0),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Var(1006),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Var(1006),
		Term::Var(1003),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Ground(2),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(1),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Ground(2),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Var(1001),
		Term::Var(1004),
		None,
	));
	test(a, b)
}
#[test]
fn iso_108() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Var(9), Term::Var(8), Term::Var(1), None));
	a.insert(Quad(Term::Var(5), Term::Var(2), Term::Var(7), None));
	a.insert(Quad(Term::Var(6), Term::Ground(4), Term::Ground(5), None));
	a.insert(Quad(Term::Var(2), Term::Ground(8), Term::Var(0), None));
	a.insert(Quad(Term::Var(8), Term::Ground(5), Term::Var(3), None));
	a.insert(Quad(Term::Var(2), Term::Var(1), Term::Var(4), None));
	a.insert(Quad(Term::Ground(8), Term::Ground(6), Term::Var(6), None));
	a.insert(Quad(
		Term::Ground(0),
		Term::Ground(4),
		Term::Ground(4),
		None,
	));
	a.insert(Quad(Term::Var(3), Term::Ground(8), Term::Ground(5), None));
	a.insert(Quad(Term::Ground(7), Term::Var(8), Term::Var(8), None));
	a.insert(Quad(Term::Ground(8), Term::Var(4), Term::Var(9), None));
	a.insert(Quad(Term::Var(8), Term::Var(0), Term::Var(9), None));
	a.insert(Quad(Term::Ground(7), Term::Var(6), Term::Var(0), None));
	a.insert(Quad(Term::Var(5), Term::Var(5), Term::Ground(7), None));
	a.insert(Quad(
		Term::Ground(8),
		Term::Ground(5),
		Term::Ground(4),
		None,
	));
	a.insert(Quad(Term::Var(6), Term::Ground(9), Term::Ground(2), None));
	a.insert(Quad(Term::Var(1), Term::Ground(3), Term::Var(9), None));
	a.insert(Quad(Term::Ground(4), Term::Var(2), Term::Var(8), None));
	a.insert(Quad(Term::Var(3), Term::Var(3), Term::Var(4), None));
	a.insert(Quad(
		Term::Ground(9),
		Term::Ground(5),
		Term::Ground(0),
		None,
	));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Var(1009),
		Term::Var(1008),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Var(1002),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Var(1006),
		Term::Ground(4),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Ground(8),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Ground(5),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Var(1001),
		Term::Var(1004),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Ground(6),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(4),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Ground(8),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Ground(7),
		Term::Var(1008),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Var(1004),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Var(1000),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Ground(7),
		Term::Var(1006),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Var(1005),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Ground(5),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Var(1006),
		Term::Ground(9),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(3),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Var(1002),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Var(1003),
		Term::Var(1004),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Ground(5),
		Term::Ground(0),
		None,
	));
	test(a, b)
}
#[test]
fn iso_109() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Ground(6), Term::Var(9), Term::Var(8), None));
	a.insert(Quad(Term::Var(1), Term::Var(8), Term::Ground(6), None));
	a.insert(Quad(Term::Ground(7), Term::Var(9), Term::Ground(7), None));
	a.insert(Quad(Term::Ground(2), Term::Var(4), Term::Ground(9), None));
	a.insert(Quad(Term::Var(2), Term::Ground(2), Term::Ground(7), None));
	a.insert(Quad(Term::Ground(4), Term::Var(8), Term::Ground(5), None));
	a.insert(Quad(
		Term::Ground(9),
		Term::Ground(0),
		Term::Ground(3),
		None,
	));
	a.insert(Quad(Term::Var(1), Term::Var(5), Term::Var(6), None));
	a.insert(Quad(
		Term::Ground(1),
		Term::Ground(7),
		Term::Ground(5),
		None,
	));
	a.insert(Quad(Term::Ground(3), Term::Var(1), Term::Ground(6), None));
	a.insert(Quad(Term::Ground(7), Term::Var(8), Term::Ground(3), None));
	a.insert(Quad(Term::Ground(6), Term::Var(9), Term::Ground(7), None));
	a.insert(Quad(Term::Var(9), Term::Ground(1), Term::Var(7), None));
	a.insert(Quad(Term::Var(0), Term::Var(7), Term::Var(0), None));
	a.insert(Quad(
		Term::Ground(7),
		Term::Ground(7),
		Term::Ground(8),
		None,
	));
	a.insert(Quad(Term::Ground(7), Term::Var(5), Term::Var(1), None));
	a.insert(Quad(Term::Var(8), Term::Var(6), Term::Var(7), None));
	a.insert(Quad(Term::Var(9), Term::Ground(3), Term::Ground(5), None));
	a.insert(Quad(Term::Ground(4), Term::Ground(2), Term::Var(0), None));
	a.insert(Quad(
		Term::Ground(1),
		Term::Ground(5),
		Term::Ground(9),
		None,
	));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(6),
		Term::Var(1009),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Var(1008),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Ground(7),
		Term::Var(1009),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Var(1004),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Ground(2),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Var(1008),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Ground(0),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Var(1005),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(7),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Var(1001),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Ground(7),
		Term::Var(1008),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Var(1009),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Ground(1),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Var(1007),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Ground(7),
		Term::Ground(7),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Ground(7),
		Term::Var(1005),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Var(1006),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Ground(3),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Ground(2),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(5),
		Term::Ground(9),
		None,
	));
	test(a, b)
}
#[test]
fn iso_110() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Var(1), Term::Var(3), Term::Var(8), None));
	a.insert(Quad(Term::Ground(8), Term::Ground(8), Term::Var(6), None));
	a.insert(Quad(Term::Ground(4), Term::Ground(6), Term::Var(3), None));
	a.insert(Quad(Term::Var(2), Term::Var(6), Term::Ground(4), None));
	a.insert(Quad(Term::Ground(5), Term::Ground(3), Term::Var(0), None));
	a.insert(Quad(Term::Var(0), Term::Var(5), Term::Var(9), None));
	a.insert(Quad(Term::Var(7), Term::Ground(0), Term::Var(9), None));
	a.insert(Quad(Term::Ground(2), Term::Var(8), Term::Ground(6), None));
	a.insert(Quad(Term::Var(3), Term::Ground(5), Term::Var(3), None));
	a.insert(Quad(
		Term::Ground(5),
		Term::Ground(5),
		Term::Ground(4),
		None,
	));
	a.insert(Quad(Term::Ground(2), Term::Ground(3), Term::Var(3), None));
	a.insert(Quad(Term::Ground(5), Term::Var(7), Term::Var(5), None));
	a.insert(Quad(Term::Var(0), Term::Var(8), Term::Var(3), None));
	a.insert(Quad(Term::Var(4), Term::Var(4), Term::Var(2), None));
	a.insert(Quad(
		Term::Ground(3),
		Term::Ground(7),
		Term::Ground(4),
		None,
	));
	a.insert(Quad(Term::Var(0), Term::Ground(7), Term::Var(6), None));
	a.insert(Quad(Term::Var(5), Term::Ground(1), Term::Ground(6), None));
	a.insert(Quad(Term::Ground(1), Term::Var(4), Term::Var(8), None));
	a.insert(Quad(Term::Var(3), Term::Var(8), Term::Ground(9), None));
	a.insert(Quad(Term::Ground(9), Term::Ground(2), Term::Var(4), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Var(1001),
		Term::Var(1003),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Ground(8),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Ground(6),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Var(1006),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Ground(3),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Var(1005),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Ground(0),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Var(1008),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Ground(5),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Ground(5),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(3),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Var(1007),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Var(1008),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Var(1004),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(7),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(7),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Ground(1),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Var(1004),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Var(1008),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Ground(2),
		Term::Var(1004),
		None,
	));
	test(a, b)
}
#[test]
fn iso_111() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(
		Term::Ground(7),
		Term::Ground(1),
		Term::Ground(2),
		None,
	));
	a.insert(Quad(Term::Var(2), Term::Var(6), Term::Ground(1), None));
	a.insert(Quad(Term::Ground(8), Term::Ground(5), Term::Var(2), None));
	a.insert(Quad(Term::Ground(9), Term::Ground(7), Term::Var(2), None));
	a.insert(Quad(Term::Var(6), Term::Var(2), Term::Var(7), None));
	a.insert(Quad(Term::Ground(2), Term::Var(9), Term::Var(9), None));
	a.insert(Quad(Term::Ground(6), Term::Var(9), Term::Ground(2), None));
	a.insert(Quad(Term::Var(9), Term::Var(8), Term::Ground(9), None));
	a.insert(Quad(Term::Var(3), Term::Ground(4), Term::Var(1), None));
	a.insert(Quad(Term::Var(8), Term::Var(3), Term::Ground(1), None));
	a.insert(Quad(Term::Var(8), Term::Ground(9), Term::Var(9), None));
	a.insert(Quad(Term::Ground(4), Term::Var(9), Term::Ground(7), None));
	a.insert(Quad(Term::Var(9), Term::Ground(8), Term::Var(6), None));
	a.insert(Quad(Term::Var(3), Term::Var(0), Term::Ground(2), None));
	a.insert(Quad(Term::Var(2), Term::Var(5), Term::Var(1), None));
	a.insert(Quad(Term::Var(3), Term::Var(5), Term::Var(3), None));
	a.insert(Quad(Term::Ground(6), Term::Var(3), Term::Ground(1), None));
	a.insert(Quad(Term::Var(7), Term::Ground(3), Term::Var(8), None));
	a.insert(Quad(Term::Var(7), Term::Ground(4), Term::Ground(8), None));
	a.insert(Quad(Term::Ground(2), Term::Ground(2), Term::Var(8), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(7),
		Term::Ground(1),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Var(1006),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Ground(5),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Ground(7),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Var(1006),
		Term::Var(1002),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Var(1009),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Var(1009),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Var(1008),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Ground(4),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Var(1003),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Ground(9),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Var(1009),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Ground(8),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Var(1000),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Var(1005),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Var(1005),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Var(1003),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Ground(3),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Ground(4),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(2),
		Term::Var(1008),
		None,
	));
	test(a, b)
}
#[test]
fn iso_112() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Var(8), Term::Ground(1), Term::Ground(5), None));
	a.insert(Quad(Term::Ground(0), Term::Ground(0), Term::Var(1), None));
	a.insert(Quad(Term::Var(6), Term::Ground(0), Term::Ground(2), None));
	a.insert(Quad(Term::Var(7), Term::Ground(9), Term::Var(2), None));
	a.insert(Quad(Term::Var(7), Term::Ground(8), Term::Ground(3), None));
	a.insert(Quad(Term::Ground(4), Term::Ground(8), Term::Var(0), None));
	a.insert(Quad(Term::Ground(2), Term::Ground(3), Term::Var(7), None));
	a.insert(Quad(Term::Var(7), Term::Ground(6), Term::Ground(9), None));
	a.insert(Quad(Term::Ground(6), Term::Var(5), Term::Var(5), None));
	a.insert(Quad(Term::Var(1), Term::Var(8), Term::Ground(0), None));
	a.insert(Quad(Term::Var(8), Term::Var(3), Term::Ground(0), None));
	a.insert(Quad(
		Term::Ground(5),
		Term::Ground(9),
		Term::Ground(0),
		None,
	));
	a.insert(Quad(Term::Ground(3), Term::Var(0), Term::Ground(2), None));
	a.insert(Quad(Term::Var(2), Term::Ground(9), Term::Ground(9), None));
	a.insert(Quad(Term::Var(0), Term::Ground(8), Term::Ground(2), None));
	a.insert(Quad(
		Term::Ground(7),
		Term::Ground(2),
		Term::Ground(2),
		None,
	));
	a.insert(Quad(Term::Ground(9), Term::Var(8), Term::Ground(1), None));
	a.insert(Quad(Term::Ground(7), Term::Var(1), Term::Var(2), None));
	a.insert(Quad(
		Term::Ground(2),
		Term::Ground(3),
		Term::Ground(3),
		None,
	));
	a.insert(Quad(Term::Var(6), Term::Var(4), Term::Var(0), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Var(1008),
		Term::Ground(1),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(0),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Var(1006),
		Term::Ground(0),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Ground(9),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Ground(8),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Ground(8),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(3),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Ground(6),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Var(1005),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Var(1008),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Var(1003),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Ground(9),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Var(1000),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Ground(9),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(8),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(7),
		Term::Ground(2),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Var(1008),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Ground(7),
		Term::Var(1001),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(3),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Var(1006),
		Term::Var(1004),
		Term::Var(1000),
		None,
	));
	test(a, b)
}
#[test]
fn iso_113() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(
		Term::Ground(3),
		Term::Ground(0),
		Term::Ground(5),
		None,
	));
	a.insert(Quad(Term::Var(9), Term::Var(6), Term::Ground(5), None));
	a.insert(Quad(Term::Ground(8), Term::Var(2), Term::Var(0), None));
	a.insert(Quad(Term::Ground(3), Term::Var(3), Term::Var(4), None));
	a.insert(Quad(Term::Ground(3), Term::Var(9), Term::Var(9), None));
	a.insert(Quad(Term::Var(8), Term::Var(3), Term::Ground(7), None));
	a.insert(Quad(
		Term::Ground(6),
		Term::Ground(0),
		Term::Ground(7),
		None,
	));
	a.insert(Quad(Term::Ground(6), Term::Var(2), Term::Var(1), None));
	a.insert(Quad(Term::Var(7), Term::Var(7), Term::Var(3), None));
	a.insert(Quad(Term::Var(9), Term::Var(0), Term::Var(9), None));
	a.insert(Quad(Term::Ground(3), Term::Ground(4), Term::Var(1), None));
	a.insert(Quad(Term::Var(1), Term::Var(4), Term::Ground(6), None));
	a.insert(Quad(Term::Var(6), Term::Var(4), Term::Ground(9), None));
	a.insert(Quad(Term::Var(0), Term::Var(5), Term::Var(3), None));
	a.insert(Quad(Term::Ground(6), Term::Ground(9), Term::Var(8), None));
	a.insert(Quad(Term::Ground(7), Term::Var(9), Term::Var(8), None));
	a.insert(Quad(Term::Var(9), Term::Var(0), Term::Ground(0), None));
	a.insert(Quad(
		Term::Ground(1),
		Term::Ground(6),
		Term::Ground(7),
		None,
	));
	a.insert(Quad(Term::Var(8), Term::Ground(9), Term::Ground(8), None));
	a.insert(Quad(Term::Var(0), Term::Var(4), Term::Var(7), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(0),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Var(1006),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Var(1002),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Var(1003),
		Term::Var(1004),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Var(1009),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Var(1003),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Ground(0),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Var(1002),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Var(1007),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Var(1000),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(4),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Var(1004),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Var(1006),
		Term::Var(1004),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Var(1005),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Ground(9),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Ground(7),
		Term::Var(1009),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Var(1000),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(6),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Ground(9),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Var(1004),
		Term::Var(1007),
		None,
	));
	test(a, b)
}
#[test]
fn iso_114() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Var(8), Term::Var(9), Term::Var(9), None));
	a.insert(Quad(Term::Ground(2), Term::Ground(7), Term::Var(6), None));
	a.insert(Quad(Term::Var(6), Term::Var(3), Term::Var(2), None));
	a.insert(Quad(Term::Var(8), Term::Ground(2), Term::Var(0), None));
	a.insert(Quad(Term::Var(6), Term::Ground(0), Term::Var(1), None));
	a.insert(Quad(Term::Ground(6), Term::Ground(0), Term::Var(0), None));
	a.insert(Quad(Term::Ground(5), Term::Var(2), Term::Var(3), None));
	a.insert(Quad(Term::Var(2), Term::Ground(7), Term::Ground(6), None));
	a.insert(Quad(
		Term::Ground(8),
		Term::Ground(0),
		Term::Ground(6),
		None,
	));
	a.insert(Quad(Term::Var(5), Term::Ground(9), Term::Ground(0), None));
	a.insert(Quad(Term::Ground(1), Term::Var(6), Term::Ground(2), None));
	a.insert(Quad(Term::Var(2), Term::Var(5), Term::Ground(9), None));
	a.insert(Quad(Term::Ground(3), Term::Ground(4), Term::Var(6), None));
	a.insert(Quad(Term::Var(0), Term::Var(9), Term::Var(0), None));
	a.insert(Quad(Term::Ground(7), Term::Var(1), Term::Var(4), None));
	a.insert(Quad(Term::Ground(6), Term::Var(0), Term::Var(3), None));
	a.insert(Quad(Term::Var(5), Term::Var(7), Term::Var(8), None));
	a.insert(Quad(Term::Var(6), Term::Var(8), Term::Ground(4), None));
	a.insert(Quad(Term::Var(1), Term::Var(9), Term::Ground(1), None));
	a.insert(Quad(Term::Ground(0), Term::Ground(3), Term::Var(2), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Var(1008),
		Term::Var(1009),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(7),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Var(1006),
		Term::Var(1003),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Ground(2),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Var(1006),
		Term::Ground(0),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Ground(0),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Var(1002),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Ground(7),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Ground(0),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Ground(9),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Var(1006),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Var(1005),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(4),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Var(1009),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Ground(7),
		Term::Var(1001),
		Term::Var(1004),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Var(1000),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Var(1007),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Var(1006),
		Term::Var(1008),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Var(1009),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(3),
		Term::Var(1002),
		None,
	));
	test(a, b)
}
#[test]
fn iso_115() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(
		Term::Ground(0),
		Term::Ground(4),
		Term::Ground(5),
		None,
	));
	a.insert(Quad(Term::Var(4), Term::Var(7), Term::Ground(9), None));
	a.insert(Quad(
		Term::Ground(4),
		Term::Ground(2),
		Term::Ground(6),
		None,
	));
	a.insert(Quad(Term::Ground(9), Term::Var(6), Term::Var(5), None));
	a.insert(Quad(Term::Var(1), Term::Var(4), Term::Ground(0), None));
	a.insert(Quad(Term::Var(8), Term::Ground(6), Term::Var(9), None));
	a.insert(Quad(
		Term::Ground(1),
		Term::Ground(2),
		Term::Ground(4),
		None,
	));
	a.insert(Quad(
		Term::Ground(4),
		Term::Ground(3),
		Term::Ground(2),
		None,
	));
	a.insert(Quad(Term::Var(2), Term::Var(5), Term::Var(5), None));
	a.insert(Quad(Term::Ground(4), Term::Var(8), Term::Ground(7), None));
	a.insert(Quad(Term::Ground(1), Term::Ground(8), Term::Var(5), None));
	a.insert(Quad(Term::Ground(8), Term::Ground(3), Term::Var(4), None));
	a.insert(Quad(Term::Var(9), Term::Ground(5), Term::Ground(0), None));
	a.insert(Quad(Term::Var(9), Term::Var(2), Term::Ground(5), None));
	a.insert(Quad(
		Term::Ground(8),
		Term::Ground(4),
		Term::Ground(9),
		None,
	));
	a.insert(Quad(Term::Ground(5), Term::Var(0), Term::Var(5), None));
	a.insert(Quad(Term::Var(2), Term::Var(9), Term::Var(5), None));
	a.insert(Quad(Term::Var(8), Term::Ground(7), Term::Ground(4), None));
	a.insert(Quad(Term::Ground(6), Term::Ground(6), Term::Var(7), None));
	a.insert(Quad(Term::Ground(3), Term::Ground(8), Term::Var(8), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(4),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Var(1007),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Ground(2),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Var(1006),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Var(1004),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Ground(6),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(2),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Ground(3),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Var(1005),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Var(1008),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(8),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Ground(3),
		Term::Var(1004),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Ground(5),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Var(1002),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Ground(4),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Var(1000),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Var(1009),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Ground(7),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Ground(6),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(8),
		Term::Var(1008),
		None,
	));
	test(a, b)
}
#[test]
fn iso_116() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Ground(3), Term::Ground(2), Term::Var(6), None));
	a.insert(Quad(Term::Ground(8), Term::Var(8), Term::Ground(1), None));
	a.insert(Quad(Term::Ground(1), Term::Ground(3), Term::Var(0), None));
	a.insert(Quad(Term::Ground(0), Term::Ground(5), Term::Var(7), None));
	a.insert(Quad(Term::Var(7), Term::Ground(7), Term::Ground(6), None));
	a.insert(Quad(Term::Var(1), Term::Ground(8), Term::Var(7), None));
	a.insert(Quad(
		Term::Ground(4),
		Term::Ground(2),
		Term::Ground(0),
		None,
	));
	a.insert(Quad(Term::Ground(9), Term::Ground(2), Term::Var(8), None));
	a.insert(Quad(Term::Ground(7), Term::Ground(4), Term::Var(8), None));
	a.insert(Quad(Term::Var(2), Term::Ground(4), Term::Var(1), None));
	a.insert(Quad(Term::Var(7), Term::Ground(1), Term::Ground(9), None));
	a.insert(Quad(Term::Var(2), Term::Var(0), Term::Ground(5), None));
	a.insert(Quad(
		Term::Ground(4),
		Term::Ground(9),
		Term::Ground(5),
		None,
	));
	a.insert(Quad(Term::Var(4), Term::Var(8), Term::Ground(0), None));
	a.insert(Quad(Term::Var(2), Term::Ground(7), Term::Ground(8), None));
	a.insert(Quad(Term::Ground(4), Term::Var(0), Term::Var(8), None));
	a.insert(Quad(Term::Ground(8), Term::Var(6), Term::Var(9), None));
	a.insert(Quad(Term::Var(8), Term::Var(2), Term::Var(4), None));
	a.insert(Quad(Term::Var(6), Term::Var(7), Term::Var(6), None));
	a.insert(Quad(Term::Ground(2), Term::Var(3), Term::Ground(6), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(2),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Var(1008),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(3),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(5),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Ground(7),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(8),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Ground(2),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Ground(2),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Ground(7),
		Term::Ground(4),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Ground(4),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Ground(1),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Var(1000),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Ground(9),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Var(1008),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Ground(7),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Var(1000),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Var(1006),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Var(1002),
		Term::Var(1004),
		None,
	));
	b.insert(Quad(
		Term::Var(1006),
		Term::Var(1007),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Var(1003),
		Term::Ground(6),
		None,
	));
	test(a, b)
}
#[test]
fn iso_117() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Var(0), Term::Ground(8), Term::Ground(1), None));
	a.insert(Quad(Term::Ground(0), Term::Ground(9), Term::Var(2), None));
	a.insert(Quad(Term::Ground(7), Term::Var(5), Term::Ground(2), None));
	a.insert(Quad(Term::Ground(7), Term::Var(0), Term::Ground(3), None));
	a.insert(Quad(Term::Ground(7), Term::Var(2), Term::Var(5), None));
	a.insert(Quad(Term::Var(8), Term::Ground(9), Term::Var(2), None));
	a.insert(Quad(Term::Var(2), Term::Ground(9), Term::Ground(1), None));
	a.insert(Quad(Term::Var(8), Term::Var(9), Term::Ground(5), None));
	a.insert(Quad(Term::Var(0), Term::Var(6), Term::Var(8), None));
	a.insert(Quad(Term::Var(3), Term::Var(9), Term::Var(0), None));
	a.insert(Quad(Term::Var(6), Term::Var(8), Term::Var(8), None));
	a.insert(Quad(Term::Ground(3), Term::Ground(1), Term::Var(8), None));
	a.insert(Quad(Term::Ground(4), Term::Var(8), Term::Var(5), None));
	a.insert(Quad(Term::Var(6), Term::Var(1), Term::Ground(8), None));
	a.insert(Quad(Term::Var(9), Term::Var(7), Term::Var(0), None));
	a.insert(Quad(Term::Var(9), Term::Ground(9), Term::Var(6), None));
	a.insert(Quad(Term::Ground(2), Term::Var(4), Term::Var(1), None));
	a.insert(Quad(Term::Var(3), Term::Ground(5), Term::Var(3), None));
	a.insert(Quad(Term::Ground(7), Term::Var(6), Term::Var(5), None));
	a.insert(Quad(Term::Var(7), Term::Ground(4), Term::Var(4), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(8),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(9),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Ground(7),
		Term::Var(1005),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(7),
		Term::Var(1000),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Ground(7),
		Term::Var(1002),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Ground(9),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Ground(9),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Var(1009),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Var(1006),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Var(1009),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Var(1006),
		Term::Var(1008),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(1),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Var(1008),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Var(1006),
		Term::Var(1001),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Var(1007),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Ground(9),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Var(1004),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Ground(5),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Ground(7),
		Term::Var(1006),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Ground(4),
		Term::Var(1004),
		None,
	));
	test(a, b)
}
#[test]
fn iso_118() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Var(5), Term::Ground(9), Term::Var(7), None));
	a.insert(Quad(Term::Var(7), Term::Ground(6), Term::Ground(6), None));
	a.insert(Quad(Term::Var(8), Term::Var(2), Term::Var(6), None));
	a.insert(Quad(Term::Ground(7), Term::Var(7), Term::Var(9), None));
	a.insert(Quad(Term::Ground(1), Term::Var(9), Term::Var(7), None));
	a.insert(Quad(Term::Ground(5), Term::Ground(2), Term::Var(9), None));
	a.insert(Quad(Term::Ground(5), Term::Var(5), Term::Ground(9), None));
	a.insert(Quad(Term::Var(6), Term::Var(4), Term::Var(5), None));
	a.insert(Quad(Term::Ground(1), Term::Ground(1), Term::Var(9), None));
	a.insert(Quad(Term::Var(6), Term::Ground(9), Term::Var(3), None));
	a.insert(Quad(Term::Var(9), Term::Ground(5), Term::Ground(7), None));
	a.insert(Quad(Term::Ground(9), Term::Ground(1), Term::Var(8), None));
	a.insert(Quad(Term::Ground(4), Term::Ground(3), Term::Var(5), None));
	a.insert(Quad(Term::Var(8), Term::Var(2), Term::Ground(4), None));
	a.insert(Quad(Term::Var(3), Term::Ground(7), Term::Var(1), None));
	a.insert(Quad(Term::Ground(0), Term::Var(2), Term::Var(8), None));
	a.insert(Quad(Term::Var(8), Term::Var(7), Term::Ground(2), None));
	a.insert(Quad(
		Term::Ground(9),
		Term::Ground(0),
		Term::Ground(4),
		None,
	));
	a.insert(Quad(Term::Var(0), Term::Ground(2), Term::Var(2), None));
	a.insert(Quad(Term::Var(8), Term::Ground(0), Term::Ground(5), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Var(1005),
		Term::Ground(9),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Ground(6),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Var(1002),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Ground(7),
		Term::Var(1007),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Var(1009),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Ground(2),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Var(1005),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Var(1006),
		Term::Var(1004),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(1),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Var(1006),
		Term::Ground(9),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Ground(5),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Ground(1),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Ground(3),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Var(1002),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Ground(7),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Var(1002),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Var(1007),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Ground(0),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(2),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Ground(0),
		Term::Ground(5),
		None,
	));
	test(a, b)
}
#[test]
fn iso_119() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(
		Term::Ground(9),
		Term::Ground(8),
		Term::Ground(0),
		None,
	));
	a.insert(Quad(Term::Var(9), Term::Ground(2), Term::Ground(6), None));
	a.insert(Quad(Term::Var(3), Term::Ground(6), Term::Var(8), None));
	a.insert(Quad(Term::Var(8), Term::Var(5), Term::Ground(4), None));
	a.insert(Quad(
		Term::Ground(1),
		Term::Ground(7),
		Term::Ground(2),
		None,
	));
	a.insert(Quad(Term::Ground(5), Term::Var(2), Term::Var(3), None));
	a.insert(Quad(Term::Var(0), Term::Var(7), Term::Ground(8), None));
	a.insert(Quad(Term::Ground(4), Term::Var(0), Term::Var(8), None));
	a.insert(Quad(Term::Ground(4), Term::Var(4), Term::Var(4), None));
	a.insert(Quad(Term::Ground(4), Term::Var(6), Term::Var(2), None));
	a.insert(Quad(Term::Ground(5), Term::Var(5), Term::Var(3), None));
	a.insert(Quad(Term::Ground(9), Term::Ground(2), Term::Var(1), None));
	a.insert(Quad(Term::Var(2), Term::Var(4), Term::Var(9), None));
	a.insert(Quad(Term::Var(5), Term::Var(7), Term::Var(6), None));
	a.insert(Quad(Term::Ground(3), Term::Ground(6), Term::Var(3), None));
	a.insert(Quad(Term::Ground(8), Term::Ground(1), Term::Var(6), None));
	a.insert(Quad(Term::Ground(3), Term::Ground(9), Term::Var(3), None));
	a.insert(Quad(Term::Var(3), Term::Var(4), Term::Ground(6), None));
	a.insert(Quad(Term::Var(3), Term::Var(6), Term::Var(5), None));
	a.insert(Quad(Term::Var(7), Term::Var(1), Term::Ground(2), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(9),
		Term::Ground(8),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Ground(2),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Ground(6),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Var(1005),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(7),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Var(1002),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Var(1007),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Var(1000),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Var(1004),
		Term::Var(1004),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Var(1006),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Var(1005),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Ground(2),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Var(1004),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Var(1007),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(6),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Ground(1),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(9),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Var(1004),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Var(1006),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Var(1001),
		Term::Ground(2),
		None,
	));
	test(a, b)
}
#[test]
fn iso_120() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Ground(3), Term::Var(3), Term::Var(8), None));
	a.insert(Quad(Term::Var(3), Term::Ground(7), Term::Ground(0), None));
	a.insert(Quad(Term::Ground(7), Term::Ground(0), Term::Var(7), None));
	a.insert(Quad(Term::Var(7), Term::Ground(5), Term::Ground(9), None));
	a.insert(Quad(Term::Var(2), Term::Ground(1), Term::Var(8), None));
	a.insert(Quad(Term::Var(1), Term::Var(1), Term::Ground(6), None));
	a.insert(Quad(
		Term::Ground(4),
		Term::Ground(2),
		Term::Ground(3),
		None,
	));
	a.insert(Quad(Term::Var(7), Term::Var(1), Term::Var(3), None));
	a.insert(Quad(Term::Var(9), Term::Ground(4), Term::Ground(9), None));
	a.insert(Quad(Term::Ground(8), Term::Ground(4), Term::Var(5), None));
	a.insert(Quad(Term::Ground(2), Term::Var(0), Term::Var(4), None));
	a.insert(Quad(Term::Var(8), Term::Var(6), Term::Var(7), None));
	a.insert(Quad(Term::Var(7), Term::Ground(8), Term::Var(5), None));
	a.insert(Quad(Term::Ground(4), Term::Ground(2), Term::Var(5), None));
	a.insert(Quad(Term::Ground(2), Term::Var(2), Term::Ground(7), None));
	a.insert(Quad(Term::Var(1), Term::Ground(9), Term::Var(7), None));
	a.insert(Quad(Term::Ground(0), Term::Var(5), Term::Ground(0), None));
	a.insert(Quad(
		Term::Ground(5),
		Term::Ground(1),
		Term::Ground(5),
		None,
	));
	a.insert(Quad(Term::Var(5), Term::Ground(4), Term::Var(5), None));
	a.insert(Quad(Term::Ground(9), Term::Ground(1), Term::Var(3), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(3),
		Term::Var(1003),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Ground(7),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(7),
		Term::Ground(0),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Ground(5),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Ground(1),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Var(1001),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Ground(2),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Var(1001),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Ground(4),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Ground(4),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Var(1000),
		Term::Var(1004),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Var(1006),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Ground(8),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Ground(2),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Var(1002),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(9),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Var(1005),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Ground(1),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Ground(4),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Ground(1),
		Term::Var(1003),
		None,
	));
	test(a, b)
}
#[test]
fn iso_121() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Ground(1), Term::Ground(0), Term::Var(5), None));
	a.insert(Quad(Term::Var(4), Term::Ground(9), Term::Ground(5), None));
	a.insert(Quad(Term::Var(5), Term::Var(0), Term::Ground(2), None));
	a.insert(Quad(Term::Var(3), Term::Ground(0), Term::Ground(3), None));
	a.insert(Quad(Term::Var(0), Term::Var(6), Term::Var(9), None));
	a.insert(Quad(Term::Var(3), Term::Var(2), Term::Var(2), None));
	a.insert(Quad(Term::Ground(1), Term::Var(5), Term::Ground(0), None));
	a.insert(Quad(Term::Ground(9), Term::Var(2), Term::Ground(6), None));
	a.insert(Quad(
		Term::Ground(2),
		Term::Ground(2),
		Term::Ground(6),
		None,
	));
	a.insert(Quad(Term::Var(5), Term::Var(9), Term::Ground(8), None));
	a.insert(Quad(Term::Ground(5), Term::Var(0), Term::Var(7), None));
	a.insert(Quad(Term::Var(5), Term::Var(3), Term::Var(7), None));
	a.insert(Quad(Term::Var(4), Term::Ground(5), Term::Var(8), None));
	a.insert(Quad(Term::Var(1), Term::Var(7), Term::Ground(2), None));
	a.insert(Quad(Term::Var(4), Term::Ground(3), Term::Ground(2), None));
	a.insert(Quad(Term::Ground(3), Term::Ground(3), Term::Var(3), None));
	a.insert(Quad(Term::Var(3), Term::Var(0), Term::Var(8), None));
	a.insert(Quad(Term::Ground(5), Term::Ground(7), Term::Var(9), None));
	a.insert(Quad(Term::Ground(3), Term::Var(6), Term::Ground(9), None));
	a.insert(Quad(Term::Var(9), Term::Var(3), Term::Var(9), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(0),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Ground(9),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Var(1000),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Ground(0),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Var(1006),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Var(1002),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Var(1005),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Var(1002),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(2),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Var(1009),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Var(1000),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Var(1003),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Ground(5),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Var(1007),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Ground(3),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(3),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Var(1000),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Ground(7),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Var(1006),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Var(1003),
		Term::Var(1009),
		None,
	));
	test(a, b)
}
#[test]
fn iso_122() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Var(5), Term::Var(5), Term::Ground(0), None));
	a.insert(Quad(Term::Var(1), Term::Ground(5), Term::Ground(6), None));
	a.insert(Quad(Term::Ground(5), Term::Ground(5), Term::Var(8), None));
	a.insert(Quad(Term::Var(7), Term::Ground(5), Term::Var(2), None));
	a.insert(Quad(Term::Ground(9), Term::Ground(2), Term::Var(8), None));
	a.insert(Quad(
		Term::Ground(4),
		Term::Ground(2),
		Term::Ground(9),
		None,
	));
	a.insert(Quad(
		Term::Ground(1),
		Term::Ground(3),
		Term::Ground(1),
		None,
	));
	a.insert(Quad(Term::Var(0), Term::Var(9), Term::Ground(5), None));
	a.insert(Quad(Term::Var(0), Term::Ground(5), Term::Ground(4), None));
	a.insert(Quad(
		Term::Ground(9),
		Term::Ground(8),
		Term::Ground(8),
		None,
	));
	a.insert(Quad(Term::Ground(7), Term::Var(0), Term::Var(9), None));
	a.insert(Quad(Term::Ground(4), Term::Ground(6), Term::Var(7), None));
	a.insert(Quad(Term::Var(4), Term::Var(9), Term::Var(8), None));
	a.insert(Quad(Term::Var(1), Term::Var(6), Term::Var(1), None));
	a.insert(Quad(
		Term::Ground(6),
		Term::Ground(3),
		Term::Ground(8),
		None,
	));
	a.insert(Quad(Term::Var(5), Term::Ground(2), Term::Ground(3), None));
	a.insert(Quad(
		Term::Ground(8),
		Term::Ground(1),
		Term::Ground(7),
		None,
	));
	a.insert(Quad(Term::Ground(5), Term::Var(3), Term::Ground(4), None));
	a.insert(Quad(Term::Var(4), Term::Var(5), Term::Var(5), None));
	a.insert(Quad(Term::Var(9), Term::Var(4), Term::Ground(9), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Var(1005),
		Term::Var(1005),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(5),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Ground(5),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Ground(5),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Ground(2),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Ground(2),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(3),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Var(1009),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(5),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Ground(8),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Ground(7),
		Term::Var(1000),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Ground(6),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Var(1009),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Var(1006),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Ground(3),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Ground(2),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Ground(1),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Var(1003),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Var(1005),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Var(1004),
		Term::Ground(9),
		None,
	));
	test(a, b)
}
#[test]
fn iso_123() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Ground(2), Term::Var(9), Term::Ground(8), None));
	a.insert(Quad(Term::Var(9), Term::Ground(4), Term::Var(9), None));
	a.insert(Quad(Term::Ground(7), Term::Var(1), Term::Var(6), None));
	a.insert(Quad(Term::Ground(7), Term::Var(3), Term::Ground(9), None));
	a.insert(Quad(Term::Ground(2), Term::Ground(2), Term::Var(0), None));
	a.insert(Quad(Term::Ground(0), Term::Var(7), Term::Var(6), None));
	a.insert(Quad(
		Term::Ground(9),
		Term::Ground(3),
		Term::Ground(3),
		None,
	));
	a.insert(Quad(Term::Ground(3), Term::Var(9), Term::Ground(9), None));
	a.insert(Quad(Term::Var(9), Term::Ground(3), Term::Var(7), None));
	a.insert(Quad(Term::Ground(1), Term::Var(4), Term::Var(2), None));
	a.insert(Quad(Term::Ground(9), Term::Var(6), Term::Ground(3), None));
	a.insert(Quad(Term::Var(8), Term::Ground(4), Term::Ground(1), None));
	a.insert(Quad(Term::Ground(4), Term::Var(8), Term::Ground(5), None));
	a.insert(Quad(
		Term::Ground(8),
		Term::Ground(5),
		Term::Ground(5),
		None,
	));
	a.insert(Quad(
		Term::Ground(3),
		Term::Ground(6),
		Term::Ground(3),
		None,
	));
	a.insert(Quad(Term::Ground(8), Term::Var(9), Term::Ground(8), None));
	a.insert(Quad(
		Term::Ground(5),
		Term::Ground(0),
		Term::Ground(5),
		None,
	));
	a.insert(Quad(Term::Var(4), Term::Var(0), Term::Var(6), None));
	a.insert(Quad(Term::Ground(2), Term::Var(7), Term::Var(5), None));
	a.insert(Quad(Term::Ground(0), Term::Ground(3), Term::Var(1), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(2),
		Term::Var(1009),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Ground(4),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Ground(7),
		Term::Var(1001),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Ground(7),
		Term::Var(1003),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(2),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Var(1007),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Ground(3),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Var(1009),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Ground(3),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Var(1004),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Var(1006),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Ground(4),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Var(1008),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Ground(5),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(6),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Var(1009),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Ground(0),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Var(1000),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Var(1007),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(3),
		Term::Var(1001),
		None,
	));
	test(a, b)
}
#[test]
fn iso_124() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Var(0), Term::Ground(7), Term::Var(7), None));
	a.insert(Quad(Term::Var(3), Term::Var(9), Term::Ground(0), None));
	a.insert(Quad(Term::Var(9), Term::Var(3), Term::Ground(7), None));
	a.insert(Quad(
		Term::Ground(8),
		Term::Ground(6),
		Term::Ground(9),
		None,
	));
	a.insert(Quad(Term::Var(1), Term::Ground(6), Term::Var(2), None));
	a.insert(Quad(Term::Var(5), Term::Var(8), Term::Ground(4), None));
	a.insert(Quad(Term::Var(1), Term::Ground(1), Term::Ground(4), None));
	a.insert(Quad(
		Term::Ground(2),
		Term::Ground(3),
		Term::Ground(3),
		None,
	));
	a.insert(Quad(Term::Ground(1), Term::Var(0), Term::Ground(4), None));
	a.insert(Quad(Term::Var(8), Term::Var(4), Term::Var(4), None));
	a.insert(Quad(Term::Ground(8), Term::Ground(9), Term::Var(2), None));
	a.insert(Quad(Term::Ground(7), Term::Var(8), Term::Var(2), None));
	a.insert(Quad(Term::Ground(5), Term::Var(5), Term::Ground(6), None));
	a.insert(Quad(Term::Ground(6), Term::Var(0), Term::Ground(0), None));
	a.insert(Quad(Term::Var(3), Term::Ground(4), Term::Var(5), None));
	a.insert(Quad(Term::Var(8), Term::Ground(0), Term::Ground(0), None));
	a.insert(Quad(Term::Var(0), Term::Var(4), Term::Ground(4), None));
	a.insert(Quad(Term::Ground(1), Term::Var(7), Term::Ground(7), None));
	a.insert(Quad(Term::Var(3), Term::Ground(0), Term::Var(9), None));
	a.insert(Quad(Term::Ground(6), Term::Var(7), Term::Ground(8), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(7),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Var(1009),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Var(1003),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Ground(6),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(6),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Var(1008),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(1),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(3),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Var(1000),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Var(1004),
		Term::Var(1004),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Ground(9),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Ground(7),
		Term::Var(1008),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Var(1005),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Var(1000),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Ground(4),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Ground(0),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Var(1004),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Var(1007),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Ground(0),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Var(1007),
		Term::Ground(8),
		None,
	));
	test(a, b)
}
#[test]
fn iso_125() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Var(1), Term::Var(2), Term::Ground(8), None));
	a.insert(Quad(Term::Ground(9), Term::Var(8), Term::Var(0), None));
	a.insert(Quad(Term::Ground(4), Term::Var(9), Term::Var(8), None));
	a.insert(Quad(Term::Ground(7), Term::Var(1), Term::Var(7), None));
	a.insert(Quad(Term::Ground(7), Term::Ground(2), Term::Var(4), None));
	a.insert(Quad(Term::Var(9), Term::Ground(5), Term::Var(7), None));
	a.insert(Quad(Term::Ground(3), Term::Var(8), Term::Ground(1), None));
	a.insert(Quad(
		Term::Ground(4),
		Term::Ground(5),
		Term::Ground(3),
		None,
	));
	a.insert(Quad(Term::Ground(4), Term::Ground(8), Term::Var(4), None));
	a.insert(Quad(Term::Ground(4), Term::Ground(2), Term::Var(3), None));
	a.insert(Quad(Term::Ground(3), Term::Ground(5), Term::Var(5), None));
	a.insert(Quad(Term::Var(2), Term::Ground(6), Term::Ground(3), None));
	a.insert(Quad(Term::Var(9), Term::Var(8), Term::Ground(2), None));
	a.insert(Quad(Term::Ground(9), Term::Var(6), Term::Var(5), None));
	a.insert(Quad(Term::Ground(3), Term::Var(1), Term::Ground(9), None));
	a.insert(Quad(
		Term::Ground(3),
		Term::Ground(7),
		Term::Ground(0),
		None,
	));
	a.insert(Quad(Term::Var(7), Term::Ground(4), Term::Var(1), None));
	a.insert(Quad(Term::Var(2), Term::Ground(0), Term::Ground(9), None));
	a.insert(Quad(Term::Ground(8), Term::Var(9), Term::Var(3), None));
	a.insert(Quad(Term::Ground(7), Term::Var(2), Term::Var(9), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Var(1001),
		Term::Var(1002),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Var(1008),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Var(1009),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Ground(7),
		Term::Var(1001),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Ground(7),
		Term::Ground(2),
		Term::Var(1004),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Ground(5),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Var(1008),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Ground(5),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Ground(8),
		Term::Var(1004),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Ground(2),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(5),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Ground(6),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Var(1008),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Var(1006),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Var(1001),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(7),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Ground(4),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Ground(0),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Var(1009),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Ground(7),
		Term::Var(1002),
		Term::Var(1009),
		None,
	));
	test(a, b)
}
#[test]
fn iso_126() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Ground(3), Term::Var(3), Term::Ground(7), None));
	a.insert(Quad(Term::Ground(6), Term::Var(7), Term::Var(4), None));
	a.insert(Quad(
		Term::Ground(8),
		Term::Ground(3),
		Term::Ground(2),
		None,
	));
	a.insert(Quad(Term::Var(7), Term::Var(2), Term::Var(5), None));
	a.insert(Quad(
		Term::Ground(9),
		Term::Ground(5),
		Term::Ground(5),
		None,
	));
	a.insert(Quad(Term::Ground(3), Term::Var(3), Term::Var(4), None));
	a.insert(Quad(Term::Ground(6), Term::Var(7), Term::Var(3), None));
	a.insert(Quad(Term::Var(7), Term::Var(1), Term::Ground(5), None));
	a.insert(Quad(Term::Ground(7), Term::Var(2), Term::Var(7), None));
	a.insert(Quad(Term::Ground(1), Term::Var(6), Term::Var(2), None));
	a.insert(Quad(Term::Ground(5), Term::Var(6), Term::Ground(2), None));
	a.insert(Quad(Term::Var(5), Term::Ground(1), Term::Var(4), None));
	a.insert(Quad(
		Term::Ground(0),
		Term::Ground(7),
		Term::Ground(0),
		None,
	));
	a.insert(Quad(Term::Var(0), Term::Ground(7), Term::Var(2), None));
	a.insert(Quad(Term::Var(7), Term::Var(3), Term::Var(3), None));
	a.insert(Quad(Term::Var(7), Term::Var(3), Term::Var(2), None));
	a.insert(Quad(Term::Var(2), Term::Ground(2), Term::Ground(2), None));
	a.insert(Quad(Term::Var(4), Term::Ground(4), Term::Var(7), None));
	a.insert(Quad(Term::Var(8), Term::Ground(6), Term::Var(8), None));
	a.insert(Quad(Term::Var(9), Term::Var(2), Term::Var(0), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(3),
		Term::Var(1003),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Var(1007),
		Term::Var(1004),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Ground(3),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Var(1002),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Ground(5),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Var(1003),
		Term::Var(1004),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Var(1007),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Var(1001),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Ground(7),
		Term::Var(1002),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Var(1006),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Var(1006),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Ground(1),
		Term::Var(1004),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(7),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(7),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Var(1003),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Var(1003),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Ground(2),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Ground(4),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Ground(6),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Var(1002),
		Term::Var(1000),
		None,
	));
	test(a, b)
}
#[test]
fn iso_127() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(
		Term::Ground(4),
		Term::Ground(5),
		Term::Ground(6),
		None,
	));
	a.insert(Quad(Term::Ground(0), Term::Var(1), Term::Ground(6), None));
	a.insert(Quad(Term::Ground(9), Term::Ground(9), Term::Var(6), None));
	a.insert(Quad(Term::Ground(8), Term::Var(8), Term::Var(7), None));
	a.insert(Quad(Term::Var(4), Term::Ground(6), Term::Ground(5), None));
	a.insert(Quad(Term::Ground(8), Term::Ground(4), Term::Var(6), None));
	a.insert(Quad(Term::Ground(8), Term::Ground(9), Term::Var(4), None));
	a.insert(Quad(Term::Ground(3), Term::Var(2), Term::Var(2), None));
	a.insert(Quad(Term::Ground(2), Term::Var(6), Term::Var(9), None));
	a.insert(Quad(Term::Ground(3), Term::Var(0), Term::Ground(3), None));
	a.insert(Quad(Term::Ground(5), Term::Var(6), Term::Ground(7), None));
	a.insert(Quad(Term::Ground(7), Term::Ground(7), Term::Var(4), None));
	a.insert(Quad(Term::Ground(1), Term::Var(0), Term::Var(1), None));
	a.insert(Quad(Term::Ground(3), Term::Var(2), Term::Var(6), None));
	a.insert(Quad(
		Term::Ground(3),
		Term::Ground(4),
		Term::Ground(0),
		None,
	));
	a.insert(Quad(Term::Var(5), Term::Var(7), Term::Ground(7), None));
	a.insert(Quad(Term::Var(6), Term::Var(4), Term::Var(3), None));
	a.insert(Quad(Term::Ground(9), Term::Var(4), Term::Var(7), None));
	a.insert(Quad(Term::Var(1), Term::Ground(3), Term::Ground(8), None));
	a.insert(Quad(Term::Var(2), Term::Var(6), Term::Var(0), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(4),
		Term::Ground(5),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Var(1001),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Ground(9),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Var(1008),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Ground(6),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Ground(4),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Ground(9),
		Term::Var(1004),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Var(1002),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Var(1006),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Var(1000),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Var(1006),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Ground(7),
		Term::Ground(7),
		Term::Var(1004),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Var(1000),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Var(1002),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(4),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Var(1007),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Var(1006),
		Term::Var(1004),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Var(1004),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(3),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Var(1006),
		Term::Var(1000),
		None,
	));
	test(a, b)
}
#[test]
fn iso_128() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Var(1), Term::Ground(7), Term::Ground(5), None));
	a.insert(Quad(
		Term::Ground(2),
		Term::Ground(7),
		Term::Ground(0),
		None,
	));
	a.insert(Quad(Term::Var(2), Term::Var(3), Term::Var(1), None));
	a.insert(Quad(Term::Var(8), Term::Var(2), Term::Ground(6), None));
	a.insert(Quad(Term::Var(1), Term::Ground(7), Term::Ground(9), None));
	a.insert(Quad(Term::Var(4), Term::Ground(1), Term::Ground(0), None));
	a.insert(Quad(Term::Var(7), Term::Ground(7), Term::Var(9), None));
	a.insert(Quad(Term::Ground(6), Term::Var(0), Term::Var(5), None));
	a.insert(Quad(
		Term::Ground(4),
		Term::Ground(8),
		Term::Ground(8),
		None,
	));
	a.insert(Quad(Term::Ground(5), Term::Ground(3), Term::Var(5), None));
	a.insert(Quad(Term::Ground(3), Term::Ground(4), Term::Var(1), None));
	a.insert(Quad(Term::Var(5), Term::Ground(7), Term::Ground(0), None));
	a.insert(Quad(
		Term::Ground(0),
		Term::Ground(8),
		Term::Ground(7),
		None,
	));
	a.insert(Quad(Term::Var(8), Term::Var(1), Term::Var(4), None));
	a.insert(Quad(Term::Var(0), Term::Ground(8), Term::Ground(2), None));
	a.insert(Quad(
		Term::Ground(0),
		Term::Ground(1),
		Term::Ground(6),
		None,
	));
	a.insert(Quad(
		Term::Ground(6),
		Term::Ground(3),
		Term::Ground(9),
		None,
	));
	a.insert(Quad(Term::Ground(9), Term::Ground(4), Term::Var(9), None));
	a.insert(Quad(Term::Var(8), Term::Var(6), Term::Ground(0), None));
	a.insert(Quad(Term::Var(5), Term::Ground(2), Term::Var(3), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(7),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(7),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Var(1003),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Var(1002),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(7),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Ground(1),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Ground(7),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Var(1000),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Ground(8),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Ground(3),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(4),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Ground(7),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(8),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Var(1001),
		Term::Var(1004),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(8),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(1),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Ground(3),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Ground(4),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Var(1006),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Ground(2),
		Term::Var(1003),
		None,
	));
	test(a, b)
}
#[test]
fn iso_129() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Var(0), Term::Ground(9), Term::Var(2), None));
	a.insert(Quad(Term::Var(4), Term::Ground(5), Term::Var(4), None));
	a.insert(Quad(Term::Var(4), Term::Var(5), Term::Ground(7), None));
	a.insert(Quad(Term::Var(1), Term::Ground(1), Term::Ground(9), None));
	a.insert(Quad(Term::Var(3), Term::Ground(0), Term::Ground(0), None));
	a.insert(Quad(Term::Var(6), Term::Ground(3), Term::Ground(6), None));
	a.insert(Quad(
		Term::Ground(5),
		Term::Ground(9),
		Term::Ground(1),
		None,
	));
	a.insert(Quad(Term::Ground(8), Term::Var(3), Term::Var(9), None));
	a.insert(Quad(Term::Var(4), Term::Ground(5), Term::Ground(9), None));
	a.insert(Quad(Term::Ground(0), Term::Var(2), Term::Var(8), None));
	a.insert(Quad(Term::Ground(7), Term::Var(9), Term::Ground(7), None));
	a.insert(Quad(Term::Var(2), Term::Var(7), Term::Ground(7), None));
	a.insert(Quad(Term::Var(3), Term::Ground(4), Term::Var(6), None));
	a.insert(Quad(Term::Ground(4), Term::Ground(2), Term::Var(4), None));
	a.insert(Quad(Term::Ground(4), Term::Ground(7), Term::Var(1), None));
	a.insert(Quad(Term::Ground(9), Term::Ground(2), Term::Var(1), None));
	a.insert(Quad(Term::Var(8), Term::Var(7), Term::Ground(8), None));
	a.insert(Quad(Term::Var(6), Term::Var(8), Term::Ground(8), None));
	a.insert(Quad(Term::Var(4), Term::Var(9), Term::Ground(2), None));
	a.insert(Quad(
		Term::Ground(1),
		Term::Ground(1),
		Term::Ground(1),
		None,
	));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(9),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Ground(5),
		Term::Var(1004),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Var(1005),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(1),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Ground(0),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Var(1006),
		Term::Ground(3),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Ground(9),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Var(1003),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Ground(5),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Var(1002),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Ground(7),
		Term::Var(1009),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Var(1007),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Ground(4),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Ground(2),
		Term::Var(1004),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Ground(7),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Ground(2),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Var(1007),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Var(1006),
		Term::Var(1008),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Var(1009),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(1),
		Term::Ground(1),
		None,
	));
	test(a, b)
}
#[test]
fn iso_130() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Var(1), Term::Ground(9), Term::Var(6), None));
	a.insert(Quad(Term::Var(3), Term::Ground(9), Term::Ground(4), None));
	a.insert(Quad(Term::Ground(6), Term::Ground(9), Term::Var(6), None));
	a.insert(Quad(Term::Var(8), Term::Ground(8), Term::Var(5), None));
	a.insert(Quad(Term::Ground(8), Term::Var(1), Term::Var(1), None));
	a.insert(Quad(Term::Ground(9), Term::Ground(2), Term::Var(0), None));
	a.insert(Quad(Term::Ground(3), Term::Var(2), Term::Ground(9), None));
	a.insert(Quad(Term::Var(0), Term::Ground(4), Term::Var(7), None));
	a.insert(Quad(Term::Var(9), Term::Var(4), Term::Ground(8), None));
	a.insert(Quad(Term::Var(5), Term::Ground(6), Term::Ground(4), None));
	a.insert(Quad(Term::Var(6), Term::Var(8), Term::Var(5), None));
	a.insert(Quad(
		Term::Ground(9),
		Term::Ground(1),
		Term::Ground(8),
		None,
	));
	a.insert(Quad(
		Term::Ground(7),
		Term::Ground(2),
		Term::Ground(8),
		None,
	));
	a.insert(Quad(Term::Var(0), Term::Var(3), Term::Ground(1), None));
	a.insert(Quad(Term::Ground(2), Term::Var(0), Term::Ground(0), None));
	a.insert(Quad(Term::Ground(8), Term::Var(4), Term::Ground(9), None));
	a.insert(Quad(Term::Var(3), Term::Ground(6), Term::Ground(9), None));
	a.insert(Quad(Term::Var(2), Term::Var(7), Term::Var(1), None));
	a.insert(Quad(
		Term::Ground(9),
		Term::Ground(9),
		Term::Ground(6),
		None,
	));
	a.insert(Quad(
		Term::Ground(6),
		Term::Ground(2),
		Term::Ground(2),
		None,
	));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(9),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Ground(9),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Ground(9),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Ground(8),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Var(1001),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Ground(2),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Var(1002),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(4),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Var(1004),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Ground(6),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Var(1006),
		Term::Var(1008),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Ground(1),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Ground(7),
		Term::Ground(2),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Var(1003),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Var(1000),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Var(1004),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Ground(6),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Var(1007),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Ground(9),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Ground(2),
		Term::Ground(2),
		None,
	));
	test(a, b)
}
#[test]
fn iso_131() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Ground(5), Term::Var(2), Term::Var(1), None));
	a.insert(Quad(Term::Var(8), Term::Ground(4), Term::Ground(8), None));
	a.insert(Quad(Term::Var(8), Term::Var(5), Term::Var(0), None));
	a.insert(Quad(Term::Var(1), Term::Var(4), Term::Var(3), None));
	a.insert(Quad(Term::Var(9), Term::Var(2), Term::Ground(2), None));
	a.insert(Quad(Term::Ground(4), Term::Var(8), Term::Ground(2), None));
	a.insert(Quad(Term::Ground(8), Term::Var(5), Term::Ground(5), None));
	a.insert(Quad(Term::Ground(5), Term::Var(4), Term::Var(0), None));
	a.insert(Quad(Term::Var(9), Term::Ground(1), Term::Var(8), None));
	a.insert(Quad(Term::Ground(9), Term::Var(8), Term::Ground(1), None));
	a.insert(Quad(
		Term::Ground(9),
		Term::Ground(4),
		Term::Ground(7),
		None,
	));
	a.insert(Quad(Term::Ground(7), Term::Ground(9), Term::Var(3), None));
	a.insert(Quad(Term::Var(0), Term::Ground(1), Term::Var(8), None));
	a.insert(Quad(Term::Ground(2), Term::Var(6), Term::Ground(5), None));
	a.insert(Quad(Term::Var(1), Term::Var(9), Term::Var(6), None));
	a.insert(Quad(Term::Var(6), Term::Ground(7), Term::Var(8), None));
	a.insert(Quad(
		Term::Ground(6),
		Term::Ground(1),
		Term::Ground(2),
		None,
	));
	a.insert(Quad(Term::Ground(4), Term::Ground(1), Term::Var(8), None));
	a.insert(Quad(Term::Var(3), Term::Ground(5), Term::Ground(5), None));
	a.insert(Quad(Term::Ground(6), Term::Var(2), Term::Var(8), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(5),
		Term::Var(1002),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Ground(4),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Var(1005),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Var(1004),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Var(1002),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Var(1008),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Var(1005),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Var(1004),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Ground(1),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Var(1008),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Ground(4),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Ground(7),
		Term::Ground(9),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(1),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Var(1006),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Var(1009),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Var(1006),
		Term::Ground(7),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Ground(1),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Ground(1),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Ground(5),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Var(1002),
		Term::Var(1008),
		None,
	));
	test(a, b)
}
#[test]
fn iso_132() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(
		Term::Ground(9),
		Term::Ground(2),
		Term::Ground(7),
		None,
	));
	a.insert(Quad(Term::Var(1), Term::Ground(3), Term::Var(5), None));
	a.insert(Quad(Term::Var(7), Term::Ground(4), Term::Var(1), None));
	a.insert(Quad(Term::Var(7), Term::Var(1), Term::Ground(6), None));
	a.insert(Quad(Term::Var(1), Term::Var(1), Term::Var(5), None));
	a.insert(Quad(Term::Var(1), Term::Var(1), Term::Ground(0), None));
	a.insert(Quad(Term::Ground(3), Term::Ground(7), Term::Var(5), None));
	a.insert(Quad(Term::Ground(4), Term::Var(6), Term::Ground(3), None));
	a.insert(Quad(Term::Ground(7), Term::Ground(0), Term::Var(7), None));
	a.insert(Quad(Term::Var(6), Term::Ground(4), Term::Var(7), None));
	a.insert(Quad(
		Term::Ground(2),
		Term::Ground(9),
		Term::Ground(7),
		None,
	));
	a.insert(Quad(Term::Ground(2), Term::Var(5), Term::Ground(3), None));
	a.insert(Quad(
		Term::Ground(3),
		Term::Ground(9),
		Term::Ground(5),
		None,
	));
	a.insert(Quad(
		Term::Ground(4),
		Term::Ground(7),
		Term::Ground(0),
		None,
	));
	a.insert(Quad(Term::Ground(2), Term::Var(5), Term::Ground(0), None));
	a.insert(Quad(Term::Ground(7), Term::Var(6), Term::Var(1), None));
	a.insert(Quad(
		Term::Ground(3),
		Term::Ground(0),
		Term::Ground(2),
		None,
	));
	a.insert(Quad(Term::Var(3), Term::Ground(2), Term::Var(6), None));
	a.insert(Quad(Term::Var(5), Term::Ground(1), Term::Var(0), None));
	a.insert(Quad(Term::Var(9), Term::Var(4), Term::Ground(8), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(9),
		Term::Ground(2),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(3),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Ground(4),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Var(1001),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Var(1001),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Var(1001),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(7),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Var(1006),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Ground(7),
		Term::Ground(0),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Var(1006),
		Term::Ground(4),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(9),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Var(1005),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(9),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Ground(7),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Var(1005),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(7),
		Term::Var(1006),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(0),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Ground(2),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Ground(1),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Var(1004),
		Term::Ground(8),
		None,
	));
	test(a, b)
}
#[test]
fn iso_133() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Var(1), Term::Var(3), Term::Ground(7), None));
	a.insert(Quad(Term::Var(3), Term::Var(2), Term::Var(3), None));
	a.insert(Quad(Term::Var(6), Term::Var(2), Term::Var(1), None));
	a.insert(Quad(Term::Var(5), Term::Ground(4), Term::Var(2), None));
	a.insert(Quad(Term::Var(5), Term::Ground(8), Term::Var(6), None));
	a.insert(Quad(Term::Ground(3), Term::Var(0), Term::Var(7), None));
	a.insert(Quad(Term::Var(6), Term::Ground(5), Term::Ground(5), None));
	a.insert(Quad(Term::Var(4), Term::Ground(7), Term::Ground(9), None));
	a.insert(Quad(Term::Var(2), Term::Var(4), Term::Var(7), None));
	a.insert(Quad(Term::Var(2), Term::Var(6), Term::Var(0), None));
	a.insert(Quad(Term::Var(3), Term::Ground(4), Term::Var(2), None));
	a.insert(Quad(Term::Var(1), Term::Ground(6), Term::Var(8), None));
	a.insert(Quad(Term::Ground(4), Term::Ground(3), Term::Var(7), None));
	a.insert(Quad(Term::Var(7), Term::Ground(8), Term::Var(8), None));
	a.insert(Quad(
		Term::Ground(3),
		Term::Ground(1),
		Term::Ground(8),
		None,
	));
	a.insert(Quad(Term::Ground(5), Term::Ground(8), Term::Var(3), None));
	a.insert(Quad(Term::Ground(1), Term::Var(4), Term::Ground(2), None));
	a.insert(Quad(Term::Var(4), Term::Ground(3), Term::Var(0), None));
	a.insert(Quad(Term::Var(2), Term::Var(6), Term::Ground(7), None));
	a.insert(Quad(Term::Var(9), Term::Var(5), Term::Ground(7), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Var(1001),
		Term::Var(1003),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Var(1002),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Var(1006),
		Term::Var(1002),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Ground(4),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Ground(8),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Var(1000),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Var(1006),
		Term::Ground(5),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Ground(7),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Var(1004),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Var(1006),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Ground(4),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(6),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Ground(3),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Ground(8),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(1),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Ground(8),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Var(1004),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Ground(3),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Var(1006),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Var(1005),
		Term::Ground(7),
		None,
	));
	test(a, b)
}
#[test]
fn iso_134() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Var(1), Term::Ground(9), Term::Var(7), None));
	a.insert(Quad(Term::Ground(8), Term::Ground(0), Term::Var(9), None));
	a.insert(Quad(Term::Ground(9), Term::Var(2), Term::Var(2), None));
	a.insert(Quad(Term::Var(0), Term::Var(2), Term::Var(4), None));
	a.insert(Quad(
		Term::Ground(0),
		Term::Ground(7),
		Term::Ground(5),
		None,
	));
	a.insert(Quad(
		Term::Ground(5),
		Term::Ground(1),
		Term::Ground(2),
		None,
	));
	a.insert(Quad(Term::Ground(0), Term::Var(4), Term::Var(3), None));
	a.insert(Quad(Term::Ground(3), Term::Ground(3), Term::Var(1), None));
	a.insert(Quad(Term::Ground(0), Term::Var(0), Term::Var(2), None));
	a.insert(Quad(Term::Ground(8), Term::Ground(8), Term::Var(4), None));
	a.insert(Quad(Term::Var(2), Term::Ground(9), Term::Var(4), None));
	a.insert(Quad(Term::Ground(9), Term::Var(0), Term::Var(0), None));
	a.insert(Quad(Term::Var(1), Term::Ground(9), Term::Ground(3), None));
	a.insert(Quad(Term::Var(7), Term::Var(1), Term::Var(0), None));
	a.insert(Quad(Term::Ground(7), Term::Var(5), Term::Ground(2), None));
	a.insert(Quad(Term::Ground(9), Term::Ground(5), Term::Var(6), None));
	a.insert(Quad(Term::Ground(1), Term::Ground(7), Term::Var(3), None));
	a.insert(Quad(Term::Var(5), Term::Ground(4), Term::Var(2), None));
	a.insert(Quad(Term::Ground(1), Term::Var(9), Term::Ground(4), None));
	a.insert(Quad(
		Term::Ground(9),
		Term::Ground(0),
		Term::Ground(6),
		None,
	));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(9),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Ground(0),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Var(1002),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Var(1002),
		Term::Var(1004),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(7),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Ground(1),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Var(1004),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(3),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Var(1000),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Ground(8),
		Term::Var(1004),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Ground(9),
		Term::Var(1004),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Var(1000),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(9),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Var(1001),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Ground(7),
		Term::Var(1005),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Ground(5),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(7),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Ground(4),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Var(1009),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Ground(0),
		Term::Ground(6),
		None,
	));
	test(a, b)
}
#[test]
fn iso_135() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(
		Term::Ground(2),
		Term::Ground(1),
		Term::Ground(2),
		None,
	));
	a.insert(Quad(Term::Ground(9), Term::Var(3), Term::Ground(8), None));
	a.insert(Quad(Term::Ground(9), Term::Var(2), Term::Var(3), None));
	a.insert(Quad(
		Term::Ground(4),
		Term::Ground(6),
		Term::Ground(1),
		None,
	));
	a.insert(Quad(Term::Ground(2), Term::Ground(5), Term::Var(5), None));
	a.insert(Quad(Term::Ground(5), Term::Var(0), Term::Ground(5), None));
	a.insert(Quad(
		Term::Ground(4),
		Term::Ground(9),
		Term::Ground(0),
		None,
	));
	a.insert(Quad(Term::Var(1), Term::Var(0), Term::Var(4), None));
	a.insert(Quad(Term::Var(9), Term::Var(7), Term::Ground(1), None));
	a.insert(Quad(Term::Var(1), Term::Ground(6), Term::Var(5), None));
	a.insert(Quad(Term::Var(2), Term::Var(3), Term::Var(8), None));
	a.insert(Quad(Term::Ground(8), Term::Ground(8), Term::Var(0), None));
	a.insert(Quad(Term::Ground(4), Term::Ground(5), Term::Var(3), None));
	a.insert(Quad(
		Term::Ground(5),
		Term::Ground(0),
		Term::Ground(8),
		None,
	));
	a.insert(Quad(Term::Var(1), Term::Var(5), Term::Var(3), None));
	a.insert(Quad(
		Term::Ground(4),
		Term::Ground(0),
		Term::Ground(0),
		None,
	));
	a.insert(Quad(Term::Ground(6), Term::Ground(2), Term::Var(4), None));
	a.insert(Quad(Term::Ground(0), Term::Var(8), Term::Ground(7), None));
	a.insert(Quad(Term::Ground(9), Term::Ground(7), Term::Var(3), None));
	a.insert(Quad(Term::Ground(1), Term::Ground(7), Term::Var(1), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(1),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Var(1003),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Var(1002),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Ground(6),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(5),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Var(1000),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Ground(9),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Var(1000),
		Term::Var(1004),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Var(1007),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(6),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Var(1003),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Ground(8),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Ground(5),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Ground(0),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Var(1005),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Ground(0),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Ground(2),
		Term::Var(1004),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Var(1008),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Ground(7),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(7),
		Term::Var(1001),
		None,
	));
	test(a, b)
}
#[test]
fn iso_136() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Ground(7), Term::Var(2), Term::Ground(0), None));
	a.insert(Quad(
		Term::Ground(6),
		Term::Ground(4),
		Term::Ground(0),
		None,
	));
	a.insert(Quad(Term::Ground(8), Term::Var(9), Term::Ground(1), None));
	a.insert(Quad(Term::Ground(3), Term::Var(7), Term::Var(3), None));
	a.insert(Quad(Term::Ground(8), Term::Var(3), Term::Ground(1), None));
	a.insert(Quad(
		Term::Ground(5),
		Term::Ground(3),
		Term::Ground(0),
		None,
	));
	a.insert(Quad(Term::Var(1), Term::Var(3), Term::Var(9), None));
	a.insert(Quad(Term::Ground(4), Term::Ground(2), Term::Var(0), None));
	a.insert(Quad(Term::Var(3), Term::Var(1), Term::Ground(2), None));
	a.insert(Quad(Term::Ground(1), Term::Var(8), Term::Ground(6), None));
	a.insert(Quad(Term::Var(8), Term::Var(0), Term::Ground(9), None));
	a.insert(Quad(Term::Var(6), Term::Var(0), Term::Var(3), None));
	a.insert(Quad(Term::Ground(2), Term::Var(8), Term::Var(8), None));
	a.insert(Quad(Term::Ground(7), Term::Var(7), Term::Var(3), None));
	a.insert(Quad(Term::Ground(6), Term::Var(0), Term::Ground(5), None));
	a.insert(Quad(Term::Var(8), Term::Ground(6), Term::Var(6), None));
	a.insert(Quad(Term::Var(0), Term::Ground(8), Term::Ground(9), None));
	a.insert(Quad(Term::Ground(8), Term::Ground(8), Term::Var(5), None));
	a.insert(Quad(Term::Var(5), Term::Var(1), Term::Ground(9), None));
	a.insert(Quad(Term::Var(8), Term::Ground(8), Term::Ground(2), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(7),
		Term::Var(1002),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Ground(4),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Var(1009),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Var(1007),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Var(1003),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Ground(3),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Var(1003),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Ground(2),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Var(1001),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Var(1008),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Var(1000),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Var(1006),
		Term::Var(1000),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Var(1008),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Ground(7),
		Term::Var(1007),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Var(1000),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Ground(6),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(8),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Ground(8),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Var(1001),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Ground(8),
		Term::Ground(2),
		None,
	));
	test(a, b)
}
#[test]
fn iso_137() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Var(6), Term::Var(5), Term::Var(3), None));
	a.insert(Quad(
		Term::Ground(5),
		Term::Ground(0),
		Term::Ground(7),
		None,
	));
	a.insert(Quad(Term::Var(2), Term::Var(1), Term::Ground(4), None));
	a.insert(Quad(Term::Var(1), Term::Var(3), Term::Var(3), None));
	a.insert(Quad(Term::Var(6), Term::Ground(3), Term::Var(1), None));
	a.insert(Quad(Term::Ground(7), Term::Var(0), Term::Ground(5), None));
	a.insert(Quad(Term::Var(0), Term::Ground(3), Term::Ground(5), None));
	a.insert(Quad(Term::Var(8), Term::Ground(8), Term::Ground(2), None));
	a.insert(Quad(Term::Var(8), Term::Var(6), Term::Var(0), None));
	a.insert(Quad(Term::Var(7), Term::Var(6), Term::Ground(0), None));
	a.insert(Quad(Term::Ground(9), Term::Var(1), Term::Var(4), None));
	a.insert(Quad(Term::Ground(6), Term::Var(1), Term::Var(4), None));
	a.insert(Quad(Term::Var(3), Term::Var(3), Term::Ground(6), None));
	a.insert(Quad(Term::Var(5), Term::Ground(3), Term::Ground(5), None));
	a.insert(Quad(Term::Ground(9), Term::Var(9), Term::Ground(9), None));
	a.insert(Quad(Term::Var(9), Term::Ground(7), Term::Ground(0), None));
	a.insert(Quad(Term::Var(0), Term::Var(1), Term::Ground(9), None));
	a.insert(Quad(Term::Var(9), Term::Var(7), Term::Var(8), None));
	a.insert(Quad(Term::Var(9), Term::Ground(6), Term::Ground(3), None));
	a.insert(Quad(Term::Ground(5), Term::Ground(0), Term::Var(1), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Var(1006),
		Term::Var(1005),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Ground(0),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Var(1001),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Var(1003),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Var(1006),
		Term::Ground(3),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(7),
		Term::Var(1000),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(3),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Ground(8),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Var(1006),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Var(1006),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Var(1001),
		Term::Var(1004),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Var(1001),
		Term::Var(1004),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Var(1003),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Ground(3),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Var(1009),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Ground(7),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Var(1001),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Var(1007),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Ground(6),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Ground(0),
		Term::Var(1001),
		None,
	));
	test(a, b)
}
#[test]
fn iso_138() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Var(3), Term::Ground(7), Term::Var(3), None));
	a.insert(Quad(
		Term::Ground(3),
		Term::Ground(4),
		Term::Ground(8),
		None,
	));
	a.insert(Quad(
		Term::Ground(1),
		Term::Ground(4),
		Term::Ground(6),
		None,
	));
	a.insert(Quad(Term::Var(1), Term::Ground(8), Term::Var(8), None));
	a.insert(Quad(Term::Var(1), Term::Var(1), Term::Var(8), None));
	a.insert(Quad(Term::Var(1), Term::Var(5), Term::Var(4), None));
	a.insert(Quad(Term::Ground(9), Term::Var(7), Term::Var(1), None));
	a.insert(Quad(Term::Var(2), Term::Ground(9), Term::Var(0), None));
	a.insert(Quad(Term::Var(0), Term::Var(0), Term::Ground(3), None));
	a.insert(Quad(Term::Ground(1), Term::Var(1), Term::Ground(7), None));
	a.insert(Quad(Term::Var(8), Term::Ground(9), Term::Var(6), None));
	a.insert(Quad(Term::Ground(0), Term::Var(5), Term::Ground(7), None));
	a.insert(Quad(Term::Var(1), Term::Ground(5), Term::Var(5), None));
	a.insert(Quad(Term::Var(5), Term::Ground(0), Term::Ground(5), None));
	a.insert(Quad(Term::Ground(0), Term::Ground(9), Term::Var(9), None));
	a.insert(Quad(Term::Ground(1), Term::Ground(1), Term::Var(8), None));
	a.insert(Quad(Term::Ground(7), Term::Var(3), Term::Var(0), None));
	a.insert(Quad(Term::Ground(9), Term::Var(3), Term::Ground(7), None));
	a.insert(Quad(Term::Ground(1), Term::Var(7), Term::Ground(8), None));
	a.insert(Quad(Term::Var(8), Term::Ground(4), Term::Ground(9), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Var(1003),
		Term::Ground(7),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(4),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(4),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(8),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Var(1001),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Var(1005),
		Term::Var(1004),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Var(1007),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Ground(9),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Var(1000),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Var(1001),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Ground(9),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Var(1005),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(5),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Ground(0),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(9),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(1),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Ground(7),
		Term::Var(1003),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Var(1003),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Var(1007),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Ground(4),
		Term::Ground(9),
		None,
	));
	test(a, b)
}
#[test]
fn iso_139() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Ground(1), Term::Var(0), Term::Var(4), None));
	a.insert(Quad(
		Term::Ground(5),
		Term::Ground(9),
		Term::Ground(5),
		None,
	));
	a.insert(Quad(Term::Ground(5), Term::Var(4), Term::Var(6), None));
	a.insert(Quad(Term::Var(7), Term::Ground(4), Term::Ground(9), None));
	a.insert(Quad(Term::Ground(6), Term::Var(3), Term::Var(0), None));
	a.insert(Quad(Term::Var(6), Term::Ground(5), Term::Ground(3), None));
	a.insert(Quad(Term::Var(3), Term::Var(6), Term::Var(7), None));
	a.insert(Quad(Term::Var(9), Term::Ground(3), Term::Ground(3), None));
	a.insert(Quad(Term::Var(9), Term::Ground(4), Term::Ground(8), None));
	a.insert(Quad(Term::Ground(0), Term::Var(6), Term::Var(2), None));
	a.insert(Quad(
		Term::Ground(2),
		Term::Ground(7),
		Term::Ground(2),
		None,
	));
	a.insert(Quad(Term::Ground(9), Term::Var(4), Term::Var(2), None));
	a.insert(Quad(Term::Var(9), Term::Ground(1), Term::Var(6), None));
	a.insert(Quad(Term::Var(4), Term::Var(2), Term::Ground(1), None));
	a.insert(Quad(Term::Var(2), Term::Var(5), Term::Var(9), None));
	a.insert(Quad(
		Term::Ground(6),
		Term::Ground(2),
		Term::Ground(1),
		None,
	));
	a.insert(Quad(
		Term::Ground(9),
		Term::Ground(8),
		Term::Ground(6),
		None,
	));
	a.insert(Quad(Term::Ground(5), Term::Var(9), Term::Ground(4), None));
	a.insert(Quad(Term::Var(3), Term::Var(5), Term::Var(3), None));
	a.insert(Quad(Term::Var(1), Term::Var(5), Term::Var(6), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(1),
		Term::Var(1000),
		Term::Var(1004),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Ground(9),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Var(1004),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Ground(4),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Var(1003),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Var(1006),
		Term::Ground(5),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Var(1006),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Ground(3),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Ground(4),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Var(1006),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(7),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Var(1004),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Ground(1),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Var(1002),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Var(1005),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Ground(2),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Ground(8),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Var(1009),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Var(1005),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Var(1005),
		Term::Var(1006),
		None,
	));
	test(a, b)
}
#[test]
fn iso_140() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Var(7), Term::Var(0), Term::Var(2), None));
	a.insert(Quad(Term::Ground(5), Term::Var(8), Term::Var(1), None));
	a.insert(Quad(Term::Ground(4), Term::Var(1), Term::Var(9), None));
	a.insert(Quad(Term::Ground(6), Term::Ground(7), Term::Var(1), None));
	a.insert(Quad(Term::Ground(9), Term::Var(6), Term::Var(4), None));
	a.insert(Quad(Term::Var(3), Term::Ground(7), Term::Var(2), None));
	a.insert(Quad(Term::Var(4), Term::Ground(3), Term::Ground(0), None));
	a.insert(Quad(Term::Var(2), Term::Ground(7), Term::Var(3), None));
	a.insert(Quad(Term::Var(8), Term::Ground(6), Term::Var(2), None));
	a.insert(Quad(Term::Var(9), Term::Ground(7), Term::Var(5), None));
	a.insert(Quad(Term::Var(4), Term::Ground(8), Term::Ground(3), None));
	a.insert(Quad(Term::Ground(1), Term::Var(8), Term::Ground(5), None));
	a.insert(Quad(Term::Ground(4), Term::Ground(5), Term::Var(4), None));
	a.insert(Quad(Term::Ground(0), Term::Ground(2), Term::Var(3), None));
	a.insert(Quad(Term::Ground(1), Term::Ground(5), Term::Var(3), None));
	a.insert(Quad(Term::Var(6), Term::Ground(3), Term::Ground(8), None));
	a.insert(Quad(
		Term::Ground(3),
		Term::Ground(0),
		Term::Ground(7),
		None,
	));
	a.insert(Quad(Term::Var(2), Term::Ground(7), Term::Ground(1), None));
	a.insert(Quad(Term::Var(3), Term::Var(4), Term::Var(0), None));
	a.insert(Quad(Term::Ground(1), Term::Var(1), Term::Ground(3), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Var(1007),
		Term::Var(1000),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Var(1008),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Var(1001),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Ground(7),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Var(1006),
		Term::Var(1004),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Ground(7),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Ground(3),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Ground(7),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Ground(6),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Ground(7),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Ground(8),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Var(1008),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Ground(5),
		Term::Var(1004),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(2),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(5),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Var(1006),
		Term::Ground(3),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(0),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Ground(7),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Var(1004),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Var(1001),
		Term::Ground(3),
		None,
	));
	test(a, b)
}
#[test]
fn iso_141() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Var(9), Term::Var(6), Term::Var(5), None));
	a.insert(Quad(Term::Ground(3), Term::Var(2), Term::Ground(2), None));
	a.insert(Quad(Term::Ground(9), Term::Ground(6), Term::Var(1), None));
	a.insert(Quad(Term::Var(3), Term::Var(7), Term::Var(3), None));
	a.insert(Quad(Term::Ground(3), Term::Var(5), Term::Ground(4), None));
	a.insert(Quad(Term::Var(0), Term::Var(8), Term::Var(9), None));
	a.insert(Quad(Term::Ground(6), Term::Var(8), Term::Var(5), None));
	a.insert(Quad(Term::Var(8), Term::Var(3), Term::Ground(0), None));
	a.insert(Quad(Term::Var(2), Term::Var(5), Term::Ground(1), None));
	a.insert(Quad(Term::Ground(1), Term::Var(9), Term::Var(9), None));
	a.insert(Quad(Term::Ground(7), Term::Var(6), Term::Var(9), None));
	a.insert(Quad(Term::Var(9), Term::Ground(6), Term::Ground(2), None));
	a.insert(Quad(Term::Var(5), Term::Ground(8), Term::Ground(5), None));
	a.insert(Quad(Term::Var(0), Term::Var(9), Term::Ground(9), None));
	a.insert(Quad(
		Term::Ground(8),
		Term::Ground(6),
		Term::Ground(1),
		None,
	));
	a.insert(Quad(Term::Var(3), Term::Var(0), Term::Ground(7), None));
	a.insert(Quad(Term::Var(5), Term::Ground(1), Term::Ground(2), None));
	a.insert(Quad(
		Term::Ground(5),
		Term::Ground(0),
		Term::Ground(3),
		None,
	));
	a.insert(Quad(Term::Ground(0), Term::Ground(9), Term::Var(0), None));
	a.insert(Quad(Term::Ground(4), Term::Ground(8), Term::Var(8), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Var(1009),
		Term::Var(1006),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Var(1002),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Ground(6),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Var(1007),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Var(1005),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Var(1008),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Var(1008),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Var(1003),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Var(1005),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Var(1009),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Ground(7),
		Term::Var(1006),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Ground(6),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Ground(8),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Var(1009),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Ground(6),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Var(1000),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Ground(1),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Ground(0),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(9),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Ground(8),
		Term::Var(1008),
		None,
	));
	test(a, b)
}
#[test]
fn iso_142() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Ground(2), Term::Ground(4), Term::Var(9), None));
	a.insert(Quad(Term::Var(0), Term::Var(0), Term::Var(5), None));
	a.insert(Quad(Term::Ground(7), Term::Var(3), Term::Var(5), None));
	a.insert(Quad(Term::Ground(3), Term::Var(8), Term::Ground(9), None));
	a.insert(Quad(
		Term::Ground(6),
		Term::Ground(0),
		Term::Ground(9),
		None,
	));
	a.insert(Quad(Term::Var(9), Term::Ground(4), Term::Ground(5), None));
	a.insert(Quad(Term::Ground(7), Term::Var(6), Term::Ground(2), None));
	a.insert(Quad(Term::Ground(5), Term::Var(6), Term::Ground(4), None));
	a.insert(Quad(
		Term::Ground(8),
		Term::Ground(1),
		Term::Ground(9),
		None,
	));
	a.insert(Quad(Term::Var(9), Term::Ground(1), Term::Var(7), None));
	a.insert(Quad(Term::Var(5), Term::Var(3), Term::Ground(5), None));
	a.insert(Quad(
		Term::Ground(2),
		Term::Ground(2),
		Term::Ground(9),
		None,
	));
	a.insert(Quad(Term::Ground(0), Term::Var(8), Term::Var(2), None));
	a.insert(Quad(Term::Ground(4), Term::Var(7), Term::Ground(9), None));
	a.insert(Quad(Term::Ground(3), Term::Var(2), Term::Var(3), None));
	a.insert(Quad(Term::Var(5), Term::Ground(0), Term::Var(4), None));
	a.insert(Quad(Term::Var(6), Term::Var(6), Term::Var(6), None));
	a.insert(Quad(Term::Var(5), Term::Ground(9), Term::Var(1), None));
	a.insert(Quad(Term::Ground(9), Term::Ground(6), Term::Var(5), None));
	a.insert(Quad(Term::Var(7), Term::Ground(1), Term::Var(9), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(4),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Var(1000),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Ground(7),
		Term::Var(1003),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Var(1008),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Ground(0),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Ground(4),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Ground(7),
		Term::Var(1006),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Var(1006),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Ground(1),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Ground(1),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Var(1003),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(2),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Var(1008),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Var(1007),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Var(1002),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Ground(0),
		Term::Var(1004),
		None,
	));
	b.insert(Quad(
		Term::Var(1006),
		Term::Var(1006),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Ground(9),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Ground(6),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Ground(1),
		Term::Var(1009),
		None,
	));
	test(a, b)
}
#[test]
fn iso_143() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Ground(0), Term::Ground(9), Term::Var(7), None));
	a.insert(Quad(Term::Var(2), Term::Ground(6), Term::Var(9), None));
	a.insert(Quad(Term::Var(4), Term::Ground(4), Term::Ground(6), None));
	a.insert(Quad(Term::Var(3), Term::Var(5), Term::Ground(7), None));
	a.insert(Quad(Term::Var(3), Term::Var(9), Term::Var(8), None));
	a.insert(Quad(
		Term::Ground(4),
		Term::Ground(8),
		Term::Ground(8),
		None,
	));
	a.insert(Quad(
		Term::Ground(4),
		Term::Ground(7),
		Term::Ground(5),
		None,
	));
	a.insert(Quad(Term::Var(9), Term::Ground(0), Term::Var(2), None));
	a.insert(Quad(
		Term::Ground(9),
		Term::Ground(8),
		Term::Ground(5),
		None,
	));
	a.insert(Quad(Term::Var(4), Term::Var(1), Term::Var(6), None));
	a.insert(Quad(Term::Var(3), Term::Var(6), Term::Ground(1), None));
	a.insert(Quad(Term::Var(3), Term::Var(8), Term::Ground(2), None));
	a.insert(Quad(Term::Var(4), Term::Ground(3), Term::Ground(8), None));
	a.insert(Quad(Term::Var(7), Term::Ground(7), Term::Ground(2), None));
	a.insert(Quad(Term::Ground(2), Term::Ground(9), Term::Var(2), None));
	a.insert(Quad(Term::Ground(3), Term::Var(4), Term::Var(9), None));
	a.insert(Quad(
		Term::Ground(3),
		Term::Ground(4),
		Term::Ground(2),
		None,
	));
	a.insert(Quad(Term::Ground(3), Term::Var(2), Term::Ground(3), None));
	a.insert(Quad(Term::Ground(4), Term::Var(9), Term::Ground(2), None));
	a.insert(Quad(Term::Var(3), Term::Var(5), Term::Ground(5), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(9),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Ground(6),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Ground(4),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Var(1005),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Var(1009),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Ground(8),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Ground(7),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Ground(0),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Ground(8),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Var(1001),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Var(1006),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Var(1008),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Ground(3),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Ground(7),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(9),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Var(1004),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(4),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Var(1002),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Var(1009),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Var(1005),
		Term::Ground(5),
		None,
	));
	test(a, b)
}
#[test]
fn iso_144() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Var(4), Term::Ground(7), Term::Ground(0), None));
	a.insert(Quad(Term::Var(1), Term::Var(9), Term::Ground(7), None));
	a.insert(Quad(Term::Ground(1), Term::Ground(0), Term::Var(9), None));
	a.insert(Quad(Term::Ground(9), Term::Ground(6), Term::Var(5), None));
	a.insert(Quad(Term::Var(2), Term::Ground(4), Term::Var(0), None));
	a.insert(Quad(Term::Var(1), Term::Var(9), Term::Var(9), None));
	a.insert(Quad(Term::Var(7), Term::Ground(0), Term::Var(3), None));
	a.insert(Quad(Term::Ground(1), Term::Ground(3), Term::Var(6), None));
	a.insert(Quad(Term::Ground(6), Term::Var(7), Term::Ground(6), None));
	a.insert(Quad(Term::Var(1), Term::Var(8), Term::Ground(6), None));
	a.insert(Quad(Term::Var(8), Term::Ground(6), Term::Ground(3), None));
	a.insert(Quad(Term::Var(7), Term::Var(7), Term::Ground(8), None));
	a.insert(Quad(Term::Ground(2), Term::Var(8), Term::Var(7), None));
	a.insert(Quad(Term::Ground(0), Term::Ground(9), Term::Var(6), None));
	a.insert(Quad(Term::Ground(9), Term::Var(6), Term::Ground(7), None));
	a.insert(Quad(Term::Var(2), Term::Var(9), Term::Var(0), None));
	a.insert(Quad(Term::Var(1), Term::Var(5), Term::Ground(7), None));
	a.insert(Quad(Term::Ground(5), Term::Var(4), Term::Var(9), None));
	a.insert(Quad(Term::Var(2), Term::Var(9), Term::Ground(1), None));
	a.insert(Quad(Term::Var(0), Term::Var(1), Term::Ground(5), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Var(1004),
		Term::Ground(7),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Var(1009),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(0),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Ground(6),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Ground(4),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Var(1009),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Ground(0),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(3),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Var(1007),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Var(1008),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Ground(6),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Var(1007),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Var(1008),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(9),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Var(1006),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Var(1009),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Var(1005),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Var(1004),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Var(1009),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Var(1001),
		Term::Ground(5),
		None,
	));
	test(a, b)
}
#[test]
fn iso_145() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Var(2), Term::Ground(7), Term::Ground(6), None));
	a.insert(Quad(Term::Var(8), Term::Ground(6), Term::Ground(2), None));
	a.insert(Quad(Term::Ground(6), Term::Ground(0), Term::Var(6), None));
	a.insert(Quad(Term::Var(5), Term::Ground(8), Term::Ground(2), None));
	a.insert(Quad(Term::Ground(6), Term::Var(7), Term::Ground(7), None));
	a.insert(Quad(Term::Var(0), Term::Var(5), Term::Var(6), None));
	a.insert(Quad(Term::Var(9), Term::Ground(1), Term::Var(4), None));
	a.insert(Quad(Term::Var(5), Term::Var(0), Term::Var(7), None));
	a.insert(Quad(Term::Var(9), Term::Var(7), Term::Ground(3), None));
	a.insert(Quad(Term::Var(4), Term::Var(0), Term::Var(1), None));
	a.insert(Quad(
		Term::Ground(9),
		Term::Ground(6),
		Term::Ground(6),
		None,
	));
	a.insert(Quad(Term::Ground(1), Term::Ground(4), Term::Var(9), None));
	a.insert(Quad(Term::Var(1), Term::Var(0), Term::Ground(8), None));
	a.insert(Quad(Term::Var(3), Term::Ground(7), Term::Ground(4), None));
	a.insert(Quad(Term::Var(5), Term::Ground(7), Term::Var(2), None));
	a.insert(Quad(Term::Var(0), Term::Ground(3), Term::Var(1), None));
	a.insert(Quad(Term::Var(1), Term::Var(7), Term::Ground(3), None));
	a.insert(Quad(Term::Var(7), Term::Var(7), Term::Ground(3), None));
	a.insert(Quad(Term::Ground(4), Term::Var(6), Term::Ground(1), None));
	a.insert(Quad(
		Term::Ground(4),
		Term::Ground(5),
		Term::Ground(1),
		None,
	));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Var(1002),
		Term::Ground(7),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Ground(6),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Ground(0),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Ground(8),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Var(1007),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Var(1005),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Ground(1),
		Term::Var(1004),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Var(1000),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Var(1007),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Var(1000),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Ground(6),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(4),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Var(1000),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Ground(7),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Ground(7),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(3),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Var(1007),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Var(1007),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Var(1006),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Ground(5),
		Term::Ground(1),
		None,
	));
	test(a, b)
}
#[test]
fn iso_146() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Var(0), Term::Ground(2), Term::Var(4), None));
	a.insert(Quad(Term::Ground(4), Term::Var(3), Term::Var(5), None));
	a.insert(Quad(Term::Ground(3), Term::Var(7), Term::Ground(7), None));
	a.insert(Quad(Term::Ground(7), Term::Var(7), Term::Var(9), None));
	a.insert(Quad(Term::Var(9), Term::Ground(3), Term::Ground(2), None));
	a.insert(Quad(Term::Ground(7), Term::Var(9), Term::Var(0), None));
	a.insert(Quad(Term::Var(1), Term::Var(9), Term::Var(0), None));
	a.insert(Quad(
		Term::Ground(4),
		Term::Ground(8),
		Term::Ground(4),
		None,
	));
	a.insert(Quad(Term::Var(5), Term::Ground(8), Term::Var(0), None));
	a.insert(Quad(Term::Ground(7), Term::Var(6), Term::Var(9), None));
	a.insert(Quad(Term::Ground(8), Term::Ground(8), Term::Var(1), None));
	a.insert(Quad(Term::Ground(8), Term::Var(7), Term::Var(8), None));
	a.insert(Quad(Term::Ground(9), Term::Var(3), Term::Ground(7), None));
	a.insert(Quad(Term::Var(8), Term::Ground(1), Term::Var(2), None));
	a.insert(Quad(Term::Ground(9), Term::Var(0), Term::Ground(2), None));
	a.insert(Quad(Term::Var(9), Term::Var(6), Term::Var(0), None));
	a.insert(Quad(Term::Var(4), Term::Ground(5), Term::Ground(4), None));
	a.insert(Quad(Term::Var(9), Term::Ground(4), Term::Var(0), None));
	a.insert(Quad(Term::Ground(9), Term::Ground(5), Term::Var(0), None));
	a.insert(Quad(Term::Var(0), Term::Var(7), Term::Var(8), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(2),
		Term::Var(1004),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Var(1003),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Var(1007),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Ground(7),
		Term::Var(1007),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Ground(3),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(7),
		Term::Var(1009),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Var(1009),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Ground(8),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Ground(8),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Ground(7),
		Term::Var(1006),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Ground(8),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Var(1007),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Var(1003),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Ground(1),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Var(1000),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Var(1006),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Ground(5),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Ground(4),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Ground(5),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Var(1007),
		Term::Var(1008),
		None,
	));
	test(a, b)
}
#[test]
fn iso_147() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Ground(0), Term::Var(3), Term::Ground(2), None));
	a.insert(Quad(Term::Var(2), Term::Ground(8), Term::Ground(9), None));
	a.insert(Quad(Term::Ground(8), Term::Ground(6), Term::Var(5), None));
	a.insert(Quad(Term::Var(0), Term::Ground(8), Term::Ground(7), None));
	a.insert(Quad(Term::Ground(6), Term::Var(4), Term::Ground(0), None));
	a.insert(Quad(Term::Var(1), Term::Ground(4), Term::Ground(5), None));
	a.insert(Quad(Term::Var(3), Term::Ground(9), Term::Var(2), None));
	a.insert(Quad(Term::Ground(9), Term::Var(3), Term::Ground(8), None));
	a.insert(Quad(Term::Ground(1), Term::Var(3), Term::Var(8), None));
	a.insert(Quad(Term::Var(8), Term::Ground(9), Term::Var(7), None));
	a.insert(Quad(Term::Var(2), Term::Ground(3), Term::Ground(6), None));
	a.insert(Quad(Term::Ground(6), Term::Ground(4), Term::Var(7), None));
	a.insert(Quad(Term::Var(9), Term::Ground(6), Term::Ground(3), None));
	a.insert(Quad(Term::Var(4), Term::Var(3), Term::Ground(2), None));
	a.insert(Quad(Term::Var(4), Term::Var(6), Term::Var(5), None));
	a.insert(Quad(Term::Ground(1), Term::Var(2), Term::Var(4), None));
	a.insert(Quad(
		Term::Ground(8),
		Term::Ground(2),
		Term::Ground(6),
		None,
	));
	a.insert(Quad(Term::Var(3), Term::Var(2), Term::Ground(1), None));
	a.insert(Quad(Term::Var(3), Term::Ground(7), Term::Ground(5), None));
	a.insert(Quad(Term::Ground(7), Term::Var(9), Term::Var(2), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(0),
		Term::Var(1003),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Ground(8),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Ground(6),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(8),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Var(1004),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(4),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Ground(9),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Var(1003),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Var(1003),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Ground(9),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Ground(3),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Ground(4),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Ground(6),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Var(1003),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Var(1006),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Var(1002),
		Term::Var(1004),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Ground(2),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Var(1002),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Ground(7),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Ground(7),
		Term::Var(1009),
		Term::Var(1002),
		None,
	));
	test(a, b)
}
#[test]
fn iso_148() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Var(6), Term::Var(3), Term::Var(3), None));
	a.insert(Quad(
		Term::Ground(3),
		Term::Ground(7),
		Term::Ground(9),
		None,
	));
	a.insert(Quad(Term::Ground(4), Term::Var(0), Term::Var(1), None));
	a.insert(Quad(Term::Var(9), Term::Var(6), Term::Var(3), None));
	a.insert(Quad(Term::Ground(3), Term::Var(4), Term::Ground(7), None));
	a.insert(Quad(Term::Var(1), Term::Var(4), Term::Var(4), None));
	a.insert(Quad(Term::Var(3), Term::Ground(9), Term::Ground(6), None));
	a.insert(Quad(Term::Ground(1), Term::Var(5), Term::Var(1), None));
	a.insert(Quad(Term::Var(8), Term::Var(7), Term::Var(0), None));
	a.insert(Quad(Term::Var(9), Term::Var(4), Term::Ground(3), None));
	a.insert(Quad(Term::Var(0), Term::Var(2), Term::Ground(3), None));
	a.insert(Quad(Term::Ground(0), Term::Ground(2), Term::Var(1), None));
	a.insert(Quad(Term::Var(6), Term::Ground(8), Term::Var(0), None));
	a.insert(Quad(Term::Var(7), Term::Ground(4), Term::Var(8), None));
	a.insert(Quad(Term::Var(9), Term::Ground(6), Term::Ground(1), None));
	a.insert(Quad(Term::Var(7), Term::Var(4), Term::Ground(7), None));
	a.insert(Quad(Term::Var(1), Term::Ground(3), Term::Ground(3), None));
	a.insert(Quad(Term::Var(3), Term::Var(1), Term::Ground(0), None));
	a.insert(Quad(
		Term::Ground(9),
		Term::Ground(6),
		Term::Ground(4),
		None,
	));
	a.insert(Quad(
		Term::Ground(7),
		Term::Ground(1),
		Term::Ground(9),
		None,
	));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Var(1006),
		Term::Var(1003),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(7),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Var(1000),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Var(1006),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Var(1004),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Var(1004),
		Term::Var(1004),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Ground(9),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Var(1005),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Var(1007),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Var(1004),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Var(1002),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(2),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Var(1006),
		Term::Ground(8),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Ground(4),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Ground(6),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Var(1004),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(3),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Var(1001),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Ground(6),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Ground(7),
		Term::Ground(1),
		Term::Ground(9),
		None,
	));
	test(a, b)
}
#[test]
fn iso_149() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(
		Term::Ground(8),
		Term::Ground(5),
		Term::Ground(0),
		None,
	));
	a.insert(Quad(Term::Var(3), Term::Var(1), Term::Var(3), None));
	a.insert(Quad(Term::Var(1), Term::Var(8), Term::Ground(2), None));
	a.insert(Quad(Term::Ground(2), Term::Var(6), Term::Var(2), None));
	a.insert(Quad(Term::Var(4), Term::Var(5), Term::Var(1), None));
	a.insert(Quad(Term::Var(6), Term::Ground(7), Term::Var(7), None));
	a.insert(Quad(
		Term::Ground(0),
		Term::Ground(5),
		Term::Ground(5),
		None,
	));
	a.insert(Quad(Term::Ground(5), Term::Var(7), Term::Var(4), None));
	a.insert(Quad(Term::Var(5), Term::Ground(0), Term::Ground(9), None));
	a.insert(Quad(Term::Ground(6), Term::Ground(8), Term::Var(9), None));
	a.insert(Quad(Term::Var(5), Term::Var(3), Term::Var(1), None));
	a.insert(Quad(Term::Ground(4), Term::Var(9), Term::Var(2), None));
	a.insert(Quad(Term::Var(7), Term::Ground(2), Term::Var(3), None));
	a.insert(Quad(Term::Ground(2), Term::Var(3), Term::Var(4), None));
	a.insert(Quad(Term::Ground(6), Term::Var(7), Term::Ground(9), None));
	a.insert(Quad(Term::Var(6), Term::Var(5), Term::Ground(4), None));
	a.insert(Quad(Term::Ground(8), Term::Var(3), Term::Ground(6), None));
	a.insert(Quad(Term::Var(6), Term::Ground(3), Term::Ground(8), None));
	a.insert(Quad(
		Term::Ground(1),
		Term::Ground(5),
		Term::Ground(6),
		None,
	));
	a.insert(Quad(Term::Var(7), Term::Ground(8), Term::Var(1), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(8),
		Term::Ground(5),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Var(1001),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Var(1008),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Var(1006),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Var(1005),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Var(1006),
		Term::Ground(7),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(5),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Var(1007),
		Term::Var(1004),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Ground(0),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Ground(8),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Var(1003),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Var(1009),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Ground(2),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Var(1003),
		Term::Var(1004),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Var(1007),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Var(1006),
		Term::Var(1005),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Var(1003),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Var(1006),
		Term::Ground(3),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(5),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Ground(8),
		Term::Var(1001),
		None,
	));
	test(a, b)
}
#[test]
fn iso_150() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Var(2), Term::Var(0), Term::Ground(9), None));
	a.insert(Quad(Term::Ground(5), Term::Var(3), Term::Var(3), None));
	a.insert(Quad(Term::Var(9), Term::Var(5), Term::Ground(5), None));
	a.insert(Quad(Term::Var(2), Term::Ground(1), Term::Var(2), None));
	a.insert(Quad(Term::Ground(5), Term::Var(9), Term::Var(6), None));
	a.insert(Quad(
		Term::Ground(3),
		Term::Ground(3),
		Term::Ground(5),
		None,
	));
	a.insert(Quad(Term::Var(0), Term::Var(3), Term::Ground(5), None));
	a.insert(Quad(Term::Ground(8), Term::Var(1), Term::Var(4), None));
	a.insert(Quad(Term::Var(8), Term::Var(7), Term::Var(6), None));
	a.insert(Quad(Term::Var(2), Term::Var(7), Term::Var(7), None));
	a.insert(Quad(Term::Var(4), Term::Ground(8), Term::Ground(9), None));
	a.insert(Quad(Term::Ground(4), Term::Var(9), Term::Ground(6), None));
	a.insert(Quad(Term::Var(5), Term::Ground(0), Term::Ground(6), None));
	a.insert(Quad(Term::Var(5), Term::Var(9), Term::Var(4), None));
	a.insert(Quad(Term::Var(3), Term::Var(8), Term::Ground(1), None));
	a.insert(Quad(Term::Ground(1), Term::Ground(6), Term::Var(8), None));
	a.insert(Quad(Term::Ground(9), Term::Var(0), Term::Var(0), None));
	a.insert(Quad(
		Term::Ground(3),
		Term::Ground(0),
		Term::Ground(8),
		None,
	));
	a.insert(Quad(
		Term::Ground(1),
		Term::Ground(8),
		Term::Ground(1),
		None,
	));
	a.insert(Quad(Term::Ground(0), Term::Ground(2), Term::Var(7), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Var(1002),
		Term::Var(1000),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Var(1003),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Var(1005),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Ground(1),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Var(1009),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(3),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Var(1003),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Var(1001),
		Term::Var(1004),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Var(1007),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Var(1007),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Ground(8),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Var(1009),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Ground(0),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Var(1009),
		Term::Var(1004),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Var(1008),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(6),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Var(1000),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(0),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(8),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(2),
		Term::Var(1007),
		None,
	));
	test(a, b)
}
#[test]
fn iso_151() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Var(4), Term::Var(1), Term::Ground(4), None));
	a.insert(Quad(Term::Var(6), Term::Ground(9), Term::Var(1), None));
	a.insert(Quad(Term::Var(1), Term::Var(9), Term::Var(5), None));
	a.insert(Quad(Term::Ground(1), Term::Ground(0), Term::Var(4), None));
	a.insert(Quad(Term::Ground(4), Term::Var(4), Term::Ground(9), None));
	a.insert(Quad(Term::Var(0), Term::Var(9), Term::Var(2), None));
	a.insert(Quad(
		Term::Ground(7),
		Term::Ground(5),
		Term::Ground(3),
		None,
	));
	a.insert(Quad(Term::Var(3), Term::Ground(9), Term::Ground(4), None));
	a.insert(Quad(
		Term::Ground(4),
		Term::Ground(9),
		Term::Ground(1),
		None,
	));
	a.insert(Quad(Term::Var(0), Term::Var(4), Term::Var(1), None));
	a.insert(Quad(Term::Var(5), Term::Var(7), Term::Ground(7), None));
	a.insert(Quad(
		Term::Ground(7),
		Term::Ground(8),
		Term::Ground(1),
		None,
	));
	a.insert(Quad(Term::Ground(2), Term::Ground(7), Term::Var(8), None));
	a.insert(Quad(Term::Ground(9), Term::Var(8), Term::Ground(0), None));
	a.insert(Quad(
		Term::Ground(3),
		Term::Ground(4),
		Term::Ground(8),
		None,
	));
	a.insert(Quad(
		Term::Ground(2),
		Term::Ground(6),
		Term::Ground(9),
		None,
	));
	a.insert(Quad(Term::Ground(5), Term::Var(5), Term::Var(8), None));
	a.insert(Quad(Term::Var(1), Term::Var(2), Term::Var(3), None));
	a.insert(Quad(Term::Ground(3), Term::Ground(5), Term::Var(4), None));
	a.insert(Quad(Term::Ground(3), Term::Ground(6), Term::Var(1), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Var(1004),
		Term::Var(1001),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Var(1006),
		Term::Ground(9),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Var(1009),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(0),
		Term::Var(1004),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Var(1004),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Var(1009),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Ground(7),
		Term::Ground(5),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Ground(9),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Ground(9),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Var(1004),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Var(1007),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Ground(7),
		Term::Ground(8),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(7),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Var(1008),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(4),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(6),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Var(1005),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Var(1002),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(5),
		Term::Var(1004),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(6),
		Term::Var(1001),
		None,
	));
	test(a, b)
}
#[test]
fn iso_152() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Ground(5), Term::Ground(4), Term::Var(4), None));
	a.insert(Quad(Term::Var(5), Term::Var(0), Term::Ground(8), None));
	a.insert(Quad(
		Term::Ground(2),
		Term::Ground(0),
		Term::Ground(8),
		None,
	));
	a.insert(Quad(Term::Ground(1), Term::Var(4), Term::Ground(8), None));
	a.insert(Quad(Term::Var(4), Term::Ground(3), Term::Var(3), None));
	a.insert(Quad(Term::Var(1), Term::Ground(3), Term::Ground(0), None));
	a.insert(Quad(Term::Ground(9), Term::Var(5), Term::Ground(7), None));
	a.insert(Quad(Term::Ground(5), Term::Var(9), Term::Ground(7), None));
	a.insert(Quad(
		Term::Ground(8),
		Term::Ground(3),
		Term::Ground(4),
		None,
	));
	a.insert(Quad(Term::Var(4), Term::Ground(0), Term::Ground(3), None));
	a.insert(Quad(Term::Var(6), Term::Ground(5), Term::Ground(7), None));
	a.insert(Quad(
		Term::Ground(6),
		Term::Ground(9),
		Term::Ground(4),
		None,
	));
	a.insert(Quad(Term::Ground(6), Term::Ground(6), Term::Var(6), None));
	a.insert(Quad(Term::Ground(6), Term::Var(1), Term::Var(3), None));
	a.insert(Quad(Term::Var(7), Term::Var(7), Term::Ground(8), None));
	a.insert(Quad(Term::Var(9), Term::Var(1), Term::Ground(5), None));
	a.insert(Quad(Term::Var(9), Term::Var(5), Term::Ground(1), None));
	a.insert(Quad(Term::Ground(2), Term::Ground(9), Term::Var(3), None));
	a.insert(Quad(Term::Ground(8), Term::Var(2), Term::Ground(0), None));
	a.insert(Quad(Term::Ground(2), Term::Ground(4), Term::Var(0), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(5),
		Term::Ground(4),
		Term::Var(1004),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Var(1000),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(0),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Var(1004),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Ground(3),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(3),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Var(1005),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Var(1009),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Ground(3),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Ground(0),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Var(1006),
		Term::Ground(5),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Ground(9),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Ground(6),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Var(1001),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Var(1007),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Var(1001),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Var(1005),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(9),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Var(1002),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(4),
		Term::Var(1000),
		None,
	));
	test(a, b)
}
#[test]
fn iso_153() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Ground(9), Term::Ground(4), Term::Var(3), None));
	a.insert(Quad(Term::Var(7), Term::Var(3), Term::Var(3), None));
	a.insert(Quad(Term::Ground(7), Term::Var(4), Term::Var(7), None));
	a.insert(Quad(
		Term::Ground(3),
		Term::Ground(4),
		Term::Ground(6),
		None,
	));
	a.insert(Quad(Term::Var(8), Term::Var(5), Term::Var(9), None));
	a.insert(Quad(Term::Var(3), Term::Ground(2), Term::Ground(0), None));
	a.insert(Quad(
		Term::Ground(3),
		Term::Ground(6),
		Term::Ground(3),
		None,
	));
	a.insert(Quad(Term::Var(2), Term::Ground(5), Term::Var(9), None));
	a.insert(Quad(Term::Var(0), Term::Var(6), Term::Var(0), None));
	a.insert(Quad(
		Term::Ground(5),
		Term::Ground(8),
		Term::Ground(2),
		None,
	));
	a.insert(Quad(Term::Var(9), Term::Ground(2), Term::Ground(8), None));
	a.insert(Quad(
		Term::Ground(4),
		Term::Ground(2),
		Term::Ground(2),
		None,
	));
	a.insert(Quad(
		Term::Ground(2),
		Term::Ground(5),
		Term::Ground(3),
		None,
	));
	a.insert(Quad(
		Term::Ground(4),
		Term::Ground(4),
		Term::Ground(1),
		None,
	));
	a.insert(Quad(Term::Var(8), Term::Var(3), Term::Var(0), None));
	a.insert(Quad(Term::Var(7), Term::Ground(1), Term::Var(1), None));
	a.insert(Quad(
		Term::Ground(8),
		Term::Ground(6),
		Term::Ground(4),
		None,
	));
	a.insert(Quad(Term::Var(4), Term::Var(7), Term::Ground(9), None));
	a.insert(Quad(Term::Ground(8), Term::Ground(1), Term::Var(6), None));
	a.insert(Quad(Term::Var(8), Term::Ground(2), Term::Ground(8), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(9),
		Term::Ground(4),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Var(1003),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Ground(7),
		Term::Var(1004),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(4),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Var(1005),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Ground(2),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(6),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Ground(5),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Var(1006),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Ground(8),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Ground(2),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Ground(2),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(5),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Ground(4),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Var(1003),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Ground(1),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Ground(6),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Var(1007),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Ground(1),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Ground(2),
		Term::Ground(8),
		None,
	));
	test(a, b)
}
#[test]
fn iso_154() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(
		Term::Ground(3),
		Term::Ground(6),
		Term::Ground(6),
		None,
	));
	a.insert(Quad(Term::Var(5), Term::Var(8), Term::Var(3), None));
	a.insert(Quad(Term::Var(8), Term::Var(8), Term::Ground(4), None));
	a.insert(Quad(Term::Var(6), Term::Var(9), Term::Var(2), None));
	a.insert(Quad(Term::Var(7), Term::Var(0), Term::Ground(1), None));
	a.insert(Quad(Term::Var(6), Term::Var(1), Term::Var(0), None));
	a.insert(Quad(Term::Ground(5), Term::Var(2), Term::Var(7), None));
	a.insert(Quad(Term::Ground(3), Term::Var(1), Term::Ground(4), None));
	a.insert(Quad(Term::Var(1), Term::Var(5), Term::Ground(1), None));
	a.insert(Quad(Term::Var(8), Term::Var(9), Term::Var(7), None));
	a.insert(Quad(Term::Var(1), Term::Var(4), Term::Ground(6), None));
	a.insert(Quad(Term::Ground(0), Term::Var(6), Term::Var(6), None));
	a.insert(Quad(Term::Var(2), Term::Ground(8), Term::Var(5), None));
	a.insert(Quad(Term::Ground(8), Term::Var(9), Term::Ground(9), None));
	a.insert(Quad(Term::Var(3), Term::Ground(6), Term::Ground(2), None));
	a.insert(Quad(Term::Var(7), Term::Var(5), Term::Ground(0), None));
	a.insert(Quad(Term::Ground(0), Term::Var(2), Term::Var(7), None));
	a.insert(Quad(
		Term::Ground(1),
		Term::Ground(6),
		Term::Ground(8),
		None,
	));
	a.insert(Quad(Term::Var(2), Term::Ground(1), Term::Ground(5), None));
	a.insert(Quad(Term::Var(5), Term::Var(8), Term::Ground(3), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(6),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Var(1008),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Var(1008),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Var(1006),
		Term::Var(1009),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Var(1000),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Var(1006),
		Term::Var(1001),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Var(1002),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Var(1001),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Var(1005),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Var(1009),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Var(1004),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Var(1006),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Ground(8),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Var(1009),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Ground(6),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Var(1005),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Var(1002),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(6),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Ground(1),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Var(1008),
		Term::Ground(3),
		None,
	));
	test(a, b)
}
#[test]
fn iso_155() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Var(4), Term::Ground(2), Term::Var(8), None));
	a.insert(Quad(Term::Var(6), Term::Var(3), Term::Var(4), None));
	a.insert(Quad(Term::Ground(0), Term::Ground(0), Term::Var(5), None));
	a.insert(Quad(Term::Ground(3), Term::Var(6), Term::Var(6), None));
	a.insert(Quad(Term::Var(2), Term::Var(6), Term::Var(6), None));
	a.insert(Quad(Term::Var(5), Term::Var(3), Term::Var(1), None));
	a.insert(Quad(Term::Var(8), Term::Var(5), Term::Var(7), None));
	a.insert(Quad(Term::Var(7), Term::Ground(2), Term::Ground(3), None));
	a.insert(Quad(Term::Var(4), Term::Var(3), Term::Var(9), None));
	a.insert(Quad(Term::Ground(5), Term::Ground(3), Term::Var(2), None));
	a.insert(Quad(Term::Ground(8), Term::Var(4), Term::Ground(0), None));
	a.insert(Quad(Term::Var(1), Term::Ground(5), Term::Ground(5), None));
	a.insert(Quad(Term::Ground(5), Term::Var(8), Term::Ground(0), None));
	a.insert(Quad(Term::Ground(6), Term::Var(8), Term::Ground(5), None));
	a.insert(Quad(Term::Ground(5), Term::Var(4), Term::Ground(7), None));
	a.insert(Quad(
		Term::Ground(0),
		Term::Ground(3),
		Term::Ground(6),
		None,
	));
	a.insert(Quad(Term::Var(6), Term::Var(1), Term::Ground(3), None));
	a.insert(Quad(
		Term::Ground(2),
		Term::Ground(9),
		Term::Ground(8),
		None,
	));
	a.insert(Quad(Term::Ground(3), Term::Var(2), Term::Ground(0), None));
	a.insert(Quad(Term::Ground(6), Term::Var(7), Term::Var(7), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Var(1004),
		Term::Ground(2),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Var(1006),
		Term::Var(1003),
		Term::Var(1004),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(0),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Var(1006),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Var(1006),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Var(1003),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Var(1005),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Ground(2),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Var(1003),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Ground(3),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Var(1004),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(5),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Var(1008),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Var(1008),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Var(1004),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(3),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Var(1006),
		Term::Var(1001),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(9),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Var(1002),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Var(1007),
		Term::Var(1007),
		None,
	));
	test(a, b)
}
#[test]
fn iso_156() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Ground(2), Term::Var(1), Term::Var(3), None));
	a.insert(Quad(Term::Ground(2), Term::Var(2), Term::Var(2), None));
	a.insert(Quad(Term::Var(2), Term::Ground(4), Term::Var(5), None));
	a.insert(Quad(Term::Var(1), Term::Var(3), Term::Var(4), None));
	a.insert(Quad(Term::Var(2), Term::Ground(6), Term::Var(3), None));
	a.insert(Quad(Term::Ground(1), Term::Var(0), Term::Var(8), None));
	a.insert(Quad(Term::Ground(5), Term::Var(1), Term::Ground(0), None));
	a.insert(Quad(Term::Ground(0), Term::Var(7), Term::Ground(0), None));
	a.insert(Quad(
		Term::Ground(3),
		Term::Ground(2),
		Term::Ground(6),
		None,
	));
	a.insert(Quad(Term::Var(4), Term::Var(5), Term::Var(6), None));
	a.insert(Quad(Term::Ground(0), Term::Ground(9), Term::Var(3), None));
	a.insert(Quad(Term::Var(7), Term::Ground(6), Term::Ground(9), None));
	a.insert(Quad(Term::Ground(2), Term::Ground(9), Term::Var(9), None));
	a.insert(Quad(Term::Var(4), Term::Ground(6), Term::Ground(3), None));
	a.insert(Quad(Term::Var(0), Term::Var(8), Term::Ground(3), None));
	a.insert(Quad(Term::Var(4), Term::Var(0), Term::Var(0), None));
	a.insert(Quad(
		Term::Ground(5),
		Term::Ground(2),
		Term::Ground(4),
		None,
	));
	a.insert(Quad(
		Term::Ground(8),
		Term::Ground(6),
		Term::Ground(9),
		None,
	));
	a.insert(Quad(Term::Var(7), Term::Var(1), Term::Var(6), None));
	a.insert(Quad(Term::Var(9), Term::Var(4), Term::Ground(8), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(2),
		Term::Var(1001),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Var(1002),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Ground(4),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Var(1003),
		Term::Var(1004),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Ground(6),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Var(1000),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Var(1001),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Var(1007),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(2),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Var(1005),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(9),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Ground(6),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(9),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Ground(6),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Var(1008),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Var(1000),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Ground(2),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Ground(6),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Var(1001),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Var(1004),
		Term::Ground(8),
		None,
	));
	test(a, b)
}
#[test]
fn iso_157() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Var(0), Term::Ground(8), Term::Var(7), None));
	a.insert(Quad(Term::Var(9), Term::Var(3), Term::Var(6), None));
	a.insert(Quad(Term::Ground(4), Term::Var(0), Term::Var(4), None));
	a.insert(Quad(Term::Ground(3), Term::Ground(8), Term::Var(5), None));
	a.insert(Quad(Term::Var(7), Term::Ground(6), Term::Ground(9), None));
	a.insert(Quad(Term::Var(4), Term::Var(2), Term::Var(0), None));
	a.insert(Quad(Term::Var(8), Term::Ground(2), Term::Ground(5), None));
	a.insert(Quad(Term::Var(1), Term::Ground(8), Term::Var(0), None));
	a.insert(Quad(Term::Ground(2), Term::Var(3), Term::Var(4), None));
	a.insert(Quad(
		Term::Ground(7),
		Term::Ground(5),
		Term::Ground(8),
		None,
	));
	a.insert(Quad(
		Term::Ground(1),
		Term::Ground(7),
		Term::Ground(1),
		None,
	));
	a.insert(Quad(Term::Var(6), Term::Ground(7), Term::Ground(6), None));
	a.insert(Quad(Term::Var(3), Term::Var(6), Term::Var(0), None));
	a.insert(Quad(Term::Var(9), Term::Ground(6), Term::Var(4), None));
	a.insert(Quad(Term::Var(3), Term::Ground(4), Term::Var(1), None));
	a.insert(Quad(Term::Var(9), Term::Ground(3), Term::Ground(9), None));
	a.insert(Quad(Term::Var(7), Term::Var(7), Term::Var(4), None));
	a.insert(Quad(
		Term::Ground(3),
		Term::Ground(1),
		Term::Ground(8),
		None,
	));
	a.insert(Quad(Term::Var(5), Term::Ground(4), Term::Ground(7), None));
	a.insert(Quad(Term::Var(8), Term::Ground(1), Term::Var(4), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(8),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Var(1003),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Var(1000),
		Term::Var(1004),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(8),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Ground(6),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Var(1002),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Ground(2),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(8),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Var(1003),
		Term::Var(1004),
		None,
	));
	b.insert(Quad(
		Term::Ground(7),
		Term::Ground(5),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(7),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Var(1006),
		Term::Ground(7),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Var(1006),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Ground(6),
		Term::Var(1004),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Ground(4),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Ground(3),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Var(1007),
		Term::Var(1004),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(1),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Ground(4),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Ground(1),
		Term::Var(1004),
		None,
	));
	test(a, b)
}
#[test]
fn iso_158() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Var(3), Term::Var(2), Term::Ground(5), None));
	a.insert(Quad(Term::Ground(0), Term::Var(8), Term::Var(8), None));
	a.insert(Quad(Term::Ground(2), Term::Var(0), Term::Ground(1), None));
	a.insert(Quad(Term::Ground(2), Term::Ground(3), Term::Var(1), None));
	a.insert(Quad(Term::Var(5), Term::Ground(9), Term::Var(7), None));
	a.insert(Quad(Term::Var(3), Term::Ground(9), Term::Ground(1), None));
	a.insert(Quad(
		Term::Ground(5),
		Term::Ground(4),
		Term::Ground(5),
		None,
	));
	a.insert(Quad(Term::Var(8), Term::Var(3), Term::Var(8), None));
	a.insert(Quad(Term::Var(0), Term::Var(9), Term::Var(5), None));
	a.insert(Quad(Term::Var(7), Term::Var(4), Term::Var(3), None));
	a.insert(Quad(Term::Ground(3), Term::Var(0), Term::Ground(4), None));
	a.insert(Quad(Term::Var(3), Term::Ground(2), Term::Var(2), None));
	a.insert(Quad(Term::Ground(9), Term::Var(1), Term::Ground(7), None));
	a.insert(Quad(Term::Var(7), Term::Var(4), Term::Var(0), None));
	a.insert(Quad(Term::Var(1), Term::Ground(6), Term::Ground(5), None));
	a.insert(Quad(Term::Ground(2), Term::Var(1), Term::Ground(1), None));
	a.insert(Quad(Term::Ground(0), Term::Ground(3), Term::Var(3), None));
	a.insert(Quad(Term::Ground(3), Term::Var(8), Term::Ground(8), None));
	a.insert(Quad(Term::Var(7), Term::Var(8), Term::Var(9), None));
	a.insert(Quad(Term::Ground(1), Term::Var(8), Term::Ground(6), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Var(1003),
		Term::Var(1002),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Var(1008),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Var(1000),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(3),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Ground(9),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Ground(9),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Ground(4),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Var(1003),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Var(1009),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Var(1004),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Var(1000),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Ground(2),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Var(1001),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Var(1004),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(6),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Var(1001),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(3),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Var(1008),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Var(1008),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Var(1008),
		Term::Ground(6),
		None,
	));
	test(a, b)
}
#[test]
fn iso_159() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Ground(9), Term::Var(8), Term::Var(8), None));
	a.insert(Quad(Term::Ground(2), Term::Ground(8), Term::Var(5), None));
	a.insert(Quad(Term::Ground(8), Term::Var(1), Term::Var(3), None));
	a.insert(Quad(Term::Var(4), Term::Var(8), Term::Var(3), None));
	a.insert(Quad(Term::Var(8), Term::Ground(6), Term::Ground(0), None));
	a.insert(Quad(Term::Var(0), Term::Ground(8), Term::Var(5), None));
	a.insert(Quad(Term::Var(1), Term::Ground(8), Term::Ground(5), None));
	a.insert(Quad(
		Term::Ground(3),
		Term::Ground(7),
		Term::Ground(8),
		None,
	));
	a.insert(Quad(Term::Var(6), Term::Ground(4), Term::Ground(7), None));
	a.insert(Quad(Term::Var(9), Term::Var(9), Term::Var(0), None));
	a.insert(Quad(Term::Var(6), Term::Var(9), Term::Var(5), None));
	a.insert(Quad(
		Term::Ground(2),
		Term::Ground(2),
		Term::Ground(0),
		None,
	));
	a.insert(Quad(Term::Var(6), Term::Ground(7), Term::Ground(9), None));
	a.insert(Quad(Term::Var(0), Term::Ground(0), Term::Ground(5), None));
	a.insert(Quad(Term::Var(2), Term::Ground(1), Term::Var(5), None));
	a.insert(Quad(Term::Var(8), Term::Ground(8), Term::Ground(5), None));
	a.insert(Quad(Term::Ground(5), Term::Var(8), Term::Var(0), None));
	a.insert(Quad(Term::Ground(5), Term::Ground(6), Term::Var(8), None));
	a.insert(Quad(Term::Var(8), Term::Ground(1), Term::Ground(4), None));
	a.insert(Quad(
		Term::Ground(7),
		Term::Ground(4),
		Term::Ground(4),
		None,
	));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(9),
		Term::Var(1008),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(8),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Var(1001),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Var(1008),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Ground(6),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(8),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(8),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(7),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Var(1006),
		Term::Ground(4),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Var(1009),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Var(1006),
		Term::Var(1009),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(2),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Var(1006),
		Term::Ground(7),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(0),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Ground(1),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Ground(8),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Var(1008),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Ground(6),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Ground(1),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Ground(7),
		Term::Ground(4),
		Term::Ground(4),
		None,
	));
	test(a, b)
}
#[test]
fn iso_160() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Var(5), Term::Var(3), Term::Var(6), None));
	a.insert(Quad(Term::Var(9), Term::Var(4), Term::Var(6), None));
	a.insert(Quad(Term::Ground(1), Term::Var(4), Term::Ground(8), None));
	a.insert(Quad(Term::Ground(7), Term::Var(3), Term::Var(0), None));
	a.insert(Quad(Term::Ground(9), Term::Var(7), Term::Var(5), None));
	a.insert(Quad(Term::Var(7), Term::Ground(2), Term::Var(3), None));
	a.insert(Quad(
		Term::Ground(8),
		Term::Ground(0),
		Term::Ground(6),
		None,
	));
	a.insert(Quad(Term::Var(4), Term::Ground(6), Term::Var(1), None));
	a.insert(Quad(Term::Var(9), Term::Ground(5), Term::Ground(9), None));
	a.insert(Quad(Term::Var(0), Term::Ground(5), Term::Ground(6), None));
	a.insert(Quad(Term::Ground(8), Term::Var(6), Term::Ground(8), None));
	a.insert(Quad(Term::Var(3), Term::Var(9), Term::Var(6), None));
	a.insert(Quad(Term::Ground(1), Term::Ground(5), Term::Var(4), None));
	a.insert(Quad(Term::Ground(1), Term::Var(4), Term::Var(9), None));
	a.insert(Quad(Term::Var(4), Term::Ground(1), Term::Var(9), None));
	a.insert(Quad(
		Term::Ground(1),
		Term::Ground(8),
		Term::Ground(1),
		None,
	));
	a.insert(Quad(
		Term::Ground(1),
		Term::Ground(8),
		Term::Ground(4),
		None,
	));
	a.insert(Quad(Term::Var(0), Term::Var(0), Term::Var(2), None));
	a.insert(Quad(Term::Var(8), Term::Var(6), Term::Var(9), None));
	a.insert(Quad(Term::Ground(0), Term::Var(6), Term::Var(4), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Var(1005),
		Term::Var(1003),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Var(1004),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Var(1004),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Ground(7),
		Term::Var(1003),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Var(1007),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Ground(2),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Ground(0),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Ground(6),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Ground(5),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(5),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Var(1006),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Var(1009),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(5),
		Term::Var(1004),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Var(1004),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Ground(1),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(8),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(8),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Var(1000),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Var(1006),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Var(1006),
		Term::Var(1004),
		None,
	));
	test(a, b)
}
#[test]
fn iso_161() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Var(6), Term::Ground(7), Term::Ground(8), None));
	a.insert(Quad(Term::Var(6), Term::Var(0), Term::Ground(1), None));
	a.insert(Quad(Term::Var(5), Term::Var(0), Term::Ground(2), None));
	a.insert(Quad(Term::Ground(2), Term::Ground(3), Term::Var(0), None));
	a.insert(Quad(Term::Var(6), Term::Var(8), Term::Var(4), None));
	a.insert(Quad(
		Term::Ground(1),
		Term::Ground(3),
		Term::Ground(7),
		None,
	));
	a.insert(Quad(Term::Ground(5), Term::Var(0), Term::Var(1), None));
	a.insert(Quad(Term::Var(6), Term::Var(6), Term::Ground(6), None));
	a.insert(Quad(Term::Ground(1), Term::Var(3), Term::Var(4), None));
	a.insert(Quad(Term::Var(4), Term::Ground(1), Term::Var(6), None));
	a.insert(Quad(Term::Var(0), Term::Ground(1), Term::Ground(8), None));
	a.insert(Quad(Term::Var(0), Term::Var(8), Term::Ground(0), None));
	a.insert(Quad(Term::Var(6), Term::Var(2), Term::Var(6), None));
	a.insert(Quad(Term::Var(7), Term::Ground(8), Term::Var(0), None));
	a.insert(Quad(Term::Ground(2), Term::Ground(0), Term::Var(7), None));
	a.insert(Quad(Term::Var(8), Term::Var(8), Term::Var(8), None));
	a.insert(Quad(Term::Var(8), Term::Var(5), Term::Var(1), None));
	a.insert(Quad(Term::Var(5), Term::Ground(0), Term::Var(8), None));
	a.insert(Quad(Term::Var(4), Term::Ground(5), Term::Ground(2), None));
	a.insert(Quad(Term::Ground(2), Term::Ground(2), Term::Var(9), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Var(1006),
		Term::Ground(7),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Var(1006),
		Term::Var(1000),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Var(1000),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(3),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Var(1006),
		Term::Var(1008),
		Term::Var(1004),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(3),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Var(1000),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Var(1006),
		Term::Var(1006),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Var(1003),
		Term::Var(1004),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Ground(1),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(1),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Var(1008),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Var(1006),
		Term::Var(1002),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Ground(8),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(0),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Var(1008),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Var(1005),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Ground(0),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Ground(5),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(2),
		Term::Var(1009),
		None,
	));
	test(a, b)
}
#[test]
fn iso_162() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(
		Term::Ground(0),
		Term::Ground(2),
		Term::Ground(6),
		None,
	));
	a.insert(Quad(Term::Var(5), Term::Ground(0), Term::Ground(8), None));
	a.insert(Quad(Term::Var(7), Term::Var(8), Term::Var(9), None));
	a.insert(Quad(
		Term::Ground(7),
		Term::Ground(2),
		Term::Ground(7),
		None,
	));
	a.insert(Quad(Term::Ground(1), Term::Var(3), Term::Ground(7), None));
	a.insert(Quad(Term::Ground(4), Term::Ground(3), Term::Var(1), None));
	a.insert(Quad(Term::Var(3), Term::Var(6), Term::Var(3), None));
	a.insert(Quad(Term::Ground(9), Term::Var(5), Term::Var(9), None));
	a.insert(Quad(Term::Var(0), Term::Var(2), Term::Var(7), None));
	a.insert(Quad(Term::Ground(3), Term::Var(6), Term::Ground(6), None));
	a.insert(Quad(Term::Var(1), Term::Ground(9), Term::Var(3), None));
	a.insert(Quad(Term::Var(2), Term::Ground(5), Term::Var(3), None));
	a.insert(Quad(Term::Ground(5), Term::Var(5), Term::Ground(8), None));
	a.insert(Quad(Term::Ground(2), Term::Var(3), Term::Ground(1), None));
	a.insert(Quad(Term::Var(9), Term::Var(4), Term::Var(6), None));
	a.insert(Quad(Term::Var(3), Term::Ground(2), Term::Var(0), None));
	a.insert(Quad(Term::Var(0), Term::Var(5), Term::Var(7), None));
	a.insert(Quad(Term::Var(1), Term::Var(8), Term::Var(6), None));
	a.insert(Quad(Term::Ground(4), Term::Var(4), Term::Var(3), None));
	a.insert(Quad(Term::Var(7), Term::Ground(2), Term::Ground(1), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(2),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Ground(0),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Var(1008),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Ground(7),
		Term::Ground(2),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Var(1003),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Ground(3),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Var(1006),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Var(1005),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Var(1002),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Var(1006),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(9),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Ground(5),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Var(1005),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Var(1003),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Var(1004),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Ground(2),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Var(1005),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Var(1008),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Var(1004),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Ground(2),
		Term::Ground(1),
		None,
	));
	test(a, b)
}
#[test]
fn iso_163() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Var(6), Term::Ground(0), Term::Ground(8), None));
	a.insert(Quad(Term::Var(0), Term::Ground(6), Term::Ground(4), None));
	a.insert(Quad(
		Term::Ground(8),
		Term::Ground(4),
		Term::Ground(6),
		None,
	));
	a.insert(Quad(Term::Var(7), Term::Ground(0), Term::Var(8), None));
	a.insert(Quad(Term::Ground(2), Term::Var(9), Term::Ground(1), None));
	a.insert(Quad(Term::Ground(0), Term::Var(6), Term::Var(2), None));
	a.insert(Quad(
		Term::Ground(1),
		Term::Ground(2),
		Term::Ground(8),
		None,
	));
	a.insert(Quad(Term::Var(4), Term::Ground(1), Term::Var(9), None));
	a.insert(Quad(Term::Ground(0), Term::Var(0), Term::Ground(0), None));
	a.insert(Quad(Term::Ground(1), Term::Var(9), Term::Ground(5), None));
	a.insert(Quad(Term::Var(4), Term::Ground(5), Term::Ground(6), None));
	a.insert(Quad(
		Term::Ground(9),
		Term::Ground(6),
		Term::Ground(9),
		None,
	));
	a.insert(Quad(Term::Ground(6), Term::Var(1), Term::Ground(0), None));
	a.insert(Quad(Term::Var(6), Term::Ground(3), Term::Ground(0), None));
	a.insert(Quad(Term::Var(3), Term::Var(5), Term::Ground(1), None));
	a.insert(Quad(Term::Var(5), Term::Ground(4), Term::Ground(6), None));
	a.insert(Quad(Term::Ground(8), Term::Ground(5), Term::Var(1), None));
	a.insert(Quad(Term::Ground(0), Term::Var(4), Term::Var(8), None));
	a.insert(Quad(
		Term::Ground(2),
		Term::Ground(1),
		Term::Ground(4),
		None,
	));
	a.insert(Quad(Term::Var(4), Term::Var(6), Term::Var(5), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Var(1006),
		Term::Ground(0),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(6),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Ground(4),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Ground(0),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Var(1009),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Var(1006),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(2),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Ground(1),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Var(1000),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Var(1009),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Ground(5),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Ground(6),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Var(1001),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Var(1006),
		Term::Ground(3),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Var(1005),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Ground(4),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Ground(5),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Var(1004),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(1),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Var(1006),
		Term::Var(1005),
		None,
	));
	test(a, b)
}
#[test]
fn iso_164() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Ground(2), Term::Var(9), Term::Var(7), None));
	a.insert(Quad(Term::Var(9), Term::Ground(8), Term::Ground(3), None));
	a.insert(Quad(
		Term::Ground(8),
		Term::Ground(6),
		Term::Ground(1),
		None,
	));
	a.insert(Quad(Term::Var(5), Term::Ground(2), Term::Var(3), None));
	a.insert(Quad(Term::Ground(1), Term::Var(6), Term::Ground(7), None));
	a.insert(Quad(
		Term::Ground(2),
		Term::Ground(8),
		Term::Ground(5),
		None,
	));
	a.insert(Quad(Term::Var(1), Term::Var(5), Term::Ground(2), None));
	a.insert(Quad(Term::Ground(5), Term::Var(0), Term::Var(9), None));
	a.insert(Quad(Term::Ground(4), Term::Var(2), Term::Var(9), None));
	a.insert(Quad(Term::Var(9), Term::Ground(7), Term::Ground(6), None));
	a.insert(Quad(Term::Var(4), Term::Ground(3), Term::Var(0), None));
	a.insert(Quad(Term::Ground(9), Term::Var(9), Term::Var(0), None));
	a.insert(Quad(Term::Var(0), Term::Var(3), Term::Var(3), None));
	a.insert(Quad(
		Term::Ground(3),
		Term::Ground(9),
		Term::Ground(6),
		None,
	));
	a.insert(Quad(Term::Var(3), Term::Var(2), Term::Var(2), None));
	a.insert(Quad(Term::Var(6), Term::Ground(0), Term::Ground(0), None));
	a.insert(Quad(Term::Ground(5), Term::Var(5), Term::Var(6), None));
	a.insert(Quad(Term::Var(9), Term::Var(1), Term::Var(3), None));
	a.insert(Quad(Term::Var(9), Term::Var(8), Term::Ground(4), None));
	a.insert(Quad(Term::Var(3), Term::Var(6), Term::Var(2), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(2),
		Term::Var(1009),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Ground(8),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Ground(6),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Ground(2),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Var(1006),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(8),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Var(1005),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Var(1000),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Var(1002),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Ground(7),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Ground(3),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Var(1009),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Var(1003),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(9),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Var(1002),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Var(1006),
		Term::Ground(0),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Var(1005),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Var(1001),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Var(1008),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Var(1006),
		Term::Var(1002),
		None,
	));
	test(a, b)
}
#[test]
fn iso_165() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Var(1), Term::Var(7), Term::Var(3), None));
	a.insert(Quad(Term::Var(1), Term::Var(8), Term::Var(1), None));
	a.insert(Quad(
		Term::Ground(5),
		Term::Ground(7),
		Term::Ground(6),
		None,
	));
	a.insert(Quad(Term::Var(5), Term::Var(1), Term::Var(4), None));
	a.insert(Quad(Term::Var(5), Term::Var(9), Term::Var(1), None));
	a.insert(Quad(Term::Var(2), Term::Var(9), Term::Ground(1), None));
	a.insert(Quad(Term::Var(6), Term::Ground(0), Term::Var(1), None));
	a.insert(Quad(Term::Ground(3), Term::Var(4), Term::Var(6), None));
	a.insert(Quad(Term::Ground(8), Term::Var(9), Term::Ground(7), None));
	a.insert(Quad(Term::Var(4), Term::Ground(8), Term::Var(8), None));
	a.insert(Quad(Term::Var(6), Term::Ground(3), Term::Ground(0), None));
	a.insert(Quad(
		Term::Ground(6),
		Term::Ground(3),
		Term::Ground(6),
		None,
	));
	a.insert(Quad(Term::Var(4), Term::Ground(9), Term::Ground(7), None));
	a.insert(Quad(
		Term::Ground(6),
		Term::Ground(7),
		Term::Ground(4),
		None,
	));
	a.insert(Quad(Term::Ground(4), Term::Var(6), Term::Ground(6), None));
	a.insert(Quad(Term::Var(4), Term::Var(7), Term::Var(4), None));
	a.insert(Quad(Term::Var(3), Term::Ground(7), Term::Ground(5), None));
	a.insert(Quad(
		Term::Ground(8),
		Term::Ground(1),
		Term::Ground(3),
		None,
	));
	a.insert(Quad(Term::Var(8), Term::Ground(2), Term::Var(1), None));
	a.insert(Quad(Term::Ground(3), Term::Ground(3), Term::Var(3), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Var(1001),
		Term::Var(1007),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Var(1008),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Ground(7),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Var(1001),
		Term::Var(1004),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Var(1009),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Var(1009),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Var(1006),
		Term::Ground(0),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Var(1004),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Var(1009),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Ground(8),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Var(1006),
		Term::Ground(3),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Ground(3),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Ground(9),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Ground(7),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Var(1006),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Var(1007),
		Term::Var(1004),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Ground(7),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Ground(1),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Ground(2),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(3),
		Term::Var(1003),
		None,
	));
	test(a, b)
}
#[test]
fn iso_166() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(
		Term::Ground(8),
		Term::Ground(7),
		Term::Ground(8),
		None,
	));
	a.insert(Quad(Term::Ground(5), Term::Ground(8), Term::Var(2), None));
	a.insert(Quad(
		Term::Ground(2),
		Term::Ground(5),
		Term::Ground(2),
		None,
	));
	a.insert(Quad(Term::Ground(0), Term::Var(6), Term::Var(1), None));
	a.insert(Quad(Term::Ground(6), Term::Var(3), Term::Var(7), None));
	a.insert(Quad(Term::Ground(4), Term::Var(0), Term::Ground(3), None));
	a.insert(Quad(
		Term::Ground(4),
		Term::Ground(1),
		Term::Ground(5),
		None,
	));
	a.insert(Quad(
		Term::Ground(6),
		Term::Ground(7),
		Term::Ground(5),
		None,
	));
	a.insert(Quad(
		Term::Ground(2),
		Term::Ground(4),
		Term::Ground(4),
		None,
	));
	a.insert(Quad(Term::Ground(2), Term::Ground(9), Term::Var(9), None));
	a.insert(Quad(
		Term::Ground(5),
		Term::Ground(2),
		Term::Ground(8),
		None,
	));
	a.insert(Quad(Term::Ground(1), Term::Var(0), Term::Ground(9), None));
	a.insert(Quad(Term::Var(0), Term::Ground(3), Term::Var(2), None));
	a.insert(Quad(Term::Ground(3), Term::Var(7), Term::Ground(8), None));
	a.insert(Quad(Term::Var(0), Term::Var(9), Term::Ground(9), None));
	a.insert(Quad(
		Term::Ground(4),
		Term::Ground(0),
		Term::Ground(1),
		None,
	));
	a.insert(Quad(Term::Var(2), Term::Var(1), Term::Var(7), None));
	a.insert(Quad(Term::Var(5), Term::Ground(2), Term::Var(8), None));
	a.insert(Quad(Term::Var(5), Term::Ground(6), Term::Ground(8), None));
	a.insert(Quad(Term::Var(6), Term::Var(0), Term::Ground(3), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(8),
		Term::Ground(7),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Ground(8),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(5),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Var(1006),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Var(1003),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Var(1000),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Ground(1),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Ground(7),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(4),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(9),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Ground(2),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Var(1000),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(3),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Var(1007),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Var(1009),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Ground(0),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Var(1001),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Ground(2),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Ground(6),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Var(1006),
		Term::Var(1000),
		Term::Ground(3),
		None,
	));
	test(a, b)
}
#[test]
fn iso_167() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(
		Term::Ground(7),
		Term::Ground(3),
		Term::Ground(6),
		None,
	));
	a.insert(Quad(Term::Var(3), Term::Var(2), Term::Var(1), None));
	a.insert(Quad(Term::Ground(7), Term::Var(4), Term::Ground(4), None));
	a.insert(Quad(Term::Ground(3), Term::Ground(1), Term::Var(0), None));
	a.insert(Quad(Term::Var(4), Term::Ground(8), Term::Ground(7), None));
	a.insert(Quad(Term::Var(7), Term::Ground(8), Term::Ground(6), None));
	a.insert(Quad(Term::Ground(1), Term::Ground(8), Term::Var(7), None));
	a.insert(Quad(Term::Ground(4), Term::Var(4), Term::Ground(2), None));
	a.insert(Quad(Term::Var(3), Term::Ground(4), Term::Ground(5), None));
	a.insert(Quad(Term::Var(1), Term::Ground(9), Term::Var(6), None));
	a.insert(Quad(Term::Var(4), Term::Var(2), Term::Var(1), None));
	a.insert(Quad(Term::Var(9), Term::Var(7), Term::Ground(2), None));
	a.insert(Quad(Term::Var(8), Term::Ground(5), Term::Ground(4), None));
	a.insert(Quad(Term::Ground(5), Term::Var(9), Term::Ground(4), None));
	a.insert(Quad(Term::Var(0), Term::Var(5), Term::Ground(0), None));
	a.insert(Quad(Term::Ground(2), Term::Ground(7), Term::Var(9), None));
	a.insert(Quad(Term::Ground(7), Term::Var(0), Term::Var(7), None));
	a.insert(Quad(Term::Var(2), Term::Ground(0), Term::Var(7), None));
	a.insert(Quad(Term::Var(5), Term::Ground(1), Term::Var(1), None));
	a.insert(Quad(
		Term::Ground(2),
		Term::Ground(1),
		Term::Ground(2),
		None,
	));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(7),
		Term::Ground(3),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Var(1002),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(7),
		Term::Var(1004),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(1),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Ground(8),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Ground(8),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(8),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Var(1004),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Ground(4),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(9),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Var(1002),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Var(1007),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Ground(5),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Var(1009),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Var(1005),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(7),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Ground(7),
		Term::Var(1000),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Ground(0),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Ground(1),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(1),
		Term::Ground(2),
		None,
	));
	test(a, b)
}
#[test]
fn iso_168() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Var(5), Term::Var(7), Term::Ground(4), None));
	a.insert(Quad(
		Term::Ground(8),
		Term::Ground(5),
		Term::Ground(6),
		None,
	));
	a.insert(Quad(Term::Ground(2), Term::Var(8), Term::Ground(8), None));
	a.insert(Quad(Term::Ground(4), Term::Ground(0), Term::Var(0), None));
	a.insert(Quad(Term::Var(7), Term::Ground(4), Term::Var(9), None));
	a.insert(Quad(Term::Ground(9), Term::Var(4), Term::Var(1), None));
	a.insert(Quad(Term::Ground(5), Term::Ground(4), Term::Var(1), None));
	a.insert(Quad(Term::Ground(5), Term::Ground(0), Term::Var(0), None));
	a.insert(Quad(Term::Ground(7), Term::Ground(7), Term::Var(9), None));
	a.insert(Quad(Term::Var(3), Term::Var(3), Term::Ground(3), None));
	a.insert(Quad(
		Term::Ground(1),
		Term::Ground(0),
		Term::Ground(0),
		None,
	));
	a.insert(Quad(Term::Var(4), Term::Var(0), Term::Ground(5), None));
	a.insert(Quad(
		Term::Ground(4),
		Term::Ground(4),
		Term::Ground(5),
		None,
	));
	a.insert(Quad(Term::Var(2), Term::Var(3), Term::Ground(3), None));
	a.insert(Quad(
		Term::Ground(9),
		Term::Ground(4),
		Term::Ground(8),
		None,
	));
	a.insert(Quad(Term::Var(9), Term::Var(1), Term::Var(0), None));
	a.insert(Quad(Term::Var(8), Term::Var(9), Term::Ground(5), None));
	a.insert(Quad(
		Term::Ground(1),
		Term::Ground(3),
		Term::Ground(6),
		None,
	));
	a.insert(Quad(Term::Var(6), Term::Ground(9), Term::Var(1), None));
	a.insert(Quad(Term::Var(5), Term::Ground(0), Term::Var(3), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Var(1005),
		Term::Var(1007),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Ground(5),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Var(1008),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Ground(0),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Ground(4),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Var(1004),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Ground(4),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Ground(0),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Ground(7),
		Term::Ground(7),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Var(1003),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(0),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Var(1000),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Ground(4),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Var(1003),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Ground(4),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Var(1001),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Var(1009),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(3),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Var(1006),
		Term::Ground(9),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Ground(0),
		Term::Var(1003),
		None,
	));
	test(a, b)
}
#[test]
fn iso_169() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Var(6), Term::Ground(1), Term::Var(8), None));
	a.insert(Quad(Term::Var(5), Term::Var(0), Term::Ground(1), None));
	a.insert(Quad(Term::Var(5), Term::Ground(2), Term::Var(1), None));
	a.insert(Quad(Term::Ground(1), Term::Var(5), Term::Ground(7), None));
	a.insert(Quad(Term::Ground(8), Term::Ground(7), Term::Var(1), None));
	a.insert(Quad(Term::Ground(3), Term::Var(2), Term::Ground(8), None));
	a.insert(Quad(Term::Var(0), Term::Ground(5), Term::Var(0), None));
	a.insert(Quad(
		Term::Ground(8),
		Term::Ground(2),
		Term::Ground(4),
		None,
	));
	a.insert(Quad(Term::Var(5), Term::Var(5), Term::Var(0), None));
	a.insert(Quad(Term::Var(3), Term::Ground(5), Term::Var(3), None));
	a.insert(Quad(Term::Ground(8), Term::Ground(3), Term::Var(6), None));
	a.insert(Quad(Term::Ground(5), Term::Var(1), Term::Ground(2), None));
	a.insert(Quad(Term::Ground(6), Term::Ground(2), Term::Var(4), None));
	a.insert(Quad(Term::Var(8), Term::Var(1), Term::Var(4), None));
	a.insert(Quad(Term::Var(1), Term::Var(1), Term::Ground(5), None));
	a.insert(Quad(Term::Var(2), Term::Ground(4), Term::Var(8), None));
	a.insert(Quad(Term::Var(9), Term::Ground(6), Term::Ground(2), None));
	a.insert(Quad(Term::Var(2), Term::Ground(4), Term::Var(0), None));
	a.insert(Quad(Term::Ground(5), Term::Var(2), Term::Ground(8), None));
	a.insert(Quad(
		Term::Ground(1),
		Term::Ground(2),
		Term::Ground(0),
		None,
	));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Var(1006),
		Term::Ground(1),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Var(1000),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Ground(2),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Var(1005),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Ground(7),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Var(1002),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(5),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Ground(2),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Var(1005),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Ground(5),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Ground(3),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Var(1001),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Ground(2),
		Term::Var(1004),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Var(1001),
		Term::Var(1004),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Var(1001),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Ground(4),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Ground(6),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Ground(4),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Var(1002),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(2),
		Term::Ground(0),
		None,
	));
	test(a, b)
}
#[test]
fn iso_170() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Ground(6), Term::Var(8), Term::Var(8), None));
	a.insert(Quad(Term::Var(3), Term::Var(2), Term::Ground(9), None));
	a.insert(Quad(Term::Var(4), Term::Ground(4), Term::Ground(2), None));
	a.insert(Quad(Term::Ground(3), Term::Var(4), Term::Ground(8), None));
	a.insert(Quad(Term::Ground(8), Term::Var(5), Term::Var(9), None));
	a.insert(Quad(Term::Var(5), Term::Ground(3), Term::Var(8), None));
	a.insert(Quad(Term::Ground(9), Term::Ground(8), Term::Var(5), None));
	a.insert(Quad(Term::Var(9), Term::Ground(0), Term::Ground(6), None));
	a.insert(Quad(
		Term::Ground(7),
		Term::Ground(8),
		Term::Ground(0),
		None,
	));
	a.insert(Quad(Term::Var(9), Term::Var(8), Term::Var(8), None));
	a.insert(Quad(Term::Ground(1), Term::Var(4), Term::Var(7), None));
	a.insert(Quad(Term::Var(5), Term::Var(9), Term::Ground(1), None));
	a.insert(Quad(Term::Ground(9), Term::Var(3), Term::Var(6), None));
	a.insert(Quad(Term::Var(2), Term::Ground(0), Term::Var(9), None));
	a.insert(Quad(Term::Var(0), Term::Ground(8), Term::Ground(2), None));
	a.insert(Quad(Term::Ground(9), Term::Ground(5), Term::Var(2), None));
	a.insert(Quad(
		Term::Ground(9),
		Term::Ground(3),
		Term::Ground(4),
		None,
	));
	a.insert(Quad(Term::Ground(4), Term::Var(9), Term::Ground(4), None));
	a.insert(Quad(Term::Var(0), Term::Var(8), Term::Ground(0), None));
	a.insert(Quad(Term::Var(2), Term::Ground(5), Term::Var(8), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(6),
		Term::Var(1008),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Var(1002),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Ground(4),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Var(1004),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Var(1005),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Ground(3),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Ground(8),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Ground(0),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Ground(7),
		Term::Ground(8),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Var(1008),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Var(1004),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Var(1009),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Var(1003),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Ground(0),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(8),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Ground(5),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Ground(3),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Var(1009),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Var(1008),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Ground(5),
		Term::Var(1008),
		None,
	));
	test(a, b)
}
#[test]
fn iso_171() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Var(4), Term::Var(8), Term::Ground(9), None));
	a.insert(Quad(Term::Var(9), Term::Ground(5), Term::Ground(0), None));
	a.insert(Quad(Term::Var(3), Term::Ground(9), Term::Ground(5), None));
	a.insert(Quad(Term::Ground(6), Term::Var(0), Term::Ground(7), None));
	a.insert(Quad(Term::Var(2), Term::Ground(4), Term::Var(7), None));
	a.insert(Quad(Term::Var(2), Term::Var(3), Term::Var(9), None));
	a.insert(Quad(
		Term::Ground(3),
		Term::Ground(8),
		Term::Ground(5),
		None,
	));
	a.insert(Quad(
		Term::Ground(1),
		Term::Ground(6),
		Term::Ground(6),
		None,
	));
	a.insert(Quad(Term::Var(4), Term::Ground(1), Term::Var(8), None));
	a.insert(Quad(Term::Ground(5), Term::Var(8), Term::Var(8), None));
	a.insert(Quad(Term::Var(1), Term::Ground(6), Term::Var(6), None));
	a.insert(Quad(Term::Var(3), Term::Ground(7), Term::Var(8), None));
	a.insert(Quad(Term::Var(6), Term::Var(8), Term::Ground(3), None));
	a.insert(Quad(Term::Ground(3), Term::Var(6), Term::Var(3), None));
	a.insert(Quad(Term::Ground(8), Term::Ground(4), Term::Var(3), None));
	a.insert(Quad(Term::Var(2), Term::Var(9), Term::Var(2), None));
	a.insert(Quad(Term::Ground(5), Term::Ground(0), Term::Var(8), None));
	a.insert(Quad(Term::Ground(1), Term::Var(2), Term::Ground(0), None));
	a.insert(Quad(Term::Var(1), Term::Ground(5), Term::Var(8), None));
	a.insert(Quad(Term::Ground(4), Term::Var(9), Term::Var(1), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Var(1004),
		Term::Var(1008),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Ground(5),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Ground(9),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Var(1000),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Ground(4),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Var(1003),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(8),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(6),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Ground(1),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Var(1008),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(6),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Ground(7),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Var(1006),
		Term::Var(1008),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Var(1006),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Ground(4),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Var(1009),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Ground(0),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Var(1002),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(5),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Var(1009),
		Term::Var(1001),
		None,
	));
	test(a, b)
}
#[test]
fn iso_172() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Ground(6), Term::Var(2), Term::Ground(8), None));
	a.insert(Quad(Term::Ground(3), Term::Var(9), Term::Ground(0), None));
	a.insert(Quad(Term::Ground(5), Term::Var(6), Term::Var(6), None));
	a.insert(Quad(Term::Var(9), Term::Var(6), Term::Var(8), None));
	a.insert(Quad(Term::Var(0), Term::Ground(2), Term::Ground(2), None));
	a.insert(Quad(Term::Ground(8), Term::Var(4), Term::Var(2), None));
	a.insert(Quad(Term::Var(7), Term::Var(7), Term::Ground(5), None));
	a.insert(Quad(Term::Var(5), Term::Var(5), Term::Ground(4), None));
	a.insert(Quad(Term::Var(1), Term::Var(6), Term::Ground(0), None));
	a.insert(Quad(Term::Var(0), Term::Ground(4), Term::Ground(0), None));
	a.insert(Quad(Term::Var(6), Term::Var(3), Term::Ground(7), None));
	a.insert(Quad(Term::Ground(8), Term::Var(3), Term::Ground(9), None));
	a.insert(Quad(
		Term::Ground(5),
		Term::Ground(0),
		Term::Ground(9),
		None,
	));
	a.insert(Quad(Term::Var(0), Term::Var(9), Term::Var(9), None));
	a.insert(Quad(Term::Var(2), Term::Ground(9), Term::Var(1), None));
	a.insert(Quad(Term::Var(4), Term::Var(6), Term::Ground(8), None));
	a.insert(Quad(Term::Var(5), Term::Ground(3), Term::Ground(7), None));
	a.insert(Quad(Term::Ground(2), Term::Ground(7), Term::Var(6), None));
	a.insert(Quad(Term::Var(3), Term::Var(3), Term::Var(5), None));
	a.insert(Quad(Term::Var(1), Term::Ground(0), Term::Ground(4), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(6),
		Term::Var(1002),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Var(1009),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Var(1006),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Var(1006),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(2),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Var(1004),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Var(1007),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Var(1005),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Var(1006),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(4),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Var(1006),
		Term::Var(1003),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Var(1003),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Ground(0),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Var(1009),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Ground(9),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Var(1006),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Ground(3),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(7),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Var(1003),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(0),
		Term::Ground(4),
		None,
	));
	test(a, b)
}
#[test]
fn iso_173() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Ground(2), Term::Var(4), Term::Var(6), None));
	a.insert(Quad(
		Term::Ground(2),
		Term::Ground(4),
		Term::Ground(4),
		None,
	));
	a.insert(Quad(
		Term::Ground(3),
		Term::Ground(8),
		Term::Ground(4),
		None,
	));
	a.insert(Quad(Term::Ground(1), Term::Ground(0), Term::Var(4), None));
	a.insert(Quad(Term::Var(9), Term::Ground(9), Term::Var(5), None));
	a.insert(Quad(Term::Ground(1), Term::Var(3), Term::Ground(3), None));
	a.insert(Quad(Term::Var(2), Term::Ground(2), Term::Ground(7), None));
	a.insert(Quad(Term::Var(0), Term::Var(7), Term::Var(7), None));
	a.insert(Quad(Term::Var(9), Term::Ground(1), Term::Ground(2), None));
	a.insert(Quad(Term::Var(7), Term::Ground(3), Term::Var(0), None));
	a.insert(Quad(Term::Var(4), Term::Var(6), Term::Ground(0), None));
	a.insert(Quad(Term::Ground(6), Term::Var(8), Term::Ground(9), None));
	a.insert(Quad(Term::Var(4), Term::Ground(0), Term::Var(1), None));
	a.insert(Quad(Term::Ground(1), Term::Ground(3), Term::Var(0), None));
	a.insert(Quad(Term::Var(8), Term::Ground(1), Term::Var(2), None));
	a.insert(Quad(Term::Var(5), Term::Var(6), Term::Ground(0), None));
	a.insert(Quad(Term::Var(6), Term::Var(0), Term::Ground(6), None));
	a.insert(Quad(
		Term::Ground(0),
		Term::Ground(6),
		Term::Ground(3),
		None,
	));
	a.insert(Quad(Term::Ground(5), Term::Ground(2), Term::Var(2), None));
	a.insert(Quad(Term::Var(4), Term::Var(2), Term::Ground(0), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(2),
		Term::Var(1004),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(4),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(8),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(0),
		Term::Var(1004),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Ground(9),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Var(1003),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Ground(2),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Var(1007),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Ground(1),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Ground(3),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Var(1006),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Var(1008),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Ground(0),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(3),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Ground(1),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Var(1006),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Var(1006),
		Term::Var(1000),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(6),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Ground(2),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Var(1002),
		Term::Ground(0),
		None,
	));
	test(a, b)
}
#[test]
fn iso_174() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(
		Term::Ground(7),
		Term::Ground(6),
		Term::Ground(1),
		None,
	));
	a.insert(Quad(Term::Var(4), Term::Ground(1), Term::Var(7), None));
	a.insert(Quad(
		Term::Ground(8),
		Term::Ground(1),
		Term::Ground(9),
		None,
	));
	a.insert(Quad(Term::Ground(6), Term::Var(2), Term::Var(2), None));
	a.insert(Quad(Term::Ground(6), Term::Ground(0), Term::Var(3), None));
	a.insert(Quad(Term::Ground(6), Term::Var(0), Term::Ground(0), None));
	a.insert(Quad(Term::Var(6), Term::Var(3), Term::Ground(3), None));
	a.insert(Quad(Term::Ground(9), Term::Var(6), Term::Ground(4), None));
	a.insert(Quad(Term::Ground(8), Term::Var(6), Term::Var(1), None));
	a.insert(Quad(
		Term::Ground(2),
		Term::Ground(2),
		Term::Ground(2),
		None,
	));
	a.insert(Quad(Term::Ground(1), Term::Var(6), Term::Ground(5), None));
	a.insert(Quad(Term::Ground(7), Term::Var(6), Term::Var(2), None));
	a.insert(Quad(Term::Ground(0), Term::Var(4), Term::Var(8), None));
	a.insert(Quad(
		Term::Ground(8),
		Term::Ground(0),
		Term::Ground(0),
		None,
	));
	a.insert(Quad(Term::Var(1), Term::Var(9), Term::Var(8), None));
	a.insert(Quad(Term::Var(2), Term::Var(4), Term::Ground(2), None));
	a.insert(Quad(Term::Ground(9), Term::Var(5), Term::Ground(2), None));
	a.insert(Quad(Term::Var(0), Term::Var(3), Term::Ground(3), None));
	a.insert(Quad(
		Term::Ground(0),
		Term::Ground(2),
		Term::Ground(4),
		None,
	));
	a.insert(Quad(Term::Ground(0), Term::Ground(3), Term::Var(0), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(7),
		Term::Ground(6),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Ground(1),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Ground(1),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Var(1002),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Ground(0),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Var(1000),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Var(1006),
		Term::Var(1003),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Var(1006),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Var(1006),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(2),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Var(1006),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Ground(7),
		Term::Var(1006),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Var(1004),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Ground(0),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Var(1009),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Var(1004),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Var(1005),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Var(1003),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(2),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(3),
		Term::Var(1000),
		None,
	));
	test(a, b)
}
#[test]
fn iso_175() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Var(2), Term::Ground(5), Term::Ground(7), None));
	a.insert(Quad(Term::Var(9), Term::Var(1), Term::Ground(4), None));
	a.insert(Quad(Term::Var(0), Term::Ground(2), Term::Ground(1), None));
	a.insert(Quad(Term::Var(5), Term::Ground(2), Term::Var(1), None));
	a.insert(Quad(
		Term::Ground(2),
		Term::Ground(0),
		Term::Ground(3),
		None,
	));
	a.insert(Quad(Term::Ground(9), Term::Var(5), Term::Ground(0), None));
	a.insert(Quad(Term::Var(7), Term::Ground(4), Term::Var(2), None));
	a.insert(Quad(Term::Ground(5), Term::Var(4), Term::Var(0), None));
	a.insert(Quad(Term::Var(4), Term::Ground(9), Term::Var(3), None));
	a.insert(Quad(Term::Var(3), Term::Ground(8), Term::Ground(5), None));
	a.insert(Quad(Term::Ground(9), Term::Var(1), Term::Ground(7), None));
	a.insert(Quad(Term::Var(2), Term::Var(6), Term::Ground(9), None));
	a.insert(Quad(Term::Ground(4), Term::Var(1), Term::Var(9), None));
	a.insert(Quad(Term::Var(7), Term::Ground(1), Term::Ground(3), None));
	a.insert(Quad(Term::Var(7), Term::Ground(4), Term::Ground(5), None));
	a.insert(Quad(
		Term::Ground(3),
		Term::Ground(5),
		Term::Ground(1),
		None,
	));
	a.insert(Quad(Term::Var(4), Term::Var(3), Term::Ground(8), None));
	a.insert(Quad(Term::Var(5), Term::Var(6), Term::Var(9), None));
	a.insert(Quad(
		Term::Ground(9),
		Term::Ground(7),
		Term::Ground(9),
		None,
	));
	a.insert(Quad(Term::Ground(1), Term::Ground(8), Term::Var(3), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Var(1002),
		Term::Ground(5),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Var(1001),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(2),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Ground(2),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(0),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Var(1005),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Ground(4),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Var(1004),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Ground(9),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Ground(8),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Var(1001),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Var(1006),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Var(1001),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Ground(1),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Ground(4),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(5),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Var(1003),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Var(1006),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Ground(7),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(8),
		Term::Var(1003),
		None,
	));
	test(a, b)
}
#[test]
fn iso_176() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Var(2), Term::Ground(5), Term::Ground(5), None));
	a.insert(Quad(Term::Ground(7), Term::Var(5), Term::Ground(0), None));
	a.insert(Quad(
		Term::Ground(1),
		Term::Ground(3),
		Term::Ground(4),
		None,
	));
	a.insert(Quad(Term::Ground(3), Term::Ground(6), Term::Var(5), None));
	a.insert(Quad(Term::Ground(1), Term::Ground(0), Term::Var(4), None));
	a.insert(Quad(Term::Var(5), Term::Var(9), Term::Ground(6), None));
	a.insert(Quad(
		Term::Ground(8),
		Term::Ground(4),
		Term::Ground(4),
		None,
	));
	a.insert(Quad(Term::Var(3), Term::Ground(1), Term::Ground(3), None));
	a.insert(Quad(
		Term::Ground(2),
		Term::Ground(7),
		Term::Ground(5),
		None,
	));
	a.insert(Quad(Term::Var(8), Term::Var(2), Term::Var(6), None));
	a.insert(Quad(Term::Ground(6), Term::Var(0), Term::Var(0), None));
	a.insert(Quad(Term::Var(6), Term::Ground(9), Term::Ground(4), None));
	a.insert(Quad(Term::Ground(6), Term::Ground(1), Term::Var(4), None));
	a.insert(Quad(Term::Ground(9), Term::Var(0), Term::Var(8), None));
	a.insert(Quad(
		Term::Ground(9),
		Term::Ground(5),
		Term::Ground(0),
		None,
	));
	a.insert(Quad(Term::Var(0), Term::Ground(0), Term::Var(0), None));
	a.insert(Quad(
		Term::Ground(5),
		Term::Ground(8),
		Term::Ground(9),
		None,
	));
	a.insert(Quad(Term::Ground(1), Term::Ground(6), Term::Var(7), None));
	a.insert(Quad(Term::Var(3), Term::Var(6), Term::Var(5), None));
	a.insert(Quad(Term::Ground(9), Term::Var(8), Term::Var(8), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Var(1002),
		Term::Ground(5),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Ground(7),
		Term::Var(1005),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(3),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(6),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(0),
		Term::Var(1004),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Var(1009),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Ground(4),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Ground(1),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(7),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Var(1002),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Var(1000),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Var(1006),
		Term::Ground(9),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Ground(1),
		Term::Var(1004),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Var(1000),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Ground(5),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(0),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Ground(8),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(6),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Var(1006),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Var(1008),
		Term::Var(1008),
		None,
	));
	test(a, b)
}
#[test]
fn iso_177() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Var(9), Term::Var(4), Term::Ground(6), None));
	a.insert(Quad(Term::Ground(6), Term::Var(4), Term::Ground(1), None));
	a.insert(Quad(Term::Var(1), Term::Ground(4), Term::Ground(8), None));
	a.insert(Quad(Term::Var(5), Term::Var(5), Term::Var(2), None));
	a.insert(Quad(Term::Var(6), Term::Ground(4), Term::Ground(8), None));
	a.insert(Quad(Term::Ground(2), Term::Ground(8), Term::Var(2), None));
	a.insert(Quad(Term::Ground(0), Term::Var(6), Term::Ground(6), None));
	a.insert(Quad(Term::Var(2), Term::Var(9), Term::Ground(1), None));
	a.insert(Quad(Term::Var(0), Term::Ground(3), Term::Var(1), None));
	a.insert(Quad(Term::Ground(1), Term::Ground(4), Term::Var(3), None));
	a.insert(Quad(Term::Ground(1), Term::Var(0), Term::Var(4), None));
	a.insert(Quad(Term::Var(8), Term::Ground(4), Term::Ground(6), None));
	a.insert(Quad(Term::Ground(9), Term::Ground(6), Term::Var(7), None));
	a.insert(Quad(Term::Var(4), Term::Ground(1), Term::Var(4), None));
	a.insert(Quad(Term::Var(4), Term::Var(5), Term::Ground(0), None));
	a.insert(Quad(Term::Var(8), Term::Var(8), Term::Var(5), None));
	a.insert(Quad(Term::Ground(6), Term::Var(5), Term::Ground(6), None));
	a.insert(Quad(Term::Ground(3), Term::Var(7), Term::Var(3), None));
	a.insert(Quad(Term::Ground(2), Term::Var(8), Term::Var(6), None));
	a.insert(Quad(Term::Var(8), Term::Var(3), Term::Var(3), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Var(1009),
		Term::Var(1004),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Var(1004),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(4),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Var(1005),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Var(1006),
		Term::Ground(4),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(8),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Var(1006),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Var(1009),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(3),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(4),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Var(1000),
		Term::Var(1004),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Ground(4),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Ground(6),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Ground(1),
		Term::Var(1004),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Var(1005),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Var(1008),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Var(1005),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Var(1007),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Var(1008),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Var(1003),
		Term::Var(1003),
		None,
	));
	test(a, b)
}
#[test]
fn iso_178() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Var(8), Term::Var(1), Term::Var(3), None));
	a.insert(Quad(Term::Var(2), Term::Var(2), Term::Var(4), None));
	a.insert(Quad(Term::Ground(8), Term::Var(0), Term::Ground(7), None));
	a.insert(Quad(Term::Ground(3), Term::Var(7), Term::Var(7), None));
	a.insert(Quad(Term::Var(6), Term::Ground(3), Term::Var(2), None));
	a.insert(Quad(Term::Var(6), Term::Ground(3), Term::Ground(8), None));
	a.insert(Quad(Term::Ground(8), Term::Ground(2), Term::Var(1), None));
	a.insert(Quad(Term::Ground(4), Term::Var(8), Term::Var(2), None));
	a.insert(Quad(Term::Ground(3), Term::Ground(2), Term::Var(3), None));
	a.insert(Quad(
		Term::Ground(6),
		Term::Ground(8),
		Term::Ground(3),
		None,
	));
	a.insert(Quad(Term::Ground(4), Term::Var(4), Term::Ground(5), None));
	a.insert(Quad(Term::Var(1), Term::Var(2), Term::Ground(1), None));
	a.insert(Quad(Term::Ground(1), Term::Var(2), Term::Ground(5), None));
	a.insert(Quad(Term::Ground(7), Term::Var(0), Term::Ground(2), None));
	a.insert(Quad(Term::Ground(3), Term::Var(5), Term::Ground(1), None));
	a.insert(Quad(Term::Ground(3), Term::Ground(0), Term::Var(3), None));
	a.insert(Quad(Term::Ground(7), Term::Ground(8), Term::Var(3), None));
	a.insert(Quad(Term::Ground(2), Term::Var(8), Term::Var(6), None));
	a.insert(Quad(Term::Var(8), Term::Ground(6), Term::Var(0), None));
	a.insert(Quad(Term::Var(5), Term::Ground(5), Term::Ground(3), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Var(1008),
		Term::Var(1001),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Var(1002),
		Term::Var(1004),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Var(1000),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Var(1007),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Var(1006),
		Term::Ground(3),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Var(1006),
		Term::Ground(3),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Ground(2),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Var(1008),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(2),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Ground(8),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Var(1004),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Var(1002),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Var(1002),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Ground(7),
		Term::Var(1000),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Var(1005),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(0),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Ground(7),
		Term::Ground(8),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Var(1008),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Ground(6),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Ground(5),
		Term::Ground(3),
		None,
	));
	test(a, b)
}
#[test]
fn iso_179() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(
		Term::Ground(1),
		Term::Ground(0),
		Term::Ground(8),
		None,
	));
	a.insert(Quad(Term::Var(8), Term::Ground(1), Term::Ground(4), None));
	a.insert(Quad(Term::Ground(4), Term::Ground(6), Term::Var(1), None));
	a.insert(Quad(Term::Ground(3), Term::Var(2), Term::Var(9), None));
	a.insert(Quad(Term::Var(1), Term::Ground(0), Term::Var(0), None));
	a.insert(Quad(Term::Var(5), Term::Ground(3), Term::Var(4), None));
	a.insert(Quad(Term::Var(6), Term::Ground(3), Term::Ground(3), None));
	a.insert(Quad(Term::Var(1), Term::Ground(7), Term::Ground(0), None));
	a.insert(Quad(Term::Var(3), Term::Ground(8), Term::Var(1), None));
	a.insert(Quad(Term::Var(9), Term::Var(6), Term::Ground(6), None));
	a.insert(Quad(Term::Ground(3), Term::Var(4), Term::Var(0), None));
	a.insert(Quad(Term::Ground(7), Term::Ground(3), Term::Var(2), None));
	a.insert(Quad(Term::Var(2), Term::Ground(5), Term::Var(8), None));
	a.insert(Quad(Term::Ground(0), Term::Var(3), Term::Var(8), None));
	a.insert(Quad(Term::Var(0), Term::Ground(5), Term::Ground(6), None));
	a.insert(Quad(Term::Ground(0), Term::Var(2), Term::Ground(8), None));
	a.insert(Quad(Term::Var(7), Term::Var(2), Term::Ground(3), None));
	a.insert(Quad(Term::Var(4), Term::Ground(0), Term::Ground(2), None));
	a.insert(Quad(Term::Var(9), Term::Ground(5), Term::Var(5), None));
	a.insert(Quad(
		Term::Ground(1),
		Term::Ground(7),
		Term::Ground(5),
		None,
	));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(0),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Ground(1),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Ground(6),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Var(1002),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(0),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Ground(3),
		Term::Var(1004),
		None,
	));
	b.insert(Quad(
		Term::Var(1006),
		Term::Ground(3),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(7),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Ground(8),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Var(1006),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Var(1004),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Ground(7),
		Term::Ground(3),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Ground(5),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Var(1003),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(5),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Var(1002),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Var(1002),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Ground(0),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Ground(5),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(7),
		Term::Ground(5),
		None,
	));
	test(a, b)
}
#[test]
fn iso_180() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Ground(5), Term::Var(8), Term::Ground(4), None));
	a.insert(Quad(Term::Var(4), Term::Ground(7), Term::Ground(6), None));
	a.insert(Quad(Term::Ground(6), Term::Var(9), Term::Var(3), None));
	a.insert(Quad(Term::Var(5), Term::Var(7), Term::Var(6), None));
	a.insert(Quad(Term::Var(8), Term::Var(5), Term::Var(1), None));
	a.insert(Quad(Term::Ground(5), Term::Ground(0), Term::Var(1), None));
	a.insert(Quad(Term::Ground(7), Term::Var(8), Term::Var(4), None));
	a.insert(Quad(Term::Var(7), Term::Ground(3), Term::Var(4), None));
	a.insert(Quad(Term::Var(5), Term::Var(6), Term::Var(5), None));
	a.insert(Quad(Term::Var(9), Term::Ground(1), Term::Var(6), None));
	a.insert(Quad(Term::Var(0), Term::Ground(0), Term::Var(7), None));
	a.insert(Quad(Term::Ground(3), Term::Var(3), Term::Var(9), None));
	a.insert(Quad(Term::Var(5), Term::Ground(7), Term::Ground(1), None));
	a.insert(Quad(Term::Ground(8), Term::Ground(8), Term::Var(9), None));
	a.insert(Quad(Term::Var(6), Term::Ground(0), Term::Var(1), None));
	a.insert(Quad(Term::Var(3), Term::Ground(5), Term::Ground(4), None));
	a.insert(Quad(Term::Var(2), Term::Ground(6), Term::Var(3), None));
	a.insert(Quad(Term::Var(2), Term::Var(0), Term::Var(5), None));
	a.insert(Quad(
		Term::Ground(1),
		Term::Ground(9),
		Term::Ground(1),
		None,
	));
	a.insert(Quad(Term::Ground(8), Term::Var(8), Term::Var(5), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(5),
		Term::Var(1008),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Ground(7),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Var(1009),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Var(1007),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Var(1005),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Ground(0),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(7),
		Term::Var(1008),
		Term::Var(1004),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Ground(3),
		Term::Var(1004),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Var(1006),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Ground(1),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(0),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Var(1003),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Ground(7),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Ground(8),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Var(1006),
		Term::Ground(0),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Ground(5),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Ground(6),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Var(1000),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(9),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Var(1008),
		Term::Var(1005),
		None,
	));
	test(a, b)
}
#[test]
fn iso_181() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Var(2), Term::Var(3), Term::Var(10), None));
	a.insert(Quad(Term::Ground(25), Term::Var(19), Term::Var(19), None));
	a.insert(Quad(
		Term::Var(29),
		Term::Ground(36),
		Term::Ground(13),
		None,
	));
	a.insert(Quad(
		Term::Ground(21),
		Term::Ground(33),
		Term::Ground(11),
		None,
	));
	a.insert(Quad(
		Term::Ground(27),
		Term::Ground(35),
		Term::Ground(49),
		None,
	));
	a.insert(Quad(
		Term::Ground(30),
		Term::Ground(39),
		Term::Var(41),
		None,
	));
	a.insert(Quad(Term::Var(47), Term::Ground(28), Term::Var(47), None));
	a.insert(Quad(
		Term::Ground(32),
		Term::Ground(40),
		Term::Ground(12),
		None,
	));
	a.insert(Quad(
		Term::Var(43),
		Term::Ground(37),
		Term::Ground(25),
		None,
	));
	a.insert(Quad(Term::Ground(40), Term::Var(0), Term::Var(43), None));
	a.insert(Quad(
		Term::Ground(34),
		Term::Ground(29),
		Term::Ground(32),
		None,
	));
	a.insert(Quad(
		Term::Ground(20),
		Term::Var(49),
		Term::Ground(21),
		None,
	));
	a.insert(Quad(Term::Ground(25), Term::Var(44), Term::Ground(4), None));
	a.insert(Quad(
		Term::Ground(22),
		Term::Var(35),
		Term::Ground(42),
		None,
	));
	a.insert(Quad(Term::Var(16), Term::Var(27), Term::Ground(8), None));
	a.insert(Quad(
		Term::Ground(44),
		Term::Var(19),
		Term::Ground(14),
		None,
	));
	a.insert(Quad(
		Term::Ground(10),
		Term::Ground(45),
		Term::Var(24),
		None,
	));
	a.insert(Quad(
		Term::Ground(11),
		Term::Ground(40),
		Term::Ground(42),
		None,
	));
	a.insert(Quad(Term::Var(46), Term::Var(49), Term::Ground(37), None));
	a.insert(Quad(Term::Var(44), Term::Ground(40), Term::Var(27), None));
	a.insert(Quad(Term::Var(13), Term::Var(30), Term::Ground(15), None));
	a.insert(Quad(
		Term::Ground(48),
		Term::Var(29),
		Term::Ground(31),
		None,
	));
	a.insert(Quad(Term::Var(10), Term::Var(31), Term::Var(38), None));
	a.insert(Quad(Term::Var(45), Term::Ground(24), Term::Var(19), None));
	a.insert(Quad(Term::Var(36), Term::Ground(41), Term::Var(24), None));
	a.insert(Quad(Term::Var(14), Term::Var(23), Term::Ground(44), None));
	a.insert(Quad(
		Term::Ground(15),
		Term::Var(30),
		Term::Ground(29),
		None,
	));
	a.insert(Quad(Term::Var(26), Term::Ground(23), Term::Ground(2), None));
	a.insert(Quad(Term::Var(4), Term::Var(32), Term::Var(20), None));
	a.insert(Quad(Term::Var(48), Term::Var(19), Term::Var(46), None));
	a.insert(Quad(Term::Var(2), Term::Ground(47), Term::Ground(23), None));
	a.insert(Quad(Term::Var(2), Term::Var(46), Term::Ground(2), None));
	a.insert(Quad(Term::Var(28), Term::Var(46), Term::Ground(12), None));
	a.insert(Quad(
		Term::Var(31),
		Term::Ground(28),
		Term::Ground(18),
		None,
	));
	a.insert(Quad(Term::Var(31), Term::Ground(16), Term::Var(34), None));
	a.insert(Quad(
		Term::Ground(24),
		Term::Ground(25),
		Term::Ground(37),
		None,
	));
	a.insert(Quad(Term::Ground(12), Term::Var(20), Term::Var(45), None));
	a.insert(Quad(Term::Var(10), Term::Var(4), Term::Var(15), None));
	a.insert(Quad(Term::Var(31), Term::Var(32), Term::Var(2), None));
	a.insert(Quad(
		Term::Ground(28),
		Term::Var(49),
		Term::Ground(38),
		None,
	));
	a.insert(Quad(Term::Var(49), Term::Ground(7), Term::Ground(29), None));
	a.insert(Quad(Term::Var(49), Term::Ground(43), Term::Var(5), None));
	a.insert(Quad(Term::Ground(32), Term::Var(32), Term::Ground(2), None));
	a.insert(Quad(Term::Ground(1), Term::Var(39), Term::Var(17), None));
	a.insert(Quad(Term::Var(10), Term::Ground(1), Term::Ground(1), None));
	a.insert(Quad(
		Term::Var(24),
		Term::Ground(29),
		Term::Ground(38),
		None,
	));
	a.insert(Quad(
		Term::Var(24),
		Term::Ground(15),
		Term::Ground(35),
		None,
	));
	a.insert(Quad(Term::Var(20), Term::Ground(42), Term::Var(30), None));
	a.insert(Quad(Term::Var(3), Term::Ground(10), Term::Var(46), None));
	a.insert(Quad(
		Term::Ground(25),
		Term::Ground(11),
		Term::Var(28),
		None,
	));
	a.insert(Quad(Term::Var(5), Term::Ground(26), Term::Var(15), None));
	a.insert(Quad(Term::Var(31), Term::Var(25), Term::Ground(46), None));
	a.insert(Quad(Term::Var(36), Term::Var(36), Term::Ground(34), None));
	a.insert(Quad(Term::Var(3), Term::Ground(13), Term::Ground(49), None));
	a.insert(Quad(Term::Var(18), Term::Ground(6), Term::Ground(39), None));
	a.insert(Quad(Term::Ground(10), Term::Var(40), Term::Ground(3), None));
	a.insert(Quad(
		Term::Ground(0),
		Term::Ground(3),
		Term::Ground(48),
		None,
	));
	a.insert(Quad(Term::Var(5), Term::Ground(8), Term::Ground(44), None));
	a.insert(Quad(Term::Ground(45), Term::Var(6), Term::Var(7), None));
	a.insert(Quad(
		Term::Ground(17),
		Term::Ground(45),
		Term::Ground(10),
		None,
	));
	a.insert(Quad(Term::Var(24), Term::Ground(4), Term::Var(8), None));
	a.insert(Quad(Term::Var(12), Term::Ground(34), Term::Var(28), None));
	a.insert(Quad(
		Term::Ground(41),
		Term::Var(24),
		Term::Ground(20),
		None,
	));
	a.insert(Quad(Term::Var(12), Term::Ground(8), Term::Ground(30), None));
	a.insert(Quad(Term::Ground(30), Term::Var(28), Term::Ground(7), None));
	a.insert(Quad(Term::Var(13), Term::Var(3), Term::Var(45), None));
	a.insert(Quad(Term::Var(44), Term::Var(15), Term::Ground(44), None));
	a.insert(Quad(Term::Var(38), Term::Var(40), Term::Ground(36), None));
	a.insert(Quad(
		Term::Ground(15),
		Term::Ground(45),
		Term::Ground(6),
		None,
	));
	a.insert(Quad(
		Term::Ground(37),
		Term::Ground(35),
		Term::Var(37),
		None,
	));
	a.insert(Quad(Term::Ground(5), Term::Ground(28), Term::Var(8), None));
	a.insert(Quad(Term::Ground(10), Term::Var(2), Term::Var(31), None));
	a.insert(Quad(
		Term::Var(18),
		Term::Ground(42),
		Term::Ground(40),
		None,
	));
	a.insert(Quad(Term::Ground(37), Term::Var(42), Term::Var(32), None));
	a.insert(Quad(Term::Var(2), Term::Var(0), Term::Var(16), None));
	a.insert(Quad(Term::Ground(4), Term::Var(5), Term::Var(5), None));
	a.insert(Quad(Term::Ground(1), Term::Var(26), Term::Ground(2), None));
	a.insert(Quad(Term::Var(30), Term::Var(37), Term::Ground(18), None));
	a.insert(Quad(Term::Ground(14), Term::Ground(14), Term::Var(3), None));
	a.insert(Quad(
		Term::Ground(41),
		Term::Ground(37),
		Term::Var(49),
		None,
	));
	a.insert(Quad(Term::Var(0), Term::Ground(15), Term::Var(16), None));
	a.insert(Quad(Term::Var(1), Term::Ground(0), Term::Var(0), None));
	a.insert(Quad(Term::Var(1), Term::Var(8), Term::Var(27), None));
	a.insert(Quad(Term::Var(23), Term::Ground(44), Term::Ground(0), None));
	a.insert(Quad(
		Term::Ground(48),
		Term::Ground(1),
		Term::Ground(29),
		None,
	));
	a.insert(Quad(
		Term::Ground(43),
		Term::Ground(4),
		Term::Ground(10),
		None,
	));
	a.insert(Quad(
		Term::Ground(40),
		Term::Ground(7),
		Term::Ground(42),
		None,
	));
	a.insert(Quad(Term::Var(5), Term::Ground(2), Term::Var(37), None));
	a.insert(Quad(Term::Var(27), Term::Ground(48), Term::Var(14), None));
	a.insert(Quad(Term::Var(43), Term::Var(42), Term::Var(49), None));
	a.insert(Quad(
		Term::Var(14),
		Term::Ground(24),
		Term::Ground(33),
		None,
	));
	a.insert(Quad(Term::Var(9), Term::Var(2), Term::Var(22), None));
	a.insert(Quad(Term::Var(42), Term::Var(41), Term::Ground(22), None));
	a.insert(Quad(Term::Var(20), Term::Var(19), Term::Ground(8), None));
	a.insert(Quad(Term::Var(15), Term::Var(33), Term::Ground(34), None));
	a.insert(Quad(Term::Ground(30), Term::Var(29), Term::Ground(8), None));
	a.insert(Quad(Term::Ground(41), Term::Var(21), Term::Ground(7), None));
	a.insert(Quad(Term::Var(27), Term::Var(46), Term::Var(27), None));
	a.insert(Quad(Term::Var(0), Term::Ground(39), Term::Ground(25), None));
	a.insert(Quad(Term::Var(4), Term::Var(10), Term::Var(13), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Var(1002),
		Term::Var(1003),
		Term::Var(1010),
		None,
	));
	b.insert(Quad(
		Term::Ground(25),
		Term::Var(1019),
		Term::Var(1019),
		None,
	));
	b.insert(Quad(
		Term::Var(1029),
		Term::Ground(36),
		Term::Ground(13),
		None,
	));
	b.insert(Quad(
		Term::Ground(21),
		Term::Ground(33),
		Term::Ground(11),
		None,
	));
	b.insert(Quad(
		Term::Ground(27),
		Term::Ground(35),
		Term::Ground(49),
		None,
	));
	b.insert(Quad(
		Term::Ground(30),
		Term::Ground(39),
		Term::Var(1041),
		None,
	));
	b.insert(Quad(
		Term::Var(1047),
		Term::Ground(28),
		Term::Var(1047),
		None,
	));
	b.insert(Quad(
		Term::Ground(32),
		Term::Ground(40),
		Term::Ground(12),
		None,
	));
	b.insert(Quad(
		Term::Var(1043),
		Term::Ground(37),
		Term::Ground(25),
		None,
	));
	b.insert(Quad(
		Term::Ground(40),
		Term::Var(1000),
		Term::Var(1043),
		None,
	));
	b.insert(Quad(
		Term::Ground(34),
		Term::Ground(29),
		Term::Ground(32),
		None,
	));
	b.insert(Quad(
		Term::Ground(20),
		Term::Var(1049),
		Term::Ground(21),
		None,
	));
	b.insert(Quad(
		Term::Ground(25),
		Term::Var(1044),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Ground(22),
		Term::Var(1035),
		Term::Ground(42),
		None,
	));
	b.insert(Quad(
		Term::Var(1016),
		Term::Var(1027),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Ground(44),
		Term::Var(1019),
		Term::Ground(14),
		None,
	));
	b.insert(Quad(
		Term::Ground(10),
		Term::Ground(45),
		Term::Var(1024),
		None,
	));
	b.insert(Quad(
		Term::Ground(11),
		Term::Ground(40),
		Term::Ground(42),
		None,
	));
	b.insert(Quad(
		Term::Var(1046),
		Term::Var(1049),
		Term::Ground(37),
		None,
	));
	b.insert(Quad(
		Term::Var(1044),
		Term::Ground(40),
		Term::Var(1027),
		None,
	));
	b.insert(Quad(
		Term::Var(1013),
		Term::Var(1030),
		Term::Ground(15),
		None,
	));
	b.insert(Quad(
		Term::Ground(48),
		Term::Var(1029),
		Term::Ground(31),
		None,
	));
	b.insert(Quad(
		Term::Var(1010),
		Term::Var(1031),
		Term::Var(1038),
		None,
	));
	b.insert(Quad(
		Term::Var(1045),
		Term::Ground(24),
		Term::Var(1019),
		None,
	));
	b.insert(Quad(
		Term::Var(1036),
		Term::Ground(41),
		Term::Var(1024),
		None,
	));
	b.insert(Quad(
		Term::Var(1014),
		Term::Var(1023),
		Term::Ground(44),
		None,
	));
	b.insert(Quad(
		Term::Ground(15),
		Term::Var(1030),
		Term::Ground(29),
		None,
	));
	b.insert(Quad(
		Term::Var(1026),
		Term::Ground(23),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Var(1032),
		Term::Var(1020),
		None,
	));
	b.insert(Quad(
		Term::Var(1048),
		Term::Var(1019),
		Term::Var(1046),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Ground(47),
		Term::Ground(23),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Var(1046),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Var(1028),
		Term::Var(1046),
		Term::Ground(12),
		None,
	));
	b.insert(Quad(
		Term::Var(1031),
		Term::Ground(28),
		Term::Ground(18),
		None,
	));
	b.insert(Quad(
		Term::Var(1031),
		Term::Ground(16),
		Term::Var(1034),
		None,
	));
	b.insert(Quad(
		Term::Ground(24),
		Term::Ground(25),
		Term::Ground(37),
		None,
	));
	b.insert(Quad(
		Term::Ground(12),
		Term::Var(1020),
		Term::Var(1045),
		None,
	));
	b.insert(Quad(
		Term::Var(1010),
		Term::Var(1004),
		Term::Var(1015),
		None,
	));
	b.insert(Quad(
		Term::Var(1031),
		Term::Var(1032),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Ground(28),
		Term::Var(1049),
		Term::Ground(38),
		None,
	));
	b.insert(Quad(
		Term::Var(1049),
		Term::Ground(7),
		Term::Ground(29),
		None,
	));
	b.insert(Quad(
		Term::Var(1049),
		Term::Ground(43),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Ground(32),
		Term::Var(1032),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Var(1039),
		Term::Var(1017),
		None,
	));
	b.insert(Quad(
		Term::Var(1010),
		Term::Ground(1),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Var(1024),
		Term::Ground(29),
		Term::Ground(38),
		None,
	));
	b.insert(Quad(
		Term::Var(1024),
		Term::Ground(15),
		Term::Ground(35),
		None,
	));
	b.insert(Quad(
		Term::Var(1020),
		Term::Ground(42),
		Term::Var(1030),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Ground(10),
		Term::Var(1046),
		None,
	));
	b.insert(Quad(
		Term::Ground(25),
		Term::Ground(11),
		Term::Var(1028),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Ground(26),
		Term::Var(1015),
		None,
	));
	b.insert(Quad(
		Term::Var(1031),
		Term::Var(1025),
		Term::Ground(46),
		None,
	));
	b.insert(Quad(
		Term::Var(1036),
		Term::Var(1036),
		Term::Ground(34),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Ground(13),
		Term::Ground(49),
		None,
	));
	b.insert(Quad(
		Term::Var(1018),
		Term::Ground(6),
		Term::Ground(39),
		None,
	));
	b.insert(Quad(
		Term::Ground(10),
		Term::Var(1040),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(3),
		Term::Ground(48),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Ground(8),
		Term::Ground(44),
		None,
	));
	b.insert(Quad(
		Term::Ground(45),
		Term::Var(1006),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Ground(17),
		Term::Ground(45),
		Term::Ground(10),
		None,
	));
	b.insert(Quad(
		Term::Var(1024),
		Term::Ground(4),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Var(1012),
		Term::Ground(34),
		Term::Var(1028),
		None,
	));
	b.insert(Quad(
		Term::Ground(41),
		Term::Var(1024),
		Term::Ground(20),
		None,
	));
	b.insert(Quad(
		Term::Var(1012),
		Term::Ground(8),
		Term::Ground(30),
		None,
	));
	b.insert(Quad(
		Term::Ground(30),
		Term::Var(1028),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Var(1013),
		Term::Var(1003),
		Term::Var(1045),
		None,
	));
	b.insert(Quad(
		Term::Var(1044),
		Term::Var(1015),
		Term::Ground(44),
		None,
	));
	b.insert(Quad(
		Term::Var(1038),
		Term::Var(1040),
		Term::Ground(36),
		None,
	));
	b.insert(Quad(
		Term::Ground(15),
		Term::Ground(45),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Ground(37),
		Term::Ground(35),
		Term::Var(1037),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Ground(28),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Ground(10),
		Term::Var(1002),
		Term::Var(1031),
		None,
	));
	b.insert(Quad(
		Term::Var(1018),
		Term::Ground(42),
		Term::Ground(40),
		None,
	));
	b.insert(Quad(
		Term::Ground(37),
		Term::Var(1042),
		Term::Var(1032),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Var(1000),
		Term::Var(1016),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Var(1005),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Var(1026),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Var(1030),
		Term::Var(1037),
		Term::Ground(18),
		None,
	));
	b.insert(Quad(
		Term::Ground(14),
		Term::Ground(14),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Ground(41),
		Term::Ground(37),
		Term::Var(1049),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(15),
		Term::Var(1016),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(0),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Var(1008),
		Term::Var(1027),
		None,
	));
	b.insert(Quad(
		Term::Var(1023),
		Term::Ground(44),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(48),
		Term::Ground(1),
		Term::Ground(29),
		None,
	));
	b.insert(Quad(
		Term::Ground(43),
		Term::Ground(4),
		Term::Ground(10),
		None,
	));
	b.insert(Quad(
		Term::Ground(40),
		Term::Ground(7),
		Term::Ground(42),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Ground(2),
		Term::Var(1037),
		None,
	));
	b.insert(Quad(
		Term::Var(1027),
		Term::Ground(48),
		Term::Var(1014),
		None,
	));
	b.insert(Quad(
		Term::Var(1043),
		Term::Var(1042),
		Term::Var(1049),
		None,
	));
	b.insert(Quad(
		Term::Var(1014),
		Term::Ground(24),
		Term::Ground(33),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Var(1002),
		Term::Var(1022),
		None,
	));
	b.insert(Quad(
		Term::Var(1042),
		Term::Var(1041),
		Term::Ground(22),
		None,
	));
	b.insert(Quad(
		Term::Var(1020),
		Term::Var(1019),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Var(1015),
		Term::Var(1033),
		Term::Ground(34),
		None,
	));
	b.insert(Quad(
		Term::Ground(30),
		Term::Var(1029),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Ground(41),
		Term::Var(1021),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Var(1027),
		Term::Var(1046),
		Term::Var(1027),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(39),
		Term::Ground(25),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Var(1010),
		Term::Var(1013),
		None,
	));
	test(a, b)
}
#[test]
fn iso_182() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Var(48), Term::Var(15), Term::Ground(39), None));
	a.insert(Quad(Term::Var(7), Term::Ground(24), Term::Var(19), None));
	a.insert(Quad(Term::Var(29), Term::Var(2), Term::Var(17), None));
	a.insert(Quad(
		Term::Ground(14),
		Term::Ground(35),
		Term::Ground(26),
		None,
	));
	a.insert(Quad(
		Term::Ground(6),
		Term::Ground(14),
		Term::Ground(3),
		None,
	));
	a.insert(Quad(Term::Ground(29), Term::Ground(6), Term::Var(37), None));
	a.insert(Quad(Term::Var(44), Term::Ground(27), Term::Var(49), None));
	a.insert(Quad(Term::Var(12), Term::Ground(13), Term::Var(34), None));
	a.insert(Quad(Term::Var(48), Term::Ground(28), Term::Var(43), None));
	a.insert(Quad(Term::Var(7), Term::Var(8), Term::Var(29), None));
	a.insert(Quad(
		Term::Ground(30),
		Term::Var(25),
		Term::Ground(25),
		None,
	));
	a.insert(Quad(Term::Var(42), Term::Var(8), Term::Ground(28), None));
	a.insert(Quad(
		Term::Var(39),
		Term::Ground(31),
		Term::Ground(46),
		None,
	));
	a.insert(Quad(Term::Var(4), Term::Ground(5), Term::Var(38), None));
	a.insert(Quad(Term::Ground(7), Term::Var(45), Term::Ground(35), None));
	a.insert(Quad(Term::Ground(4), Term::Var(33), Term::Ground(48), None));
	a.insert(Quad(Term::Ground(9), Term::Ground(21), Term::Var(0), None));
	a.insert(Quad(
		Term::Ground(27),
		Term::Var(17),
		Term::Ground(28),
		None,
	));
	a.insert(Quad(Term::Var(23), Term::Var(17), Term::Ground(24), None));
	a.insert(Quad(Term::Var(30), Term::Var(22), Term::Var(48), None));
	a.insert(Quad(Term::Var(16), Term::Var(29), Term::Ground(7), None));
	a.insert(Quad(
		Term::Ground(44),
		Term::Ground(41),
		Term::Var(38),
		None,
	));
	a.insert(Quad(
		Term::Ground(25),
		Term::Var(34),
		Term::Ground(10),
		None,
	));
	a.insert(Quad(
		Term::Ground(37),
		Term::Var(34),
		Term::Ground(25),
		None,
	));
	a.insert(Quad(
		Term::Ground(14),
		Term::Ground(10),
		Term::Ground(5),
		None,
	));
	a.insert(Quad(
		Term::Ground(47),
		Term::Ground(20),
		Term::Var(19),
		None,
	));
	a.insert(Quad(
		Term::Ground(44),
		Term::Ground(45),
		Term::Ground(8),
		None,
	));
	a.insert(Quad(Term::Ground(21), Term::Var(48), Term::Var(17), None));
	a.insert(Quad(Term::Ground(21), Term::Var(15), Term::Var(8), None));
	a.insert(Quad(Term::Var(17), Term::Var(25), Term::Ground(21), None));
	a.insert(Quad(Term::Ground(27), Term::Ground(48), Term::Var(0), None));
	a.insert(Quad(
		Term::Ground(16),
		Term::Var(25),
		Term::Ground(47),
		None,
	));
	a.insert(Quad(Term::Ground(35), Term::Var(47), Term::Var(23), None));
	a.insert(Quad(Term::Var(46), Term::Var(49), Term::Ground(43), None));
	a.insert(Quad(Term::Var(15), Term::Ground(49), Term::Var(21), None));
	a.insert(Quad(Term::Var(7), Term::Ground(3), Term::Var(35), None));
	a.insert(Quad(
		Term::Ground(26),
		Term::Ground(6),
		Term::Ground(16),
		None,
	));
	a.insert(Quad(
		Term::Ground(27),
		Term::Ground(48),
		Term::Ground(18),
		None,
	));
	a.insert(Quad(Term::Var(14), Term::Var(28), Term::Var(3), None));
	a.insert(Quad(
		Term::Ground(10),
		Term::Var(39),
		Term::Ground(48),
		None,
	));
	a.insert(Quad(Term::Var(33), Term::Ground(42), Term::Var(44), None));
	a.insert(Quad(Term::Var(33), Term::Var(15), Term::Ground(35), None));
	a.insert(Quad(Term::Ground(44), Term::Var(37), Term::Var(4), None));
	a.insert(Quad(Term::Var(37), Term::Ground(3), Term::Ground(35), None));
	a.insert(Quad(
		Term::Ground(13),
		Term::Var(41),
		Term::Ground(41),
		None,
	));
	a.insert(Quad(Term::Var(31), Term::Ground(25), Term::Ground(7), None));
	a.insert(Quad(
		Term::Ground(18),
		Term::Ground(27),
		Term::Ground(6),
		None,
	));
	a.insert(Quad(Term::Var(21), Term::Ground(25), Term::Ground(3), None));
	a.insert(Quad(Term::Ground(17), Term::Var(30), Term::Var(41), None));
	a.insert(Quad(Term::Var(14), Term::Var(26), Term::Ground(4), None));
	a.insert(Quad(Term::Ground(3), Term::Var(40), Term::Var(31), None));
	a.insert(Quad(Term::Var(27), Term::Ground(11), Term::Var(33), None));
	a.insert(Quad(Term::Var(19), Term::Var(17), Term::Var(43), None));
	a.insert(Quad(Term::Var(19), Term::Var(32), Term::Ground(8), None));
	a.insert(Quad(Term::Var(26), Term::Ground(42), Term::Var(10), None));
	a.insert(Quad(Term::Var(7), Term::Ground(23), Term::Ground(13), None));
	a.insert(Quad(Term::Var(13), Term::Ground(32), Term::Var(47), None));
	a.insert(Quad(
		Term::Ground(44),
		Term::Ground(40),
		Term::Ground(8),
		None,
	));
	a.insert(Quad(Term::Var(1), Term::Ground(0), Term::Ground(37), None));
	a.insert(Quad(Term::Ground(10), Term::Var(25), Term::Ground(7), None));
	a.insert(Quad(
		Term::Ground(22),
		Term::Ground(37),
		Term::Var(12),
		None,
	));
	a.insert(Quad(Term::Ground(42), Term::Var(7), Term::Var(12), None));
	a.insert(Quad(Term::Ground(17), Term::Var(40), Term::Var(4), None));
	a.insert(Quad(Term::Ground(18), Term::Var(34), Term::Ground(9), None));
	a.insert(Quad(Term::Ground(8), Term::Ground(20), Term::Var(11), None));
	a.insert(Quad(Term::Var(21), Term::Ground(27), Term::Ground(8), None));
	a.insert(Quad(Term::Var(10), Term::Var(10), Term::Ground(40), None));
	a.insert(Quad(Term::Var(38), Term::Var(37), Term::Ground(4), None));
	a.insert(Quad(Term::Ground(48), Term::Var(24), Term::Var(16), None));
	a.insert(Quad(
		Term::Ground(48),
		Term::Var(14),
		Term::Ground(27),
		None,
	));
	a.insert(Quad(
		Term::Ground(20),
		Term::Var(21),
		Term::Ground(18),
		None,
	));
	a.insert(Quad(Term::Ground(12), Term::Var(36), Term::Var(20), None));
	a.insert(Quad(
		Term::Ground(11),
		Term::Ground(27),
		Term::Ground(32),
		None,
	));
	a.insert(Quad(Term::Var(34), Term::Var(24), Term::Var(40), None));
	a.insert(Quad(
		Term::Ground(31),
		Term::Ground(26),
		Term::Ground(24),
		None,
	));
	a.insert(Quad(
		Term::Ground(18),
		Term::Var(38),
		Term::Ground(44),
		None,
	));
	a.insert(Quad(Term::Ground(8), Term::Var(37), Term::Var(40), None));
	a.insert(Quad(Term::Var(7), Term::Ground(27), Term::Var(12), None));
	a.insert(Quad(Term::Var(30), Term::Var(4), Term::Var(18), None));
	a.insert(Quad(Term::Ground(36), Term::Ground(2), Term::Var(41), None));
	a.insert(Quad(Term::Var(11), Term::Var(15), Term::Ground(38), None));
	a.insert(Quad(
		Term::Var(17),
		Term::Ground(16),
		Term::Ground(17),
		None,
	));
	a.insert(Quad(
		Term::Var(37),
		Term::Ground(36),
		Term::Ground(38),
		None,
	));
	a.insert(Quad(
		Term::Ground(17),
		Term::Var(48),
		Term::Ground(17),
		None,
	));
	a.insert(Quad(
		Term::Ground(24),
		Term::Ground(26),
		Term::Var(13),
		None,
	));
	a.insert(Quad(Term::Var(2), Term::Var(20), Term::Ground(44), None));
	a.insert(Quad(Term::Ground(30), Term::Var(30), Term::Var(8), None));
	a.insert(Quad(Term::Ground(5), Term::Var(42), Term::Var(30), None));
	a.insert(Quad(Term::Var(34), Term::Var(27), Term::Ground(39), None));
	a.insert(Quad(Term::Var(23), Term::Var(24), Term::Var(22), None));
	a.insert(Quad(Term::Var(18), Term::Var(22), Term::Var(16), None));
	a.insert(Quad(
		Term::Ground(12),
		Term::Ground(3),
		Term::Ground(20),
		None,
	));
	a.insert(Quad(
		Term::Ground(43),
		Term::Ground(10),
		Term::Ground(13),
		None,
	));
	a.insert(Quad(
		Term::Ground(5),
		Term::Ground(38),
		Term::Ground(49),
		None,
	));
	a.insert(Quad(Term::Var(44), Term::Var(33), Term::Var(37), None));
	a.insert(Quad(Term::Var(47), Term::Var(49), Term::Var(35), None));
	a.insert(Quad(Term::Ground(46), Term::Ground(8), Term::Var(46), None));
	a.insert(Quad(Term::Ground(4), Term::Var(47), Term::Ground(16), None));
	a.insert(Quad(Term::Var(27), Term::Var(14), Term::Ground(31), None));
	a.insert(Quad(Term::Var(28), Term::Var(22), Term::Ground(1), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Var(1048),
		Term::Var(1015),
		Term::Ground(39),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Ground(24),
		Term::Var(1019),
		None,
	));
	b.insert(Quad(
		Term::Var(1029),
		Term::Var(1002),
		Term::Var(1017),
		None,
	));
	b.insert(Quad(
		Term::Ground(14),
		Term::Ground(35),
		Term::Ground(26),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Ground(14),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Ground(29),
		Term::Ground(6),
		Term::Var(1037),
		None,
	));
	b.insert(Quad(
		Term::Var(1044),
		Term::Ground(27),
		Term::Var(1049),
		None,
	));
	b.insert(Quad(
		Term::Var(1012),
		Term::Ground(13),
		Term::Var(1034),
		None,
	));
	b.insert(Quad(
		Term::Var(1048),
		Term::Ground(28),
		Term::Var(1043),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Var(1008),
		Term::Var(1029),
		None,
	));
	b.insert(Quad(
		Term::Ground(30),
		Term::Var(1025),
		Term::Ground(25),
		None,
	));
	b.insert(Quad(
		Term::Var(1042),
		Term::Var(1008),
		Term::Ground(28),
		None,
	));
	b.insert(Quad(
		Term::Var(1039),
		Term::Ground(31),
		Term::Ground(46),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Ground(5),
		Term::Var(1038),
		None,
	));
	b.insert(Quad(
		Term::Ground(7),
		Term::Var(1045),
		Term::Ground(35),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Var(1033),
		Term::Ground(48),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Ground(21),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Ground(27),
		Term::Var(1017),
		Term::Ground(28),
		None,
	));
	b.insert(Quad(
		Term::Var(1023),
		Term::Var(1017),
		Term::Ground(24),
		None,
	));
	b.insert(Quad(
		Term::Var(1030),
		Term::Var(1022),
		Term::Var(1048),
		None,
	));
	b.insert(Quad(
		Term::Var(1016),
		Term::Var(1029),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Ground(44),
		Term::Ground(41),
		Term::Var(1038),
		None,
	));
	b.insert(Quad(
		Term::Ground(25),
		Term::Var(1034),
		Term::Ground(10),
		None,
	));
	b.insert(Quad(
		Term::Ground(37),
		Term::Var(1034),
		Term::Ground(25),
		None,
	));
	b.insert(Quad(
		Term::Ground(14),
		Term::Ground(10),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Ground(47),
		Term::Ground(20),
		Term::Var(1019),
		None,
	));
	b.insert(Quad(
		Term::Ground(44),
		Term::Ground(45),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Ground(21),
		Term::Var(1048),
		Term::Var(1017),
		None,
	));
	b.insert(Quad(
		Term::Ground(21),
		Term::Var(1015),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Var(1017),
		Term::Var(1025),
		Term::Ground(21),
		None,
	));
	b.insert(Quad(
		Term::Ground(27),
		Term::Ground(48),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Ground(16),
		Term::Var(1025),
		Term::Ground(47),
		None,
	));
	b.insert(Quad(
		Term::Ground(35),
		Term::Var(1047),
		Term::Var(1023),
		None,
	));
	b.insert(Quad(
		Term::Var(1046),
		Term::Var(1049),
		Term::Ground(43),
		None,
	));
	b.insert(Quad(
		Term::Var(1015),
		Term::Ground(49),
		Term::Var(1021),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Ground(3),
		Term::Var(1035),
		None,
	));
	b.insert(Quad(
		Term::Ground(26),
		Term::Ground(6),
		Term::Ground(16),
		None,
	));
	b.insert(Quad(
		Term::Ground(27),
		Term::Ground(48),
		Term::Ground(18),
		None,
	));
	b.insert(Quad(
		Term::Var(1014),
		Term::Var(1028),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Ground(10),
		Term::Var(1039),
		Term::Ground(48),
		None,
	));
	b.insert(Quad(
		Term::Var(1033),
		Term::Ground(42),
		Term::Var(1044),
		None,
	));
	b.insert(Quad(
		Term::Var(1033),
		Term::Var(1015),
		Term::Ground(35),
		None,
	));
	b.insert(Quad(
		Term::Ground(44),
		Term::Var(1037),
		Term::Var(1004),
		None,
	));
	b.insert(Quad(
		Term::Var(1037),
		Term::Ground(3),
		Term::Ground(35),
		None,
	));
	b.insert(Quad(
		Term::Ground(13),
		Term::Var(1041),
		Term::Ground(41),
		None,
	));
	b.insert(Quad(
		Term::Var(1031),
		Term::Ground(25),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Ground(18),
		Term::Ground(27),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Var(1021),
		Term::Ground(25),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Ground(17),
		Term::Var(1030),
		Term::Var(1041),
		None,
	));
	b.insert(Quad(
		Term::Var(1014),
		Term::Var(1026),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Var(1040),
		Term::Var(1031),
		None,
	));
	b.insert(Quad(
		Term::Var(1027),
		Term::Ground(11),
		Term::Var(1033),
		None,
	));
	b.insert(Quad(
		Term::Var(1019),
		Term::Var(1017),
		Term::Var(1043),
		None,
	));
	b.insert(Quad(
		Term::Var(1019),
		Term::Var(1032),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Var(1026),
		Term::Ground(42),
		Term::Var(1010),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Ground(23),
		Term::Ground(13),
		None,
	));
	b.insert(Quad(
		Term::Var(1013),
		Term::Ground(32),
		Term::Var(1047),
		None,
	));
	b.insert(Quad(
		Term::Ground(44),
		Term::Ground(40),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(0),
		Term::Ground(37),
		None,
	));
	b.insert(Quad(
		Term::Ground(10),
		Term::Var(1025),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Ground(22),
		Term::Ground(37),
		Term::Var(1012),
		None,
	));
	b.insert(Quad(
		Term::Ground(42),
		Term::Var(1007),
		Term::Var(1012),
		None,
	));
	b.insert(Quad(
		Term::Ground(17),
		Term::Var(1040),
		Term::Var(1004),
		None,
	));
	b.insert(Quad(
		Term::Ground(18),
		Term::Var(1034),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Ground(20),
		Term::Var(1011),
		None,
	));
	b.insert(Quad(
		Term::Var(1021),
		Term::Ground(27),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Var(1010),
		Term::Var(1010),
		Term::Ground(40),
		None,
	));
	b.insert(Quad(
		Term::Var(1038),
		Term::Var(1037),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Ground(48),
		Term::Var(1024),
		Term::Var(1016),
		None,
	));
	b.insert(Quad(
		Term::Ground(48),
		Term::Var(1014),
		Term::Ground(27),
		None,
	));
	b.insert(Quad(
		Term::Ground(20),
		Term::Var(1021),
		Term::Ground(18),
		None,
	));
	b.insert(Quad(
		Term::Ground(12),
		Term::Var(1036),
		Term::Var(1020),
		None,
	));
	b.insert(Quad(
		Term::Ground(11),
		Term::Ground(27),
		Term::Ground(32),
		None,
	));
	b.insert(Quad(
		Term::Var(1034),
		Term::Var(1024),
		Term::Var(1040),
		None,
	));
	b.insert(Quad(
		Term::Ground(31),
		Term::Ground(26),
		Term::Ground(24),
		None,
	));
	b.insert(Quad(
		Term::Ground(18),
		Term::Var(1038),
		Term::Ground(44),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Var(1037),
		Term::Var(1040),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Ground(27),
		Term::Var(1012),
		None,
	));
	b.insert(Quad(
		Term::Var(1030),
		Term::Var(1004),
		Term::Var(1018),
		None,
	));
	b.insert(Quad(
		Term::Ground(36),
		Term::Ground(2),
		Term::Var(1041),
		None,
	));
	b.insert(Quad(
		Term::Var(1011),
		Term::Var(1015),
		Term::Ground(38),
		None,
	));
	b.insert(Quad(
		Term::Var(1017),
		Term::Ground(16),
		Term::Ground(17),
		None,
	));
	b.insert(Quad(
		Term::Var(1037),
		Term::Ground(36),
		Term::Ground(38),
		None,
	));
	b.insert(Quad(
		Term::Ground(17),
		Term::Var(1048),
		Term::Ground(17),
		None,
	));
	b.insert(Quad(
		Term::Ground(24),
		Term::Ground(26),
		Term::Var(1013),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Var(1020),
		Term::Ground(44),
		None,
	));
	b.insert(Quad(
		Term::Ground(30),
		Term::Var(1030),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Var(1042),
		Term::Var(1030),
		None,
	));
	b.insert(Quad(
		Term::Var(1034),
		Term::Var(1027),
		Term::Ground(39),
		None,
	));
	b.insert(Quad(
		Term::Var(1023),
		Term::Var(1024),
		Term::Var(1022),
		None,
	));
	b.insert(Quad(
		Term::Var(1018),
		Term::Var(1022),
		Term::Var(1016),
		None,
	));
	b.insert(Quad(
		Term::Ground(12),
		Term::Ground(3),
		Term::Ground(20),
		None,
	));
	b.insert(Quad(
		Term::Ground(43),
		Term::Ground(10),
		Term::Ground(13),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Ground(38),
		Term::Ground(49),
		None,
	));
	b.insert(Quad(
		Term::Var(1044),
		Term::Var(1033),
		Term::Var(1037),
		None,
	));
	b.insert(Quad(
		Term::Var(1047),
		Term::Var(1049),
		Term::Var(1035),
		None,
	));
	b.insert(Quad(
		Term::Ground(46),
		Term::Ground(8),
		Term::Var(1046),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Var(1047),
		Term::Ground(16),
		None,
	));
	b.insert(Quad(
		Term::Var(1027),
		Term::Var(1014),
		Term::Ground(31),
		None,
	));
	b.insert(Quad(
		Term::Var(1028),
		Term::Var(1022),
		Term::Ground(1),
		None,
	));
	test(a, b)
}
#[test]
fn iso_183() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Ground(6), Term::Var(22), Term::Ground(9), None));
	a.insert(Quad(
		Term::Ground(31),
		Term::Ground(33),
		Term::Ground(21),
		None,
	));
	a.insert(Quad(Term::Var(20), Term::Var(36), Term::Ground(27), None));
	a.insert(Quad(
		Term::Ground(35),
		Term::Var(47),
		Term::Ground(26),
		None,
	));
	a.insert(Quad(
		Term::Var(40),
		Term::Ground(28),
		Term::Ground(16),
		None,
	));
	a.insert(Quad(Term::Var(28), Term::Var(27), Term::Var(31), None));
	a.insert(Quad(
		Term::Var(24),
		Term::Ground(38),
		Term::Ground(25),
		None,
	));
	a.insert(Quad(Term::Var(32), Term::Var(14), Term::Var(20), None));
	a.insert(Quad(
		Term::Ground(19),
		Term::Var(11),
		Term::Ground(14),
		None,
	));
	a.insert(Quad(Term::Ground(1), Term::Var(0), Term::Ground(47), None));
	a.insert(Quad(Term::Ground(11), Term::Var(37), Term::Var(26), None));
	a.insert(Quad(Term::Ground(33), Term::Ground(33), Term::Var(1), None));
	a.insert(Quad(
		Term::Ground(33),
		Term::Ground(42),
		Term::Ground(40),
		None,
	));
	a.insert(Quad(
		Term::Ground(26),
		Term::Var(40),
		Term::Ground(44),
		None,
	));
	a.insert(Quad(Term::Var(26), Term::Var(30), Term::Ground(16), None));
	a.insert(Quad(Term::Var(42), Term::Ground(31), Term::Var(34), None));
	a.insert(Quad(
		Term::Ground(13),
		Term::Ground(27),
		Term::Ground(6),
		None,
	));
	a.insert(Quad(Term::Ground(26), Term::Ground(8), Term::Var(23), None));
	a.insert(Quad(Term::Var(20), Term::Var(30), Term::Var(12), None));
	a.insert(Quad(
		Term::Ground(19),
		Term::Ground(19),
		Term::Ground(5),
		None,
	));
	a.insert(Quad(Term::Ground(29), Term::Ground(9), Term::Var(22), None));
	a.insert(Quad(Term::Var(34), Term::Ground(31), Term::Var(5), None));
	a.insert(Quad(Term::Var(5), Term::Ground(32), Term::Var(34), None));
	a.insert(Quad(Term::Ground(15), Term::Var(17), Term::Var(19), None));
	a.insert(Quad(Term::Ground(22), Term::Ground(46), Term::Var(0), None));
	a.insert(Quad(Term::Var(10), Term::Var(16), Term::Ground(33), None));
	a.insert(Quad(
		Term::Ground(26),
		Term::Ground(34),
		Term::Var(46),
		None,
	));
	a.insert(Quad(Term::Ground(27), Term::Var(18), Term::Var(27), None));
	a.insert(Quad(Term::Ground(0), Term::Ground(2), Term::Var(6), None));
	a.insert(Quad(Term::Var(40), Term::Ground(9), Term::Var(30), None));
	a.insert(Quad(Term::Ground(21), Term::Var(19), Term::Var(27), None));
	a.insert(Quad(Term::Var(14), Term::Ground(37), Term::Var(36), None));
	a.insert(Quad(Term::Var(7), Term::Ground(17), Term::Var(38), None));
	a.insert(Quad(Term::Ground(43), Term::Var(47), Term::Var(28), None));
	a.insert(Quad(
		Term::Ground(44),
		Term::Ground(19),
		Term::Ground(23),
		None,
	));
	a.insert(Quad(Term::Var(42), Term::Var(6), Term::Var(5), None));
	a.insert(Quad(Term::Ground(0), Term::Var(45), Term::Ground(21), None));
	a.insert(Quad(Term::Var(9), Term::Ground(3), Term::Ground(46), None));
	a.insert(Quad(
		Term::Ground(18),
		Term::Ground(35),
		Term::Ground(38),
		None,
	));
	a.insert(Quad(Term::Var(15), Term::Ground(3), Term::Var(29), None));
	a.insert(Quad(Term::Var(38), Term::Var(0), Term::Ground(40), None));
	a.insert(Quad(Term::Var(13), Term::Var(34), Term::Ground(8), None));
	a.insert(Quad(
		Term::Ground(2),
		Term::Ground(21),
		Term::Ground(20),
		None,
	));
	a.insert(Quad(Term::Var(16), Term::Ground(29), Term::Var(3), None));
	a.insert(Quad(
		Term::Ground(15),
		Term::Var(49),
		Term::Ground(29),
		None,
	));
	a.insert(Quad(Term::Var(19), Term::Var(38), Term::Ground(10), None));
	a.insert(Quad(Term::Ground(14), Term::Ground(45), Term::Var(8), None));
	a.insert(Quad(
		Term::Var(11),
		Term::Ground(42),
		Term::Ground(41),
		None,
	));
	a.insert(Quad(
		Term::Ground(46),
		Term::Ground(12),
		Term::Var(30),
		None,
	));
	a.insert(Quad(
		Term::Ground(31),
		Term::Var(21),
		Term::Ground(28),
		None,
	));
	a.insert(Quad(
		Term::Ground(11),
		Term::Ground(14),
		Term::Var(25),
		None,
	));
	a.insert(Quad(Term::Ground(8), Term::Var(42), Term::Ground(41), None));
	a.insert(Quad(
		Term::Var(24),
		Term::Ground(43),
		Term::Ground(39),
		None,
	));
	a.insert(Quad(
		Term::Var(35),
		Term::Ground(34),
		Term::Ground(31),
		None,
	));
	a.insert(Quad(Term::Var(32), Term::Ground(7), Term::Ground(48), None));
	a.insert(Quad(Term::Var(23), Term::Var(34), Term::Var(46), None));
	a.insert(Quad(Term::Var(5), Term::Ground(25), Term::Ground(7), None));
	a.insert(Quad(
		Term::Ground(18),
		Term::Ground(47),
		Term::Var(26),
		None,
	));
	a.insert(Quad(Term::Ground(5), Term::Ground(30), Term::Var(18), None));
	a.insert(Quad(
		Term::Ground(44),
		Term::Ground(46),
		Term::Ground(41),
		None,
	));
	a.insert(Quad(Term::Var(16), Term::Var(5), Term::Var(37), None));
	a.insert(Quad(Term::Var(41), Term::Var(24), Term::Ground(44), None));
	a.insert(Quad(Term::Ground(45), Term::Var(24), Term::Var(18), None));
	a.insert(Quad(Term::Var(41), Term::Var(25), Term::Ground(29), None));
	a.insert(Quad(
		Term::Var(15),
		Term::Ground(28),
		Term::Ground(38),
		None,
	));
	a.insert(Quad(Term::Ground(18), Term::Var(42), Term::Var(12), None));
	a.insert(Quad(
		Term::Ground(36),
		Term::Ground(30),
		Term::Ground(29),
		None,
	));
	a.insert(Quad(Term::Var(24), Term::Var(40), Term::Var(23), None));
	a.insert(Quad(Term::Ground(36), Term::Var(25), Term::Var(19), None));
	a.insert(Quad(
		Term::Var(21),
		Term::Ground(43),
		Term::Ground(25),
		None,
	));
	a.insert(Quad(
		Term::Ground(21),
		Term::Var(20),
		Term::Ground(33),
		None,
	));
	a.insert(Quad(Term::Ground(3), Term::Var(39), Term::Ground(9), None));
	a.insert(Quad(Term::Ground(9), Term::Var(48), Term::Var(20), None));
	a.insert(Quad(
		Term::Ground(6),
		Term::Ground(14),
		Term::Ground(45),
		None,
	));
	a.insert(Quad(
		Term::Ground(24),
		Term::Ground(27),
		Term::Ground(28),
		None,
	));
	a.insert(Quad(Term::Var(43), Term::Var(32), Term::Var(1), None));
	a.insert(Quad(
		Term::Ground(42),
		Term::Var(28),
		Term::Ground(31),
		None,
	));
	a.insert(Quad(
		Term::Var(30),
		Term::Ground(46),
		Term::Ground(27),
		None,
	));
	a.insert(Quad(Term::Var(10), Term::Var(25), Term::Ground(38), None));
	a.insert(Quad(Term::Var(28), Term::Ground(0), Term::Ground(36), None));
	a.insert(Quad(
		Term::Var(46),
		Term::Ground(48),
		Term::Ground(19),
		None,
	));
	a.insert(Quad(
		Term::Ground(22),
		Term::Ground(3),
		Term::Ground(8),
		None,
	));
	a.insert(Quad(Term::Ground(48), Term::Var(3), Term::Var(6), None));
	a.insert(Quad(
		Term::Ground(18),
		Term::Ground(31),
		Term::Ground(23),
		None,
	));
	a.insert(Quad(Term::Var(28), Term::Ground(10), Term::Var(4), None));
	a.insert(Quad(Term::Ground(8), Term::Var(28), Term::Ground(11), None));
	a.insert(Quad(Term::Var(19), Term::Var(44), Term::Ground(42), None));
	a.insert(Quad(Term::Var(23), Term::Var(28), Term::Ground(17), None));
	a.insert(Quad(
		Term::Var(49),
		Term::Ground(46),
		Term::Ground(12),
		None,
	));
	a.insert(Quad(Term::Var(33), Term::Ground(0), Term::Var(13), None));
	a.insert(Quad(Term::Var(0), Term::Ground(27), Term::Ground(13), None));
	a.insert(Quad(Term::Var(11), Term::Var(4), Term::Var(24), None));
	a.insert(Quad(Term::Var(25), Term::Ground(41), Term::Ground(6), None));
	a.insert(Quad(Term::Var(0), Term::Ground(31), Term::Var(6), None));
	a.insert(Quad(Term::Ground(13), Term::Var(26), Term::Var(7), None));
	a.insert(Quad(Term::Ground(9), Term::Ground(33), Term::Var(11), None));
	a.insert(Quad(Term::Ground(42), Term::Ground(19), Term::Var(6), None));
	a.insert(Quad(Term::Var(12), Term::Ground(22), Term::Ground(2), None));
	a.insert(Quad(Term::Var(34), Term::Var(38), Term::Ground(25), None));
	a.insert(Quad(Term::Var(23), Term::Ground(5), Term::Var(11), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(6),
		Term::Var(1022),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Ground(31),
		Term::Ground(33),
		Term::Ground(21),
		None,
	));
	b.insert(Quad(
		Term::Var(1020),
		Term::Var(1036),
		Term::Ground(27),
		None,
	));
	b.insert(Quad(
		Term::Ground(35),
		Term::Var(1047),
		Term::Ground(26),
		None,
	));
	b.insert(Quad(
		Term::Var(1040),
		Term::Ground(28),
		Term::Ground(16),
		None,
	));
	b.insert(Quad(
		Term::Var(1028),
		Term::Var(1027),
		Term::Var(1031),
		None,
	));
	b.insert(Quad(
		Term::Var(1024),
		Term::Ground(38),
		Term::Ground(25),
		None,
	));
	b.insert(Quad(
		Term::Var(1032),
		Term::Var(1014),
		Term::Var(1020),
		None,
	));
	b.insert(Quad(
		Term::Ground(19),
		Term::Var(1011),
		Term::Ground(14),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Var(1000),
		Term::Ground(47),
		None,
	));
	b.insert(Quad(
		Term::Ground(11),
		Term::Var(1037),
		Term::Var(1026),
		None,
	));
	b.insert(Quad(
		Term::Ground(33),
		Term::Ground(33),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(33),
		Term::Ground(42),
		Term::Ground(40),
		None,
	));
	b.insert(Quad(
		Term::Ground(26),
		Term::Var(1040),
		Term::Ground(44),
		None,
	));
	b.insert(Quad(
		Term::Var(1026),
		Term::Var(1030),
		Term::Ground(16),
		None,
	));
	b.insert(Quad(
		Term::Var(1042),
		Term::Ground(31),
		Term::Var(1034),
		None,
	));
	b.insert(Quad(
		Term::Ground(13),
		Term::Ground(27),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Ground(26),
		Term::Ground(8),
		Term::Var(1023),
		None,
	));
	b.insert(Quad(
		Term::Var(1020),
		Term::Var(1030),
		Term::Var(1012),
		None,
	));
	b.insert(Quad(
		Term::Ground(19),
		Term::Ground(19),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Ground(29),
		Term::Ground(9),
		Term::Var(1022),
		None,
	));
	b.insert(Quad(
		Term::Var(1034),
		Term::Ground(31),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Ground(32),
		Term::Var(1034),
		None,
	));
	b.insert(Quad(
		Term::Ground(15),
		Term::Var(1017),
		Term::Var(1019),
		None,
	));
	b.insert(Quad(
		Term::Ground(22),
		Term::Ground(46),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Var(1010),
		Term::Var(1016),
		Term::Ground(33),
		None,
	));
	b.insert(Quad(
		Term::Ground(26),
		Term::Ground(34),
		Term::Var(1046),
		None,
	));
	b.insert(Quad(
		Term::Ground(27),
		Term::Var(1018),
		Term::Var(1027),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(2),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Var(1040),
		Term::Ground(9),
		Term::Var(1030),
		None,
	));
	b.insert(Quad(
		Term::Ground(21),
		Term::Var(1019),
		Term::Var(1027),
		None,
	));
	b.insert(Quad(
		Term::Var(1014),
		Term::Ground(37),
		Term::Var(1036),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Ground(17),
		Term::Var(1038),
		None,
	));
	b.insert(Quad(
		Term::Ground(43),
		Term::Var(1047),
		Term::Var(1028),
		None,
	));
	b.insert(Quad(
		Term::Ground(44),
		Term::Ground(19),
		Term::Ground(23),
		None,
	));
	b.insert(Quad(
		Term::Var(1042),
		Term::Var(1006),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Var(1045),
		Term::Ground(21),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Ground(3),
		Term::Ground(46),
		None,
	));
	b.insert(Quad(
		Term::Ground(18),
		Term::Ground(35),
		Term::Ground(38),
		None,
	));
	b.insert(Quad(
		Term::Var(1015),
		Term::Ground(3),
		Term::Var(1029),
		None,
	));
	b.insert(Quad(
		Term::Var(1038),
		Term::Var(1000),
		Term::Ground(40),
		None,
	));
	b.insert(Quad(
		Term::Var(1013),
		Term::Var(1034),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(21),
		Term::Ground(20),
		None,
	));
	b.insert(Quad(
		Term::Var(1016),
		Term::Ground(29),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Ground(15),
		Term::Var(1049),
		Term::Ground(29),
		None,
	));
	b.insert(Quad(
		Term::Var(1019),
		Term::Var(1038),
		Term::Ground(10),
		None,
	));
	b.insert(Quad(
		Term::Ground(14),
		Term::Ground(45),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Var(1011),
		Term::Ground(42),
		Term::Ground(41),
		None,
	));
	b.insert(Quad(
		Term::Ground(46),
		Term::Ground(12),
		Term::Var(1030),
		None,
	));
	b.insert(Quad(
		Term::Ground(31),
		Term::Var(1021),
		Term::Ground(28),
		None,
	));
	b.insert(Quad(
		Term::Ground(11),
		Term::Ground(14),
		Term::Var(1025),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Var(1042),
		Term::Ground(41),
		None,
	));
	b.insert(Quad(
		Term::Var(1024),
		Term::Ground(43),
		Term::Ground(39),
		None,
	));
	b.insert(Quad(
		Term::Var(1035),
		Term::Ground(34),
		Term::Ground(31),
		None,
	));
	b.insert(Quad(
		Term::Var(1032),
		Term::Ground(7),
		Term::Ground(48),
		None,
	));
	b.insert(Quad(
		Term::Var(1023),
		Term::Var(1034),
		Term::Var(1046),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Ground(25),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Ground(18),
		Term::Ground(47),
		Term::Var(1026),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Ground(30),
		Term::Var(1018),
		None,
	));
	b.insert(Quad(
		Term::Ground(44),
		Term::Ground(46),
		Term::Ground(41),
		None,
	));
	b.insert(Quad(
		Term::Var(1016),
		Term::Var(1005),
		Term::Var(1037),
		None,
	));
	b.insert(Quad(
		Term::Var(1041),
		Term::Var(1024),
		Term::Ground(44),
		None,
	));
	b.insert(Quad(
		Term::Ground(45),
		Term::Var(1024),
		Term::Var(1018),
		None,
	));
	b.insert(Quad(
		Term::Var(1041),
		Term::Var(1025),
		Term::Ground(29),
		None,
	));
	b.insert(Quad(
		Term::Var(1015),
		Term::Ground(28),
		Term::Ground(38),
		None,
	));
	b.insert(Quad(
		Term::Ground(18),
		Term::Var(1042),
		Term::Var(1012),
		None,
	));
	b.insert(Quad(
		Term::Ground(36),
		Term::Ground(30),
		Term::Ground(29),
		None,
	));
	b.insert(Quad(
		Term::Var(1024),
		Term::Var(1040),
		Term::Var(1023),
		None,
	));
	b.insert(Quad(
		Term::Ground(36),
		Term::Var(1025),
		Term::Var(1019),
		None,
	));
	b.insert(Quad(
		Term::Var(1021),
		Term::Ground(43),
		Term::Ground(25),
		None,
	));
	b.insert(Quad(
		Term::Ground(21),
		Term::Var(1020),
		Term::Ground(33),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Var(1039),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Var(1048),
		Term::Var(1020),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Ground(14),
		Term::Ground(45),
		None,
	));
	b.insert(Quad(
		Term::Ground(24),
		Term::Ground(27),
		Term::Ground(28),
		None,
	));
	b.insert(Quad(
		Term::Var(1043),
		Term::Var(1032),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(42),
		Term::Var(1028),
		Term::Ground(31),
		None,
	));
	b.insert(Quad(
		Term::Var(1030),
		Term::Ground(46),
		Term::Ground(27),
		None,
	));
	b.insert(Quad(
		Term::Var(1010),
		Term::Var(1025),
		Term::Ground(38),
		None,
	));
	b.insert(Quad(
		Term::Var(1028),
		Term::Ground(0),
		Term::Ground(36),
		None,
	));
	b.insert(Quad(
		Term::Var(1046),
		Term::Ground(48),
		Term::Ground(19),
		None,
	));
	b.insert(Quad(
		Term::Ground(22),
		Term::Ground(3),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Ground(48),
		Term::Var(1003),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Ground(18),
		Term::Ground(31),
		Term::Ground(23),
		None,
	));
	b.insert(Quad(
		Term::Var(1028),
		Term::Ground(10),
		Term::Var(1004),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Var(1028),
		Term::Ground(11),
		None,
	));
	b.insert(Quad(
		Term::Var(1019),
		Term::Var(1044),
		Term::Ground(42),
		None,
	));
	b.insert(Quad(
		Term::Var(1023),
		Term::Var(1028),
		Term::Ground(17),
		None,
	));
	b.insert(Quad(
		Term::Var(1049),
		Term::Ground(46),
		Term::Ground(12),
		None,
	));
	b.insert(Quad(
		Term::Var(1033),
		Term::Ground(0),
		Term::Var(1013),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(27),
		Term::Ground(13),
		None,
	));
	b.insert(Quad(
		Term::Var(1011),
		Term::Var(1004),
		Term::Var(1024),
		None,
	));
	b.insert(Quad(
		Term::Var(1025),
		Term::Ground(41),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(31),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Ground(13),
		Term::Var(1026),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Ground(33),
		Term::Var(1011),
		None,
	));
	b.insert(Quad(
		Term::Ground(42),
		Term::Ground(19),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Var(1012),
		Term::Ground(22),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Var(1034),
		Term::Var(1038),
		Term::Ground(25),
		None,
	));
	b.insert(Quad(
		Term::Var(1023),
		Term::Ground(5),
		Term::Var(1011),
		None,
	));
	test(a, b)
}
#[test]
fn iso_184() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Ground(14), Term::Var(17), Term::Var(6), None));
	a.insert(Quad(
		Term::Ground(28),
		Term::Var(25),
		Term::Ground(23),
		None,
	));
	a.insert(Quad(
		Term::Var(31),
		Term::Ground(12),
		Term::Ground(12),
		None,
	));
	a.insert(Quad(
		Term::Ground(43),
		Term::Ground(13),
		Term::Ground(28),
		None,
	));
	a.insert(Quad(Term::Ground(34), Term::Var(35), Term::Var(46), None));
	a.insert(Quad(Term::Var(41), Term::Var(40), Term::Var(33), None));
	a.insert(Quad(
		Term::Ground(42),
		Term::Var(47),
		Term::Ground(46),
		None,
	));
	a.insert(Quad(Term::Ground(11), Term::Ground(2), Term::Var(37), None));
	a.insert(Quad(Term::Var(47), Term::Var(39), Term::Var(8), None));
	a.insert(Quad(Term::Ground(20), Term::Var(6), Term::Var(5), None));
	a.insert(Quad(
		Term::Ground(41),
		Term::Var(14),
		Term::Ground(26),
		None,
	));
	a.insert(Quad(Term::Ground(19), Term::Var(5), Term::Ground(26), None));
	a.insert(Quad(Term::Var(4), Term::Var(11), Term::Var(21), None));
	a.insert(Quad(
		Term::Ground(42),
		Term::Ground(11),
		Term::Ground(10),
		None,
	));
	a.insert(Quad(
		Term::Ground(29),
		Term::Ground(30),
		Term::Ground(42),
		None,
	));
	a.insert(Quad(
		Term::Var(23),
		Term::Ground(48),
		Term::Ground(15),
		None,
	));
	a.insert(Quad(
		Term::Ground(43),
		Term::Ground(47),
		Term::Ground(39),
		None,
	));
	a.insert(Quad(
		Term::Var(28),
		Term::Ground(27),
		Term::Ground(11),
		None,
	));
	a.insert(Quad(
		Term::Ground(32),
		Term::Var(21),
		Term::Ground(49),
		None,
	));
	a.insert(Quad(Term::Var(46), Term::Ground(47), Term::Var(26), None));
	a.insert(Quad(Term::Var(30), Term::Var(17), Term::Ground(48), None));
	a.insert(Quad(Term::Var(30), Term::Var(44), Term::Ground(46), None));
	a.insert(Quad(Term::Var(10), Term::Ground(3), Term::Ground(16), None));
	a.insert(Quad(Term::Ground(46), Term::Var(25), Term::Ground(1), None));
	a.insert(Quad(
		Term::Ground(21),
		Term::Ground(14),
		Term::Ground(32),
		None,
	));
	a.insert(Quad(Term::Var(42), Term::Ground(13), Term::Var(10), None));
	a.insert(Quad(Term::Ground(22), Term::Var(43), Term::Var(41), None));
	a.insert(Quad(Term::Var(6), Term::Var(7), Term::Ground(34), None));
	a.insert(Quad(Term::Var(11), Term::Var(10), Term::Var(0), None));
	a.insert(Quad(
		Term::Ground(21),
		Term::Ground(31),
		Term::Ground(11),
		None,
	));
	a.insert(Quad(
		Term::Ground(41),
		Term::Ground(36),
		Term::Ground(9),
		None,
	));
	a.insert(Quad(Term::Var(12), Term::Ground(33), Term::Var(16), None));
	a.insert(Quad(
		Term::Ground(21),
		Term::Ground(21),
		Term::Ground(12),
		None,
	));
	a.insert(Quad(Term::Ground(4), Term::Ground(34), Term::Var(30), None));
	a.insert(Quad(
		Term::Var(30),
		Term::Ground(42),
		Term::Ground(43),
		None,
	));
	a.insert(Quad(Term::Var(41), Term::Ground(14), Term::Var(27), None));
	a.insert(Quad(
		Term::Var(45),
		Term::Ground(26),
		Term::Ground(40),
		None,
	));
	a.insert(Quad(Term::Var(44), Term::Var(23), Term::Var(21), None));
	a.insert(Quad(Term::Var(38), Term::Var(30), Term::Var(7), None));
	a.insert(Quad(Term::Var(15), Term::Var(13), Term::Ground(48), None));
	a.insert(Quad(
		Term::Var(22),
		Term::Ground(25),
		Term::Ground(31),
		None,
	));
	a.insert(Quad(
		Term::Ground(16),
		Term::Ground(30),
		Term::Var(28),
		None,
	));
	a.insert(Quad(Term::Var(22), Term::Var(26), Term::Var(29), None));
	a.insert(Quad(Term::Ground(0), Term::Var(20), Term::Var(16), None));
	a.insert(Quad(Term::Ground(1), Term::Var(1), Term::Var(47), None));
	a.insert(Quad(Term::Var(37), Term::Var(3), Term::Ground(4), None));
	a.insert(Quad(Term::Var(12), Term::Var(39), Term::Var(20), None));
	a.insert(Quad(Term::Ground(22), Term::Ground(1), Term::Var(44), None));
	a.insert(Quad(
		Term::Ground(5),
		Term::Ground(22),
		Term::Ground(4),
		None,
	));
	a.insert(Quad(Term::Ground(3), Term::Var(44), Term::Ground(3), None));
	a.insert(Quad(Term::Var(18), Term::Ground(20), Term::Ground(1), None));
	a.insert(Quad(Term::Var(26), Term::Ground(3), Term::Ground(17), None));
	a.insert(Quad(
		Term::Ground(33),
		Term::Ground(48),
		Term::Ground(39),
		None,
	));
	a.insert(Quad(
		Term::Ground(16),
		Term::Ground(12),
		Term::Ground(10),
		None,
	));
	a.insert(Quad(Term::Var(36), Term::Var(42), Term::Var(43), None));
	a.insert(Quad(
		Term::Ground(35),
		Term::Ground(0),
		Term::Ground(1),
		None,
	));
	a.insert(Quad(
		Term::Var(40),
		Term::Ground(45),
		Term::Ground(16),
		None,
	));
	a.insert(Quad(Term::Ground(9), Term::Var(46), Term::Ground(31), None));
	a.insert(Quad(Term::Ground(15), Term::Var(18), Term::Var(0), None));
	a.insert(Quad(Term::Var(5), Term::Var(5), Term::Var(37), None));
	a.insert(Quad(Term::Var(38), Term::Ground(2), Term::Ground(9), None));
	a.insert(Quad(Term::Var(35), Term::Ground(18), Term::Var(39), None));
	a.insert(Quad(Term::Var(42), Term::Var(15), Term::Ground(4), None));
	a.insert(Quad(Term::Ground(24), Term::Var(9), Term::Ground(43), None));
	a.insert(Quad(
		Term::Ground(37),
		Term::Ground(23),
		Term::Ground(47),
		None,
	));
	a.insert(Quad(Term::Ground(28), Term::Var(43), Term::Var(29), None));
	a.insert(Quad(
		Term::Ground(10),
		Term::Var(34),
		Term::Ground(14),
		None,
	));
	a.insert(Quad(
		Term::Ground(12),
		Term::Var(29),
		Term::Ground(49),
		None,
	));
	a.insert(Quad(Term::Var(3), Term::Ground(33), Term::Var(24), None));
	a.insert(Quad(
		Term::Ground(28),
		Term::Ground(4),
		Term::Ground(2),
		None,
	));
	a.insert(Quad(Term::Var(4), Term::Var(31), Term::Ground(12), None));
	a.insert(Quad(Term::Ground(13), Term::Var(31), Term::Var(48), None));
	a.insert(Quad(Term::Var(9), Term::Ground(37), Term::Var(49), None));
	a.insert(Quad(
		Term::Ground(41),
		Term::Ground(39),
		Term::Ground(36),
		None,
	));
	a.insert(Quad(
		Term::Ground(37),
		Term::Ground(47),
		Term::Ground(17),
		None,
	));
	a.insert(Quad(Term::Ground(18), Term::Ground(8), Term::Var(22), None));
	a.insert(Quad(Term::Var(4), Term::Var(33), Term::Var(33), None));
	a.insert(Quad(
		Term::Ground(1),
		Term::Ground(5),
		Term::Ground(39),
		None,
	));
	a.insert(Quad(Term::Var(28), Term::Ground(44), Term::Var(46), None));
	a.insert(Quad(
		Term::Ground(32),
		Term::Ground(31),
		Term::Ground(49),
		None,
	));
	a.insert(Quad(Term::Var(39), Term::Var(42), Term::Ground(25), None));
	a.insert(Quad(Term::Var(36), Term::Var(42), Term::Var(36), None));
	a.insert(Quad(Term::Var(16), Term::Ground(32), Term::Var(42), None));
	a.insert(Quad(
		Term::Ground(44),
		Term::Ground(40),
		Term::Var(15),
		None,
	));
	a.insert(Quad(
		Term::Ground(16),
		Term::Ground(32),
		Term::Ground(10),
		None,
	));
	a.insert(Quad(Term::Ground(33), Term::Ground(5), Term::Var(18), None));
	a.insert(Quad(
		Term::Ground(25),
		Term::Ground(22),
		Term::Var(39),
		None,
	));
	a.insert(Quad(Term::Var(12), Term::Var(24), Term::Ground(29), None));
	a.insert(Quad(Term::Var(36), Term::Ground(47), Term::Ground(8), None));
	a.insert(Quad(
		Term::Ground(44),
		Term::Var(25),
		Term::Ground(48),
		None,
	));
	a.insert(Quad(Term::Var(40), Term::Var(27), Term::Ground(23), None));
	a.insert(Quad(Term::Var(30), Term::Var(39), Term::Ground(19), None));
	a.insert(Quad(
		Term::Ground(32),
		Term::Ground(44),
		Term::Var(34),
		None,
	));
	a.insert(Quad(Term::Ground(38), Term::Var(4), Term::Ground(32), None));
	a.insert(Quad(Term::Var(8), Term::Ground(44), Term::Var(49), None));
	a.insert(Quad(
		Term::Ground(5),
		Term::Ground(8),
		Term::Ground(42),
		None,
	));
	a.insert(Quad(Term::Ground(1), Term::Ground(48), Term::Var(37), None));
	a.insert(Quad(
		Term::Var(15),
		Term::Ground(18),
		Term::Ground(37),
		None,
	));
	a.insert(Quad(
		Term::Var(37),
		Term::Ground(42),
		Term::Ground(24),
		None,
	));
	a.insert(Quad(Term::Ground(27), Term::Var(48), Term::Var(11), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(14),
		Term::Var(1017),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Ground(28),
		Term::Var(1025),
		Term::Ground(23),
		None,
	));
	b.insert(Quad(
		Term::Var(1031),
		Term::Ground(12),
		Term::Ground(12),
		None,
	));
	b.insert(Quad(
		Term::Ground(43),
		Term::Ground(13),
		Term::Ground(28),
		None,
	));
	b.insert(Quad(
		Term::Ground(34),
		Term::Var(1035),
		Term::Var(1046),
		None,
	));
	b.insert(Quad(
		Term::Var(1041),
		Term::Var(1040),
		Term::Var(1033),
		None,
	));
	b.insert(Quad(
		Term::Ground(42),
		Term::Var(1047),
		Term::Ground(46),
		None,
	));
	b.insert(Quad(
		Term::Ground(11),
		Term::Ground(2),
		Term::Var(1037),
		None,
	));
	b.insert(Quad(
		Term::Var(1047),
		Term::Var(1039),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Ground(20),
		Term::Var(1006),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Ground(41),
		Term::Var(1014),
		Term::Ground(26),
		None,
	));
	b.insert(Quad(
		Term::Ground(19),
		Term::Var(1005),
		Term::Ground(26),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Var(1011),
		Term::Var(1021),
		None,
	));
	b.insert(Quad(
		Term::Ground(42),
		Term::Ground(11),
		Term::Ground(10),
		None,
	));
	b.insert(Quad(
		Term::Ground(29),
		Term::Ground(30),
		Term::Ground(42),
		None,
	));
	b.insert(Quad(
		Term::Var(1023),
		Term::Ground(48),
		Term::Ground(15),
		None,
	));
	b.insert(Quad(
		Term::Ground(43),
		Term::Ground(47),
		Term::Ground(39),
		None,
	));
	b.insert(Quad(
		Term::Var(1028),
		Term::Ground(27),
		Term::Ground(11),
		None,
	));
	b.insert(Quad(
		Term::Ground(32),
		Term::Var(1021),
		Term::Ground(49),
		None,
	));
	b.insert(Quad(
		Term::Var(1046),
		Term::Ground(47),
		Term::Var(1026),
		None,
	));
	b.insert(Quad(
		Term::Var(1030),
		Term::Var(1017),
		Term::Ground(48),
		None,
	));
	b.insert(Quad(
		Term::Var(1030),
		Term::Var(1044),
		Term::Ground(46),
		None,
	));
	b.insert(Quad(
		Term::Var(1010),
		Term::Ground(3),
		Term::Ground(16),
		None,
	));
	b.insert(Quad(
		Term::Ground(46),
		Term::Var(1025),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Ground(21),
		Term::Ground(14),
		Term::Ground(32),
		None,
	));
	b.insert(Quad(
		Term::Var(1042),
		Term::Ground(13),
		Term::Var(1010),
		None,
	));
	b.insert(Quad(
		Term::Ground(22),
		Term::Var(1043),
		Term::Var(1041),
		None,
	));
	b.insert(Quad(
		Term::Var(1006),
		Term::Var(1007),
		Term::Ground(34),
		None,
	));
	b.insert(Quad(
		Term::Var(1011),
		Term::Var(1010),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Ground(21),
		Term::Ground(31),
		Term::Ground(11),
		None,
	));
	b.insert(Quad(
		Term::Ground(41),
		Term::Ground(36),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Var(1012),
		Term::Ground(33),
		Term::Var(1016),
		None,
	));
	b.insert(Quad(
		Term::Ground(21),
		Term::Ground(21),
		Term::Ground(12),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Ground(34),
		Term::Var(1030),
		None,
	));
	b.insert(Quad(
		Term::Var(1030),
		Term::Ground(42),
		Term::Ground(43),
		None,
	));
	b.insert(Quad(
		Term::Var(1041),
		Term::Ground(14),
		Term::Var(1027),
		None,
	));
	b.insert(Quad(
		Term::Var(1045),
		Term::Ground(26),
		Term::Ground(40),
		None,
	));
	b.insert(Quad(
		Term::Var(1044),
		Term::Var(1023),
		Term::Var(1021),
		None,
	));
	b.insert(Quad(
		Term::Var(1038),
		Term::Var(1030),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Var(1015),
		Term::Var(1013),
		Term::Ground(48),
		None,
	));
	b.insert(Quad(
		Term::Var(1022),
		Term::Ground(25),
		Term::Ground(31),
		None,
	));
	b.insert(Quad(
		Term::Ground(16),
		Term::Ground(30),
		Term::Var(1028),
		None,
	));
	b.insert(Quad(
		Term::Var(1022),
		Term::Var(1026),
		Term::Var(1029),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Var(1020),
		Term::Var(1016),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Var(1001),
		Term::Var(1047),
		None,
	));
	b.insert(Quad(
		Term::Var(1037),
		Term::Var(1003),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Var(1012),
		Term::Var(1039),
		Term::Var(1020),
		None,
	));
	b.insert(Quad(
		Term::Ground(22),
		Term::Ground(1),
		Term::Var(1044),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Ground(22),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Var(1044),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Var(1018),
		Term::Ground(20),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Var(1026),
		Term::Ground(3),
		Term::Ground(17),
		None,
	));
	b.insert(Quad(
		Term::Ground(33),
		Term::Ground(48),
		Term::Ground(39),
		None,
	));
	b.insert(Quad(
		Term::Ground(16),
		Term::Ground(12),
		Term::Ground(10),
		None,
	));
	b.insert(Quad(
		Term::Var(1036),
		Term::Var(1042),
		Term::Var(1043),
		None,
	));
	b.insert(Quad(
		Term::Ground(35),
		Term::Ground(0),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Var(1040),
		Term::Ground(45),
		Term::Ground(16),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Var(1046),
		Term::Ground(31),
		None,
	));
	b.insert(Quad(
		Term::Ground(15),
		Term::Var(1018),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Var(1005),
		Term::Var(1037),
		None,
	));
	b.insert(Quad(
		Term::Var(1038),
		Term::Ground(2),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Var(1035),
		Term::Ground(18),
		Term::Var(1039),
		None,
	));
	b.insert(Quad(
		Term::Var(1042),
		Term::Var(1015),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Ground(24),
		Term::Var(1009),
		Term::Ground(43),
		None,
	));
	b.insert(Quad(
		Term::Ground(37),
		Term::Ground(23),
		Term::Ground(47),
		None,
	));
	b.insert(Quad(
		Term::Ground(28),
		Term::Var(1043),
		Term::Var(1029),
		None,
	));
	b.insert(Quad(
		Term::Ground(10),
		Term::Var(1034),
		Term::Ground(14),
		None,
	));
	b.insert(Quad(
		Term::Ground(12),
		Term::Var(1029),
		Term::Ground(49),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Ground(33),
		Term::Var(1024),
		None,
	));
	b.insert(Quad(
		Term::Ground(28),
		Term::Ground(4),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Var(1031),
		Term::Ground(12),
		None,
	));
	b.insert(Quad(
		Term::Ground(13),
		Term::Var(1031),
		Term::Var(1048),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Ground(37),
		Term::Var(1049),
		None,
	));
	b.insert(Quad(
		Term::Ground(41),
		Term::Ground(39),
		Term::Ground(36),
		None,
	));
	b.insert(Quad(
		Term::Ground(37),
		Term::Ground(47),
		Term::Ground(17),
		None,
	));
	b.insert(Quad(
		Term::Ground(18),
		Term::Ground(8),
		Term::Var(1022),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Var(1033),
		Term::Var(1033),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(5),
		Term::Ground(39),
		None,
	));
	b.insert(Quad(
		Term::Var(1028),
		Term::Ground(44),
		Term::Var(1046),
		None,
	));
	b.insert(Quad(
		Term::Ground(32),
		Term::Ground(31),
		Term::Ground(49),
		None,
	));
	b.insert(Quad(
		Term::Var(1039),
		Term::Var(1042),
		Term::Ground(25),
		None,
	));
	b.insert(Quad(
		Term::Var(1036),
		Term::Var(1042),
		Term::Var(1036),
		None,
	));
	b.insert(Quad(
		Term::Var(1016),
		Term::Ground(32),
		Term::Var(1042),
		None,
	));
	b.insert(Quad(
		Term::Ground(44),
		Term::Ground(40),
		Term::Var(1015),
		None,
	));
	b.insert(Quad(
		Term::Ground(16),
		Term::Ground(32),
		Term::Ground(10),
		None,
	));
	b.insert(Quad(
		Term::Ground(33),
		Term::Ground(5),
		Term::Var(1018),
		None,
	));
	b.insert(Quad(
		Term::Ground(25),
		Term::Ground(22),
		Term::Var(1039),
		None,
	));
	b.insert(Quad(
		Term::Var(1012),
		Term::Var(1024),
		Term::Ground(29),
		None,
	));
	b.insert(Quad(
		Term::Var(1036),
		Term::Ground(47),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Ground(44),
		Term::Var(1025),
		Term::Ground(48),
		None,
	));
	b.insert(Quad(
		Term::Var(1040),
		Term::Var(1027),
		Term::Ground(23),
		None,
	));
	b.insert(Quad(
		Term::Var(1030),
		Term::Var(1039),
		Term::Ground(19),
		None,
	));
	b.insert(Quad(
		Term::Ground(32),
		Term::Ground(44),
		Term::Var(1034),
		None,
	));
	b.insert(Quad(
		Term::Ground(38),
		Term::Var(1004),
		Term::Ground(32),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Ground(44),
		Term::Var(1049),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Ground(8),
		Term::Ground(42),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(48),
		Term::Var(1037),
		None,
	));
	b.insert(Quad(
		Term::Var(1015),
		Term::Ground(18),
		Term::Ground(37),
		None,
	));
	b.insert(Quad(
		Term::Var(1037),
		Term::Ground(42),
		Term::Ground(24),
		None,
	));
	b.insert(Quad(
		Term::Ground(27),
		Term::Var(1048),
		Term::Var(1011),
		None,
	));
	test(a, b)
}
#[test]
fn iso_185() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(
		Term::Ground(42),
		Term::Var(23),
		Term::Ground(11),
		None,
	));
	a.insert(Quad(
		Term::Ground(46),
		Term::Var(38),
		Term::Ground(11),
		None,
	));
	a.insert(Quad(Term::Var(37), Term::Var(45), Term::Var(15), None));
	a.insert(Quad(Term::Var(12), Term::Ground(2), Term::Var(31), None));
	a.insert(Quad(Term::Ground(25), Term::Var(46), Term::Var(25), None));
	a.insert(Quad(Term::Ground(47), Term::Var(0), Term::Var(11), None));
	a.insert(Quad(Term::Var(32), Term::Var(18), Term::Var(13), None));
	a.insert(Quad(Term::Var(42), Term::Var(37), Term::Ground(4), None));
	a.insert(Quad(Term::Ground(49), Term::Var(24), Term::Var(17), None));
	a.insert(Quad(
		Term::Ground(18),
		Term::Ground(11),
		Term::Ground(28),
		None,
	));
	a.insert(Quad(Term::Ground(22), Term::Var(44), Term::Ground(8), None));
	a.insert(Quad(
		Term::Ground(22),
		Term::Ground(43),
		Term::Ground(47),
		None,
	));
	a.insert(Quad(Term::Var(4), Term::Var(49), Term::Ground(40), None));
	a.insert(Quad(Term::Var(40), Term::Var(37), Term::Ground(11), None));
	a.insert(Quad(Term::Var(9), Term::Var(39), Term::Var(16), None));
	a.insert(Quad(Term::Var(48), Term::Var(20), Term::Ground(11), None));
	a.insert(Quad(Term::Ground(38), Term::Var(4), Term::Var(25), None));
	a.insert(Quad(Term::Var(0), Term::Var(12), Term::Ground(31), None));
	a.insert(Quad(Term::Ground(31), Term::Var(7), Term::Var(35), None));
	a.insert(Quad(Term::Var(20), Term::Var(22), Term::Ground(3), None));
	a.insert(Quad(Term::Var(32), Term::Var(22), Term::Var(27), None));
	a.insert(Quad(Term::Ground(6), Term::Var(14), Term::Ground(41), None));
	a.insert(Quad(
		Term::Ground(21),
		Term::Var(41),
		Term::Ground(37),
		None,
	));
	a.insert(Quad(
		Term::Ground(18),
		Term::Ground(44),
		Term::Ground(37),
		None,
	));
	a.insert(Quad(Term::Ground(1), Term::Ground(27), Term::Var(22), None));
	a.insert(Quad(Term::Ground(33), Term::Ground(46), Term::Var(1), None));
	a.insert(Quad(Term::Ground(25), Term::Var(18), Term::Var(29), None));
	a.insert(Quad(Term::Var(8), Term::Var(40), Term::Var(9), None));
	a.insert(Quad(Term::Var(16), Term::Var(20), Term::Ground(16), None));
	a.insert(Quad(Term::Var(6), Term::Var(46), Term::Var(49), None));
	a.insert(Quad(Term::Var(10), Term::Ground(25), Term::Var(24), None));
	a.insert(Quad(Term::Var(10), Term::Var(3), Term::Var(14), None));
	a.insert(Quad(Term::Var(33), Term::Var(10), Term::Ground(30), None));
	a.insert(Quad(Term::Ground(26), Term::Var(45), Term::Var(24), None));
	a.insert(Quad(Term::Var(38), Term::Var(7), Term::Ground(47), None));
	a.insert(Quad(Term::Ground(14), Term::Var(33), Term::Var(37), None));
	a.insert(Quad(Term::Var(45), Term::Var(18), Term::Var(30), None));
	a.insert(Quad(Term::Var(27), Term::Ground(30), Term::Var(35), None));
	a.insert(Quad(Term::Var(33), Term::Ground(9), Term::Var(30), None));
	a.insert(Quad(Term::Ground(5), Term::Var(29), Term::Var(19), None));
	a.insert(Quad(Term::Var(4), Term::Ground(18), Term::Var(3), None));
	a.insert(Quad(
		Term::Var(19),
		Term::Ground(16),
		Term::Ground(41),
		None,
	));
	a.insert(Quad(Term::Var(30), Term::Var(1), Term::Var(26), None));
	a.insert(Quad(Term::Ground(29), Term::Var(7), Term::Ground(40), None));
	a.insert(Quad(Term::Var(17), Term::Ground(41), Term::Ground(2), None));
	a.insert(Quad(Term::Var(27), Term::Var(48), Term::Ground(2), None));
	a.insert(Quad(
		Term::Ground(35),
		Term::Ground(17),
		Term::Var(29),
		None,
	));
	a.insert(Quad(
		Term::Var(49),
		Term::Ground(37),
		Term::Ground(21),
		None,
	));
	a.insert(Quad(Term::Ground(3), Term::Ground(49), Term::Var(17), None));
	a.insert(Quad(Term::Var(7), Term::Var(10), Term::Var(41), None));
	a.insert(Quad(Term::Ground(0), Term::Ground(13), Term::Var(6), None));
	a.insert(Quad(Term::Var(49), Term::Ground(15), Term::Var(48), None));
	a.insert(Quad(
		Term::Ground(48),
		Term::Ground(3),
		Term::Ground(11),
		None,
	));
	a.insert(Quad(Term::Ground(20), Term::Var(1), Term::Var(48), None));
	a.insert(Quad(Term::Ground(44), Term::Ground(1), Term::Var(6), None));
	a.insert(Quad(Term::Var(23), Term::Var(9), Term::Var(21), None));
	a.insert(Quad(Term::Var(39), Term::Ground(20), Term::Ground(4), None));
	a.insert(Quad(Term::Var(28), Term::Var(49), Term::Ground(14), None));
	a.insert(Quad(
		Term::Ground(43),
		Term::Ground(12),
		Term::Var(30),
		None,
	));
	a.insert(Quad(Term::Var(0), Term::Ground(19), Term::Var(24), None));
	a.insert(Quad(Term::Var(27), Term::Var(12), Term::Ground(3), None));
	a.insert(Quad(
		Term::Var(14),
		Term::Ground(27),
		Term::Ground(26),
		None,
	));
	a.insert(Quad(Term::Var(42), Term::Var(28), Term::Ground(5), None));
	a.insert(Quad(
		Term::Ground(42),
		Term::Ground(23),
		Term::Ground(16),
		None,
	));
	a.insert(Quad(Term::Ground(3), Term::Var(9), Term::Ground(27), None));
	a.insert(Quad(
		Term::Ground(14),
		Term::Var(47),
		Term::Ground(32),
		None,
	));
	a.insert(Quad(
		Term::Ground(17),
		Term::Ground(12),
		Term::Ground(14),
		None,
	));
	a.insert(Quad(
		Term::Ground(29),
		Term::Ground(32),
		Term::Ground(3),
		None,
	));
	a.insert(Quad(Term::Var(49), Term::Var(20), Term::Var(10), None));
	a.insert(Quad(Term::Ground(37), Term::Var(19), Term::Var(12), None));
	a.insert(Quad(Term::Var(35), Term::Var(49), Term::Ground(10), None));
	a.insert(Quad(
		Term::Ground(15),
		Term::Ground(14),
		Term::Ground(0),
		None,
	));
	a.insert(Quad(Term::Var(28), Term::Var(28), Term::Ground(2), None));
	a.insert(Quad(Term::Var(29), Term::Var(33), Term::Ground(26), None));
	a.insert(Quad(Term::Var(49), Term::Var(21), Term::Ground(31), None));
	a.insert(Quad(
		Term::Ground(32),
		Term::Ground(24),
		Term::Ground(35),
		None,
	));
	a.insert(Quad(Term::Var(22), Term::Ground(2), Term::Ground(1), None));
	a.insert(Quad(Term::Var(16), Term::Ground(44), Term::Ground(2), None));
	a.insert(Quad(Term::Var(34), Term::Ground(12), Term::Var(19), None));
	a.insert(Quad(
		Term::Var(28),
		Term::Ground(34),
		Term::Ground(34),
		None,
	));
	a.insert(Quad(
		Term::Ground(37),
		Term::Var(47),
		Term::Ground(39),
		None,
	));
	a.insert(Quad(Term::Var(41), Term::Ground(23), Term::Var(20), None));
	a.insert(Quad(Term::Var(4), Term::Var(38), Term::Var(39), None));
	a.insert(Quad(
		Term::Ground(26),
		Term::Var(35),
		Term::Ground(49),
		None,
	));
	a.insert(Quad(
		Term::Ground(6),
		Term::Ground(26),
		Term::Ground(23),
		None,
	));
	a.insert(Quad(Term::Var(32), Term::Var(16), Term::Var(7), None));
	a.insert(Quad(Term::Var(28), Term::Ground(49), Term::Var(4), None));
	a.insert(Quad(Term::Ground(37), Term::Ground(13), Term::Var(6), None));
	a.insert(Quad(Term::Var(49), Term::Var(13), Term::Var(47), None));
	a.insert(Quad(
		Term::Var(40),
		Term::Ground(10),
		Term::Ground(11),
		None,
	));
	a.insert(Quad(Term::Var(27), Term::Var(18), Term::Var(5), None));
	a.insert(Quad(
		Term::Var(36),
		Term::Ground(23),
		Term::Ground(29),
		None,
	));
	a.insert(Quad(
		Term::Var(15),
		Term::Ground(17),
		Term::Ground(49),
		None,
	));
	a.insert(Quad(Term::Ground(13), Term::Var(47), Term::Var(14), None));
	a.insert(Quad(Term::Var(21), Term::Var(13), Term::Var(26), None));
	a.insert(Quad(
		Term::Ground(1),
		Term::Ground(44),
		Term::Ground(9),
		None,
	));
	a.insert(Quad(Term::Var(5), Term::Ground(37), Term::Ground(33), None));
	a.insert(Quad(
		Term::Ground(41),
		Term::Ground(23),
		Term::Var(39),
		None,
	));
	a.insert(Quad(
		Term::Ground(38),
		Term::Ground(3),
		Term::Ground(11),
		None,
	));
	a.insert(Quad(Term::Ground(23), Term::Var(3), Term::Ground(32), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(42),
		Term::Var(1023),
		Term::Ground(11),
		None,
	));
	b.insert(Quad(
		Term::Ground(46),
		Term::Var(1038),
		Term::Ground(11),
		None,
	));
	b.insert(Quad(
		Term::Var(1037),
		Term::Var(1045),
		Term::Var(1015),
		None,
	));
	b.insert(Quad(
		Term::Var(1012),
		Term::Ground(2),
		Term::Var(1031),
		None,
	));
	b.insert(Quad(
		Term::Ground(25),
		Term::Var(1046),
		Term::Var(1025),
		None,
	));
	b.insert(Quad(
		Term::Ground(47),
		Term::Var(1000),
		Term::Var(1011),
		None,
	));
	b.insert(Quad(
		Term::Var(1032),
		Term::Var(1018),
		Term::Var(1013),
		None,
	));
	b.insert(Quad(
		Term::Var(1042),
		Term::Var(1037),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Ground(49),
		Term::Var(1024),
		Term::Var(1017),
		None,
	));
	b.insert(Quad(
		Term::Ground(18),
		Term::Ground(11),
		Term::Ground(28),
		None,
	));
	b.insert(Quad(
		Term::Ground(22),
		Term::Var(1044),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Ground(22),
		Term::Ground(43),
		Term::Ground(47),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Var(1049),
		Term::Ground(40),
		None,
	));
	b.insert(Quad(
		Term::Var(1040),
		Term::Var(1037),
		Term::Ground(11),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Var(1039),
		Term::Var(1016),
		None,
	));
	b.insert(Quad(
		Term::Var(1048),
		Term::Var(1020),
		Term::Ground(11),
		None,
	));
	b.insert(Quad(
		Term::Ground(38),
		Term::Var(1004),
		Term::Var(1025),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Var(1012),
		Term::Ground(31),
		None,
	));
	b.insert(Quad(
		Term::Ground(31),
		Term::Var(1007),
		Term::Var(1035),
		None,
	));
	b.insert(Quad(
		Term::Var(1020),
		Term::Var(1022),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Var(1032),
		Term::Var(1022),
		Term::Var(1027),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Var(1014),
		Term::Ground(41),
		None,
	));
	b.insert(Quad(
		Term::Ground(21),
		Term::Var(1041),
		Term::Ground(37),
		None,
	));
	b.insert(Quad(
		Term::Ground(18),
		Term::Ground(44),
		Term::Ground(37),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(27),
		Term::Var(1022),
		None,
	));
	b.insert(Quad(
		Term::Ground(33),
		Term::Ground(46),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(25),
		Term::Var(1018),
		Term::Var(1029),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Var(1040),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Var(1016),
		Term::Var(1020),
		Term::Ground(16),
		None,
	));
	b.insert(Quad(
		Term::Var(1006),
		Term::Var(1046),
		Term::Var(1049),
		None,
	));
	b.insert(Quad(
		Term::Var(1010),
		Term::Ground(25),
		Term::Var(1024),
		None,
	));
	b.insert(Quad(
		Term::Var(1010),
		Term::Var(1003),
		Term::Var(1014),
		None,
	));
	b.insert(Quad(
		Term::Var(1033),
		Term::Var(1010),
		Term::Ground(30),
		None,
	));
	b.insert(Quad(
		Term::Ground(26),
		Term::Var(1045),
		Term::Var(1024),
		None,
	));
	b.insert(Quad(
		Term::Var(1038),
		Term::Var(1007),
		Term::Ground(47),
		None,
	));
	b.insert(Quad(
		Term::Ground(14),
		Term::Var(1033),
		Term::Var(1037),
		None,
	));
	b.insert(Quad(
		Term::Var(1045),
		Term::Var(1018),
		Term::Var(1030),
		None,
	));
	b.insert(Quad(
		Term::Var(1027),
		Term::Ground(30),
		Term::Var(1035),
		None,
	));
	b.insert(Quad(
		Term::Var(1033),
		Term::Ground(9),
		Term::Var(1030),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Var(1029),
		Term::Var(1019),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Ground(18),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Var(1019),
		Term::Ground(16),
		Term::Ground(41),
		None,
	));
	b.insert(Quad(
		Term::Var(1030),
		Term::Var(1001),
		Term::Var(1026),
		None,
	));
	b.insert(Quad(
		Term::Ground(29),
		Term::Var(1007),
		Term::Ground(40),
		None,
	));
	b.insert(Quad(
		Term::Var(1017),
		Term::Ground(41),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Var(1027),
		Term::Var(1048),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(35),
		Term::Ground(17),
		Term::Var(1029),
		None,
	));
	b.insert(Quad(
		Term::Var(1049),
		Term::Ground(37),
		Term::Ground(21),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(49),
		Term::Var(1017),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Var(1010),
		Term::Var(1041),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(13),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Var(1049),
		Term::Ground(15),
		Term::Var(1048),
		None,
	));
	b.insert(Quad(
		Term::Ground(48),
		Term::Ground(3),
		Term::Ground(11),
		None,
	));
	b.insert(Quad(
		Term::Ground(20),
		Term::Var(1001),
		Term::Var(1048),
		None,
	));
	b.insert(Quad(
		Term::Ground(44),
		Term::Ground(1),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Var(1023),
		Term::Var(1009),
		Term::Var(1021),
		None,
	));
	b.insert(Quad(
		Term::Var(1039),
		Term::Ground(20),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Var(1028),
		Term::Var(1049),
		Term::Ground(14),
		None,
	));
	b.insert(Quad(
		Term::Ground(43),
		Term::Ground(12),
		Term::Var(1030),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(19),
		Term::Var(1024),
		None,
	));
	b.insert(Quad(
		Term::Var(1027),
		Term::Var(1012),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Var(1014),
		Term::Ground(27),
		Term::Ground(26),
		None,
	));
	b.insert(Quad(
		Term::Var(1042),
		Term::Var(1028),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Ground(42),
		Term::Ground(23),
		Term::Ground(16),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Var(1009),
		Term::Ground(27),
		None,
	));
	b.insert(Quad(
		Term::Ground(14),
		Term::Var(1047),
		Term::Ground(32),
		None,
	));
	b.insert(Quad(
		Term::Ground(17),
		Term::Ground(12),
		Term::Ground(14),
		None,
	));
	b.insert(Quad(
		Term::Ground(29),
		Term::Ground(32),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Var(1049),
		Term::Var(1020),
		Term::Var(1010),
		None,
	));
	b.insert(Quad(
		Term::Ground(37),
		Term::Var(1019),
		Term::Var(1012),
		None,
	));
	b.insert(Quad(
		Term::Var(1035),
		Term::Var(1049),
		Term::Ground(10),
		None,
	));
	b.insert(Quad(
		Term::Ground(15),
		Term::Ground(14),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Var(1028),
		Term::Var(1028),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Var(1029),
		Term::Var(1033),
		Term::Ground(26),
		None,
	));
	b.insert(Quad(
		Term::Var(1049),
		Term::Var(1021),
		Term::Ground(31),
		None,
	));
	b.insert(Quad(
		Term::Ground(32),
		Term::Ground(24),
		Term::Ground(35),
		None,
	));
	b.insert(Quad(
		Term::Var(1022),
		Term::Ground(2),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Var(1016),
		Term::Ground(44),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Var(1034),
		Term::Ground(12),
		Term::Var(1019),
		None,
	));
	b.insert(Quad(
		Term::Var(1028),
		Term::Ground(34),
		Term::Ground(34),
		None,
	));
	b.insert(Quad(
		Term::Ground(37),
		Term::Var(1047),
		Term::Ground(39),
		None,
	));
	b.insert(Quad(
		Term::Var(1041),
		Term::Ground(23),
		Term::Var(1020),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Var(1038),
		Term::Var(1039),
		None,
	));
	b.insert(Quad(
		Term::Ground(26),
		Term::Var(1035),
		Term::Ground(49),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Ground(26),
		Term::Ground(23),
		None,
	));
	b.insert(Quad(
		Term::Var(1032),
		Term::Var(1016),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Var(1028),
		Term::Ground(49),
		Term::Var(1004),
		None,
	));
	b.insert(Quad(
		Term::Ground(37),
		Term::Ground(13),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Var(1049),
		Term::Var(1013),
		Term::Var(1047),
		None,
	));
	b.insert(Quad(
		Term::Var(1040),
		Term::Ground(10),
		Term::Ground(11),
		None,
	));
	b.insert(Quad(
		Term::Var(1027),
		Term::Var(1018),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Var(1036),
		Term::Ground(23),
		Term::Ground(29),
		None,
	));
	b.insert(Quad(
		Term::Var(1015),
		Term::Ground(17),
		Term::Ground(49),
		None,
	));
	b.insert(Quad(
		Term::Ground(13),
		Term::Var(1047),
		Term::Var(1014),
		None,
	));
	b.insert(Quad(
		Term::Var(1021),
		Term::Var(1013),
		Term::Var(1026),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(44),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Ground(37),
		Term::Ground(33),
		None,
	));
	b.insert(Quad(
		Term::Ground(41),
		Term::Ground(23),
		Term::Var(1039),
		None,
	));
	b.insert(Quad(
		Term::Ground(38),
		Term::Ground(3),
		Term::Ground(11),
		None,
	));
	b.insert(Quad(
		Term::Ground(23),
		Term::Var(1003),
		Term::Ground(32),
		None,
	));
	test(a, b)
}
#[test]
fn iso_186() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(
		Term::Ground(12),
		Term::Ground(5),
		Term::Ground(35),
		None,
	));
	a.insert(Quad(Term::Var(8), Term::Ground(40), Term::Var(12), None));
	a.insert(Quad(Term::Ground(41), Term::Var(11), Term::Var(44), None));
	a.insert(Quad(
		Term::Ground(15),
		Term::Ground(11),
		Term::Ground(40),
		None,
	));
	a.insert(Quad(Term::Var(48), Term::Var(14), Term::Ground(26), None));
	a.insert(Quad(
		Term::Ground(48),
		Term::Ground(24),
		Term::Var(42),
		None,
	));
	a.insert(Quad(Term::Ground(6), Term::Var(13), Term::Var(0), None));
	a.insert(Quad(Term::Ground(34), Term::Ground(48), Term::Var(0), None));
	a.insert(Quad(Term::Var(11), Term::Var(32), Term::Ground(16), None));
	a.insert(Quad(Term::Ground(4), Term::Ground(49), Term::Var(16), None));
	a.insert(Quad(Term::Var(2), Term::Var(18), Term::Ground(28), None));
	a.insert(Quad(Term::Var(38), Term::Var(42), Term::Var(42), None));
	a.insert(Quad(Term::Ground(37), Term::Ground(17), Term::Var(5), None));
	a.insert(Quad(
		Term::Ground(18),
		Term::Var(27),
		Term::Ground(20),
		None,
	));
	a.insert(Quad(Term::Ground(24), Term::Ground(18), Term::Var(7), None));
	a.insert(Quad(Term::Var(19), Term::Var(17), Term::Var(24), None));
	a.insert(Quad(
		Term::Ground(41),
		Term::Ground(17),
		Term::Var(25),
		None,
	));
	a.insert(Quad(Term::Var(27), Term::Var(36), Term::Var(5), None));
	a.insert(Quad(Term::Ground(2), Term::Ground(3), Term::Var(41), None));
	a.insert(Quad(Term::Ground(27), Term::Var(37), Term::Var(44), None));
	a.insert(Quad(Term::Var(26), Term::Var(33), Term::Ground(43), None));
	a.insert(Quad(Term::Ground(44), Term::Var(43), Term::Var(30), None));
	a.insert(Quad(Term::Ground(24), Term::Var(40), Term::Var(17), None));
	a.insert(Quad(
		Term::Ground(23),
		Term::Ground(4),
		Term::Ground(47),
		None,
	));
	a.insert(Quad(Term::Var(33), Term::Ground(19), Term::Var(43), None));
	a.insert(Quad(Term::Var(12), Term::Ground(3), Term::Ground(20), None));
	a.insert(Quad(Term::Var(33), Term::Ground(3), Term::Var(10), None));
	a.insert(Quad(Term::Ground(8), Term::Ground(21), Term::Var(40), None));
	a.insert(Quad(Term::Ground(20), Term::Var(12), Term::Var(48), None));
	a.insert(Quad(Term::Var(40), Term::Ground(15), Term::Var(9), None));
	a.insert(Quad(Term::Var(4), Term::Var(36), Term::Var(7), None));
	a.insert(Quad(Term::Var(11), Term::Var(0), Term::Var(11), None));
	a.insert(Quad(Term::Var(39), Term::Var(30), Term::Var(6), None));
	a.insert(Quad(Term::Var(3), Term::Var(28), Term::Ground(32), None));
	a.insert(Quad(Term::Var(32), Term::Var(9), Term::Var(13), None));
	a.insert(Quad(
		Term::Ground(43),
		Term::Var(39),
		Term::Ground(12),
		None,
	));
	a.insert(Quad(Term::Ground(3), Term::Var(4), Term::Var(8), None));
	a.insert(Quad(Term::Var(17), Term::Var(9), Term::Var(41), None));
	a.insert(Quad(Term::Var(31), Term::Var(48), Term::Ground(33), None));
	a.insert(Quad(Term::Ground(6), Term::Var(22), Term::Var(41), None));
	a.insert(Quad(Term::Ground(47), Term::Ground(3), Term::Var(28), None));
	a.insert(Quad(Term::Var(25), Term::Ground(49), Term::Var(48), None));
	a.insert(Quad(Term::Var(18), Term::Ground(21), Term::Var(2), None));
	a.insert(Quad(Term::Ground(39), Term::Var(44), Term::Var(30), None));
	a.insert(Quad(Term::Ground(39), Term::Ground(28), Term::Var(2), None));
	a.insert(Quad(
		Term::Ground(29),
		Term::Var(14),
		Term::Ground(21),
		None,
	));
	a.insert(Quad(
		Term::Ground(22),
		Term::Ground(29),
		Term::Ground(30),
		None,
	));
	a.insert(Quad(Term::Var(5), Term::Var(25), Term::Var(20), None));
	a.insert(Quad(Term::Ground(40), Term::Var(17), Term::Var(19), None));
	a.insert(Quad(Term::Var(43), Term::Ground(16), Term::Var(18), None));
	a.insert(Quad(
		Term::Ground(5),
		Term::Ground(38),
		Term::Ground(38),
		None,
	));
	a.insert(Quad(Term::Ground(18), Term::Var(4), Term::Var(16), None));
	a.insert(Quad(Term::Ground(17), Term::Var(8), Term::Ground(18), None));
	a.insert(Quad(
		Term::Ground(25),
		Term::Ground(42),
		Term::Ground(21),
		None,
	));
	a.insert(Quad(Term::Var(7), Term::Var(30), Term::Var(49), None));
	a.insert(Quad(Term::Var(44), Term::Var(23), Term::Var(15), None));
	a.insert(Quad(
		Term::Ground(37),
		Term::Ground(45),
		Term::Var(47),
		None,
	));
	a.insert(Quad(Term::Ground(10), Term::Ground(1), Term::Var(45), None));
	a.insert(Quad(Term::Ground(42), Term::Var(5), Term::Var(23), None));
	a.insert(Quad(
		Term::Ground(18),
		Term::Var(29),
		Term::Ground(39),
		None,
	));
	a.insert(Quad(Term::Ground(25), Term::Ground(6), Term::Var(44), None));
	a.insert(Quad(Term::Var(18), Term::Var(33), Term::Var(16), None));
	a.insert(Quad(Term::Var(33), Term::Ground(44), Term::Var(6), None));
	a.insert(Quad(
		Term::Ground(42),
		Term::Ground(46),
		Term::Ground(34),
		None,
	));
	a.insert(Quad(Term::Var(3), Term::Ground(14), Term::Ground(39), None));
	a.insert(Quad(Term::Ground(2), Term::Ground(38), Term::Var(15), None));
	a.insert(Quad(Term::Ground(28), Term::Var(16), Term::Var(45), None));
	a.insert(Quad(
		Term::Var(41),
		Term::Ground(37),
		Term::Ground(47),
		None,
	));
	a.insert(Quad(
		Term::Var(46),
		Term::Ground(48),
		Term::Ground(11),
		None,
	));
	a.insert(Quad(
		Term::Ground(28),
		Term::Ground(46),
		Term::Ground(7),
		None,
	));
	a.insert(Quad(Term::Var(24), Term::Ground(40), Term::Var(35), None));
	a.insert(Quad(Term::Var(1), Term::Var(46), Term::Var(27), None));
	a.insert(Quad(Term::Ground(2), Term::Var(17), Term::Ground(6), None));
	a.insert(Quad(
		Term::Ground(24),
		Term::Ground(6),
		Term::Ground(32),
		None,
	));
	a.insert(Quad(
		Term::Ground(33),
		Term::Ground(13),
		Term::Ground(30),
		None,
	));
	a.insert(Quad(Term::Var(23), Term::Var(13), Term::Var(0), None));
	a.insert(Quad(Term::Var(10), Term::Var(33), Term::Ground(4), None));
	a.insert(Quad(Term::Ground(2), Term::Var(39), Term::Ground(22), None));
	a.insert(Quad(
		Term::Ground(20),
		Term::Ground(25),
		Term::Ground(14),
		None,
	));
	a.insert(Quad(Term::Var(10), Term::Ground(32), Term::Ground(6), None));
	a.insert(Quad(Term::Var(47), Term::Var(31), Term::Ground(10), None));
	a.insert(Quad(
		Term::Var(26),
		Term::Ground(48),
		Term::Ground(33),
		None,
	));
	a.insert(Quad(Term::Ground(28), Term::Var(35), Term::Var(11), None));
	a.insert(Quad(
		Term::Ground(9),
		Term::Ground(40),
		Term::Ground(38),
		None,
	));
	a.insert(Quad(
		Term::Ground(13),
		Term::Ground(21),
		Term::Ground(0),
		None,
	));
	a.insert(Quad(Term::Var(3), Term::Var(39), Term::Ground(5), None));
	a.insert(Quad(Term::Ground(30), Term::Var(22), Term::Ground(7), None));
	a.insert(Quad(Term::Ground(12), Term::Ground(30), Term::Var(9), None));
	a.insert(Quad(Term::Ground(35), Term::Var(47), Term::Var(8), None));
	a.insert(Quad(Term::Ground(8), Term::Var(30), Term::Var(30), None));
	a.insert(Quad(Term::Var(48), Term::Var(15), Term::Ground(23), None));
	a.insert(Quad(Term::Var(2), Term::Ground(5), Term::Ground(10), None));
	a.insert(Quad(Term::Var(27), Term::Var(20), Term::Ground(19), None));
	a.insert(Quad(Term::Var(27), Term::Var(23), Term::Var(18), None));
	a.insert(Quad(
		Term::Ground(15),
		Term::Ground(19),
		Term::Var(14),
		None,
	));
	a.insert(Quad(Term::Var(46), Term::Ground(35), Term::Var(37), None));
	a.insert(Quad(
		Term::Ground(31),
		Term::Var(24),
		Term::Ground(44),
		None,
	));
	a.insert(Quad(Term::Ground(18), Term::Var(35), Term::Var(16), None));
	a.insert(Quad(Term::Var(44), Term::Var(5), Term::Var(35), None));
	a.insert(Quad(Term::Var(31), Term::Var(23), Term::Ground(22), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(12),
		Term::Ground(5),
		Term::Ground(35),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Ground(40),
		Term::Var(1012),
		None,
	));
	b.insert(Quad(
		Term::Ground(41),
		Term::Var(1011),
		Term::Var(1044),
		None,
	));
	b.insert(Quad(
		Term::Ground(15),
		Term::Ground(11),
		Term::Ground(40),
		None,
	));
	b.insert(Quad(
		Term::Var(1048),
		Term::Var(1014),
		Term::Ground(26),
		None,
	));
	b.insert(Quad(
		Term::Ground(48),
		Term::Ground(24),
		Term::Var(1042),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Var(1013),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Ground(34),
		Term::Ground(48),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Var(1011),
		Term::Var(1032),
		Term::Ground(16),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Ground(49),
		Term::Var(1016),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Var(1018),
		Term::Ground(28),
		None,
	));
	b.insert(Quad(
		Term::Var(1038),
		Term::Var(1042),
		Term::Var(1042),
		None,
	));
	b.insert(Quad(
		Term::Ground(37),
		Term::Ground(17),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Ground(18),
		Term::Var(1027),
		Term::Ground(20),
		None,
	));
	b.insert(Quad(
		Term::Ground(24),
		Term::Ground(18),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Var(1019),
		Term::Var(1017),
		Term::Var(1024),
		None,
	));
	b.insert(Quad(
		Term::Ground(41),
		Term::Ground(17),
		Term::Var(1025),
		None,
	));
	b.insert(Quad(
		Term::Var(1027),
		Term::Var(1036),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(3),
		Term::Var(1041),
		None,
	));
	b.insert(Quad(
		Term::Ground(27),
		Term::Var(1037),
		Term::Var(1044),
		None,
	));
	b.insert(Quad(
		Term::Var(1026),
		Term::Var(1033),
		Term::Ground(43),
		None,
	));
	b.insert(Quad(
		Term::Ground(44),
		Term::Var(1043),
		Term::Var(1030),
		None,
	));
	b.insert(Quad(
		Term::Ground(24),
		Term::Var(1040),
		Term::Var(1017),
		None,
	));
	b.insert(Quad(
		Term::Ground(23),
		Term::Ground(4),
		Term::Ground(47),
		None,
	));
	b.insert(Quad(
		Term::Var(1033),
		Term::Ground(19),
		Term::Var(1043),
		None,
	));
	b.insert(Quad(
		Term::Var(1012),
		Term::Ground(3),
		Term::Ground(20),
		None,
	));
	b.insert(Quad(
		Term::Var(1033),
		Term::Ground(3),
		Term::Var(1010),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Ground(21),
		Term::Var(1040),
		None,
	));
	b.insert(Quad(
		Term::Ground(20),
		Term::Var(1012),
		Term::Var(1048),
		None,
	));
	b.insert(Quad(
		Term::Var(1040),
		Term::Ground(15),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Var(1036),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Var(1011),
		Term::Var(1000),
		Term::Var(1011),
		None,
	));
	b.insert(Quad(
		Term::Var(1039),
		Term::Var(1030),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Var(1028),
		Term::Ground(32),
		None,
	));
	b.insert(Quad(
		Term::Var(1032),
		Term::Var(1009),
		Term::Var(1013),
		None,
	));
	b.insert(Quad(
		Term::Ground(43),
		Term::Var(1039),
		Term::Ground(12),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Var(1004),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Var(1017),
		Term::Var(1009),
		Term::Var(1041),
		None,
	));
	b.insert(Quad(
		Term::Var(1031),
		Term::Var(1048),
		Term::Ground(33),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Var(1022),
		Term::Var(1041),
		None,
	));
	b.insert(Quad(
		Term::Ground(47),
		Term::Ground(3),
		Term::Var(1028),
		None,
	));
	b.insert(Quad(
		Term::Var(1025),
		Term::Ground(49),
		Term::Var(1048),
		None,
	));
	b.insert(Quad(
		Term::Var(1018),
		Term::Ground(21),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Ground(39),
		Term::Var(1044),
		Term::Var(1030),
		None,
	));
	b.insert(Quad(
		Term::Ground(39),
		Term::Ground(28),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Ground(29),
		Term::Var(1014),
		Term::Ground(21),
		None,
	));
	b.insert(Quad(
		Term::Ground(22),
		Term::Ground(29),
		Term::Ground(30),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Var(1025),
		Term::Var(1020),
		None,
	));
	b.insert(Quad(
		Term::Ground(40),
		Term::Var(1017),
		Term::Var(1019),
		None,
	));
	b.insert(Quad(
		Term::Var(1043),
		Term::Ground(16),
		Term::Var(1018),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Ground(38),
		Term::Ground(38),
		None,
	));
	b.insert(Quad(
		Term::Ground(18),
		Term::Var(1004),
		Term::Var(1016),
		None,
	));
	b.insert(Quad(
		Term::Ground(17),
		Term::Var(1008),
		Term::Ground(18),
		None,
	));
	b.insert(Quad(
		Term::Ground(25),
		Term::Ground(42),
		Term::Ground(21),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Var(1030),
		Term::Var(1049),
		None,
	));
	b.insert(Quad(
		Term::Var(1044),
		Term::Var(1023),
		Term::Var(1015),
		None,
	));
	b.insert(Quad(
		Term::Ground(37),
		Term::Ground(45),
		Term::Var(1047),
		None,
	));
	b.insert(Quad(
		Term::Ground(10),
		Term::Ground(1),
		Term::Var(1045),
		None,
	));
	b.insert(Quad(
		Term::Ground(42),
		Term::Var(1005),
		Term::Var(1023),
		None,
	));
	b.insert(Quad(
		Term::Ground(18),
		Term::Var(1029),
		Term::Ground(39),
		None,
	));
	b.insert(Quad(
		Term::Ground(25),
		Term::Ground(6),
		Term::Var(1044),
		None,
	));
	b.insert(Quad(
		Term::Var(1018),
		Term::Var(1033),
		Term::Var(1016),
		None,
	));
	b.insert(Quad(
		Term::Var(1033),
		Term::Ground(44),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Ground(42),
		Term::Ground(46),
		Term::Ground(34),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Ground(14),
		Term::Ground(39),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(38),
		Term::Var(1015),
		None,
	));
	b.insert(Quad(
		Term::Ground(28),
		Term::Var(1016),
		Term::Var(1045),
		None,
	));
	b.insert(Quad(
		Term::Var(1041),
		Term::Ground(37),
		Term::Ground(47),
		None,
	));
	b.insert(Quad(
		Term::Var(1046),
		Term::Ground(48),
		Term::Ground(11),
		None,
	));
	b.insert(Quad(
		Term::Ground(28),
		Term::Ground(46),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Var(1024),
		Term::Ground(40),
		Term::Var(1035),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Var(1046),
		Term::Var(1027),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Var(1017),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Ground(24),
		Term::Ground(6),
		Term::Ground(32),
		None,
	));
	b.insert(Quad(
		Term::Ground(33),
		Term::Ground(13),
		Term::Ground(30),
		None,
	));
	b.insert(Quad(
		Term::Var(1023),
		Term::Var(1013),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Var(1010),
		Term::Var(1033),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Var(1039),
		Term::Ground(22),
		None,
	));
	b.insert(Quad(
		Term::Ground(20),
		Term::Ground(25),
		Term::Ground(14),
		None,
	));
	b.insert(Quad(
		Term::Var(1010),
		Term::Ground(32),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Var(1047),
		Term::Var(1031),
		Term::Ground(10),
		None,
	));
	b.insert(Quad(
		Term::Var(1026),
		Term::Ground(48),
		Term::Ground(33),
		None,
	));
	b.insert(Quad(
		Term::Ground(28),
		Term::Var(1035),
		Term::Var(1011),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Ground(40),
		Term::Ground(38),
		None,
	));
	b.insert(Quad(
		Term::Ground(13),
		Term::Ground(21),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Var(1039),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Ground(30),
		Term::Var(1022),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Ground(12),
		Term::Ground(30),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Ground(35),
		Term::Var(1047),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Var(1030),
		Term::Var(1030),
		None,
	));
	b.insert(Quad(
		Term::Var(1048),
		Term::Var(1015),
		Term::Ground(23),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Ground(5),
		Term::Ground(10),
		None,
	));
	b.insert(Quad(
		Term::Var(1027),
		Term::Var(1020),
		Term::Ground(19),
		None,
	));
	b.insert(Quad(
		Term::Var(1027),
		Term::Var(1023),
		Term::Var(1018),
		None,
	));
	b.insert(Quad(
		Term::Ground(15),
		Term::Ground(19),
		Term::Var(1014),
		None,
	));
	b.insert(Quad(
		Term::Var(1046),
		Term::Ground(35),
		Term::Var(1037),
		None,
	));
	b.insert(Quad(
		Term::Ground(31),
		Term::Var(1024),
		Term::Ground(44),
		None,
	));
	b.insert(Quad(
		Term::Ground(18),
		Term::Var(1035),
		Term::Var(1016),
		None,
	));
	b.insert(Quad(
		Term::Var(1044),
		Term::Var(1005),
		Term::Var(1035),
		None,
	));
	b.insert(Quad(
		Term::Var(1031),
		Term::Var(1023),
		Term::Ground(22),
		None,
	));
	test(a, b)
}
#[test]
fn iso_187() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(
		Term::Ground(33),
		Term::Ground(10),
		Term::Var(11),
		None,
	));
	a.insert(Quad(Term::Var(12), Term::Var(48), Term::Var(8), None));
	a.insert(Quad(Term::Ground(29), Term::Ground(8), Term::Var(0), None));
	a.insert(Quad(
		Term::Ground(19),
		Term::Ground(17),
		Term::Ground(25),
		None,
	));
	a.insert(Quad(
		Term::Ground(30),
		Term::Ground(13),
		Term::Var(39),
		None,
	));
	a.insert(Quad(
		Term::Ground(14),
		Term::Ground(20),
		Term::Var(23),
		None,
	));
	a.insert(Quad(Term::Ground(31), Term::Var(40), Term::Var(31), None));
	a.insert(Quad(Term::Var(46), Term::Var(15), Term::Var(39), None));
	a.insert(Quad(Term::Ground(2), Term::Var(5), Term::Var(27), None));
	a.insert(Quad(Term::Var(26), Term::Var(17), Term::Ground(33), None));
	a.insert(Quad(Term::Var(45), Term::Ground(0), Term::Var(21), None));
	a.insert(Quad(Term::Ground(10), Term::Var(26), Term::Var(44), None));
	a.insert(Quad(
		Term::Ground(22),
		Term::Ground(0),
		Term::Ground(0),
		None,
	));
	a.insert(Quad(Term::Var(20), Term::Var(41), Term::Ground(39), None));
	a.insert(Quad(Term::Var(3), Term::Ground(17), Term::Ground(4), None));
	a.insert(Quad(
		Term::Ground(37),
		Term::Ground(0),
		Term::Ground(49),
		None,
	));
	a.insert(Quad(Term::Var(43), Term::Var(10), Term::Var(43), None));
	a.insert(Quad(
		Term::Ground(15),
		Term::Ground(19),
		Term::Var(11),
		None,
	));
	a.insert(Quad(Term::Var(12), Term::Var(15), Term::Ground(9), None));
	a.insert(Quad(Term::Ground(11), Term::Var(4), Term::Var(32), None));
	a.insert(Quad(
		Term::Var(10),
		Term::Ground(10),
		Term::Ground(42),
		None,
	));
	a.insert(Quad(Term::Ground(4), Term::Var(8), Term::Var(22), None));
	a.insert(Quad(Term::Var(12), Term::Var(35), Term::Ground(31), None));
	a.insert(Quad(
		Term::Ground(16),
		Term::Var(44),
		Term::Ground(16),
		None,
	));
	a.insert(Quad(
		Term::Var(49),
		Term::Ground(30),
		Term::Ground(23),
		None,
	));
	a.insert(Quad(
		Term::Ground(48),
		Term::Ground(46),
		Term::Var(25),
		None,
	));
	a.insert(Quad(
		Term::Ground(31),
		Term::Var(21),
		Term::Ground(47),
		None,
	));
	a.insert(Quad(Term::Var(14), Term::Ground(16), Term::Var(7), None));
	a.insert(Quad(Term::Var(44), Term::Var(10), Term::Ground(8), None));
	a.insert(Quad(
		Term::Var(44),
		Term::Ground(26),
		Term::Ground(34),
		None,
	));
	a.insert(Quad(
		Term::Ground(37),
		Term::Ground(11),
		Term::Ground(11),
		None,
	));
	a.insert(Quad(
		Term::Ground(15),
		Term::Ground(4),
		Term::Ground(11),
		None,
	));
	a.insert(Quad(Term::Var(16), Term::Ground(18), Term::Var(18), None));
	a.insert(Quad(Term::Ground(6), Term::Var(27), Term::Ground(38), None));
	a.insert(Quad(Term::Var(42), Term::Ground(6), Term::Ground(2), None));
	a.insert(Quad(Term::Var(3), Term::Ground(43), Term::Var(16), None));
	a.insert(Quad(Term::Var(15), Term::Ground(30), Term::Var(28), None));
	a.insert(Quad(Term::Ground(15), Term::Var(43), Term::Var(25), None));
	a.insert(Quad(Term::Var(47), Term::Ground(29), Term::Ground(2), None));
	a.insert(Quad(
		Term::Ground(13),
		Term::Ground(12),
		Term::Ground(30),
		None,
	));
	a.insert(Quad(Term::Var(1), Term::Var(49), Term::Ground(27), None));
	a.insert(Quad(
		Term::Ground(44),
		Term::Ground(13),
		Term::Ground(15),
		None,
	));
	a.insert(Quad(Term::Var(7), Term::Var(35), Term::Var(30), None));
	a.insert(Quad(
		Term::Ground(44),
		Term::Ground(18),
		Term::Ground(26),
		None,
	));
	a.insert(Quad(Term::Ground(44), Term::Var(11), Term::Ground(6), None));
	a.insert(Quad(
		Term::Ground(37),
		Term::Ground(19),
		Term::Var(11),
		None,
	));
	a.insert(Quad(
		Term::Ground(39),
		Term::Ground(4),
		Term::Ground(34),
		None,
	));
	a.insert(Quad(Term::Var(49), Term::Var(34), Term::Ground(24), None));
	a.insert(Quad(
		Term::Ground(25),
		Term::Var(49),
		Term::Ground(17),
		None,
	));
	a.insert(Quad(Term::Var(49), Term::Var(13), Term::Ground(37), None));
	a.insert(Quad(Term::Var(17), Term::Ground(40), Term::Var(23), None));
	a.insert(Quad(Term::Var(25), Term::Ground(43), Term::Var(5), None));
	a.insert(Quad(Term::Var(47), Term::Ground(22), Term::Var(9), None));
	a.insert(Quad(Term::Var(39), Term::Var(46), Term::Ground(5), None));
	a.insert(Quad(
		Term::Ground(36),
		Term::Var(47),
		Term::Ground(34),
		None,
	));
	a.insert(Quad(Term::Var(32), Term::Var(48), Term::Var(24), None));
	a.insert(Quad(Term::Var(21), Term::Ground(18), Term::Var(47), None));
	a.insert(Quad(Term::Var(4), Term::Ground(29), Term::Ground(26), None));
	a.insert(Quad(Term::Var(29), Term::Var(39), Term::Ground(14), None));
	a.insert(Quad(Term::Var(42), Term::Ground(5), Term::Var(28), None));
	a.insert(Quad(Term::Var(37), Term::Var(21), Term::Var(6), None));
	a.insert(Quad(
		Term::Var(45),
		Term::Ground(11),
		Term::Ground(47),
		None,
	));
	a.insert(Quad(Term::Var(37), Term::Var(4), Term::Var(46), None));
	a.insert(Quad(Term::Ground(18), Term::Var(20), Term::Var(5), None));
	a.insert(Quad(
		Term::Var(40),
		Term::Ground(16),
		Term::Ground(14),
		None,
	));
	a.insert(Quad(Term::Var(27), Term::Var(42), Term::Ground(1), None));
	a.insert(Quad(Term::Var(27), Term::Var(16), Term::Var(1), None));
	a.insert(Quad(Term::Ground(41), Term::Var(15), Term::Var(20), None));
	a.insert(Quad(
		Term::Ground(31),
		Term::Var(48),
		Term::Ground(22),
		None,
	));
	a.insert(Quad(Term::Var(39), Term::Ground(9), Term::Var(15), None));
	a.insert(Quad(Term::Var(27), Term::Ground(35), Term::Var(26), None));
	a.insert(Quad(Term::Ground(8), Term::Ground(43), Term::Var(49), None));
	a.insert(Quad(Term::Ground(11), Term::Ground(18), Term::Var(1), None));
	a.insert(Quad(Term::Ground(28), Term::Var(31), Term::Var(27), None));
	a.insert(Quad(Term::Var(1), Term::Ground(8), Term::Var(41), None));
	a.insert(Quad(Term::Var(20), Term::Var(47), Term::Var(43), None));
	a.insert(Quad(Term::Var(27), Term::Ground(31), Term::Var(3), None));
	a.insert(Quad(
		Term::Var(37),
		Term::Ground(10),
		Term::Ground(12),
		None,
	));
	a.insert(Quad(Term::Ground(4), Term::Var(7), Term::Ground(18), None));
	a.insert(Quad(Term::Var(42), Term::Var(1), Term::Ground(26), None));
	a.insert(Quad(
		Term::Ground(34),
		Term::Var(44),
		Term::Ground(16),
		None,
	));
	a.insert(Quad(Term::Var(24), Term::Var(10), Term::Var(15), None));
	a.insert(Quad(
		Term::Ground(43),
		Term::Ground(14),
		Term::Var(32),
		None,
	));
	a.insert(Quad(Term::Var(26), Term::Ground(5), Term::Ground(0), None));
	a.insert(Quad(
		Term::Ground(37),
		Term::Ground(37),
		Term::Ground(29),
		None,
	));
	a.insert(Quad(Term::Var(32), Term::Var(35), Term::Ground(21), None));
	a.insert(Quad(Term::Var(22), Term::Var(36), Term::Ground(1), None));
	a.insert(Quad(
		Term::Ground(18),
		Term::Ground(14),
		Term::Ground(33),
		None,
	));
	a.insert(Quad(
		Term::Ground(10),
		Term::Var(38),
		Term::Ground(40),
		None,
	));
	a.insert(Quad(
		Term::Ground(13),
		Term::Ground(12),
		Term::Var(44),
		None,
	));
	a.insert(Quad(Term::Ground(25), Term::Var(45), Term::Var(20), None));
	a.insert(Quad(Term::Var(19), Term::Var(20), Term::Var(17), None));
	a.insert(Quad(
		Term::Ground(49),
		Term::Ground(38),
		Term::Ground(1),
		None,
	));
	a.insert(Quad(
		Term::Var(16),
		Term::Ground(49),
		Term::Ground(18),
		None,
	));
	a.insert(Quad(
		Term::Ground(23),
		Term::Ground(40),
		Term::Ground(33),
		None,
	));
	a.insert(Quad(Term::Var(2), Term::Var(26), Term::Var(20), None));
	a.insert(Quad(Term::Var(12), Term::Ground(32), Term::Var(28), None));
	a.insert(Quad(Term::Ground(32), Term::Var(4), Term::Ground(44), None));
	a.insert(Quad(Term::Var(48), Term::Var(46), Term::Ground(24), None));
	a.insert(Quad(Term::Var(5), Term::Var(11), Term::Ground(32), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(33),
		Term::Ground(10),
		Term::Var(1011),
		None,
	));
	b.insert(Quad(
		Term::Var(1012),
		Term::Var(1048),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Ground(29),
		Term::Ground(8),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Ground(19),
		Term::Ground(17),
		Term::Ground(25),
		None,
	));
	b.insert(Quad(
		Term::Ground(30),
		Term::Ground(13),
		Term::Var(1039),
		None,
	));
	b.insert(Quad(
		Term::Ground(14),
		Term::Ground(20),
		Term::Var(1023),
		None,
	));
	b.insert(Quad(
		Term::Ground(31),
		Term::Var(1040),
		Term::Var(1031),
		None,
	));
	b.insert(Quad(
		Term::Var(1046),
		Term::Var(1015),
		Term::Var(1039),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Var(1005),
		Term::Var(1027),
		None,
	));
	b.insert(Quad(
		Term::Var(1026),
		Term::Var(1017),
		Term::Ground(33),
		None,
	));
	b.insert(Quad(
		Term::Var(1045),
		Term::Ground(0),
		Term::Var(1021),
		None,
	));
	b.insert(Quad(
		Term::Ground(10),
		Term::Var(1026),
		Term::Var(1044),
		None,
	));
	b.insert(Quad(
		Term::Ground(22),
		Term::Ground(0),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Var(1020),
		Term::Var(1041),
		Term::Ground(39),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Ground(17),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Ground(37),
		Term::Ground(0),
		Term::Ground(49),
		None,
	));
	b.insert(Quad(
		Term::Var(1043),
		Term::Var(1010),
		Term::Var(1043),
		None,
	));
	b.insert(Quad(
		Term::Ground(15),
		Term::Ground(19),
		Term::Var(1011),
		None,
	));
	b.insert(Quad(
		Term::Var(1012),
		Term::Var(1015),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Ground(11),
		Term::Var(1004),
		Term::Var(1032),
		None,
	));
	b.insert(Quad(
		Term::Var(1010),
		Term::Ground(10),
		Term::Ground(42),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Var(1008),
		Term::Var(1022),
		None,
	));
	b.insert(Quad(
		Term::Var(1012),
		Term::Var(1035),
		Term::Ground(31),
		None,
	));
	b.insert(Quad(
		Term::Ground(16),
		Term::Var(1044),
		Term::Ground(16),
		None,
	));
	b.insert(Quad(
		Term::Var(1049),
		Term::Ground(30),
		Term::Ground(23),
		None,
	));
	b.insert(Quad(
		Term::Ground(48),
		Term::Ground(46),
		Term::Var(1025),
		None,
	));
	b.insert(Quad(
		Term::Ground(31),
		Term::Var(1021),
		Term::Ground(47),
		None,
	));
	b.insert(Quad(
		Term::Var(1014),
		Term::Ground(16),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Var(1044),
		Term::Var(1010),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Var(1044),
		Term::Ground(26),
		Term::Ground(34),
		None,
	));
	b.insert(Quad(
		Term::Ground(37),
		Term::Ground(11),
		Term::Ground(11),
		None,
	));
	b.insert(Quad(
		Term::Ground(15),
		Term::Ground(4),
		Term::Ground(11),
		None,
	));
	b.insert(Quad(
		Term::Var(1016),
		Term::Ground(18),
		Term::Var(1018),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Var(1027),
		Term::Ground(38),
		None,
	));
	b.insert(Quad(
		Term::Var(1042),
		Term::Ground(6),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Ground(43),
		Term::Var(1016),
		None,
	));
	b.insert(Quad(
		Term::Var(1015),
		Term::Ground(30),
		Term::Var(1028),
		None,
	));
	b.insert(Quad(
		Term::Ground(15),
		Term::Var(1043),
		Term::Var(1025),
		None,
	));
	b.insert(Quad(
		Term::Var(1047),
		Term::Ground(29),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(13),
		Term::Ground(12),
		Term::Ground(30),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Var(1049),
		Term::Ground(27),
		None,
	));
	b.insert(Quad(
		Term::Ground(44),
		Term::Ground(13),
		Term::Ground(15),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Var(1035),
		Term::Var(1030),
		None,
	));
	b.insert(Quad(
		Term::Ground(44),
		Term::Ground(18),
		Term::Ground(26),
		None,
	));
	b.insert(Quad(
		Term::Ground(44),
		Term::Var(1011),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Ground(37),
		Term::Ground(19),
		Term::Var(1011),
		None,
	));
	b.insert(Quad(
		Term::Ground(39),
		Term::Ground(4),
		Term::Ground(34),
		None,
	));
	b.insert(Quad(
		Term::Var(1049),
		Term::Var(1034),
		Term::Ground(24),
		None,
	));
	b.insert(Quad(
		Term::Ground(25),
		Term::Var(1049),
		Term::Ground(17),
		None,
	));
	b.insert(Quad(
		Term::Var(1049),
		Term::Var(1013),
		Term::Ground(37),
		None,
	));
	b.insert(Quad(
		Term::Var(1017),
		Term::Ground(40),
		Term::Var(1023),
		None,
	));
	b.insert(Quad(
		Term::Var(1025),
		Term::Ground(43),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Var(1047),
		Term::Ground(22),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Var(1039),
		Term::Var(1046),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Ground(36),
		Term::Var(1047),
		Term::Ground(34),
		None,
	));
	b.insert(Quad(
		Term::Var(1032),
		Term::Var(1048),
		Term::Var(1024),
		None,
	));
	b.insert(Quad(
		Term::Var(1021),
		Term::Ground(18),
		Term::Var(1047),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Ground(29),
		Term::Ground(26),
		None,
	));
	b.insert(Quad(
		Term::Var(1029),
		Term::Var(1039),
		Term::Ground(14),
		None,
	));
	b.insert(Quad(
		Term::Var(1042),
		Term::Ground(5),
		Term::Var(1028),
		None,
	));
	b.insert(Quad(
		Term::Var(1037),
		Term::Var(1021),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Var(1045),
		Term::Ground(11),
		Term::Ground(47),
		None,
	));
	b.insert(Quad(
		Term::Var(1037),
		Term::Var(1004),
		Term::Var(1046),
		None,
	));
	b.insert(Quad(
		Term::Ground(18),
		Term::Var(1020),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Var(1040),
		Term::Ground(16),
		Term::Ground(14),
		None,
	));
	b.insert(Quad(
		Term::Var(1027),
		Term::Var(1042),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Var(1027),
		Term::Var(1016),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(41),
		Term::Var(1015),
		Term::Var(1020),
		None,
	));
	b.insert(Quad(
		Term::Ground(31),
		Term::Var(1048),
		Term::Ground(22),
		None,
	));
	b.insert(Quad(
		Term::Var(1039),
		Term::Ground(9),
		Term::Var(1015),
		None,
	));
	b.insert(Quad(
		Term::Var(1027),
		Term::Ground(35),
		Term::Var(1026),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Ground(43),
		Term::Var(1049),
		None,
	));
	b.insert(Quad(
		Term::Ground(11),
		Term::Ground(18),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(28),
		Term::Var(1031),
		Term::Var(1027),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(8),
		Term::Var(1041),
		None,
	));
	b.insert(Quad(
		Term::Var(1020),
		Term::Var(1047),
		Term::Var(1043),
		None,
	));
	b.insert(Quad(
		Term::Var(1027),
		Term::Ground(31),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Var(1037),
		Term::Ground(10),
		Term::Ground(12),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Var(1007),
		Term::Ground(18),
		None,
	));
	b.insert(Quad(
		Term::Var(1042),
		Term::Var(1001),
		Term::Ground(26),
		None,
	));
	b.insert(Quad(
		Term::Ground(34),
		Term::Var(1044),
		Term::Ground(16),
		None,
	));
	b.insert(Quad(
		Term::Var(1024),
		Term::Var(1010),
		Term::Var(1015),
		None,
	));
	b.insert(Quad(
		Term::Ground(43),
		Term::Ground(14),
		Term::Var(1032),
		None,
	));
	b.insert(Quad(
		Term::Var(1026),
		Term::Ground(5),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(37),
		Term::Ground(37),
		Term::Ground(29),
		None,
	));
	b.insert(Quad(
		Term::Var(1032),
		Term::Var(1035),
		Term::Ground(21),
		None,
	));
	b.insert(Quad(
		Term::Var(1022),
		Term::Var(1036),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Ground(18),
		Term::Ground(14),
		Term::Ground(33),
		None,
	));
	b.insert(Quad(
		Term::Ground(10),
		Term::Var(1038),
		Term::Ground(40),
		None,
	));
	b.insert(Quad(
		Term::Ground(13),
		Term::Ground(12),
		Term::Var(1044),
		None,
	));
	b.insert(Quad(
		Term::Ground(25),
		Term::Var(1045),
		Term::Var(1020),
		None,
	));
	b.insert(Quad(
		Term::Var(1019),
		Term::Var(1020),
		Term::Var(1017),
		None,
	));
	b.insert(Quad(
		Term::Ground(49),
		Term::Ground(38),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Var(1016),
		Term::Ground(49),
		Term::Ground(18),
		None,
	));
	b.insert(Quad(
		Term::Ground(23),
		Term::Ground(40),
		Term::Ground(33),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Var(1026),
		Term::Var(1020),
		None,
	));
	b.insert(Quad(
		Term::Var(1012),
		Term::Ground(32),
		Term::Var(1028),
		None,
	));
	b.insert(Quad(
		Term::Ground(32),
		Term::Var(1004),
		Term::Ground(44),
		None,
	));
	b.insert(Quad(
		Term::Var(1048),
		Term::Var(1046),
		Term::Ground(24),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Var(1011),
		Term::Ground(32),
		None,
	));
	test(a, b)
}
#[test]
fn iso_188() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Var(18), Term::Ground(23), Term::Var(19), None));
	a.insert(Quad(Term::Var(23), Term::Var(3), Term::Ground(22), None));
	a.insert(Quad(Term::Var(14), Term::Var(38), Term::Ground(39), None));
	a.insert(Quad(Term::Ground(24), Term::Var(31), Term::Var(26), None));
	a.insert(Quad(
		Term::Ground(48),
		Term::Var(32),
		Term::Ground(19),
		None,
	));
	a.insert(Quad(Term::Var(18), Term::Ground(2), Term::Ground(7), None));
	a.insert(Quad(
		Term::Var(27),
		Term::Ground(29),
		Term::Ground(48),
		None,
	));
	a.insert(Quad(
		Term::Ground(48),
		Term::Ground(25),
		Term::Ground(15),
		None,
	));
	a.insert(Quad(
		Term::Ground(45),
		Term::Ground(24),
		Term::Var(42),
		None,
	));
	a.insert(Quad(
		Term::Ground(39),
		Term::Ground(48),
		Term::Var(26),
		None,
	));
	a.insert(Quad(Term::Ground(26), Term::Ground(33), Term::Var(0), None));
	a.insert(Quad(Term::Var(9), Term::Var(28), Term::Var(32), None));
	a.insert(Quad(Term::Ground(7), Term::Var(37), Term::Var(25), None));
	a.insert(Quad(Term::Var(30), Term::Var(27), Term::Var(12), None));
	a.insert(Quad(Term::Var(12), Term::Var(12), Term::Ground(15), None));
	a.insert(Quad(Term::Var(37), Term::Var(5), Term::Ground(4), None));
	a.insert(Quad(Term::Ground(20), Term::Var(15), Term::Var(4), None));
	a.insert(Quad(Term::Var(21), Term::Var(8), Term::Var(5), None));
	a.insert(Quad(Term::Var(34), Term::Var(35), Term::Var(46), None));
	a.insert(Quad(Term::Ground(11), Term::Var(10), Term::Var(16), None));
	a.insert(Quad(
		Term::Var(30),
		Term::Ground(30),
		Term::Ground(37),
		None,
	));
	a.insert(Quad(Term::Ground(39), Term::Ground(17), Term::Var(1), None));
	a.insert(Quad(Term::Var(6), Term::Var(39), Term::Var(29), None));
	a.insert(Quad(
		Term::Ground(35),
		Term::Var(14),
		Term::Ground(36),
		None,
	));
	a.insert(Quad(Term::Var(2), Term::Var(29), Term::Var(33), None));
	a.insert(Quad(Term::Var(42), Term::Var(1), Term::Var(33), None));
	a.insert(Quad(
		Term::Ground(24),
		Term::Var(30),
		Term::Ground(35),
		None,
	));
	a.insert(Quad(Term::Ground(37), Term::Var(37), Term::Var(38), None));
	a.insert(Quad(Term::Var(45), Term::Ground(47), Term::Var(9), None));
	a.insert(Quad(Term::Ground(37), Term::Var(34), Term::Var(31), None));
	a.insert(Quad(Term::Var(17), Term::Var(19), Term::Ground(48), None));
	a.insert(Quad(Term::Ground(6), Term::Ground(33), Term::Var(0), None));
	a.insert(Quad(Term::Var(35), Term::Var(2), Term::Ground(28), None));
	a.insert(Quad(
		Term::Ground(36),
		Term::Ground(13),
		Term::Ground(22),
		None,
	));
	a.insert(Quad(
		Term::Var(45),
		Term::Ground(47),
		Term::Ground(15),
		None,
	));
	a.insert(Quad(Term::Var(34), Term::Var(36), Term::Var(16), None));
	a.insert(Quad(Term::Ground(4), Term::Var(32), Term::Var(4), None));
	a.insert(Quad(
		Term::Ground(0),
		Term::Ground(32),
		Term::Ground(49),
		None,
	));
	a.insert(Quad(Term::Var(38), Term::Var(38), Term::Var(4), None));
	a.insert(Quad(Term::Ground(1), Term::Var(27), Term::Ground(24), None));
	a.insert(Quad(
		Term::Ground(13),
		Term::Var(14),
		Term::Ground(40),
		None,
	));
	a.insert(Quad(
		Term::Ground(34),
		Term::Ground(49),
		Term::Ground(49),
		None,
	));
	a.insert(Quad(Term::Var(31), Term::Ground(12), Term::Ground(4), None));
	a.insert(Quad(
		Term::Ground(26),
		Term::Ground(43),
		Term::Ground(31),
		None,
	));
	a.insert(Quad(
		Term::Ground(25),
		Term::Ground(28),
		Term::Ground(26),
		None,
	));
	a.insert(Quad(Term::Var(27), Term::Ground(23), Term::Var(23), None));
	a.insert(Quad(Term::Var(13), Term::Ground(15), Term::Var(21), None));
	a.insert(Quad(Term::Ground(28), Term::Var(25), Term::Var(41), None));
	a.insert(Quad(Term::Ground(5), Term::Var(43), Term::Ground(29), None));
	a.insert(Quad(Term::Ground(3), Term::Ground(0), Term::Var(11), None));
	a.insert(Quad(Term::Ground(3), Term::Ground(23), Term::Var(21), None));
	a.insert(Quad(Term::Var(15), Term::Ground(5), Term::Var(6), None));
	a.insert(Quad(Term::Ground(48), Term::Var(33), Term::Var(15), None));
	a.insert(Quad(
		Term::Ground(45),
		Term::Ground(14),
		Term::Ground(42),
		None,
	));
	a.insert(Quad(Term::Var(30), Term::Ground(0), Term::Var(39), None));
	a.insert(Quad(Term::Var(46), Term::Var(31), Term::Ground(47), None));
	a.insert(Quad(Term::Var(38), Term::Ground(20), Term::Ground(2), None));
	a.insert(Quad(Term::Var(4), Term::Var(18), Term::Ground(1), None));
	a.insert(Quad(
		Term::Ground(13),
		Term::Ground(38),
		Term::Var(29),
		None,
	));
	a.insert(Quad(Term::Var(18), Term::Ground(26), Term::Var(40), None));
	a.insert(Quad(Term::Var(5), Term::Var(44), Term::Ground(38), None));
	a.insert(Quad(Term::Var(8), Term::Var(30), Term::Ground(27), None));
	a.insert(Quad(Term::Var(2), Term::Var(23), Term::Ground(2), None));
	a.insert(Quad(Term::Var(28), Term::Var(41), Term::Ground(15), None));
	a.insert(Quad(Term::Ground(17), Term::Var(44), Term::Ground(5), None));
	a.insert(Quad(Term::Var(44), Term::Ground(19), Term::Var(46), None));
	a.insert(Quad(Term::Var(20), Term::Var(47), Term::Ground(46), None));
	a.insert(Quad(Term::Var(22), Term::Var(20), Term::Ground(13), None));
	a.insert(Quad(
		Term::Ground(10),
		Term::Ground(20),
		Term::Var(43),
		None,
	));
	a.insert(Quad(Term::Var(30), Term::Ground(25), Term::Var(15), None));
	a.insert(Quad(
		Term::Ground(47),
		Term::Ground(33),
		Term::Var(22),
		None,
	));
	a.insert(Quad(Term::Var(48), Term::Var(16), Term::Ground(47), None));
	a.insert(Quad(Term::Ground(25), Term::Var(42), Term::Var(22), None));
	a.insert(Quad(Term::Ground(11), Term::Var(27), Term::Var(41), None));
	a.insert(Quad(
		Term::Ground(33),
		Term::Ground(8),
		Term::Ground(29),
		None,
	));
	a.insert(Quad(
		Term::Var(42),
		Term::Ground(42),
		Term::Ground(19),
		None,
	));
	a.insert(Quad(Term::Var(12), Term::Ground(0), Term::Var(3), None));
	a.insert(Quad(Term::Ground(7), Term::Var(8), Term::Ground(41), None));
	a.insert(Quad(Term::Ground(2), Term::Ground(20), Term::Var(8), None));
	a.insert(Quad(
		Term::Ground(42),
		Term::Var(12),
		Term::Ground(17),
		None,
	));
	a.insert(Quad(Term::Var(35), Term::Ground(16), Term::Var(49), None));
	a.insert(Quad(
		Term::Ground(38),
		Term::Ground(43),
		Term::Ground(29),
		None,
	));
	a.insert(Quad(
		Term::Ground(25),
		Term::Ground(26),
		Term::Ground(0),
		None,
	));
	a.insert(Quad(
		Term::Ground(21),
		Term::Ground(11),
		Term::Var(22),
		None,
	));
	a.insert(Quad(
		Term::Var(14),
		Term::Ground(16),
		Term::Ground(47),
		None,
	));
	a.insert(Quad(Term::Var(3), Term::Var(17), Term::Var(30), None));
	a.insert(Quad(Term::Var(4), Term::Ground(45), Term::Var(49), None));
	a.insert(Quad(
		Term::Ground(12),
		Term::Ground(37),
		Term::Ground(24),
		None,
	));
	a.insert(Quad(
		Term::Var(29),
		Term::Ground(22),
		Term::Ground(35),
		None,
	));
	a.insert(Quad(Term::Var(5), Term::Ground(48), Term::Var(21), None));
	a.insert(Quad(Term::Ground(48), Term::Ground(36), Term::Var(0), None));
	a.insert(Quad(
		Term::Ground(28),
		Term::Ground(10),
		Term::Ground(15),
		None,
	));
	a.insert(Quad(Term::Ground(25), Term::Var(26), Term::Var(11), None));
	a.insert(Quad(Term::Var(19), Term::Var(10), Term::Var(36), None));
	a.insert(Quad(Term::Ground(24), Term::Var(48), Term::Var(41), None));
	a.insert(Quad(Term::Var(37), Term::Ground(5), Term::Var(12), None));
	a.insert(Quad(Term::Ground(18), Term::Var(30), Term::Var(37), None));
	a.insert(Quad(Term::Var(43), Term::Ground(48), Term::Ground(1), None));
	a.insert(Quad(Term::Var(5), Term::Ground(14), Term::Var(30), None));
	a.insert(Quad(
		Term::Ground(6),
		Term::Ground(11),
		Term::Ground(30),
		None,
	));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Var(1018),
		Term::Ground(23),
		Term::Var(1019),
		None,
	));
	b.insert(Quad(
		Term::Var(1023),
		Term::Var(1003),
		Term::Ground(22),
		None,
	));
	b.insert(Quad(
		Term::Var(1014),
		Term::Var(1038),
		Term::Ground(39),
		None,
	));
	b.insert(Quad(
		Term::Ground(24),
		Term::Var(1031),
		Term::Var(1026),
		None,
	));
	b.insert(Quad(
		Term::Ground(48),
		Term::Var(1032),
		Term::Ground(19),
		None,
	));
	b.insert(Quad(
		Term::Var(1018),
		Term::Ground(2),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Var(1027),
		Term::Ground(29),
		Term::Ground(48),
		None,
	));
	b.insert(Quad(
		Term::Ground(48),
		Term::Ground(25),
		Term::Ground(15),
		None,
	));
	b.insert(Quad(
		Term::Ground(45),
		Term::Ground(24),
		Term::Var(1042),
		None,
	));
	b.insert(Quad(
		Term::Ground(39),
		Term::Ground(48),
		Term::Var(1026),
		None,
	));
	b.insert(Quad(
		Term::Ground(26),
		Term::Ground(33),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Var(1028),
		Term::Var(1032),
		None,
	));
	b.insert(Quad(
		Term::Ground(7),
		Term::Var(1037),
		Term::Var(1025),
		None,
	));
	b.insert(Quad(
		Term::Var(1030),
		Term::Var(1027),
		Term::Var(1012),
		None,
	));
	b.insert(Quad(
		Term::Var(1012),
		Term::Var(1012),
		Term::Ground(15),
		None,
	));
	b.insert(Quad(
		Term::Var(1037),
		Term::Var(1005),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Ground(20),
		Term::Var(1015),
		Term::Var(1004),
		None,
	));
	b.insert(Quad(
		Term::Var(1021),
		Term::Var(1008),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Var(1034),
		Term::Var(1035),
		Term::Var(1046),
		None,
	));
	b.insert(Quad(
		Term::Ground(11),
		Term::Var(1010),
		Term::Var(1016),
		None,
	));
	b.insert(Quad(
		Term::Var(1030),
		Term::Ground(30),
		Term::Ground(37),
		None,
	));
	b.insert(Quad(
		Term::Ground(39),
		Term::Ground(17),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Var(1006),
		Term::Var(1039),
		Term::Var(1029),
		None,
	));
	b.insert(Quad(
		Term::Ground(35),
		Term::Var(1014),
		Term::Ground(36),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Var(1029),
		Term::Var(1033),
		None,
	));
	b.insert(Quad(
		Term::Var(1042),
		Term::Var(1001),
		Term::Var(1033),
		None,
	));
	b.insert(Quad(
		Term::Ground(24),
		Term::Var(1030),
		Term::Ground(35),
		None,
	));
	b.insert(Quad(
		Term::Ground(37),
		Term::Var(1037),
		Term::Var(1038),
		None,
	));
	b.insert(Quad(
		Term::Var(1045),
		Term::Ground(47),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Ground(37),
		Term::Var(1034),
		Term::Var(1031),
		None,
	));
	b.insert(Quad(
		Term::Var(1017),
		Term::Var(1019),
		Term::Ground(48),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Ground(33),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Var(1035),
		Term::Var(1002),
		Term::Ground(28),
		None,
	));
	b.insert(Quad(
		Term::Ground(36),
		Term::Ground(13),
		Term::Ground(22),
		None,
	));
	b.insert(Quad(
		Term::Var(1045),
		Term::Ground(47),
		Term::Ground(15),
		None,
	));
	b.insert(Quad(
		Term::Var(1034),
		Term::Var(1036),
		Term::Var(1016),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Var(1032),
		Term::Var(1004),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(32),
		Term::Ground(49),
		None,
	));
	b.insert(Quad(
		Term::Var(1038),
		Term::Var(1038),
		Term::Var(1004),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Var(1027),
		Term::Ground(24),
		None,
	));
	b.insert(Quad(
		Term::Ground(13),
		Term::Var(1014),
		Term::Ground(40),
		None,
	));
	b.insert(Quad(
		Term::Ground(34),
		Term::Ground(49),
		Term::Ground(49),
		None,
	));
	b.insert(Quad(
		Term::Var(1031),
		Term::Ground(12),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Ground(26),
		Term::Ground(43),
		Term::Ground(31),
		None,
	));
	b.insert(Quad(
		Term::Ground(25),
		Term::Ground(28),
		Term::Ground(26),
		None,
	));
	b.insert(Quad(
		Term::Var(1027),
		Term::Ground(23),
		Term::Var(1023),
		None,
	));
	b.insert(Quad(
		Term::Var(1013),
		Term::Ground(15),
		Term::Var(1021),
		None,
	));
	b.insert(Quad(
		Term::Ground(28),
		Term::Var(1025),
		Term::Var(1041),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Var(1043),
		Term::Ground(29),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(0),
		Term::Var(1011),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(23),
		Term::Var(1021),
		None,
	));
	b.insert(Quad(
		Term::Var(1015),
		Term::Ground(5),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Ground(48),
		Term::Var(1033),
		Term::Var(1015),
		None,
	));
	b.insert(Quad(
		Term::Ground(45),
		Term::Ground(14),
		Term::Ground(42),
		None,
	));
	b.insert(Quad(
		Term::Var(1030),
		Term::Ground(0),
		Term::Var(1039),
		None,
	));
	b.insert(Quad(
		Term::Var(1046),
		Term::Var(1031),
		Term::Ground(47),
		None,
	));
	b.insert(Quad(
		Term::Var(1038),
		Term::Ground(20),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Var(1018),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Ground(13),
		Term::Ground(38),
		Term::Var(1029),
		None,
	));
	b.insert(Quad(
		Term::Var(1018),
		Term::Ground(26),
		Term::Var(1040),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Var(1044),
		Term::Ground(38),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Var(1030),
		Term::Ground(27),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Var(1023),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Var(1028),
		Term::Var(1041),
		Term::Ground(15),
		None,
	));
	b.insert(Quad(
		Term::Ground(17),
		Term::Var(1044),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Var(1044),
		Term::Ground(19),
		Term::Var(1046),
		None,
	));
	b.insert(Quad(
		Term::Var(1020),
		Term::Var(1047),
		Term::Ground(46),
		None,
	));
	b.insert(Quad(
		Term::Var(1022),
		Term::Var(1020),
		Term::Ground(13),
		None,
	));
	b.insert(Quad(
		Term::Ground(10),
		Term::Ground(20),
		Term::Var(1043),
		None,
	));
	b.insert(Quad(
		Term::Var(1030),
		Term::Ground(25),
		Term::Var(1015),
		None,
	));
	b.insert(Quad(
		Term::Ground(47),
		Term::Ground(33),
		Term::Var(1022),
		None,
	));
	b.insert(Quad(
		Term::Var(1048),
		Term::Var(1016),
		Term::Ground(47),
		None,
	));
	b.insert(Quad(
		Term::Ground(25),
		Term::Var(1042),
		Term::Var(1022),
		None,
	));
	b.insert(Quad(
		Term::Ground(11),
		Term::Var(1027),
		Term::Var(1041),
		None,
	));
	b.insert(Quad(
		Term::Ground(33),
		Term::Ground(8),
		Term::Ground(29),
		None,
	));
	b.insert(Quad(
		Term::Var(1042),
		Term::Ground(42),
		Term::Ground(19),
		None,
	));
	b.insert(Quad(
		Term::Var(1012),
		Term::Ground(0),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Ground(7),
		Term::Var(1008),
		Term::Ground(41),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(20),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Ground(42),
		Term::Var(1012),
		Term::Ground(17),
		None,
	));
	b.insert(Quad(
		Term::Var(1035),
		Term::Ground(16),
		Term::Var(1049),
		None,
	));
	b.insert(Quad(
		Term::Ground(38),
		Term::Ground(43),
		Term::Ground(29),
		None,
	));
	b.insert(Quad(
		Term::Ground(25),
		Term::Ground(26),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(21),
		Term::Ground(11),
		Term::Var(1022),
		None,
	));
	b.insert(Quad(
		Term::Var(1014),
		Term::Ground(16),
		Term::Ground(47),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Var(1017),
		Term::Var(1030),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Ground(45),
		Term::Var(1049),
		None,
	));
	b.insert(Quad(
		Term::Ground(12),
		Term::Ground(37),
		Term::Ground(24),
		None,
	));
	b.insert(Quad(
		Term::Var(1029),
		Term::Ground(22),
		Term::Ground(35),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Ground(48),
		Term::Var(1021),
		None,
	));
	b.insert(Quad(
		Term::Ground(48),
		Term::Ground(36),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Ground(28),
		Term::Ground(10),
		Term::Ground(15),
		None,
	));
	b.insert(Quad(
		Term::Ground(25),
		Term::Var(1026),
		Term::Var(1011),
		None,
	));
	b.insert(Quad(
		Term::Var(1019),
		Term::Var(1010),
		Term::Var(1036),
		None,
	));
	b.insert(Quad(
		Term::Ground(24),
		Term::Var(1048),
		Term::Var(1041),
		None,
	));
	b.insert(Quad(
		Term::Var(1037),
		Term::Ground(5),
		Term::Var(1012),
		None,
	));
	b.insert(Quad(
		Term::Ground(18),
		Term::Var(1030),
		Term::Var(1037),
		None,
	));
	b.insert(Quad(
		Term::Var(1043),
		Term::Ground(48),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Ground(14),
		Term::Var(1030),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Ground(11),
		Term::Ground(30),
		None,
	));
	test(a, b)
}
#[test]
fn iso_189() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Ground(4), Term::Ground(15), Term::Var(17), None));
	a.insert(Quad(Term::Ground(16), Term::Var(1), Term::Var(47), None));
	a.insert(Quad(Term::Var(35), Term::Var(9), Term::Var(11), None));
	a.insert(Quad(
		Term::Ground(46),
		Term::Ground(28),
		Term::Ground(11),
		None,
	));
	a.insert(Quad(Term::Ground(0), Term::Ground(25), Term::Var(25), None));
	a.insert(Quad(Term::Var(13), Term::Var(14), Term::Ground(5), None));
	a.insert(Quad(
		Term::Ground(43),
		Term::Ground(18),
		Term::Ground(35),
		None,
	));
	a.insert(Quad(Term::Var(27), Term::Var(16), Term::Var(35), None));
	a.insert(Quad(
		Term::Var(37),
		Term::Ground(45),
		Term::Ground(23),
		None,
	));
	a.insert(Quad(Term::Var(30), Term::Var(28), Term::Ground(7), None));
	a.insert(Quad(
		Term::Ground(27),
		Term::Ground(29),
		Term::Var(40),
		None,
	));
	a.insert(Quad(Term::Var(5), Term::Var(27), Term::Var(41), None));
	a.insert(Quad(
		Term::Ground(24),
		Term::Var(16),
		Term::Ground(27),
		None,
	));
	a.insert(Quad(Term::Ground(0), Term::Ground(32), Term::Var(48), None));
	a.insert(Quad(Term::Var(29), Term::Var(2), Term::Var(13), None));
	a.insert(Quad(
		Term::Ground(35),
		Term::Ground(22),
		Term::Var(20),
		None,
	));
	a.insert(Quad(
		Term::Var(31),
		Term::Ground(29),
		Term::Ground(15),
		None,
	));
	a.insert(Quad(
		Term::Ground(34),
		Term::Ground(36),
		Term::Ground(46),
		None,
	));
	a.insert(Quad(
		Term::Ground(26),
		Term::Ground(33),
		Term::Var(43),
		None,
	));
	a.insert(Quad(Term::Ground(28), Term::Var(20), Term::Var(31), None));
	a.insert(Quad(Term::Var(7), Term::Var(17), Term::Var(5), None));
	a.insert(Quad(Term::Ground(45), Term::Var(16), Term::Var(21), None));
	a.insert(Quad(
		Term::Ground(33),
		Term::Var(43),
		Term::Ground(35),
		None,
	));
	a.insert(Quad(
		Term::Var(45),
		Term::Ground(49),
		Term::Ground(25),
		None,
	));
	a.insert(Quad(
		Term::Ground(26),
		Term::Ground(17),
		Term::Ground(28),
		None,
	));
	a.insert(Quad(Term::Var(33), Term::Ground(8), Term::Var(42), None));
	a.insert(Quad(Term::Var(16), Term::Var(47), Term::Ground(14), None));
	a.insert(Quad(Term::Var(5), Term::Var(32), Term::Ground(9), None));
	a.insert(Quad(
		Term::Ground(41),
		Term::Ground(32),
		Term::Var(46),
		None,
	));
	a.insert(Quad(
		Term::Var(39),
		Term::Ground(21),
		Term::Ground(34),
		None,
	));
	a.insert(Quad(Term::Ground(0), Term::Ground(43), Term::Var(23), None));
	a.insert(Quad(Term::Var(49), Term::Var(37), Term::Var(47), None));
	a.insert(Quad(
		Term::Ground(9),
		Term::Ground(34),
		Term::Ground(44),
		None,
	));
	a.insert(Quad(
		Term::Ground(0),
		Term::Ground(29),
		Term::Ground(7),
		None,
	));
	a.insert(Quad(Term::Var(29), Term::Ground(41), Term::Var(23), None));
	a.insert(Quad(Term::Var(24), Term::Ground(31), Term::Var(11), None));
	a.insert(Quad(Term::Ground(3), Term::Var(12), Term::Ground(2), None));
	a.insert(Quad(Term::Var(0), Term::Var(47), Term::Ground(43), None));
	a.insert(Quad(Term::Var(18), Term::Ground(33), Term::Var(5), None));
	a.insert(Quad(Term::Var(42), Term::Var(18), Term::Var(12), None));
	a.insert(Quad(
		Term::Ground(28),
		Term::Ground(29),
		Term::Ground(46),
		None,
	));
	a.insert(Quad(
		Term::Var(36),
		Term::Ground(32),
		Term::Ground(40),
		None,
	));
	a.insert(Quad(Term::Var(45), Term::Ground(24), Term::Var(27), None));
	a.insert(Quad(Term::Var(45), Term::Var(23), Term::Ground(17), None));
	a.insert(Quad(
		Term::Ground(22),
		Term::Ground(27),
		Term::Ground(17),
		None,
	));
	a.insert(Quad(
		Term::Ground(23),
		Term::Ground(15),
		Term::Ground(16),
		None,
	));
	a.insert(Quad(Term::Ground(24), Term::Var(13), Term::Var(3), None));
	a.insert(Quad(Term::Var(20), Term::Ground(2), Term::Var(33), None));
	a.insert(Quad(Term::Var(11), Term::Var(16), Term::Var(48), None));
	a.insert(Quad(Term::Var(13), Term::Var(3), Term::Var(1), None));
	a.insert(Quad(
		Term::Ground(12),
		Term::Var(28),
		Term::Ground(35),
		None,
	));
	a.insert(Quad(
		Term::Ground(48),
		Term::Ground(23),
		Term::Var(26),
		None,
	));
	a.insert(Quad(Term::Var(42), Term::Var(16), Term::Var(24), None));
	a.insert(Quad(
		Term::Ground(39),
		Term::Var(23),
		Term::Ground(15),
		None,
	));
	a.insert(Quad(Term::Var(45), Term::Var(31), Term::Var(22), None));
	a.insert(Quad(
		Term::Var(33),
		Term::Ground(34),
		Term::Ground(31),
		None,
	));
	a.insert(Quad(Term::Var(9), Term::Var(8), Term::Ground(6), None));
	a.insert(Quad(Term::Ground(12), Term::Var(11), Term::Var(18), None));
	a.insert(Quad(Term::Ground(24), Term::Var(13), Term::Var(27), None));
	a.insert(Quad(Term::Var(39), Term::Ground(25), Term::Var(0), None));
	a.insert(Quad(Term::Var(3), Term::Var(5), Term::Ground(23), None));
	a.insert(Quad(Term::Var(7), Term::Ground(49), Term::Ground(19), None));
	a.insert(Quad(Term::Var(39), Term::Var(17), Term::Var(43), None));
	a.insert(Quad(Term::Var(29), Term::Ground(3), Term::Ground(44), None));
	a.insert(Quad(Term::Ground(25), Term::Var(39), Term::Var(34), None));
	a.insert(Quad(Term::Ground(14), Term::Var(9), Term::Ground(43), None));
	a.insert(Quad(Term::Var(1), Term::Var(19), Term::Var(10), None));
	a.insert(Quad(Term::Var(7), Term::Var(0), Term::Var(22), None));
	a.insert(Quad(Term::Ground(25), Term::Var(42), Term::Var(44), None));
	a.insert(Quad(
		Term::Ground(39),
		Term::Ground(24),
		Term::Ground(26),
		None,
	));
	a.insert(Quad(
		Term::Ground(11),
		Term::Var(13),
		Term::Ground(43),
		None,
	));
	a.insert(Quad(Term::Ground(3), Term::Var(15), Term::Ground(10), None));
	a.insert(Quad(
		Term::Ground(13),
		Term::Var(20),
		Term::Ground(10),
		None,
	));
	a.insert(Quad(Term::Var(40), Term::Ground(47), Term::Var(6), None));
	a.insert(Quad(Term::Ground(35), Term::Var(16), Term::Ground(9), None));
	a.insert(Quad(Term::Ground(9), Term::Var(14), Term::Var(24), None));
	a.insert(Quad(
		Term::Ground(37),
		Term::Ground(45),
		Term::Ground(6),
		None,
	));
	a.insert(Quad(Term::Var(38), Term::Var(35), Term::Ground(11), None));
	a.insert(Quad(
		Term::Var(15),
		Term::Ground(24),
		Term::Ground(29),
		None,
	));
	a.insert(Quad(Term::Var(1), Term::Var(48), Term::Ground(47), None));
	a.insert(Quad(
		Term::Ground(31),
		Term::Ground(13),
		Term::Ground(28),
		None,
	));
	a.insert(Quad(Term::Var(13), Term::Var(39), Term::Var(18), None));
	a.insert(Quad(
		Term::Ground(44),
		Term::Ground(19),
		Term::Ground(33),
		None,
	));
	a.insert(Quad(
		Term::Ground(24),
		Term::Ground(46),
		Term::Ground(18),
		None,
	));
	a.insert(Quad(
		Term::Ground(48),
		Term::Ground(1),
		Term::Ground(23),
		None,
	));
	a.insert(Quad(Term::Var(17), Term::Var(15), Term::Ground(36), None));
	a.insert(Quad(
		Term::Ground(20),
		Term::Ground(4),
		Term::Ground(28),
		None,
	));
	a.insert(Quad(
		Term::Ground(42),
		Term::Ground(39),
		Term::Ground(29),
		None,
	));
	a.insert(Quad(Term::Var(5), Term::Ground(26), Term::Ground(34), None));
	a.insert(Quad(Term::Ground(7), Term::Var(5), Term::Var(45), None));
	a.insert(Quad(
		Term::Ground(44),
		Term::Ground(18),
		Term::Var(28),
		None,
	));
	a.insert(Quad(Term::Ground(39), Term::Var(15), Term::Var(37), None));
	a.insert(Quad(Term::Ground(13), Term::Var(9), Term::Ground(23), None));
	a.insert(Quad(Term::Ground(49), Term::Var(5), Term::Var(30), None));
	a.insert(Quad(Term::Ground(33), Term::Var(47), Term::Var(3), None));
	a.insert(Quad(Term::Var(15), Term::Var(38), Term::Var(32), None));
	a.insert(Quad(
		Term::Ground(24),
		Term::Ground(28),
		Term::Ground(24),
		None,
	));
	a.insert(Quad(
		Term::Ground(30),
		Term::Ground(19),
		Term::Var(19),
		None,
	));
	a.insert(Quad(Term::Var(45), Term::Ground(16), Term::Var(35), None));
	a.insert(Quad(
		Term::Ground(2),
		Term::Ground(49),
		Term::Ground(22),
		None,
	));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(4),
		Term::Ground(15),
		Term::Var(1017),
		None,
	));
	b.insert(Quad(
		Term::Ground(16),
		Term::Var(1001),
		Term::Var(1047),
		None,
	));
	b.insert(Quad(
		Term::Var(1035),
		Term::Var(1009),
		Term::Var(1011),
		None,
	));
	b.insert(Quad(
		Term::Ground(46),
		Term::Ground(28),
		Term::Ground(11),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(25),
		Term::Var(1025),
		None,
	));
	b.insert(Quad(
		Term::Var(1013),
		Term::Var(1014),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Ground(43),
		Term::Ground(18),
		Term::Ground(35),
		None,
	));
	b.insert(Quad(
		Term::Var(1027),
		Term::Var(1016),
		Term::Var(1035),
		None,
	));
	b.insert(Quad(
		Term::Var(1037),
		Term::Ground(45),
		Term::Ground(23),
		None,
	));
	b.insert(Quad(
		Term::Var(1030),
		Term::Var(1028),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Ground(27),
		Term::Ground(29),
		Term::Var(1040),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Var(1027),
		Term::Var(1041),
		None,
	));
	b.insert(Quad(
		Term::Ground(24),
		Term::Var(1016),
		Term::Ground(27),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(32),
		Term::Var(1048),
		None,
	));
	b.insert(Quad(
		Term::Var(1029),
		Term::Var(1002),
		Term::Var(1013),
		None,
	));
	b.insert(Quad(
		Term::Ground(35),
		Term::Ground(22),
		Term::Var(1020),
		None,
	));
	b.insert(Quad(
		Term::Var(1031),
		Term::Ground(29),
		Term::Ground(15),
		None,
	));
	b.insert(Quad(
		Term::Ground(34),
		Term::Ground(36),
		Term::Ground(46),
		None,
	));
	b.insert(Quad(
		Term::Ground(26),
		Term::Ground(33),
		Term::Var(1043),
		None,
	));
	b.insert(Quad(
		Term::Ground(28),
		Term::Var(1020),
		Term::Var(1031),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Var(1017),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Ground(45),
		Term::Var(1016),
		Term::Var(1021),
		None,
	));
	b.insert(Quad(
		Term::Ground(33),
		Term::Var(1043),
		Term::Ground(35),
		None,
	));
	b.insert(Quad(
		Term::Var(1045),
		Term::Ground(49),
		Term::Ground(25),
		None,
	));
	b.insert(Quad(
		Term::Ground(26),
		Term::Ground(17),
		Term::Ground(28),
		None,
	));
	b.insert(Quad(
		Term::Var(1033),
		Term::Ground(8),
		Term::Var(1042),
		None,
	));
	b.insert(Quad(
		Term::Var(1016),
		Term::Var(1047),
		Term::Ground(14),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Var(1032),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Ground(41),
		Term::Ground(32),
		Term::Var(1046),
		None,
	));
	b.insert(Quad(
		Term::Var(1039),
		Term::Ground(21),
		Term::Ground(34),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(43),
		Term::Var(1023),
		None,
	));
	b.insert(Quad(
		Term::Var(1049),
		Term::Var(1037),
		Term::Var(1047),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Ground(34),
		Term::Ground(44),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(29),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Var(1029),
		Term::Ground(41),
		Term::Var(1023),
		None,
	));
	b.insert(Quad(
		Term::Var(1024),
		Term::Ground(31),
		Term::Var(1011),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Var(1012),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Var(1047),
		Term::Ground(43),
		None,
	));
	b.insert(Quad(
		Term::Var(1018),
		Term::Ground(33),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Var(1042),
		Term::Var(1018),
		Term::Var(1012),
		None,
	));
	b.insert(Quad(
		Term::Ground(28),
		Term::Ground(29),
		Term::Ground(46),
		None,
	));
	b.insert(Quad(
		Term::Var(1036),
		Term::Ground(32),
		Term::Ground(40),
		None,
	));
	b.insert(Quad(
		Term::Var(1045),
		Term::Ground(24),
		Term::Var(1027),
		None,
	));
	b.insert(Quad(
		Term::Var(1045),
		Term::Var(1023),
		Term::Ground(17),
		None,
	));
	b.insert(Quad(
		Term::Ground(22),
		Term::Ground(27),
		Term::Ground(17),
		None,
	));
	b.insert(Quad(
		Term::Ground(23),
		Term::Ground(15),
		Term::Ground(16),
		None,
	));
	b.insert(Quad(
		Term::Ground(24),
		Term::Var(1013),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Var(1020),
		Term::Ground(2),
		Term::Var(1033),
		None,
	));
	b.insert(Quad(
		Term::Var(1011),
		Term::Var(1016),
		Term::Var(1048),
		None,
	));
	b.insert(Quad(
		Term::Var(1013),
		Term::Var(1003),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(12),
		Term::Var(1028),
		Term::Ground(35),
		None,
	));
	b.insert(Quad(
		Term::Ground(48),
		Term::Ground(23),
		Term::Var(1026),
		None,
	));
	b.insert(Quad(
		Term::Var(1042),
		Term::Var(1016),
		Term::Var(1024),
		None,
	));
	b.insert(Quad(
		Term::Ground(39),
		Term::Var(1023),
		Term::Ground(15),
		None,
	));
	b.insert(Quad(
		Term::Var(1045),
		Term::Var(1031),
		Term::Var(1022),
		None,
	));
	b.insert(Quad(
		Term::Var(1033),
		Term::Ground(34),
		Term::Ground(31),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Var(1008),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Ground(12),
		Term::Var(1011),
		Term::Var(1018),
		None,
	));
	b.insert(Quad(
		Term::Ground(24),
		Term::Var(1013),
		Term::Var(1027),
		None,
	));
	b.insert(Quad(
		Term::Var(1039),
		Term::Ground(25),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Var(1005),
		Term::Ground(23),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Ground(49),
		Term::Ground(19),
		None,
	));
	b.insert(Quad(
		Term::Var(1039),
		Term::Var(1017),
		Term::Var(1043),
		None,
	));
	b.insert(Quad(
		Term::Var(1029),
		Term::Ground(3),
		Term::Ground(44),
		None,
	));
	b.insert(Quad(
		Term::Ground(25),
		Term::Var(1039),
		Term::Var(1034),
		None,
	));
	b.insert(Quad(
		Term::Ground(14),
		Term::Var(1009),
		Term::Ground(43),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Var(1019),
		Term::Var(1010),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Var(1000),
		Term::Var(1022),
		None,
	));
	b.insert(Quad(
		Term::Ground(25),
		Term::Var(1042),
		Term::Var(1044),
		None,
	));
	b.insert(Quad(
		Term::Ground(39),
		Term::Ground(24),
		Term::Ground(26),
		None,
	));
	b.insert(Quad(
		Term::Ground(11),
		Term::Var(1013),
		Term::Ground(43),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Var(1015),
		Term::Ground(10),
		None,
	));
	b.insert(Quad(
		Term::Ground(13),
		Term::Var(1020),
		Term::Ground(10),
		None,
	));
	b.insert(Quad(
		Term::Var(1040),
		Term::Ground(47),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Ground(35),
		Term::Var(1016),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Var(1014),
		Term::Var(1024),
		None,
	));
	b.insert(Quad(
		Term::Ground(37),
		Term::Ground(45),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Var(1038),
		Term::Var(1035),
		Term::Ground(11),
		None,
	));
	b.insert(Quad(
		Term::Var(1015),
		Term::Ground(24),
		Term::Ground(29),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Var(1048),
		Term::Ground(47),
		None,
	));
	b.insert(Quad(
		Term::Ground(31),
		Term::Ground(13),
		Term::Ground(28),
		None,
	));
	b.insert(Quad(
		Term::Var(1013),
		Term::Var(1039),
		Term::Var(1018),
		None,
	));
	b.insert(Quad(
		Term::Ground(44),
		Term::Ground(19),
		Term::Ground(33),
		None,
	));
	b.insert(Quad(
		Term::Ground(24),
		Term::Ground(46),
		Term::Ground(18),
		None,
	));
	b.insert(Quad(
		Term::Ground(48),
		Term::Ground(1),
		Term::Ground(23),
		None,
	));
	b.insert(Quad(
		Term::Var(1017),
		Term::Var(1015),
		Term::Ground(36),
		None,
	));
	b.insert(Quad(
		Term::Ground(20),
		Term::Ground(4),
		Term::Ground(28),
		None,
	));
	b.insert(Quad(
		Term::Ground(42),
		Term::Ground(39),
		Term::Ground(29),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Ground(26),
		Term::Ground(34),
		None,
	));
	b.insert(Quad(
		Term::Ground(7),
		Term::Var(1005),
		Term::Var(1045),
		None,
	));
	b.insert(Quad(
		Term::Ground(44),
		Term::Ground(18),
		Term::Var(1028),
		None,
	));
	b.insert(Quad(
		Term::Ground(39),
		Term::Var(1015),
		Term::Var(1037),
		None,
	));
	b.insert(Quad(
		Term::Ground(13),
		Term::Var(1009),
		Term::Ground(23),
		None,
	));
	b.insert(Quad(
		Term::Ground(49),
		Term::Var(1005),
		Term::Var(1030),
		None,
	));
	b.insert(Quad(
		Term::Ground(33),
		Term::Var(1047),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Var(1015),
		Term::Var(1038),
		Term::Var(1032),
		None,
	));
	b.insert(Quad(
		Term::Ground(24),
		Term::Ground(28),
		Term::Ground(24),
		None,
	));
	b.insert(Quad(
		Term::Ground(30),
		Term::Ground(19),
		Term::Var(1019),
		None,
	));
	b.insert(Quad(
		Term::Var(1045),
		Term::Ground(16),
		Term::Var(1035),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(49),
		Term::Ground(22),
		None,
	));
	test(a, b)
}
#[test]
fn iso_190() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Var(42), Term::Var(11), Term::Var(44), None));
	a.insert(Quad(Term::Var(9), Term::Var(48), Term::Ground(1), None));
	a.insert(Quad(
		Term::Ground(19),
		Term::Var(44),
		Term::Ground(40),
		None,
	));
	a.insert(Quad(
		Term::Ground(22),
		Term::Ground(31),
		Term::Ground(16),
		None,
	));
	a.insert(Quad(
		Term::Ground(15),
		Term::Ground(32),
		Term::Var(17),
		None,
	));
	a.insert(Quad(Term::Ground(2), Term::Var(29), Term::Ground(32), None));
	a.insert(Quad(
		Term::Ground(34),
		Term::Ground(25),
		Term::Var(18),
		None,
	));
	a.insert(Quad(
		Term::Ground(29),
		Term::Var(14),
		Term::Ground(23),
		None,
	));
	a.insert(Quad(
		Term::Ground(41),
		Term::Ground(15),
		Term::Var(16),
		None,
	));
	a.insert(Quad(Term::Ground(23), Term::Ground(2), Term::Var(47), None));
	a.insert(Quad(Term::Ground(27), Term::Var(25), Term::Ground(3), None));
	a.insert(Quad(Term::Ground(27), Term::Var(29), Term::Ground(2), None));
	a.insert(Quad(Term::Var(39), Term::Ground(9), Term::Var(44), None));
	a.insert(Quad(Term::Var(24), Term::Ground(46), Term::Var(14), None));
	a.insert(Quad(Term::Var(16), Term::Ground(19), Term::Ground(3), None));
	a.insert(Quad(
		Term::Var(42),
		Term::Ground(11),
		Term::Ground(39),
		None,
	));
	a.insert(Quad(Term::Ground(38), Term::Ground(6), Term::Var(5), None));
	a.insert(Quad(Term::Var(46), Term::Var(1), Term::Ground(35), None));
	a.insert(Quad(Term::Var(7), Term::Ground(9), Term::Var(15), None));
	a.insert(Quad(Term::Var(9), Term::Ground(14), Term::Ground(18), None));
	a.insert(Quad(
		Term::Ground(26),
		Term::Ground(44),
		Term::Ground(11),
		None,
	));
	a.insert(Quad(Term::Var(36), Term::Var(29), Term::Var(48), None));
	a.insert(Quad(Term::Var(30), Term::Var(2), Term::Var(32), None));
	a.insert(Quad(Term::Ground(21), Term::Var(0), Term::Ground(25), None));
	a.insert(Quad(
		Term::Ground(35),
		Term::Ground(25),
		Term::Var(23),
		None,
	));
	a.insert(Quad(Term::Ground(9), Term::Ground(20), Term::Var(28), None));
	a.insert(Quad(
		Term::Var(40),
		Term::Ground(35),
		Term::Ground(12),
		None,
	));
	a.insert(Quad(
		Term::Ground(23),
		Term::Ground(16),
		Term::Ground(16),
		None,
	));
	a.insert(Quad(
		Term::Ground(22),
		Term::Ground(36),
		Term::Var(31),
		None,
	));
	a.insert(Quad(Term::Var(42), Term::Ground(8), Term::Var(31), None));
	a.insert(Quad(Term::Ground(14), Term::Var(36), Term::Var(30), None));
	a.insert(Quad(Term::Var(30), Term::Var(10), Term::Ground(18), None));
	a.insert(Quad(Term::Ground(18), Term::Var(7), Term::Var(43), None));
	a.insert(Quad(Term::Ground(3), Term::Ground(31), Term::Var(20), None));
	a.insert(Quad(Term::Ground(4), Term::Ground(35), Term::Var(23), None));
	a.insert(Quad(Term::Ground(24), Term::Var(44), Term::Ground(4), None));
	a.insert(Quad(Term::Var(9), Term::Ground(21), Term::Ground(25), None));
	a.insert(Quad(Term::Ground(21), Term::Var(34), Term::Var(2), None));
	a.insert(Quad(Term::Ground(41), Term::Var(15), Term::Var(31), None));
	a.insert(Quad(Term::Var(45), Term::Var(2), Term::Ground(49), None));
	a.insert(Quad(Term::Ground(47), Term::Var(17), Term::Var(46), None));
	a.insert(Quad(Term::Var(30), Term::Var(11), Term::Var(33), None));
	a.insert(Quad(Term::Var(27), Term::Ground(42), Term::Ground(0), None));
	a.insert(Quad(Term::Var(30), Term::Ground(4), Term::Ground(18), None));
	a.insert(Quad(Term::Ground(11), Term::Var(0), Term::Ground(20), None));
	a.insert(Quad(Term::Var(13), Term::Var(11), Term::Ground(22), None));
	a.insert(Quad(Term::Var(35), Term::Var(14), Term::Ground(43), None));
	a.insert(Quad(Term::Var(26), Term::Var(25), Term::Var(41), None));
	a.insert(Quad(Term::Var(2), Term::Var(20), Term::Var(34), None));
	a.insert(Quad(
		Term::Ground(9),
		Term::Ground(24),
		Term::Ground(19),
		None,
	));
	a.insert(Quad(Term::Var(8), Term::Ground(26), Term::Var(43), None));
	a.insert(Quad(Term::Var(19), Term::Var(40), Term::Ground(6), None));
	a.insert(Quad(Term::Var(23), Term::Var(38), Term::Var(37), None));
	a.insert(Quad(Term::Ground(20), Term::Var(41), Term::Ground(6), None));
	a.insert(Quad(Term::Var(1), Term::Ground(18), Term::Ground(15), None));
	a.insert(Quad(Term::Ground(24), Term::Var(25), Term::Var(49), None));
	a.insert(Quad(Term::Var(6), Term::Ground(8), Term::Var(22), None));
	a.insert(Quad(
		Term::Ground(32),
		Term::Var(29),
		Term::Ground(20),
		None,
	));
	a.insert(Quad(
		Term::Var(41),
		Term::Ground(14),
		Term::Ground(30),
		None,
	));
	a.insert(Quad(
		Term::Ground(36),
		Term::Ground(41),
		Term::Ground(27),
		None,
	));
	a.insert(Quad(Term::Var(21), Term::Var(24), Term::Ground(26), None));
	a.insert(Quad(Term::Var(21), Term::Var(34), Term::Var(10), None));
	a.insert(Quad(Term::Var(30), Term::Ground(0), Term::Ground(34), None));
	a.insert(Quad(Term::Var(15), Term::Ground(18), Term::Ground(1), None));
	a.insert(Quad(Term::Var(31), Term::Ground(20), Term::Var(20), None));
	a.insert(Quad(Term::Var(32), Term::Var(14), Term::Ground(5), None));
	a.insert(Quad(
		Term::Ground(28),
		Term::Ground(48),
		Term::Ground(5),
		None,
	));
	a.insert(Quad(
		Term::Var(28),
		Term::Ground(16),
		Term::Ground(27),
		None,
	));
	a.insert(Quad(
		Term::Ground(45),
		Term::Var(18),
		Term::Ground(16),
		None,
	));
	a.insert(Quad(Term::Var(37), Term::Ground(20), Term::Var(4), None));
	a.insert(Quad(Term::Ground(36), Term::Var(1), Term::Var(44), None));
	a.insert(Quad(Term::Var(5), Term::Var(2), Term::Var(36), None));
	a.insert(Quad(Term::Var(48), Term::Var(49), Term::Ground(41), None));
	a.insert(Quad(Term::Var(19), Term::Var(21), Term::Var(3), None));
	a.insert(Quad(Term::Var(1), Term::Ground(42), Term::Var(3), None));
	a.insert(Quad(Term::Var(32), Term::Var(14), Term::Var(31), None));
	a.insert(Quad(Term::Var(49), Term::Ground(8), Term::Var(35), None));
	a.insert(Quad(Term::Var(23), Term::Ground(47), Term::Ground(9), None));
	a.insert(Quad(Term::Ground(49), Term::Var(40), Term::Var(3), None));
	a.insert(Quad(Term::Ground(18), Term::Var(13), Term::Var(33), None));
	a.insert(Quad(Term::Var(17), Term::Ground(42), Term::Ground(0), None));
	a.insert(Quad(Term::Var(4), Term::Var(26), Term::Ground(17), None));
	a.insert(Quad(Term::Ground(5), Term::Var(3), Term::Ground(5), None));
	a.insert(Quad(
		Term::Var(25),
		Term::Ground(44),
		Term::Ground(46),
		None,
	));
	a.insert(Quad(Term::Var(11), Term::Ground(32), Term::Var(42), None));
	a.insert(Quad(Term::Var(1), Term::Var(25), Term::Var(33), None));
	a.insert(Quad(Term::Var(47), Term::Var(22), Term::Var(7), None));
	a.insert(Quad(
		Term::Ground(47),
		Term::Var(19),
		Term::Ground(17),
		None,
	));
	a.insert(Quad(Term::Var(26), Term::Var(13), Term::Ground(24), None));
	a.insert(Quad(Term::Ground(42), Term::Var(28), Term::Var(23), None));
	a.insert(Quad(
		Term::Ground(4),
		Term::Ground(20),
		Term::Ground(27),
		None,
	));
	a.insert(Quad(Term::Ground(48), Term::Var(24), Term::Ground(1), None));
	a.insert(Quad(
		Term::Ground(22),
		Term::Ground(28),
		Term::Var(47),
		None,
	));
	a.insert(Quad(Term::Ground(11), Term::Var(48), Term::Var(11), None));
	a.insert(Quad(Term::Var(1), Term::Ground(16), Term::Var(47), None));
	a.insert(Quad(Term::Var(28), Term::Var(30), Term::Var(34), None));
	a.insert(Quad(Term::Ground(19), Term::Var(2), Term::Var(24), None));
	a.insert(Quad(Term::Ground(9), Term::Ground(7), Term::Var(38), None));
	a.insert(Quad(Term::Var(8), Term::Ground(28), Term::Var(42), None));
	a.insert(Quad(Term::Var(21), Term::Var(7), Term::Ground(16), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Var(1042),
		Term::Var(1011),
		Term::Var(1044),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Var(1048),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Ground(19),
		Term::Var(1044),
		Term::Ground(40),
		None,
	));
	b.insert(Quad(
		Term::Ground(22),
		Term::Ground(31),
		Term::Ground(16),
		None,
	));
	b.insert(Quad(
		Term::Ground(15),
		Term::Ground(32),
		Term::Var(1017),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Var(1029),
		Term::Ground(32),
		None,
	));
	b.insert(Quad(
		Term::Ground(34),
		Term::Ground(25),
		Term::Var(1018),
		None,
	));
	b.insert(Quad(
		Term::Ground(29),
		Term::Var(1014),
		Term::Ground(23),
		None,
	));
	b.insert(Quad(
		Term::Ground(41),
		Term::Ground(15),
		Term::Var(1016),
		None,
	));
	b.insert(Quad(
		Term::Ground(23),
		Term::Ground(2),
		Term::Var(1047),
		None,
	));
	b.insert(Quad(
		Term::Ground(27),
		Term::Var(1025),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Ground(27),
		Term::Var(1029),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Var(1039),
		Term::Ground(9),
		Term::Var(1044),
		None,
	));
	b.insert(Quad(
		Term::Var(1024),
		Term::Ground(46),
		Term::Var(1014),
		None,
	));
	b.insert(Quad(
		Term::Var(1016),
		Term::Ground(19),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Var(1042),
		Term::Ground(11),
		Term::Ground(39),
		None,
	));
	b.insert(Quad(
		Term::Ground(38),
		Term::Ground(6),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Var(1046),
		Term::Var(1001),
		Term::Ground(35),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Ground(9),
		Term::Var(1015),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Ground(14),
		Term::Ground(18),
		None,
	));
	b.insert(Quad(
		Term::Ground(26),
		Term::Ground(44),
		Term::Ground(11),
		None,
	));
	b.insert(Quad(
		Term::Var(1036),
		Term::Var(1029),
		Term::Var(1048),
		None,
	));
	b.insert(Quad(
		Term::Var(1030),
		Term::Var(1002),
		Term::Var(1032),
		None,
	));
	b.insert(Quad(
		Term::Ground(21),
		Term::Var(1000),
		Term::Ground(25),
		None,
	));
	b.insert(Quad(
		Term::Ground(35),
		Term::Ground(25),
		Term::Var(1023),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Ground(20),
		Term::Var(1028),
		None,
	));
	b.insert(Quad(
		Term::Var(1040),
		Term::Ground(35),
		Term::Ground(12),
		None,
	));
	b.insert(Quad(
		Term::Ground(23),
		Term::Ground(16),
		Term::Ground(16),
		None,
	));
	b.insert(Quad(
		Term::Ground(22),
		Term::Ground(36),
		Term::Var(1031),
		None,
	));
	b.insert(Quad(
		Term::Var(1042),
		Term::Ground(8),
		Term::Var(1031),
		None,
	));
	b.insert(Quad(
		Term::Ground(14),
		Term::Var(1036),
		Term::Var(1030),
		None,
	));
	b.insert(Quad(
		Term::Var(1030),
		Term::Var(1010),
		Term::Ground(18),
		None,
	));
	b.insert(Quad(
		Term::Ground(18),
		Term::Var(1007),
		Term::Var(1043),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(31),
		Term::Var(1020),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Ground(35),
		Term::Var(1023),
		None,
	));
	b.insert(Quad(
		Term::Ground(24),
		Term::Var(1044),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Ground(21),
		Term::Ground(25),
		None,
	));
	b.insert(Quad(
		Term::Ground(21),
		Term::Var(1034),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Ground(41),
		Term::Var(1015),
		Term::Var(1031),
		None,
	));
	b.insert(Quad(
		Term::Var(1045),
		Term::Var(1002),
		Term::Ground(49),
		None,
	));
	b.insert(Quad(
		Term::Ground(47),
		Term::Var(1017),
		Term::Var(1046),
		None,
	));
	b.insert(Quad(
		Term::Var(1030),
		Term::Var(1011),
		Term::Var(1033),
		None,
	));
	b.insert(Quad(
		Term::Var(1027),
		Term::Ground(42),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Var(1030),
		Term::Ground(4),
		Term::Ground(18),
		None,
	));
	b.insert(Quad(
		Term::Ground(11),
		Term::Var(1000),
		Term::Ground(20),
		None,
	));
	b.insert(Quad(
		Term::Var(1013),
		Term::Var(1011),
		Term::Ground(22),
		None,
	));
	b.insert(Quad(
		Term::Var(1035),
		Term::Var(1014),
		Term::Ground(43),
		None,
	));
	b.insert(Quad(
		Term::Var(1026),
		Term::Var(1025),
		Term::Var(1041),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Var(1020),
		Term::Var(1034),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Ground(24),
		Term::Ground(19),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Ground(26),
		Term::Var(1043),
		None,
	));
	b.insert(Quad(
		Term::Var(1019),
		Term::Var(1040),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Var(1023),
		Term::Var(1038),
		Term::Var(1037),
		None,
	));
	b.insert(Quad(
		Term::Ground(20),
		Term::Var(1041),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(18),
		Term::Ground(15),
		None,
	));
	b.insert(Quad(
		Term::Ground(24),
		Term::Var(1025),
		Term::Var(1049),
		None,
	));
	b.insert(Quad(
		Term::Var(1006),
		Term::Ground(8),
		Term::Var(1022),
		None,
	));
	b.insert(Quad(
		Term::Ground(32),
		Term::Var(1029),
		Term::Ground(20),
		None,
	));
	b.insert(Quad(
		Term::Var(1041),
		Term::Ground(14),
		Term::Ground(30),
		None,
	));
	b.insert(Quad(
		Term::Ground(36),
		Term::Ground(41),
		Term::Ground(27),
		None,
	));
	b.insert(Quad(
		Term::Var(1021),
		Term::Var(1024),
		Term::Ground(26),
		None,
	));
	b.insert(Quad(
		Term::Var(1021),
		Term::Var(1034),
		Term::Var(1010),
		None,
	));
	b.insert(Quad(
		Term::Var(1030),
		Term::Ground(0),
		Term::Ground(34),
		None,
	));
	b.insert(Quad(
		Term::Var(1015),
		Term::Ground(18),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Var(1031),
		Term::Ground(20),
		Term::Var(1020),
		None,
	));
	b.insert(Quad(
		Term::Var(1032),
		Term::Var(1014),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Ground(28),
		Term::Ground(48),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Var(1028),
		Term::Ground(16),
		Term::Ground(27),
		None,
	));
	b.insert(Quad(
		Term::Ground(45),
		Term::Var(1018),
		Term::Ground(16),
		None,
	));
	b.insert(Quad(
		Term::Var(1037),
		Term::Ground(20),
		Term::Var(1004),
		None,
	));
	b.insert(Quad(
		Term::Ground(36),
		Term::Var(1001),
		Term::Var(1044),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Var(1002),
		Term::Var(1036),
		None,
	));
	b.insert(Quad(
		Term::Var(1048),
		Term::Var(1049),
		Term::Ground(41),
		None,
	));
	b.insert(Quad(
		Term::Var(1019),
		Term::Var(1021),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(42),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Var(1032),
		Term::Var(1014),
		Term::Var(1031),
		None,
	));
	b.insert(Quad(
		Term::Var(1049),
		Term::Ground(8),
		Term::Var(1035),
		None,
	));
	b.insert(Quad(
		Term::Var(1023),
		Term::Ground(47),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Ground(49),
		Term::Var(1040),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Ground(18),
		Term::Var(1013),
		Term::Var(1033),
		None,
	));
	b.insert(Quad(
		Term::Var(1017),
		Term::Ground(42),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Var(1026),
		Term::Ground(17),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Var(1003),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Var(1025),
		Term::Ground(44),
		Term::Ground(46),
		None,
	));
	b.insert(Quad(
		Term::Var(1011),
		Term::Ground(32),
		Term::Var(1042),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Var(1025),
		Term::Var(1033),
		None,
	));
	b.insert(Quad(
		Term::Var(1047),
		Term::Var(1022),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Ground(47),
		Term::Var(1019),
		Term::Ground(17),
		None,
	));
	b.insert(Quad(
		Term::Var(1026),
		Term::Var(1013),
		Term::Ground(24),
		None,
	));
	b.insert(Quad(
		Term::Ground(42),
		Term::Var(1028),
		Term::Var(1023),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Ground(20),
		Term::Ground(27),
		None,
	));
	b.insert(Quad(
		Term::Ground(48),
		Term::Var(1024),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Ground(22),
		Term::Ground(28),
		Term::Var(1047),
		None,
	));
	b.insert(Quad(
		Term::Ground(11),
		Term::Var(1048),
		Term::Var(1011),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(16),
		Term::Var(1047),
		None,
	));
	b.insert(Quad(
		Term::Var(1028),
		Term::Var(1030),
		Term::Var(1034),
		None,
	));
	b.insert(Quad(
		Term::Ground(19),
		Term::Var(1002),
		Term::Var(1024),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Ground(7),
		Term::Var(1038),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Ground(28),
		Term::Var(1042),
		None,
	));
	b.insert(Quad(
		Term::Var(1021),
		Term::Var(1007),
		Term::Ground(16),
		None,
	));
	test(a, b)
}
#[test]
fn iso_191() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Var(40), Term::Ground(27), Term::Ground(6), None));
	a.insert(Quad(
		Term::Ground(12),
		Term::Var(30),
		Term::Ground(24),
		None,
	));
	a.insert(Quad(Term::Ground(24), Term::Ground(15), Term::Var(2), None));
	a.insert(Quad(Term::Var(25), Term::Var(9), Term::Ground(35), None));
	a.insert(Quad(
		Term::Ground(22),
		Term::Var(43),
		Term::Ground(33),
		None,
	));
	a.insert(Quad(
		Term::Ground(39),
		Term::Var(20),
		Term::Ground(40),
		None,
	));
	a.insert(Quad(Term::Var(31), Term::Var(40), Term::Var(18), None));
	a.insert(Quad(Term::Var(45), Term::Var(21), Term::Var(5), None));
	a.insert(Quad(Term::Var(40), Term::Var(27), Term::Var(42), None));
	a.insert(Quad(Term::Var(49), Term::Var(15), Term::Ground(29), None));
	a.insert(Quad(Term::Var(32), Term::Ground(34), Term::Var(2), None));
	a.insert(Quad(Term::Var(40), Term::Ground(17), Term::Ground(1), None));
	a.insert(Quad(
		Term::Ground(44),
		Term::Var(19),
		Term::Ground(44),
		None,
	));
	a.insert(Quad(
		Term::Ground(17),
		Term::Ground(15),
		Term::Var(34),
		None,
	));
	a.insert(Quad(Term::Var(33), Term::Ground(40), Term::Var(23), None));
	a.insert(Quad(Term::Var(46), Term::Ground(13), Term::Ground(5), None));
	a.insert(Quad(Term::Var(21), Term::Var(29), Term::Ground(34), None));
	a.insert(Quad(Term::Ground(43), Term::Ground(5), Term::Var(19), None));
	a.insert(Quad(
		Term::Ground(41),
		Term::Var(27),
		Term::Ground(23),
		None,
	));
	a.insert(Quad(Term::Var(6), Term::Var(48), Term::Ground(8), None));
	a.insert(Quad(Term::Ground(11), Term::Var(19), Term::Var(40), None));
	a.insert(Quad(Term::Var(49), Term::Var(34), Term::Ground(8), None));
	a.insert(Quad(
		Term::Ground(11),
		Term::Ground(6),
		Term::Ground(48),
		None,
	));
	a.insert(Quad(
		Term::Ground(5),
		Term::Ground(1),
		Term::Ground(39),
		None,
	));
	a.insert(Quad(Term::Ground(13), Term::Ground(49), Term::Var(6), None));
	a.insert(Quad(Term::Var(18), Term::Var(17), Term::Var(9), None));
	a.insert(Quad(Term::Var(44), Term::Ground(25), Term::Var(12), None));
	a.insert(Quad(Term::Var(18), Term::Var(13), Term::Ground(10), None));
	a.insert(Quad(
		Term::Ground(19),
		Term::Ground(28),
		Term::Var(10),
		None,
	));
	a.insert(Quad(Term::Ground(3), Term::Var(32), Term::Var(5), None));
	a.insert(Quad(Term::Ground(22), Term::Var(9), Term::Var(11), None));
	a.insert(Quad(
		Term::Ground(45),
		Term::Ground(39),
		Term::Var(23),
		None,
	));
	a.insert(Quad(Term::Var(31), Term::Var(28), Term::Ground(19), None));
	a.insert(Quad(Term::Var(1), Term::Ground(0), Term::Ground(18), None));
	a.insert(Quad(Term::Ground(4), Term::Var(34), Term::Var(40), None));
	a.insert(Quad(Term::Ground(1), Term::Var(20), Term::Ground(33), None));
	a.insert(Quad(Term::Ground(28), Term::Var(41), Term::Var(20), None));
	a.insert(Quad(Term::Ground(40), Term::Ground(3), Term::Var(18), None));
	a.insert(Quad(Term::Var(41), Term::Var(42), Term::Var(41), None));
	a.insert(Quad(Term::Var(2), Term::Ground(12), Term::Ground(3), None));
	a.insert(Quad(Term::Ground(20), Term::Var(6), Term::Ground(37), None));
	a.insert(Quad(
		Term::Ground(32),
		Term::Ground(39),
		Term::Ground(28),
		None,
	));
	a.insert(Quad(
		Term::Var(18),
		Term::Ground(42),
		Term::Ground(48),
		None,
	));
	a.insert(Quad(Term::Ground(13), Term::Var(32), Term::Var(44), None));
	a.insert(Quad(Term::Ground(39), Term::Var(49), Term::Var(10), None));
	a.insert(Quad(Term::Ground(5), Term::Var(1), Term::Var(44), None));
	a.insert(Quad(
		Term::Ground(44),
		Term::Ground(48),
		Term::Ground(35),
		None,
	));
	a.insert(Quad(Term::Ground(32), Term::Ground(7), Term::Var(12), None));
	a.insert(Quad(
		Term::Ground(22),
		Term::Var(11),
		Term::Ground(11),
		None,
	));
	a.insert(Quad(Term::Ground(44), Term::Ground(8), Term::Var(34), None));
	a.insert(Quad(Term::Var(14), Term::Var(25), Term::Var(33), None));
	a.insert(Quad(Term::Ground(35), Term::Var(29), Term::Var(22), None));
	a.insert(Quad(
		Term::Ground(44),
		Term::Var(42),
		Term::Ground(21),
		None,
	));
	a.insert(Quad(
		Term::Ground(20),
		Term::Ground(16),
		Term::Var(13),
		None,
	));
	a.insert(Quad(Term::Var(8), Term::Var(32), Term::Var(12), None));
	a.insert(Quad(Term::Ground(11), Term::Var(5), Term::Var(1), None));
	a.insert(Quad(Term::Var(32), Term::Ground(36), Term::Var(25), None));
	a.insert(Quad(Term::Ground(21), Term::Ground(8), Term::Var(29), None));
	a.insert(Quad(Term::Ground(43), Term::Ground(7), Term::Var(44), None));
	a.insert(Quad(Term::Ground(48), Term::Var(35), Term::Ground(5), None));
	a.insert(Quad(Term::Ground(13), Term::Ground(40), Term::Var(3), None));
	a.insert(Quad(
		Term::Ground(47),
		Term::Var(10),
		Term::Ground(41),
		None,
	));
	a.insert(Quad(Term::Var(35), Term::Var(41), Term::Var(43), None));
	a.insert(Quad(Term::Var(35), Term::Var(21), Term::Ground(35), None));
	a.insert(Quad(
		Term::Ground(22),
		Term::Ground(45),
		Term::Ground(35),
		None,
	));
	a.insert(Quad(
		Term::Ground(30),
		Term::Ground(11),
		Term::Var(30),
		None,
	));
	a.insert(Quad(Term::Ground(40), Term::Ground(29), Term::Var(4), None));
	a.insert(Quad(
		Term::Ground(19),
		Term::Ground(17),
		Term::Ground(8),
		None,
	));
	a.insert(Quad(
		Term::Ground(11),
		Term::Ground(10),
		Term::Var(18),
		None,
	));
	a.insert(Quad(Term::Var(5), Term::Var(40), Term::Var(28), None));
	a.insert(Quad(Term::Var(32), Term::Var(44), Term::Var(38), None));
	a.insert(Quad(Term::Var(6), Term::Ground(22), Term::Ground(24), None));
	a.insert(Quad(Term::Ground(49), Term::Var(2), Term::Ground(33), None));
	a.insert(Quad(Term::Ground(29), Term::Var(21), Term::Var(45), None));
	a.insert(Quad(Term::Ground(46), Term::Ground(15), Term::Var(8), None));
	a.insert(Quad(Term::Var(26), Term::Ground(17), Term::Var(7), None));
	a.insert(Quad(Term::Var(47), Term::Ground(3), Term::Ground(35), None));
	a.insert(Quad(
		Term::Var(40),
		Term::Ground(33),
		Term::Ground(14),
		None,
	));
	a.insert(Quad(Term::Ground(3), Term::Var(11), Term::Ground(42), None));
	a.insert(Quad(Term::Var(10), Term::Var(41), Term::Var(0), None));
	a.insert(Quad(Term::Var(44), Term::Var(25), Term::Var(7), None));
	a.insert(Quad(Term::Ground(24), Term::Var(32), Term::Var(19), None));
	a.insert(Quad(
		Term::Var(33),
		Term::Ground(35),
		Term::Ground(15),
		None,
	));
	a.insert(Quad(Term::Var(38), Term::Var(8), Term::Ground(13), None));
	a.insert(Quad(
		Term::Ground(26),
		Term::Ground(42),
		Term::Var(30),
		None,
	));
	a.insert(Quad(
		Term::Ground(24),
		Term::Ground(8),
		Term::Ground(33),
		None,
	));
	a.insert(Quad(Term::Var(16), Term::Var(41), Term::Ground(33), None));
	a.insert(Quad(
		Term::Ground(45),
		Term::Ground(36),
		Term::Ground(37),
		None,
	));
	a.insert(Quad(Term::Var(20), Term::Ground(18), Term::Var(21), None));
	a.insert(Quad(Term::Var(18), Term::Var(43), Term::Ground(33), None));
	a.insert(Quad(Term::Var(1), Term::Ground(46), Term::Var(17), None));
	a.insert(Quad(Term::Ground(45), Term::Var(28), Term::Var(16), None));
	a.insert(Quad(
		Term::Ground(48),
		Term::Ground(5),
		Term::Ground(6),
		None,
	));
	a.insert(Quad(Term::Ground(36), Term::Var(43), Term::Var(41), None));
	a.insert(Quad(Term::Var(23), Term::Ground(44), Term::Var(27), None));
	a.insert(Quad(
		Term::Ground(21),
		Term::Var(19),
		Term::Ground(41),
		None,
	));
	a.insert(Quad(Term::Ground(14), Term::Var(36), Term::Var(16), None));
	a.insert(Quad(Term::Var(26), Term::Ground(44), Term::Var(5), None));
	a.insert(Quad(Term::Var(4), Term::Var(18), Term::Ground(25), None));
	a.insert(Quad(Term::Var(8), Term::Var(27), Term::Var(46), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Var(1040),
		Term::Ground(27),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Ground(12),
		Term::Var(1030),
		Term::Ground(24),
		None,
	));
	b.insert(Quad(
		Term::Ground(24),
		Term::Ground(15),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Var(1025),
		Term::Var(1009),
		Term::Ground(35),
		None,
	));
	b.insert(Quad(
		Term::Ground(22),
		Term::Var(1043),
		Term::Ground(33),
		None,
	));
	b.insert(Quad(
		Term::Ground(39),
		Term::Var(1020),
		Term::Ground(40),
		None,
	));
	b.insert(Quad(
		Term::Var(1031),
		Term::Var(1040),
		Term::Var(1018),
		None,
	));
	b.insert(Quad(
		Term::Var(1045),
		Term::Var(1021),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Var(1040),
		Term::Var(1027),
		Term::Var(1042),
		None,
	));
	b.insert(Quad(
		Term::Var(1049),
		Term::Var(1015),
		Term::Ground(29),
		None,
	));
	b.insert(Quad(
		Term::Var(1032),
		Term::Ground(34),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Var(1040),
		Term::Ground(17),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Ground(44),
		Term::Var(1019),
		Term::Ground(44),
		None,
	));
	b.insert(Quad(
		Term::Ground(17),
		Term::Ground(15),
		Term::Var(1034),
		None,
	));
	b.insert(Quad(
		Term::Var(1033),
		Term::Ground(40),
		Term::Var(1023),
		None,
	));
	b.insert(Quad(
		Term::Var(1046),
		Term::Ground(13),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Var(1021),
		Term::Var(1029),
		Term::Ground(34),
		None,
	));
	b.insert(Quad(
		Term::Ground(43),
		Term::Ground(5),
		Term::Var(1019),
		None,
	));
	b.insert(Quad(
		Term::Ground(41),
		Term::Var(1027),
		Term::Ground(23),
		None,
	));
	b.insert(Quad(
		Term::Var(1006),
		Term::Var(1048),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Ground(11),
		Term::Var(1019),
		Term::Var(1040),
		None,
	));
	b.insert(Quad(
		Term::Var(1049),
		Term::Var(1034),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Ground(11),
		Term::Ground(6),
		Term::Ground(48),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Ground(1),
		Term::Ground(39),
		None,
	));
	b.insert(Quad(
		Term::Ground(13),
		Term::Ground(49),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Var(1018),
		Term::Var(1017),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Var(1044),
		Term::Ground(25),
		Term::Var(1012),
		None,
	));
	b.insert(Quad(
		Term::Var(1018),
		Term::Var(1013),
		Term::Ground(10),
		None,
	));
	b.insert(Quad(
		Term::Ground(19),
		Term::Ground(28),
		Term::Var(1010),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Var(1032),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Ground(22),
		Term::Var(1009),
		Term::Var(1011),
		None,
	));
	b.insert(Quad(
		Term::Ground(45),
		Term::Ground(39),
		Term::Var(1023),
		None,
	));
	b.insert(Quad(
		Term::Var(1031),
		Term::Var(1028),
		Term::Ground(19),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(0),
		Term::Ground(18),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Var(1034),
		Term::Var(1040),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Var(1020),
		Term::Ground(33),
		None,
	));
	b.insert(Quad(
		Term::Ground(28),
		Term::Var(1041),
		Term::Var(1020),
		None,
	));
	b.insert(Quad(
		Term::Ground(40),
		Term::Ground(3),
		Term::Var(1018),
		None,
	));
	b.insert(Quad(
		Term::Var(1041),
		Term::Var(1042),
		Term::Var(1041),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Ground(12),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Ground(20),
		Term::Var(1006),
		Term::Ground(37),
		None,
	));
	b.insert(Quad(
		Term::Ground(32),
		Term::Ground(39),
		Term::Ground(28),
		None,
	));
	b.insert(Quad(
		Term::Var(1018),
		Term::Ground(42),
		Term::Ground(48),
		None,
	));
	b.insert(Quad(
		Term::Ground(13),
		Term::Var(1032),
		Term::Var(1044),
		None,
	));
	b.insert(Quad(
		Term::Ground(39),
		Term::Var(1049),
		Term::Var(1010),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Var(1001),
		Term::Var(1044),
		None,
	));
	b.insert(Quad(
		Term::Ground(44),
		Term::Ground(48),
		Term::Ground(35),
		None,
	));
	b.insert(Quad(
		Term::Ground(32),
		Term::Ground(7),
		Term::Var(1012),
		None,
	));
	b.insert(Quad(
		Term::Ground(22),
		Term::Var(1011),
		Term::Ground(11),
		None,
	));
	b.insert(Quad(
		Term::Ground(44),
		Term::Ground(8),
		Term::Var(1034),
		None,
	));
	b.insert(Quad(
		Term::Var(1014),
		Term::Var(1025),
		Term::Var(1033),
		None,
	));
	b.insert(Quad(
		Term::Ground(35),
		Term::Var(1029),
		Term::Var(1022),
		None,
	));
	b.insert(Quad(
		Term::Ground(44),
		Term::Var(1042),
		Term::Ground(21),
		None,
	));
	b.insert(Quad(
		Term::Ground(20),
		Term::Ground(16),
		Term::Var(1013),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Var(1032),
		Term::Var(1012),
		None,
	));
	b.insert(Quad(
		Term::Ground(11),
		Term::Var(1005),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Var(1032),
		Term::Ground(36),
		Term::Var(1025),
		None,
	));
	b.insert(Quad(
		Term::Ground(21),
		Term::Ground(8),
		Term::Var(1029),
		None,
	));
	b.insert(Quad(
		Term::Ground(43),
		Term::Ground(7),
		Term::Var(1044),
		None,
	));
	b.insert(Quad(
		Term::Ground(48),
		Term::Var(1035),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Ground(13),
		Term::Ground(40),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Ground(47),
		Term::Var(1010),
		Term::Ground(41),
		None,
	));
	b.insert(Quad(
		Term::Var(1035),
		Term::Var(1041),
		Term::Var(1043),
		None,
	));
	b.insert(Quad(
		Term::Var(1035),
		Term::Var(1021),
		Term::Ground(35),
		None,
	));
	b.insert(Quad(
		Term::Ground(22),
		Term::Ground(45),
		Term::Ground(35),
		None,
	));
	b.insert(Quad(
		Term::Ground(30),
		Term::Ground(11),
		Term::Var(1030),
		None,
	));
	b.insert(Quad(
		Term::Ground(40),
		Term::Ground(29),
		Term::Var(1004),
		None,
	));
	b.insert(Quad(
		Term::Ground(19),
		Term::Ground(17),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Ground(11),
		Term::Ground(10),
		Term::Var(1018),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Var(1040),
		Term::Var(1028),
		None,
	));
	b.insert(Quad(
		Term::Var(1032),
		Term::Var(1044),
		Term::Var(1038),
		None,
	));
	b.insert(Quad(
		Term::Var(1006),
		Term::Ground(22),
		Term::Ground(24),
		None,
	));
	b.insert(Quad(
		Term::Ground(49),
		Term::Var(1002),
		Term::Ground(33),
		None,
	));
	b.insert(Quad(
		Term::Ground(29),
		Term::Var(1021),
		Term::Var(1045),
		None,
	));
	b.insert(Quad(
		Term::Ground(46),
		Term::Ground(15),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Var(1026),
		Term::Ground(17),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Var(1047),
		Term::Ground(3),
		Term::Ground(35),
		None,
	));
	b.insert(Quad(
		Term::Var(1040),
		Term::Ground(33),
		Term::Ground(14),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Var(1011),
		Term::Ground(42),
		None,
	));
	b.insert(Quad(
		Term::Var(1010),
		Term::Var(1041),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Var(1044),
		Term::Var(1025),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Ground(24),
		Term::Var(1032),
		Term::Var(1019),
		None,
	));
	b.insert(Quad(
		Term::Var(1033),
		Term::Ground(35),
		Term::Ground(15),
		None,
	));
	b.insert(Quad(
		Term::Var(1038),
		Term::Var(1008),
		Term::Ground(13),
		None,
	));
	b.insert(Quad(
		Term::Ground(26),
		Term::Ground(42),
		Term::Var(1030),
		None,
	));
	b.insert(Quad(
		Term::Ground(24),
		Term::Ground(8),
		Term::Ground(33),
		None,
	));
	b.insert(Quad(
		Term::Var(1016),
		Term::Var(1041),
		Term::Ground(33),
		None,
	));
	b.insert(Quad(
		Term::Ground(45),
		Term::Ground(36),
		Term::Ground(37),
		None,
	));
	b.insert(Quad(
		Term::Var(1020),
		Term::Ground(18),
		Term::Var(1021),
		None,
	));
	b.insert(Quad(
		Term::Var(1018),
		Term::Var(1043),
		Term::Ground(33),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(46),
		Term::Var(1017),
		None,
	));
	b.insert(Quad(
		Term::Ground(45),
		Term::Var(1028),
		Term::Var(1016),
		None,
	));
	b.insert(Quad(
		Term::Ground(48),
		Term::Ground(5),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Ground(36),
		Term::Var(1043),
		Term::Var(1041),
		None,
	));
	b.insert(Quad(
		Term::Var(1023),
		Term::Ground(44),
		Term::Var(1027),
		None,
	));
	b.insert(Quad(
		Term::Ground(21),
		Term::Var(1019),
		Term::Ground(41),
		None,
	));
	b.insert(Quad(
		Term::Ground(14),
		Term::Var(1036),
		Term::Var(1016),
		None,
	));
	b.insert(Quad(
		Term::Var(1026),
		Term::Ground(44),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Var(1018),
		Term::Ground(25),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Var(1027),
		Term::Var(1046),
		None,
	));
	test(a, b)
}
#[test]
fn iso_192() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Var(43), Term::Var(15), Term::Ground(20), None));
	a.insert(Quad(Term::Ground(8), Term::Var(39), Term::Var(14), None));
	a.insert(Quad(Term::Var(5), Term::Ground(28), Term::Var(38), None));
	a.insert(Quad(Term::Var(10), Term::Var(10), Term::Var(48), None));
	a.insert(Quad(Term::Var(26), Term::Var(7), Term::Ground(25), None));
	a.insert(Quad(
		Term::Ground(36),
		Term::Var(41),
		Term::Ground(24),
		None,
	));
	a.insert(Quad(Term::Var(48), Term::Ground(39), Term::Var(46), None));
	a.insert(Quad(Term::Var(7), Term::Ground(38), Term::Ground(26), None));
	a.insert(Quad(Term::Var(26), Term::Ground(32), Term::Var(15), None));
	a.insert(Quad(Term::Ground(0), Term::Ground(44), Term::Var(7), None));
	a.insert(Quad(Term::Ground(0), Term::Var(8), Term::Var(19), None));
	a.insert(Quad(Term::Var(23), Term::Var(31), Term::Var(49), None));
	a.insert(Quad(Term::Ground(1), Term::Var(13), Term::Var(13), None));
	a.insert(Quad(Term::Var(30), Term::Ground(49), Term::Var(13), None));
	a.insert(Quad(Term::Var(48), Term::Ground(2), Term::Var(35), None));
	a.insert(Quad(Term::Ground(38), Term::Ground(46), Term::Var(8), None));
	a.insert(Quad(
		Term::Ground(11),
		Term::Var(18),
		Term::Ground(19),
		None,
	));
	a.insert(Quad(
		Term::Var(11),
		Term::Ground(21),
		Term::Ground(41),
		None,
	));
	a.insert(Quad(Term::Var(23), Term::Ground(29), Term::Var(33), None));
	a.insert(Quad(
		Term::Ground(22),
		Term::Ground(48),
		Term::Ground(43),
		None,
	));
	a.insert(Quad(
		Term::Var(12),
		Term::Ground(15),
		Term::Ground(24),
		None,
	));
	a.insert(Quad(Term::Ground(8), Term::Var(25), Term::Ground(47), None));
	a.insert(Quad(Term::Ground(32), Term::Var(9), Term::Ground(2), None));
	a.insert(Quad(Term::Var(4), Term::Var(20), Term::Var(19), None));
	a.insert(Quad(
		Term::Ground(45),
		Term::Ground(27),
		Term::Var(16),
		None,
	));
	a.insert(Quad(Term::Var(40), Term::Var(6), Term::Var(27), None));
	a.insert(Quad(Term::Var(19), Term::Var(48), Term::Var(40), None));
	a.insert(Quad(
		Term::Ground(16),
		Term::Var(46),
		Term::Ground(43),
		None,
	));
	a.insert(Quad(Term::Var(7), Term::Var(46), Term::Var(26), None));
	a.insert(Quad(Term::Var(8), Term::Ground(39), Term::Var(28), None));
	a.insert(Quad(Term::Var(8), Term::Var(24), Term::Var(9), None));
	a.insert(Quad(
		Term::Ground(45),
		Term::Var(39),
		Term::Ground(33),
		None,
	));
	a.insert(Quad(Term::Var(39), Term::Ground(2), Term::Var(42), None));
	a.insert(Quad(Term::Var(46), Term::Var(42), Term::Ground(19), None));
	a.insert(Quad(Term::Var(3), Term::Ground(38), Term::Ground(20), None));
	a.insert(Quad(Term::Var(5), Term::Ground(46), Term::Ground(26), None));
	a.insert(Quad(Term::Var(48), Term::Var(47), Term::Ground(40), None));
	a.insert(Quad(Term::Var(39), Term::Var(4), Term::Ground(8), None));
	a.insert(Quad(
		Term::Ground(44),
		Term::Ground(19),
		Term::Ground(1),
		None,
	));
	a.insert(Quad(Term::Var(0), Term::Var(11), Term::Ground(31), None));
	a.insert(Quad(Term::Var(35), Term::Var(31), Term::Var(38), None));
	a.insert(Quad(
		Term::Ground(33),
		Term::Ground(34),
		Term::Var(10),
		None,
	));
	a.insert(Quad(
		Term::Ground(29),
		Term::Var(14),
		Term::Ground(36),
		None,
	));
	a.insert(Quad(Term::Var(8), Term::Ground(4), Term::Ground(13), None));
	a.insert(Quad(Term::Ground(21), Term::Var(12), Term::Var(45), None));
	a.insert(Quad(Term::Var(49), Term::Var(6), Term::Ground(43), None));
	a.insert(Quad(Term::Var(40), Term::Var(27), Term::Var(26), None));
	a.insert(Quad(Term::Var(21), Term::Ground(4), Term::Var(40), None));
	a.insert(Quad(
		Term::Ground(19),
		Term::Ground(19),
		Term::Var(13),
		None,
	));
	a.insert(Quad(Term::Ground(36), Term::Var(2), Term::Ground(8), None));
	a.insert(Quad(Term::Ground(48), Term::Var(35), Term::Var(36), None));
	a.insert(Quad(
		Term::Ground(29),
		Term::Var(30),
		Term::Ground(18),
		None,
	));
	a.insert(Quad(Term::Var(47), Term::Ground(42), Term::Var(27), None));
	a.insert(Quad(Term::Var(26), Term::Var(14), Term::Var(30), None));
	a.insert(Quad(Term::Ground(2), Term::Var(21), Term::Ground(47), None));
	a.insert(Quad(Term::Var(41), Term::Ground(35), Term::Var(8), None));
	a.insert(Quad(Term::Var(2), Term::Var(37), Term::Ground(26), None));
	a.insert(Quad(
		Term::Ground(30),
		Term::Ground(5),
		Term::Ground(9),
		None,
	));
	a.insert(Quad(
		Term::Var(17),
		Term::Ground(39),
		Term::Ground(39),
		None,
	));
	a.insert(Quad(
		Term::Var(20),
		Term::Ground(41),
		Term::Ground(13),
		None,
	));
	a.insert(Quad(Term::Var(44), Term::Ground(6), Term::Var(49), None));
	a.insert(Quad(
		Term::Ground(46),
		Term::Ground(5),
		Term::Ground(9),
		None,
	));
	a.insert(Quad(Term::Ground(4), Term::Var(3), Term::Var(44), None));
	a.insert(Quad(
		Term::Ground(34),
		Term::Ground(29),
		Term::Ground(7),
		None,
	));
	a.insert(Quad(
		Term::Ground(16),
		Term::Ground(16),
		Term::Var(15),
		None,
	));
	a.insert(Quad(
		Term::Var(24),
		Term::Ground(31),
		Term::Ground(28),
		None,
	));
	a.insert(Quad(Term::Var(2), Term::Ground(33), Term::Ground(21), None));
	a.insert(Quad(Term::Var(9), Term::Var(22), Term::Ground(35), None));
	a.insert(Quad(Term::Var(12), Term::Ground(41), Term::Var(19), None));
	a.insert(Quad(Term::Ground(33), Term::Ground(36), Term::Var(7), None));
	a.insert(Quad(
		Term::Ground(12),
		Term::Var(40),
		Term::Ground(46),
		None,
	));
	a.insert(Quad(
		Term::Var(32),
		Term::Ground(20),
		Term::Ground(48),
		None,
	));
	a.insert(Quad(
		Term::Ground(11),
		Term::Ground(44),
		Term::Ground(19),
		None,
	));
	a.insert(Quad(Term::Ground(42), Term::Var(38), Term::Var(7), None));
	a.insert(Quad(
		Term::Var(24),
		Term::Ground(20),
		Term::Ground(46),
		None,
	));
	a.insert(Quad(
		Term::Ground(10),
		Term::Ground(35),
		Term::Ground(8),
		None,
	));
	a.insert(Quad(Term::Var(0), Term::Ground(20), Term::Ground(10), None));
	a.insert(Quad(
		Term::Ground(28),
		Term::Ground(20),
		Term::Ground(34),
		None,
	));
	a.insert(Quad(
		Term::Ground(12),
		Term::Ground(24),
		Term::Ground(6),
		None,
	));
	a.insert(Quad(
		Term::Var(24),
		Term::Ground(23),
		Term::Ground(42),
		None,
	));
	a.insert(Quad(Term::Var(9), Term::Ground(39), Term::Ground(43), None));
	a.insert(Quad(Term::Var(40), Term::Var(46), Term::Var(37), None));
	a.insert(Quad(Term::Ground(39), Term::Var(45), Term::Var(46), None));
	a.insert(Quad(Term::Ground(18), Term::Var(30), Term::Ground(4), None));
	a.insert(Quad(
		Term::Ground(33),
		Term::Var(19),
		Term::Ground(18),
		None,
	));
	a.insert(Quad(Term::Var(48), Term::Var(41), Term::Var(21), None));
	a.insert(Quad(Term::Var(42), Term::Var(28), Term::Var(5), None));
	a.insert(Quad(Term::Ground(7), Term::Var(40), Term::Var(25), None));
	a.insert(Quad(Term::Ground(47), Term::Var(18), Term::Var(2), None));
	a.insert(Quad(Term::Var(32), Term::Ground(20), Term::Var(44), None));
	a.insert(Quad(Term::Var(12), Term::Ground(0), Term::Var(14), None));
	a.insert(Quad(Term::Var(46), Term::Var(32), Term::Ground(26), None));
	a.insert(Quad(Term::Var(22), Term::Var(49), Term::Var(31), None));
	a.insert(Quad(Term::Var(42), Term::Var(18), Term::Ground(32), None));
	a.insert(Quad(Term::Var(7), Term::Ground(45), Term::Ground(2), None));
	a.insert(Quad(Term::Ground(29), Term::Ground(3), Term::Var(49), None));
	a.insert(Quad(Term::Ground(33), Term::Var(34), Term::Var(41), None));
	a.insert(Quad(Term::Ground(6), Term::Var(13), Term::Ground(8), None));
	a.insert(Quad(Term::Var(39), Term::Var(16), Term::Ground(25), None));
	a.insert(Quad(Term::Ground(49), Term::Var(17), Term::Var(27), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Var(1043),
		Term::Var(1015),
		Term::Ground(20),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Var(1039),
		Term::Var(1014),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Ground(28),
		Term::Var(1038),
		None,
	));
	b.insert(Quad(
		Term::Var(1010),
		Term::Var(1010),
		Term::Var(1048),
		None,
	));
	b.insert(Quad(
		Term::Var(1026),
		Term::Var(1007),
		Term::Ground(25),
		None,
	));
	b.insert(Quad(
		Term::Ground(36),
		Term::Var(1041),
		Term::Ground(24),
		None,
	));
	b.insert(Quad(
		Term::Var(1048),
		Term::Ground(39),
		Term::Var(1046),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Ground(38),
		Term::Ground(26),
		None,
	));
	b.insert(Quad(
		Term::Var(1026),
		Term::Ground(32),
		Term::Var(1015),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(44),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Var(1008),
		Term::Var(1019),
		None,
	));
	b.insert(Quad(
		Term::Var(1023),
		Term::Var(1031),
		Term::Var(1049),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Var(1013),
		Term::Var(1013),
		None,
	));
	b.insert(Quad(
		Term::Var(1030),
		Term::Ground(49),
		Term::Var(1013),
		None,
	));
	b.insert(Quad(
		Term::Var(1048),
		Term::Ground(2),
		Term::Var(1035),
		None,
	));
	b.insert(Quad(
		Term::Ground(38),
		Term::Ground(46),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Ground(11),
		Term::Var(1018),
		Term::Ground(19),
		None,
	));
	b.insert(Quad(
		Term::Var(1011),
		Term::Ground(21),
		Term::Ground(41),
		None,
	));
	b.insert(Quad(
		Term::Var(1023),
		Term::Ground(29),
		Term::Var(1033),
		None,
	));
	b.insert(Quad(
		Term::Ground(22),
		Term::Ground(48),
		Term::Ground(43),
		None,
	));
	b.insert(Quad(
		Term::Var(1012),
		Term::Ground(15),
		Term::Ground(24),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Var(1025),
		Term::Ground(47),
		None,
	));
	b.insert(Quad(
		Term::Ground(32),
		Term::Var(1009),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Var(1020),
		Term::Var(1019),
		None,
	));
	b.insert(Quad(
		Term::Ground(45),
		Term::Ground(27),
		Term::Var(1016),
		None,
	));
	b.insert(Quad(
		Term::Var(1040),
		Term::Var(1006),
		Term::Var(1027),
		None,
	));
	b.insert(Quad(
		Term::Var(1019),
		Term::Var(1048),
		Term::Var(1040),
		None,
	));
	b.insert(Quad(
		Term::Ground(16),
		Term::Var(1046),
		Term::Ground(43),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Var(1046),
		Term::Var(1026),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Ground(39),
		Term::Var(1028),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Var(1024),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Ground(45),
		Term::Var(1039),
		Term::Ground(33),
		None,
	));
	b.insert(Quad(
		Term::Var(1039),
		Term::Ground(2),
		Term::Var(1042),
		None,
	));
	b.insert(Quad(
		Term::Var(1046),
		Term::Var(1042),
		Term::Ground(19),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Ground(38),
		Term::Ground(20),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Ground(46),
		Term::Ground(26),
		None,
	));
	b.insert(Quad(
		Term::Var(1048),
		Term::Var(1047),
		Term::Ground(40),
		None,
	));
	b.insert(Quad(
		Term::Var(1039),
		Term::Var(1004),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Ground(44),
		Term::Ground(19),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Var(1011),
		Term::Ground(31),
		None,
	));
	b.insert(Quad(
		Term::Var(1035),
		Term::Var(1031),
		Term::Var(1038),
		None,
	));
	b.insert(Quad(
		Term::Ground(33),
		Term::Ground(34),
		Term::Var(1010),
		None,
	));
	b.insert(Quad(
		Term::Ground(29),
		Term::Var(1014),
		Term::Ground(36),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Ground(4),
		Term::Ground(13),
		None,
	));
	b.insert(Quad(
		Term::Ground(21),
		Term::Var(1012),
		Term::Var(1045),
		None,
	));
	b.insert(Quad(
		Term::Var(1049),
		Term::Var(1006),
		Term::Ground(43),
		None,
	));
	b.insert(Quad(
		Term::Var(1040),
		Term::Var(1027),
		Term::Var(1026),
		None,
	));
	b.insert(Quad(
		Term::Var(1021),
		Term::Ground(4),
		Term::Var(1040),
		None,
	));
	b.insert(Quad(
		Term::Ground(19),
		Term::Ground(19),
		Term::Var(1013),
		None,
	));
	b.insert(Quad(
		Term::Ground(36),
		Term::Var(1002),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Ground(48),
		Term::Var(1035),
		Term::Var(1036),
		None,
	));
	b.insert(Quad(
		Term::Ground(29),
		Term::Var(1030),
		Term::Ground(18),
		None,
	));
	b.insert(Quad(
		Term::Var(1047),
		Term::Ground(42),
		Term::Var(1027),
		None,
	));
	b.insert(Quad(
		Term::Var(1026),
		Term::Var(1014),
		Term::Var(1030),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Var(1021),
		Term::Ground(47),
		None,
	));
	b.insert(Quad(
		Term::Var(1041),
		Term::Ground(35),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Var(1037),
		Term::Ground(26),
		None,
	));
	b.insert(Quad(
		Term::Ground(30),
		Term::Ground(5),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Var(1017),
		Term::Ground(39),
		Term::Ground(39),
		None,
	));
	b.insert(Quad(
		Term::Var(1020),
		Term::Ground(41),
		Term::Ground(13),
		None,
	));
	b.insert(Quad(
		Term::Var(1044),
		Term::Ground(6),
		Term::Var(1049),
		None,
	));
	b.insert(Quad(
		Term::Ground(46),
		Term::Ground(5),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Var(1003),
		Term::Var(1044),
		None,
	));
	b.insert(Quad(
		Term::Ground(34),
		Term::Ground(29),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Ground(16),
		Term::Ground(16),
		Term::Var(1015),
		None,
	));
	b.insert(Quad(
		Term::Var(1024),
		Term::Ground(31),
		Term::Ground(28),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Ground(33),
		Term::Ground(21),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Var(1022),
		Term::Ground(35),
		None,
	));
	b.insert(Quad(
		Term::Var(1012),
		Term::Ground(41),
		Term::Var(1019),
		None,
	));
	b.insert(Quad(
		Term::Ground(33),
		Term::Ground(36),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Ground(12),
		Term::Var(1040),
		Term::Ground(46),
		None,
	));
	b.insert(Quad(
		Term::Var(1032),
		Term::Ground(20),
		Term::Ground(48),
		None,
	));
	b.insert(Quad(
		Term::Ground(11),
		Term::Ground(44),
		Term::Ground(19),
		None,
	));
	b.insert(Quad(
		Term::Ground(42),
		Term::Var(1038),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Var(1024),
		Term::Ground(20),
		Term::Ground(46),
		None,
	));
	b.insert(Quad(
		Term::Ground(10),
		Term::Ground(35),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(20),
		Term::Ground(10),
		None,
	));
	b.insert(Quad(
		Term::Ground(28),
		Term::Ground(20),
		Term::Ground(34),
		None,
	));
	b.insert(Quad(
		Term::Ground(12),
		Term::Ground(24),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Var(1024),
		Term::Ground(23),
		Term::Ground(42),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Ground(39),
		Term::Ground(43),
		None,
	));
	b.insert(Quad(
		Term::Var(1040),
		Term::Var(1046),
		Term::Var(1037),
		None,
	));
	b.insert(Quad(
		Term::Ground(39),
		Term::Var(1045),
		Term::Var(1046),
		None,
	));
	b.insert(Quad(
		Term::Ground(18),
		Term::Var(1030),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Ground(33),
		Term::Var(1019),
		Term::Ground(18),
		None,
	));
	b.insert(Quad(
		Term::Var(1048),
		Term::Var(1041),
		Term::Var(1021),
		None,
	));
	b.insert(Quad(
		Term::Var(1042),
		Term::Var(1028),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Ground(7),
		Term::Var(1040),
		Term::Var(1025),
		None,
	));
	b.insert(Quad(
		Term::Ground(47),
		Term::Var(1018),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Var(1032),
		Term::Ground(20),
		Term::Var(1044),
		None,
	));
	b.insert(Quad(
		Term::Var(1012),
		Term::Ground(0),
		Term::Var(1014),
		None,
	));
	b.insert(Quad(
		Term::Var(1046),
		Term::Var(1032),
		Term::Ground(26),
		None,
	));
	b.insert(Quad(
		Term::Var(1022),
		Term::Var(1049),
		Term::Var(1031),
		None,
	));
	b.insert(Quad(
		Term::Var(1042),
		Term::Var(1018),
		Term::Ground(32),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Ground(45),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(29),
		Term::Ground(3),
		Term::Var(1049),
		None,
	));
	b.insert(Quad(
		Term::Ground(33),
		Term::Var(1034),
		Term::Var(1041),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Var(1013),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Var(1039),
		Term::Var(1016),
		Term::Ground(25),
		None,
	));
	b.insert(Quad(
		Term::Ground(49),
		Term::Var(1017),
		Term::Var(1027),
		None,
	));
	test(a, b)
}
#[test]
fn iso_193() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(
		Term::Ground(21),
		Term::Var(32),
		Term::Ground(35),
		None,
	));
	a.insert(Quad(
		Term::Var(46),
		Term::Ground(35),
		Term::Ground(49),
		None,
	));
	a.insert(Quad(Term::Var(20), Term::Ground(33), Term::Var(21), None));
	a.insert(Quad(Term::Var(33), Term::Var(1), Term::Ground(24), None));
	a.insert(Quad(Term::Ground(11), Term::Var(2), Term::Var(17), None));
	a.insert(Quad(Term::Ground(5), Term::Ground(49), Term::Var(27), None));
	a.insert(Quad(Term::Ground(22), Term::Var(31), Term::Var(29), None));
	a.insert(Quad(Term::Var(15), Term::Var(20), Term::Var(23), None));
	a.insert(Quad(Term::Var(16), Term::Ground(5), Term::Var(36), None));
	a.insert(Quad(Term::Var(22), Term::Ground(36), Term::Ground(8), None));
	a.insert(Quad(Term::Var(19), Term::Ground(0), Term::Var(9), None));
	a.insert(Quad(Term::Var(2), Term::Var(33), Term::Ground(46), None));
	a.insert(Quad(Term::Var(34), Term::Ground(40), Term::Var(46), None));
	a.insert(Quad(Term::Var(20), Term::Var(17), Term::Ground(37), None));
	a.insert(Quad(
		Term::Ground(14),
		Term::Var(20),
		Term::Ground(18),
		None,
	));
	a.insert(Quad(Term::Var(23), Term::Var(13), Term::Ground(14), None));
	a.insert(Quad(Term::Var(38), Term::Var(34), Term::Ground(43), None));
	a.insert(Quad(Term::Var(30), Term::Ground(39), Term::Var(27), None));
	a.insert(Quad(
		Term::Var(42),
		Term::Ground(46),
		Term::Ground(48),
		None,
	));
	a.insert(Quad(Term::Var(45), Term::Var(13), Term::Var(15), None));
	a.insert(Quad(Term::Var(14), Term::Ground(9), Term::Ground(10), None));
	a.insert(Quad(
		Term::Ground(23),
		Term::Ground(36),
		Term::Ground(26),
		None,
	));
	a.insert(Quad(Term::Ground(26), Term::Ground(4), Term::Var(36), None));
	a.insert(Quad(Term::Ground(37), Term::Ground(39), Term::Var(6), None));
	a.insert(Quad(
		Term::Ground(45),
		Term::Ground(36),
		Term::Ground(28),
		None,
	));
	a.insert(Quad(Term::Ground(34), Term::Ground(3), Term::Var(2), None));
	a.insert(Quad(Term::Ground(26), Term::Var(26), Term::Var(28), None));
	a.insert(Quad(Term::Var(39), Term::Var(12), Term::Ground(26), None));
	a.insert(Quad(
		Term::Ground(20),
		Term::Ground(27),
		Term::Ground(34),
		None,
	));
	a.insert(Quad(Term::Var(23), Term::Ground(29), Term::Var(10), None));
	a.insert(Quad(Term::Ground(21), Term::Var(4), Term::Var(0), None));
	a.insert(Quad(Term::Var(21), Term::Ground(20), Term::Var(11), None));
	a.insert(Quad(Term::Ground(4), Term::Var(31), Term::Var(0), None));
	a.insert(Quad(Term::Ground(48), Term::Var(33), Term::Var(13), None));
	a.insert(Quad(
		Term::Ground(18),
		Term::Ground(42),
		Term::Var(14),
		None,
	));
	a.insert(Quad(
		Term::Ground(23),
		Term::Var(44),
		Term::Ground(23),
		None,
	));
	a.insert(Quad(Term::Var(23), Term::Var(5), Term::Ground(6), None));
	a.insert(Quad(Term::Var(49), Term::Ground(8), Term::Var(39), None));
	a.insert(Quad(Term::Var(15), Term::Ground(21), Term::Var(9), None));
	a.insert(Quad(
		Term::Ground(49),
		Term::Ground(38),
		Term::Ground(1),
		None,
	));
	a.insert(Quad(Term::Ground(30), Term::Ground(20), Term::Var(9), None));
	a.insert(Quad(Term::Ground(31), Term::Var(12), Term::Var(46), None));
	a.insert(Quad(Term::Var(23), Term::Ground(13), Term::Var(28), None));
	a.insert(Quad(Term::Var(13), Term::Ground(36), Term::Var(26), None));
	a.insert(Quad(Term::Ground(8), Term::Ground(13), Term::Var(18), None));
	a.insert(Quad(Term::Var(21), Term::Var(46), Term::Ground(36), None));
	a.insert(Quad(Term::Ground(2), Term::Ground(19), Term::Var(26), None));
	a.insert(Quad(
		Term::Ground(11),
		Term::Ground(29),
		Term::Ground(9),
		None,
	));
	a.insert(Quad(
		Term::Ground(12),
		Term::Ground(19),
		Term::Var(24),
		None,
	));
	a.insert(Quad(Term::Var(3), Term::Var(27), Term::Ground(38), None));
	a.insert(Quad(Term::Var(22), Term::Ground(19), Term::Var(5), None));
	a.insert(Quad(Term::Ground(7), Term::Ground(49), Term::Var(22), None));
	a.insert(Quad(Term::Var(18), Term::Var(16), Term::Ground(7), None));
	a.insert(Quad(Term::Ground(42), Term::Var(1), Term::Ground(2), None));
	a.insert(Quad(
		Term::Var(20),
		Term::Ground(38),
		Term::Ground(39),
		None,
	));
	a.insert(Quad(Term::Var(46), Term::Var(27), Term::Var(2), None));
	a.insert(Quad(Term::Ground(9), Term::Ground(30), Term::Var(39), None));
	a.insert(Quad(Term::Var(41), Term::Var(28), Term::Ground(2), None));
	a.insert(Quad(
		Term::Ground(15),
		Term::Ground(32),
		Term::Ground(26),
		None,
	));
	a.insert(Quad(Term::Var(49), Term::Ground(24), Term::Ground(9), None));
	a.insert(Quad(Term::Ground(6), Term::Ground(19), Term::Var(3), None));
	a.insert(Quad(Term::Ground(8), Term::Var(4), Term::Var(17), None));
	a.insert(Quad(Term::Var(29), Term::Var(11), Term::Ground(25), None));
	a.insert(Quad(Term::Var(41), Term::Var(21), Term::Var(32), None));
	a.insert(Quad(
		Term::Ground(29),
		Term::Ground(13),
		Term::Var(17),
		None,
	));
	a.insert(Quad(Term::Ground(38), Term::Var(25), Term::Ground(9), None));
	a.insert(Quad(Term::Ground(15), Term::Ground(23), Term::Var(9), None));
	a.insert(Quad(
		Term::Ground(30),
		Term::Ground(41),
		Term::Ground(7),
		None,
	));
	a.insert(Quad(Term::Var(40), Term::Ground(39), Term::Var(0), None));
	a.insert(Quad(
		Term::Ground(18),
		Term::Var(41),
		Term::Ground(47),
		None,
	));
	a.insert(Quad(
		Term::Ground(20),
		Term::Ground(48),
		Term::Ground(15),
		None,
	));
	a.insert(Quad(Term::Var(26), Term::Ground(4), Term::Ground(0), None));
	a.insert(Quad(Term::Ground(2), Term::Var(23), Term::Var(26), None));
	a.insert(Quad(Term::Ground(1), Term::Ground(39), Term::Var(1), None));
	a.insert(Quad(
		Term::Ground(3),
		Term::Ground(33),
		Term::Ground(34),
		None,
	));
	a.insert(Quad(Term::Ground(9), Term::Var(44), Term::Var(49), None));
	a.insert(Quad(Term::Var(20), Term::Ground(31), Term::Ground(0), None));
	a.insert(Quad(Term::Ground(22), Term::Ground(9), Term::Var(38), None));
	a.insert(Quad(Term::Var(12), Term::Ground(15), Term::Var(32), None));
	a.insert(Quad(Term::Var(25), Term::Var(1), Term::Ground(25), None));
	a.insert(Quad(
		Term::Ground(40),
		Term::Ground(25),
		Term::Var(31),
		None,
	));
	a.insert(Quad(
		Term::Ground(10),
		Term::Var(24),
		Term::Ground(44),
		None,
	));
	a.insert(Quad(Term::Var(23), Term::Var(31), Term::Var(30), None));
	a.insert(Quad(Term::Ground(6), Term::Var(33), Term::Var(25), None));
	a.insert(Quad(
		Term::Ground(28),
		Term::Ground(14),
		Term::Var(16),
		None,
	));
	a.insert(Quad(
		Term::Var(40),
		Term::Ground(44),
		Term::Ground(14),
		None,
	));
	a.insert(Quad(
		Term::Ground(17),
		Term::Ground(45),
		Term::Var(48),
		None,
	));
	a.insert(Quad(Term::Var(47), Term::Ground(47), Term::Var(12), None));
	a.insert(Quad(Term::Var(19), Term::Var(13), Term::Var(35), None));
	a.insert(Quad(Term::Ground(37), Term::Ground(4), Term::Var(33), None));
	a.insert(Quad(Term::Var(5), Term::Var(15), Term::Var(22), None));
	a.insert(Quad(
		Term::Ground(38),
		Term::Ground(12),
		Term::Ground(40),
		None,
	));
	a.insert(Quad(Term::Ground(3), Term::Var(46), Term::Ground(35), None));
	a.insert(Quad(Term::Var(10), Term::Var(4), Term::Ground(49), None));
	a.insert(Quad(Term::Var(13), Term::Ground(37), Term::Ground(5), None));
	a.insert(Quad(Term::Ground(31), Term::Var(2), Term::Var(42), None));
	a.insert(Quad(Term::Ground(6), Term::Var(14), Term::Var(23), None));
	a.insert(Quad(Term::Var(47), Term::Var(22), Term::Ground(46), None));
	a.insert(Quad(Term::Var(37), Term::Var(42), Term::Ground(15), None));
	a.insert(Quad(
		Term::Ground(24),
		Term::Var(45),
		Term::Ground(41),
		None,
	));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(21),
		Term::Var(1032),
		Term::Ground(35),
		None,
	));
	b.insert(Quad(
		Term::Var(1046),
		Term::Ground(35),
		Term::Ground(49),
		None,
	));
	b.insert(Quad(
		Term::Var(1020),
		Term::Ground(33),
		Term::Var(1021),
		None,
	));
	b.insert(Quad(
		Term::Var(1033),
		Term::Var(1001),
		Term::Ground(24),
		None,
	));
	b.insert(Quad(
		Term::Ground(11),
		Term::Var(1002),
		Term::Var(1017),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Ground(49),
		Term::Var(1027),
		None,
	));
	b.insert(Quad(
		Term::Ground(22),
		Term::Var(1031),
		Term::Var(1029),
		None,
	));
	b.insert(Quad(
		Term::Var(1015),
		Term::Var(1020),
		Term::Var(1023),
		None,
	));
	b.insert(Quad(
		Term::Var(1016),
		Term::Ground(5),
		Term::Var(1036),
		None,
	));
	b.insert(Quad(
		Term::Var(1022),
		Term::Ground(36),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Var(1019),
		Term::Ground(0),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Var(1033),
		Term::Ground(46),
		None,
	));
	b.insert(Quad(
		Term::Var(1034),
		Term::Ground(40),
		Term::Var(1046),
		None,
	));
	b.insert(Quad(
		Term::Var(1020),
		Term::Var(1017),
		Term::Ground(37),
		None,
	));
	b.insert(Quad(
		Term::Ground(14),
		Term::Var(1020),
		Term::Ground(18),
		None,
	));
	b.insert(Quad(
		Term::Var(1023),
		Term::Var(1013),
		Term::Ground(14),
		None,
	));
	b.insert(Quad(
		Term::Var(1038),
		Term::Var(1034),
		Term::Ground(43),
		None,
	));
	b.insert(Quad(
		Term::Var(1030),
		Term::Ground(39),
		Term::Var(1027),
		None,
	));
	b.insert(Quad(
		Term::Var(1042),
		Term::Ground(46),
		Term::Ground(48),
		None,
	));
	b.insert(Quad(
		Term::Var(1045),
		Term::Var(1013),
		Term::Var(1015),
		None,
	));
	b.insert(Quad(
		Term::Var(1014),
		Term::Ground(9),
		Term::Ground(10),
		None,
	));
	b.insert(Quad(
		Term::Ground(23),
		Term::Ground(36),
		Term::Ground(26),
		None,
	));
	b.insert(Quad(
		Term::Ground(26),
		Term::Ground(4),
		Term::Var(1036),
		None,
	));
	b.insert(Quad(
		Term::Ground(37),
		Term::Ground(39),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Ground(45),
		Term::Ground(36),
		Term::Ground(28),
		None,
	));
	b.insert(Quad(
		Term::Ground(34),
		Term::Ground(3),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Ground(26),
		Term::Var(1026),
		Term::Var(1028),
		None,
	));
	b.insert(Quad(
		Term::Var(1039),
		Term::Var(1012),
		Term::Ground(26),
		None,
	));
	b.insert(Quad(
		Term::Ground(20),
		Term::Ground(27),
		Term::Ground(34),
		None,
	));
	b.insert(Quad(
		Term::Var(1023),
		Term::Ground(29),
		Term::Var(1010),
		None,
	));
	b.insert(Quad(
		Term::Ground(21),
		Term::Var(1004),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Var(1021),
		Term::Ground(20),
		Term::Var(1011),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Var(1031),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Ground(48),
		Term::Var(1033),
		Term::Var(1013),
		None,
	));
	b.insert(Quad(
		Term::Ground(18),
		Term::Ground(42),
		Term::Var(1014),
		None,
	));
	b.insert(Quad(
		Term::Ground(23),
		Term::Var(1044),
		Term::Ground(23),
		None,
	));
	b.insert(Quad(
		Term::Var(1023),
		Term::Var(1005),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Var(1049),
		Term::Ground(8),
		Term::Var(1039),
		None,
	));
	b.insert(Quad(
		Term::Var(1015),
		Term::Ground(21),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Ground(49),
		Term::Ground(38),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Ground(30),
		Term::Ground(20),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Ground(31),
		Term::Var(1012),
		Term::Var(1046),
		None,
	));
	b.insert(Quad(
		Term::Var(1023),
		Term::Ground(13),
		Term::Var(1028),
		None,
	));
	b.insert(Quad(
		Term::Var(1013),
		Term::Ground(36),
		Term::Var(1026),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Ground(13),
		Term::Var(1018),
		None,
	));
	b.insert(Quad(
		Term::Var(1021),
		Term::Var(1046),
		Term::Ground(36),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(19),
		Term::Var(1026),
		None,
	));
	b.insert(Quad(
		Term::Ground(11),
		Term::Ground(29),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Ground(12),
		Term::Ground(19),
		Term::Var(1024),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Var(1027),
		Term::Ground(38),
		None,
	));
	b.insert(Quad(
		Term::Var(1022),
		Term::Ground(19),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Ground(7),
		Term::Ground(49),
		Term::Var(1022),
		None,
	));
	b.insert(Quad(
		Term::Var(1018),
		Term::Var(1016),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Ground(42),
		Term::Var(1001),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Var(1020),
		Term::Ground(38),
		Term::Ground(39),
		None,
	));
	b.insert(Quad(
		Term::Var(1046),
		Term::Var(1027),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Ground(30),
		Term::Var(1039),
		None,
	));
	b.insert(Quad(
		Term::Var(1041),
		Term::Var(1028),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(15),
		Term::Ground(32),
		Term::Ground(26),
		None,
	));
	b.insert(Quad(
		Term::Var(1049),
		Term::Ground(24),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Ground(19),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Var(1004),
		Term::Var(1017),
		None,
	));
	b.insert(Quad(
		Term::Var(1029),
		Term::Var(1011),
		Term::Ground(25),
		None,
	));
	b.insert(Quad(
		Term::Var(1041),
		Term::Var(1021),
		Term::Var(1032),
		None,
	));
	b.insert(Quad(
		Term::Ground(29),
		Term::Ground(13),
		Term::Var(1017),
		None,
	));
	b.insert(Quad(
		Term::Ground(38),
		Term::Var(1025),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Ground(15),
		Term::Ground(23),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Ground(30),
		Term::Ground(41),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Var(1040),
		Term::Ground(39),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Ground(18),
		Term::Var(1041),
		Term::Ground(47),
		None,
	));
	b.insert(Quad(
		Term::Ground(20),
		Term::Ground(48),
		Term::Ground(15),
		None,
	));
	b.insert(Quad(
		Term::Var(1026),
		Term::Ground(4),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Var(1023),
		Term::Var(1026),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(39),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(33),
		Term::Ground(34),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Var(1044),
		Term::Var(1049),
		None,
	));
	b.insert(Quad(
		Term::Var(1020),
		Term::Ground(31),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(22),
		Term::Ground(9),
		Term::Var(1038),
		None,
	));
	b.insert(Quad(
		Term::Var(1012),
		Term::Ground(15),
		Term::Var(1032),
		None,
	));
	b.insert(Quad(
		Term::Var(1025),
		Term::Var(1001),
		Term::Ground(25),
		None,
	));
	b.insert(Quad(
		Term::Ground(40),
		Term::Ground(25),
		Term::Var(1031),
		None,
	));
	b.insert(Quad(
		Term::Ground(10),
		Term::Var(1024),
		Term::Ground(44),
		None,
	));
	b.insert(Quad(
		Term::Var(1023),
		Term::Var(1031),
		Term::Var(1030),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Var(1033),
		Term::Var(1025),
		None,
	));
	b.insert(Quad(
		Term::Ground(28),
		Term::Ground(14),
		Term::Var(1016),
		None,
	));
	b.insert(Quad(
		Term::Var(1040),
		Term::Ground(44),
		Term::Ground(14),
		None,
	));
	b.insert(Quad(
		Term::Ground(17),
		Term::Ground(45),
		Term::Var(1048),
		None,
	));
	b.insert(Quad(
		Term::Var(1047),
		Term::Ground(47),
		Term::Var(1012),
		None,
	));
	b.insert(Quad(
		Term::Var(1019),
		Term::Var(1013),
		Term::Var(1035),
		None,
	));
	b.insert(Quad(
		Term::Ground(37),
		Term::Ground(4),
		Term::Var(1033),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Var(1015),
		Term::Var(1022),
		None,
	));
	b.insert(Quad(
		Term::Ground(38),
		Term::Ground(12),
		Term::Ground(40),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Var(1046),
		Term::Ground(35),
		None,
	));
	b.insert(Quad(
		Term::Var(1010),
		Term::Var(1004),
		Term::Ground(49),
		None,
	));
	b.insert(Quad(
		Term::Var(1013),
		Term::Ground(37),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Ground(31),
		Term::Var(1002),
		Term::Var(1042),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Var(1014),
		Term::Var(1023),
		None,
	));
	b.insert(Quad(
		Term::Var(1047),
		Term::Var(1022),
		Term::Ground(46),
		None,
	));
	b.insert(Quad(
		Term::Var(1037),
		Term::Var(1042),
		Term::Ground(15),
		None,
	));
	b.insert(Quad(
		Term::Ground(24),
		Term::Var(1045),
		Term::Ground(41),
		None,
	));
	test(a, b)
}
#[test]
fn iso_194() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Var(12), Term::Var(40), Term::Ground(26), None));
	a.insert(Quad(Term::Var(7), Term::Var(45), Term::Ground(38), None));
	a.insert(Quad(Term::Var(40), Term::Var(31), Term::Var(44), None));
	a.insert(Quad(
		Term::Ground(28),
		Term::Ground(38),
		Term::Ground(9),
		None,
	));
	a.insert(Quad(Term::Ground(29), Term::Var(31), Term::Ground(7), None));
	a.insert(Quad(Term::Ground(20), Term::Var(21), Term::Var(43), None));
	a.insert(Quad(Term::Var(15), Term::Ground(43), Term::Var(41), None));
	a.insert(Quad(Term::Var(24), Term::Ground(37), Term::Var(27), None));
	a.insert(Quad(
		Term::Ground(7),
		Term::Ground(8),
		Term::Ground(17),
		None,
	));
	a.insert(Quad(Term::Ground(21), Term::Var(9), Term::Ground(33), None));
	a.insert(Quad(
		Term::Ground(42),
		Term::Ground(26),
		Term::Ground(20),
		None,
	));
	a.insert(Quad(Term::Var(3), Term::Ground(24), Term::Var(39), None));
	a.insert(Quad(
		Term::Ground(45),
		Term::Ground(20),
		Term::Ground(42),
		None,
	));
	a.insert(Quad(Term::Var(22), Term::Ground(3), Term::Ground(42), None));
	a.insert(Quad(Term::Var(14), Term::Var(12), Term::Var(49), None));
	a.insert(Quad(Term::Ground(4), Term::Var(7), Term::Var(33), None));
	a.insert(Quad(Term::Var(36), Term::Var(23), Term::Ground(0), None));
	a.insert(Quad(Term::Var(2), Term::Ground(17), Term::Ground(7), None));
	a.insert(Quad(Term::Ground(14), Term::Var(14), Term::Var(3), None));
	a.insert(Quad(
		Term::Ground(34),
		Term::Var(46),
		Term::Ground(12),
		None,
	));
	a.insert(Quad(Term::Var(4), Term::Ground(32), Term::Var(28), None));
	a.insert(Quad(Term::Ground(45), Term::Ground(38), Term::Var(4), None));
	a.insert(Quad(Term::Ground(27), Term::Var(22), Term::Var(39), None));
	a.insert(Quad(
		Term::Ground(7),
		Term::Ground(32),
		Term::Ground(41),
		None,
	));
	a.insert(Quad(Term::Ground(49), Term::Var(37), Term::Var(31), None));
	a.insert(Quad(Term::Var(3), Term::Var(40), Term::Ground(23), None));
	a.insert(Quad(Term::Var(25), Term::Var(32), Term::Ground(7), None));
	a.insert(Quad(
		Term::Ground(4),
		Term::Ground(11),
		Term::Ground(39),
		None,
	));
	a.insert(Quad(
		Term::Ground(13),
		Term::Ground(15),
		Term::Ground(47),
		None,
	));
	a.insert(Quad(Term::Var(29), Term::Var(43), Term::Var(47), None));
	a.insert(Quad(
		Term::Ground(29),
		Term::Ground(34),
		Term::Var(35),
		None,
	));
	a.insert(Quad(Term::Var(18), Term::Ground(28), Term::Var(43), None));
	a.insert(Quad(Term::Var(24), Term::Var(2), Term::Ground(2), None));
	a.insert(Quad(Term::Var(45), Term::Var(24), Term::Ground(25), None));
	a.insert(Quad(Term::Ground(4), Term::Var(48), Term::Ground(35), None));
	a.insert(Quad(Term::Var(25), Term::Ground(2), Term::Var(1), None));
	a.insert(Quad(Term::Var(42), Term::Ground(3), Term::Ground(9), None));
	a.insert(Quad(Term::Var(21), Term::Var(16), Term::Ground(37), None));
	a.insert(Quad(Term::Var(17), Term::Var(7), Term::Ground(43), None));
	a.insert(Quad(Term::Ground(10), Term::Var(14), Term::Var(47), None));
	a.insert(Quad(
		Term::Ground(38),
		Term::Ground(10),
		Term::Ground(11),
		None,
	));
	a.insert(Quad(
		Term::Ground(25),
		Term::Ground(48),
		Term::Ground(0),
		None,
	));
	a.insert(Quad(Term::Var(28), Term::Ground(14), Term::Var(10), None));
	a.insert(Quad(Term::Ground(31), Term::Var(34), Term::Var(20), None));
	a.insert(Quad(
		Term::Ground(39),
		Term::Ground(39),
		Term::Ground(31),
		None,
	));
	a.insert(Quad(Term::Ground(42), Term::Var(4), Term::Var(20), None));
	a.insert(Quad(Term::Var(19), Term::Ground(26), Term::Var(21), None));
	a.insert(Quad(
		Term::Ground(49),
		Term::Ground(35),
		Term::Ground(13),
		None,
	));
	a.insert(Quad(
		Term::Ground(33),
		Term::Ground(10),
		Term::Var(44),
		None,
	));
	a.insert(Quad(Term::Ground(48), Term::Var(26), Term::Ground(7), None));
	a.insert(Quad(Term::Ground(5), Term::Var(41), Term::Var(41), None));
	a.insert(Quad(Term::Var(21), Term::Var(40), Term::Var(14), None));
	a.insert(Quad(Term::Ground(39), Term::Var(26), Term::Var(42), None));
	a.insert(Quad(
		Term::Ground(23),
		Term::Ground(46),
		Term::Ground(9),
		None,
	));
	a.insert(Quad(Term::Ground(10), Term::Var(46), Term::Var(12), None));
	a.insert(Quad(Term::Var(28), Term::Ground(20), Term::Var(1), None));
	a.insert(Quad(
		Term::Ground(24),
		Term::Ground(48),
		Term::Var(11),
		None,
	));
	a.insert(Quad(Term::Ground(38), Term::Var(38), Term::Var(31), None));
	a.insert(Quad(
		Term::Ground(20),
		Term::Ground(39),
		Term::Ground(39),
		None,
	));
	a.insert(Quad(Term::Ground(2), Term::Var(28), Term::Ground(20), None));
	a.insert(Quad(
		Term::Ground(35),
		Term::Ground(17),
		Term::Ground(41),
		None,
	));
	a.insert(Quad(
		Term::Ground(27),
		Term::Var(36),
		Term::Ground(47),
		None,
	));
	a.insert(Quad(
		Term::Ground(23),
		Term::Ground(38),
		Term::Ground(4),
		None,
	));
	a.insert(Quad(Term::Var(15), Term::Var(32), Term::Var(0), None));
	a.insert(Quad(
		Term::Ground(22),
		Term::Ground(44),
		Term::Var(35),
		None,
	));
	a.insert(Quad(
		Term::Ground(30),
		Term::Ground(12),
		Term::Var(39),
		None,
	));
	a.insert(Quad(
		Term::Ground(10),
		Term::Ground(1),
		Term::Ground(31),
		None,
	));
	a.insert(Quad(
		Term::Var(37),
		Term::Ground(24),
		Term::Ground(33),
		None,
	));
	a.insert(Quad(
		Term::Var(21),
		Term::Ground(46),
		Term::Ground(39),
		None,
	));
	a.insert(Quad(
		Term::Var(23),
		Term::Ground(38),
		Term::Ground(33),
		None,
	));
	a.insert(Quad(Term::Var(45), Term::Ground(8), Term::Ground(40), None));
	a.insert(Quad(Term::Var(36), Term::Var(29), Term::Var(4), None));
	a.insert(Quad(Term::Var(44), Term::Var(49), Term::Var(3), None));
	a.insert(Quad(Term::Var(0), Term::Var(2), Term::Var(31), None));
	a.insert(Quad(Term::Var(39), Term::Var(45), Term::Var(10), None));
	a.insert(Quad(
		Term::Ground(12),
		Term::Var(27),
		Term::Ground(34),
		None,
	));
	a.insert(Quad(Term::Ground(28), Term::Var(35), Term::Ground(5), None));
	a.insert(Quad(
		Term::Ground(37),
		Term::Var(11),
		Term::Ground(18),
		None,
	));
	a.insert(Quad(Term::Var(6), Term::Ground(11), Term::Ground(34), None));
	a.insert(Quad(Term::Var(5), Term::Var(47), Term::Ground(32), None));
	a.insert(Quad(
		Term::Ground(31),
		Term::Var(46),
		Term::Ground(17),
		None,
	));
	a.insert(Quad(Term::Var(30), Term::Ground(27), Term::Var(21), None));
	a.insert(Quad(
		Term::Var(33),
		Term::Ground(12),
		Term::Ground(49),
		None,
	));
	a.insert(Quad(Term::Ground(33), Term::Var(2), Term::Var(12), None));
	a.insert(Quad(
		Term::Var(37),
		Term::Ground(27),
		Term::Ground(23),
		None,
	));
	a.insert(Quad(Term::Ground(39), Term::Var(22), Term::Var(43), None));
	a.insert(Quad(Term::Ground(26), Term::Ground(13), Term::Var(0), None));
	a.insert(Quad(Term::Ground(43), Term::Ground(25), Term::Var(5), None));
	a.insert(Quad(Term::Var(29), Term::Var(49), Term::Var(15), None));
	a.insert(Quad(Term::Ground(5), Term::Var(34), Term::Ground(5), None));
	a.insert(Quad(Term::Ground(47), Term::Ground(17), Term::Var(5), None));
	a.insert(Quad(
		Term::Ground(45),
		Term::Ground(28),
		Term::Var(37),
		None,
	));
	a.insert(Quad(Term::Ground(6), Term::Ground(1), Term::Var(23), None));
	a.insert(Quad(Term::Var(7), Term::Var(7), Term::Var(9), None));
	a.insert(Quad(Term::Var(42), Term::Ground(22), Term::Var(35), None));
	a.insert(Quad(
		Term::Ground(20),
		Term::Var(28),
		Term::Ground(49),
		None,
	));
	a.insert(Quad(Term::Var(21), Term::Ground(10), Term::Var(36), None));
	a.insert(Quad(
		Term::Ground(47),
		Term::Var(32),
		Term::Ground(49),
		None,
	));
	a.insert(Quad(
		Term::Var(14),
		Term::Ground(25),
		Term::Ground(12),
		None,
	));
	a.insert(Quad(
		Term::Ground(17),
		Term::Var(19),
		Term::Ground(40),
		None,
	));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Var(1012),
		Term::Var(1040),
		Term::Ground(26),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Var(1045),
		Term::Ground(38),
		None,
	));
	b.insert(Quad(
		Term::Var(1040),
		Term::Var(1031),
		Term::Var(1044),
		None,
	));
	b.insert(Quad(
		Term::Ground(28),
		Term::Ground(38),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Ground(29),
		Term::Var(1031),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Ground(20),
		Term::Var(1021),
		Term::Var(1043),
		None,
	));
	b.insert(Quad(
		Term::Var(1015),
		Term::Ground(43),
		Term::Var(1041),
		None,
	));
	b.insert(Quad(
		Term::Var(1024),
		Term::Ground(37),
		Term::Var(1027),
		None,
	));
	b.insert(Quad(
		Term::Ground(7),
		Term::Ground(8),
		Term::Ground(17),
		None,
	));
	b.insert(Quad(
		Term::Ground(21),
		Term::Var(1009),
		Term::Ground(33),
		None,
	));
	b.insert(Quad(
		Term::Ground(42),
		Term::Ground(26),
		Term::Ground(20),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Ground(24),
		Term::Var(1039),
		None,
	));
	b.insert(Quad(
		Term::Ground(45),
		Term::Ground(20),
		Term::Ground(42),
		None,
	));
	b.insert(Quad(
		Term::Var(1022),
		Term::Ground(3),
		Term::Ground(42),
		None,
	));
	b.insert(Quad(
		Term::Var(1014),
		Term::Var(1012),
		Term::Var(1049),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Var(1007),
		Term::Var(1033),
		None,
	));
	b.insert(Quad(
		Term::Var(1036),
		Term::Var(1023),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Ground(17),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Ground(14),
		Term::Var(1014),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Ground(34),
		Term::Var(1046),
		Term::Ground(12),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Ground(32),
		Term::Var(1028),
		None,
	));
	b.insert(Quad(
		Term::Ground(45),
		Term::Ground(38),
		Term::Var(1004),
		None,
	));
	b.insert(Quad(
		Term::Ground(27),
		Term::Var(1022),
		Term::Var(1039),
		None,
	));
	b.insert(Quad(
		Term::Ground(7),
		Term::Ground(32),
		Term::Ground(41),
		None,
	));
	b.insert(Quad(
		Term::Ground(49),
		Term::Var(1037),
		Term::Var(1031),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Var(1040),
		Term::Ground(23),
		None,
	));
	b.insert(Quad(
		Term::Var(1025),
		Term::Var(1032),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Ground(11),
		Term::Ground(39),
		None,
	));
	b.insert(Quad(
		Term::Ground(13),
		Term::Ground(15),
		Term::Ground(47),
		None,
	));
	b.insert(Quad(
		Term::Var(1029),
		Term::Var(1043),
		Term::Var(1047),
		None,
	));
	b.insert(Quad(
		Term::Ground(29),
		Term::Ground(34),
		Term::Var(1035),
		None,
	));
	b.insert(Quad(
		Term::Var(1018),
		Term::Ground(28),
		Term::Var(1043),
		None,
	));
	b.insert(Quad(
		Term::Var(1024),
		Term::Var(1002),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Var(1045),
		Term::Var(1024),
		Term::Ground(25),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Var(1048),
		Term::Ground(35),
		None,
	));
	b.insert(Quad(
		Term::Var(1025),
		Term::Ground(2),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Var(1042),
		Term::Ground(3),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Var(1021),
		Term::Var(1016),
		Term::Ground(37),
		None,
	));
	b.insert(Quad(
		Term::Var(1017),
		Term::Var(1007),
		Term::Ground(43),
		None,
	));
	b.insert(Quad(
		Term::Ground(10),
		Term::Var(1014),
		Term::Var(1047),
		None,
	));
	b.insert(Quad(
		Term::Ground(38),
		Term::Ground(10),
		Term::Ground(11),
		None,
	));
	b.insert(Quad(
		Term::Ground(25),
		Term::Ground(48),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Var(1028),
		Term::Ground(14),
		Term::Var(1010),
		None,
	));
	b.insert(Quad(
		Term::Ground(31),
		Term::Var(1034),
		Term::Var(1020),
		None,
	));
	b.insert(Quad(
		Term::Ground(39),
		Term::Ground(39),
		Term::Ground(31),
		None,
	));
	b.insert(Quad(
		Term::Ground(42),
		Term::Var(1004),
		Term::Var(1020),
		None,
	));
	b.insert(Quad(
		Term::Var(1019),
		Term::Ground(26),
		Term::Var(1021),
		None,
	));
	b.insert(Quad(
		Term::Ground(49),
		Term::Ground(35),
		Term::Ground(13),
		None,
	));
	b.insert(Quad(
		Term::Ground(33),
		Term::Ground(10),
		Term::Var(1044),
		None,
	));
	b.insert(Quad(
		Term::Ground(48),
		Term::Var(1026),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Var(1041),
		Term::Var(1041),
		None,
	));
	b.insert(Quad(
		Term::Var(1021),
		Term::Var(1040),
		Term::Var(1014),
		None,
	));
	b.insert(Quad(
		Term::Ground(39),
		Term::Var(1026),
		Term::Var(1042),
		None,
	));
	b.insert(Quad(
		Term::Ground(23),
		Term::Ground(46),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Ground(10),
		Term::Var(1046),
		Term::Var(1012),
		None,
	));
	b.insert(Quad(
		Term::Var(1028),
		Term::Ground(20),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(24),
		Term::Ground(48),
		Term::Var(1011),
		None,
	));
	b.insert(Quad(
		Term::Ground(38),
		Term::Var(1038),
		Term::Var(1031),
		None,
	));
	b.insert(Quad(
		Term::Ground(20),
		Term::Ground(39),
		Term::Ground(39),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Var(1028),
		Term::Ground(20),
		None,
	));
	b.insert(Quad(
		Term::Ground(35),
		Term::Ground(17),
		Term::Ground(41),
		None,
	));
	b.insert(Quad(
		Term::Ground(27),
		Term::Var(1036),
		Term::Ground(47),
		None,
	));
	b.insert(Quad(
		Term::Ground(23),
		Term::Ground(38),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Var(1015),
		Term::Var(1032),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Ground(22),
		Term::Ground(44),
		Term::Var(1035),
		None,
	));
	b.insert(Quad(
		Term::Ground(30),
		Term::Ground(12),
		Term::Var(1039),
		None,
	));
	b.insert(Quad(
		Term::Ground(10),
		Term::Ground(1),
		Term::Ground(31),
		None,
	));
	b.insert(Quad(
		Term::Var(1037),
		Term::Ground(24),
		Term::Ground(33),
		None,
	));
	b.insert(Quad(
		Term::Var(1021),
		Term::Ground(46),
		Term::Ground(39),
		None,
	));
	b.insert(Quad(
		Term::Var(1023),
		Term::Ground(38),
		Term::Ground(33),
		None,
	));
	b.insert(Quad(
		Term::Var(1045),
		Term::Ground(8),
		Term::Ground(40),
		None,
	));
	b.insert(Quad(
		Term::Var(1036),
		Term::Var(1029),
		Term::Var(1004),
		None,
	));
	b.insert(Quad(
		Term::Var(1044),
		Term::Var(1049),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Var(1002),
		Term::Var(1031),
		None,
	));
	b.insert(Quad(
		Term::Var(1039),
		Term::Var(1045),
		Term::Var(1010),
		None,
	));
	b.insert(Quad(
		Term::Ground(12),
		Term::Var(1027),
		Term::Ground(34),
		None,
	));
	b.insert(Quad(
		Term::Ground(28),
		Term::Var(1035),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Ground(37),
		Term::Var(1011),
		Term::Ground(18),
		None,
	));
	b.insert(Quad(
		Term::Var(1006),
		Term::Ground(11),
		Term::Ground(34),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Var(1047),
		Term::Ground(32),
		None,
	));
	b.insert(Quad(
		Term::Ground(31),
		Term::Var(1046),
		Term::Ground(17),
		None,
	));
	b.insert(Quad(
		Term::Var(1030),
		Term::Ground(27),
		Term::Var(1021),
		None,
	));
	b.insert(Quad(
		Term::Var(1033),
		Term::Ground(12),
		Term::Ground(49),
		None,
	));
	b.insert(Quad(
		Term::Ground(33),
		Term::Var(1002),
		Term::Var(1012),
		None,
	));
	b.insert(Quad(
		Term::Var(1037),
		Term::Ground(27),
		Term::Ground(23),
		None,
	));
	b.insert(Quad(
		Term::Ground(39),
		Term::Var(1022),
		Term::Var(1043),
		None,
	));
	b.insert(Quad(
		Term::Ground(26),
		Term::Ground(13),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Ground(43),
		Term::Ground(25),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Var(1029),
		Term::Var(1049),
		Term::Var(1015),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Var(1034),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Ground(47),
		Term::Ground(17),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Ground(45),
		Term::Ground(28),
		Term::Var(1037),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Ground(1),
		Term::Var(1023),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Var(1007),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Var(1042),
		Term::Ground(22),
		Term::Var(1035),
		None,
	));
	b.insert(Quad(
		Term::Ground(20),
		Term::Var(1028),
		Term::Ground(49),
		None,
	));
	b.insert(Quad(
		Term::Var(1021),
		Term::Ground(10),
		Term::Var(1036),
		None,
	));
	b.insert(Quad(
		Term::Ground(47),
		Term::Var(1032),
		Term::Ground(49),
		None,
	));
	b.insert(Quad(
		Term::Var(1014),
		Term::Ground(25),
		Term::Ground(12),
		None,
	));
	b.insert(Quad(
		Term::Ground(17),
		Term::Var(1019),
		Term::Ground(40),
		None,
	));
	test(a, b)
}
#[test]
fn iso_195() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(
		Term::Ground(15),
		Term::Ground(28),
		Term::Var(25),
		None,
	));
	a.insert(Quad(Term::Var(31), Term::Ground(25), Term::Var(38), None));
	a.insert(Quad(Term::Var(0), Term::Var(48), Term::Ground(3), None));
	a.insert(Quad(Term::Var(41), Term::Var(47), Term::Var(37), None));
	a.insert(Quad(
		Term::Ground(17),
		Term::Ground(24),
		Term::Ground(25),
		None,
	));
	a.insert(Quad(Term::Var(19), Term::Ground(13), Term::Var(25), None));
	a.insert(Quad(Term::Ground(0), Term::Var(21), Term::Ground(33), None));
	a.insert(Quad(
		Term::Ground(23),
		Term::Ground(48),
		Term::Var(25),
		None,
	));
	a.insert(Quad(
		Term::Ground(18),
		Term::Ground(38),
		Term::Var(21),
		None,
	));
	a.insert(Quad(
		Term::Ground(15),
		Term::Ground(32),
		Term::Ground(14),
		None,
	));
	a.insert(Quad(Term::Var(49), Term::Ground(21), Term::Var(19), None));
	a.insert(Quad(Term::Var(7), Term::Var(3), Term::Var(26), None));
	a.insert(Quad(
		Term::Ground(17),
		Term::Ground(45),
		Term::Ground(29),
		None,
	));
	a.insert(Quad(Term::Ground(34), Term::Ground(4), Term::Var(30), None));
	a.insert(Quad(Term::Ground(18), Term::Var(42), Term::Var(44), None));
	a.insert(Quad(Term::Ground(10), Term::Var(39), Term::Var(29), None));
	a.insert(Quad(Term::Ground(17), Term::Var(14), Term::Var(25), None));
	a.insert(Quad(Term::Var(27), Term::Var(37), Term::Ground(43), None));
	a.insert(Quad(Term::Var(4), Term::Var(0), Term::Var(4), None));
	a.insert(Quad(
		Term::Ground(6),
		Term::Ground(25),
		Term::Ground(22),
		None,
	));
	a.insert(Quad(
		Term::Ground(38),
		Term::Ground(47),
		Term::Var(43),
		None,
	));
	a.insert(Quad(Term::Ground(37), Term::Var(3), Term::Var(9), None));
	a.insert(Quad(Term::Var(48), Term::Ground(27), Term::Var(30), None));
	a.insert(Quad(
		Term::Ground(23),
		Term::Ground(14),
		Term::Var(26),
		None,
	));
	a.insert(Quad(
		Term::Ground(34),
		Term::Ground(43),
		Term::Ground(26),
		None,
	));
	a.insert(Quad(Term::Var(40), Term::Var(33), Term::Ground(31), None));
	a.insert(Quad(Term::Ground(17), Term::Var(40), Term::Var(5), None));
	a.insert(Quad(
		Term::Var(48),
		Term::Ground(10),
		Term::Ground(19),
		None,
	));
	a.insert(Quad(Term::Var(6), Term::Var(2), Term::Ground(13), None));
	a.insert(Quad(Term::Ground(41), Term::Var(6), Term::Ground(23), None));
	a.insert(Quad(Term::Ground(34), Term::Var(41), Term::Var(47), None));
	a.insert(Quad(
		Term::Ground(19),
		Term::Ground(12),
		Term::Ground(35),
		None,
	));
	a.insert(Quad(
		Term::Var(13),
		Term::Ground(19),
		Term::Ground(43),
		None,
	));
	a.insert(Quad(Term::Var(35), Term::Var(30), Term::Ground(48), None));
	a.insert(Quad(Term::Var(45), Term::Var(30), Term::Var(11), None));
	a.insert(Quad(Term::Var(36), Term::Var(38), Term::Ground(36), None));
	a.insert(Quad(Term::Var(15), Term::Var(43), Term::Ground(15), None));
	a.insert(Quad(Term::Ground(3), Term::Var(24), Term::Var(19), None));
	a.insert(Quad(Term::Ground(8), Term::Var(37), Term::Var(18), None));
	a.insert(Quad(Term::Ground(25), Term::Var(12), Term::Var(3), None));
	a.insert(Quad(
		Term::Ground(27),
		Term::Ground(22),
		Term::Ground(38),
		None,
	));
	a.insert(Quad(Term::Var(36), Term::Var(28), Term::Ground(21), None));
	a.insert(Quad(Term::Ground(34), Term::Var(14), Term::Var(18), None));
	a.insert(Quad(Term::Ground(47), Term::Var(39), Term::Ground(1), None));
	a.insert(Quad(Term::Var(6), Term::Ground(11), Term::Var(21), None));
	a.insert(Quad(Term::Var(33), Term::Var(34), Term::Ground(30), None));
	a.insert(Quad(Term::Ground(33), Term::Var(0), Term::Ground(11), None));
	a.insert(Quad(Term::Ground(22), Term::Var(8), Term::Ground(48), None));
	a.insert(Quad(Term::Var(43), Term::Var(27), Term::Ground(36), None));
	a.insert(Quad(Term::Var(37), Term::Var(45), Term::Var(46), None));
	a.insert(Quad(
		Term::Var(14),
		Term::Ground(38),
		Term::Ground(34),
		None,
	));
	a.insert(Quad(Term::Ground(6), Term::Var(46), Term::Var(36), None));
	a.insert(Quad(
		Term::Ground(27),
		Term::Ground(38),
		Term::Ground(9),
		None,
	));
	a.insert(Quad(Term::Ground(33), Term::Var(42), Term::Ground(9), None));
	a.insert(Quad(Term::Ground(47), Term::Var(36), Term::Var(28), None));
	a.insert(Quad(Term::Var(0), Term::Var(28), Term::Var(19), None));
	a.insert(Quad(Term::Var(28), Term::Var(11), Term::Var(13), None));
	a.insert(Quad(Term::Ground(28), Term::Var(45), Term::Var(9), None));
	a.insert(Quad(
		Term::Ground(36),
		Term::Ground(44),
		Term::Ground(6),
		None,
	));
	a.insert(Quad(Term::Ground(35), Term::Ground(8), Term::Var(49), None));
	a.insert(Quad(Term::Var(20), Term::Var(16), Term::Var(9), None));
	a.insert(Quad(Term::Ground(2), Term::Ground(30), Term::Var(34), None));
	a.insert(Quad(
		Term::Ground(46),
		Term::Ground(46),
		Term::Var(33),
		None,
	));
	a.insert(Quad(
		Term::Ground(43),
		Term::Var(17),
		Term::Ground(26),
		None,
	));
	a.insert(Quad(
		Term::Ground(11),
		Term::Ground(31),
		Term::Ground(2),
		None,
	));
	a.insert(Quad(
		Term::Ground(41),
		Term::Ground(11),
		Term::Var(23),
		None,
	));
	a.insert(Quad(Term::Ground(0), Term::Var(45), Term::Var(32), None));
	a.insert(Quad(Term::Var(14), Term::Ground(17), Term::Var(1), None));
	a.insert(Quad(Term::Ground(18), Term::Ground(25), Term::Var(3), None));
	a.insert(Quad(Term::Var(18), Term::Var(49), Term::Ground(27), None));
	a.insert(Quad(Term::Var(10), Term::Ground(5), Term::Var(4), None));
	a.insert(Quad(
		Term::Ground(7),
		Term::Ground(9),
		Term::Ground(1),
		None,
	));
	a.insert(Quad(Term::Ground(31), Term::Ground(6), Term::Var(18), None));
	a.insert(Quad(Term::Var(48), Term::Ground(20), Term::Var(47), None));
	a.insert(Quad(
		Term::Ground(12),
		Term::Var(38),
		Term::Ground(31),
		None,
	));
	a.insert(Quad(Term::Var(16), Term::Ground(16), Term::Var(11), None));
	a.insert(Quad(Term::Var(8), Term::Var(31), Term::Var(46), None));
	a.insert(Quad(Term::Var(27), Term::Var(24), Term::Ground(2), None));
	a.insert(Quad(Term::Ground(37), Term::Var(29), Term::Var(4), None));
	a.insert(Quad(Term::Var(49), Term::Ground(16), Term::Var(5), None));
	a.insert(Quad(Term::Var(22), Term::Var(7), Term::Ground(38), None));
	a.insert(Quad(Term::Var(49), Term::Var(11), Term::Var(7), None));
	a.insert(Quad(Term::Ground(8), Term::Var(2), Term::Ground(39), None));
	a.insert(Quad(Term::Ground(39), Term::Var(17), Term::Ground(7), None));
	a.insert(Quad(
		Term::Var(15),
		Term::Ground(19),
		Term::Ground(29),
		None,
	));
	a.insert(Quad(Term::Var(21), Term::Var(49), Term::Ground(28), None));
	a.insert(Quad(Term::Var(24), Term::Var(31), Term::Ground(8), None));
	a.insert(Quad(Term::Var(6), Term::Ground(24), Term::Var(6), None));
	a.insert(Quad(
		Term::Ground(26),
		Term::Var(11),
		Term::Ground(40),
		None,
	));
	a.insert(Quad(
		Term::Ground(40),
		Term::Ground(45),
		Term::Ground(31),
		None,
	));
	a.insert(Quad(
		Term::Ground(39),
		Term::Ground(16),
		Term::Ground(15),
		None,
	));
	a.insert(Quad(Term::Var(34), Term::Var(31), Term::Var(47), None));
	a.insert(Quad(
		Term::Ground(1),
		Term::Ground(2),
		Term::Ground(38),
		None,
	));
	a.insert(Quad(
		Term::Ground(27),
		Term::Ground(41),
		Term::Ground(38),
		None,
	));
	a.insert(Quad(Term::Ground(45), Term::Ground(11), Term::Var(2), None));
	a.insert(Quad(Term::Ground(32), Term::Var(0), Term::Ground(43), None));
	a.insert(Quad(
		Term::Ground(46),
		Term::Ground(48),
		Term::Ground(41),
		None,
	));
	a.insert(Quad(Term::Ground(36), Term::Var(14), Term::Var(49), None));
	a.insert(Quad(Term::Var(10), Term::Ground(4), Term::Var(15), None));
	a.insert(Quad(
		Term::Ground(38),
		Term::Ground(13),
		Term::Ground(47),
		None,
	));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(15),
		Term::Ground(28),
		Term::Var(1025),
		None,
	));
	b.insert(Quad(
		Term::Var(1031),
		Term::Ground(25),
		Term::Var(1038),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Var(1048),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Var(1041),
		Term::Var(1047),
		Term::Var(1037),
		None,
	));
	b.insert(Quad(
		Term::Ground(17),
		Term::Ground(24),
		Term::Ground(25),
		None,
	));
	b.insert(Quad(
		Term::Var(1019),
		Term::Ground(13),
		Term::Var(1025),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Var(1021),
		Term::Ground(33),
		None,
	));
	b.insert(Quad(
		Term::Ground(23),
		Term::Ground(48),
		Term::Var(1025),
		None,
	));
	b.insert(Quad(
		Term::Ground(18),
		Term::Ground(38),
		Term::Var(1021),
		None,
	));
	b.insert(Quad(
		Term::Ground(15),
		Term::Ground(32),
		Term::Ground(14),
		None,
	));
	b.insert(Quad(
		Term::Var(1049),
		Term::Ground(21),
		Term::Var(1019),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Var(1003),
		Term::Var(1026),
		None,
	));
	b.insert(Quad(
		Term::Ground(17),
		Term::Ground(45),
		Term::Ground(29),
		None,
	));
	b.insert(Quad(
		Term::Ground(34),
		Term::Ground(4),
		Term::Var(1030),
		None,
	));
	b.insert(Quad(
		Term::Ground(18),
		Term::Var(1042),
		Term::Var(1044),
		None,
	));
	b.insert(Quad(
		Term::Ground(10),
		Term::Var(1039),
		Term::Var(1029),
		None,
	));
	b.insert(Quad(
		Term::Ground(17),
		Term::Var(1014),
		Term::Var(1025),
		None,
	));
	b.insert(Quad(
		Term::Var(1027),
		Term::Var(1037),
		Term::Ground(43),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Var(1000),
		Term::Var(1004),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Ground(25),
		Term::Ground(22),
		None,
	));
	b.insert(Quad(
		Term::Ground(38),
		Term::Ground(47),
		Term::Var(1043),
		None,
	));
	b.insert(Quad(
		Term::Ground(37),
		Term::Var(1003),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Var(1048),
		Term::Ground(27),
		Term::Var(1030),
		None,
	));
	b.insert(Quad(
		Term::Ground(23),
		Term::Ground(14),
		Term::Var(1026),
		None,
	));
	b.insert(Quad(
		Term::Ground(34),
		Term::Ground(43),
		Term::Ground(26),
		None,
	));
	b.insert(Quad(
		Term::Var(1040),
		Term::Var(1033),
		Term::Ground(31),
		None,
	));
	b.insert(Quad(
		Term::Ground(17),
		Term::Var(1040),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Var(1048),
		Term::Ground(10),
		Term::Ground(19),
		None,
	));
	b.insert(Quad(
		Term::Var(1006),
		Term::Var(1002),
		Term::Ground(13),
		None,
	));
	b.insert(Quad(
		Term::Ground(41),
		Term::Var(1006),
		Term::Ground(23),
		None,
	));
	b.insert(Quad(
		Term::Ground(34),
		Term::Var(1041),
		Term::Var(1047),
		None,
	));
	b.insert(Quad(
		Term::Ground(19),
		Term::Ground(12),
		Term::Ground(35),
		None,
	));
	b.insert(Quad(
		Term::Var(1013),
		Term::Ground(19),
		Term::Ground(43),
		None,
	));
	b.insert(Quad(
		Term::Var(1035),
		Term::Var(1030),
		Term::Ground(48),
		None,
	));
	b.insert(Quad(
		Term::Var(1045),
		Term::Var(1030),
		Term::Var(1011),
		None,
	));
	b.insert(Quad(
		Term::Var(1036),
		Term::Var(1038),
		Term::Ground(36),
		None,
	));
	b.insert(Quad(
		Term::Var(1015),
		Term::Var(1043),
		Term::Ground(15),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Var(1024),
		Term::Var(1019),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Var(1037),
		Term::Var(1018),
		None,
	));
	b.insert(Quad(
		Term::Ground(25),
		Term::Var(1012),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Ground(27),
		Term::Ground(22),
		Term::Ground(38),
		None,
	));
	b.insert(Quad(
		Term::Var(1036),
		Term::Var(1028),
		Term::Ground(21),
		None,
	));
	b.insert(Quad(
		Term::Ground(34),
		Term::Var(1014),
		Term::Var(1018),
		None,
	));
	b.insert(Quad(
		Term::Ground(47),
		Term::Var(1039),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Var(1006),
		Term::Ground(11),
		Term::Var(1021),
		None,
	));
	b.insert(Quad(
		Term::Var(1033),
		Term::Var(1034),
		Term::Ground(30),
		None,
	));
	b.insert(Quad(
		Term::Ground(33),
		Term::Var(1000),
		Term::Ground(11),
		None,
	));
	b.insert(Quad(
		Term::Ground(22),
		Term::Var(1008),
		Term::Ground(48),
		None,
	));
	b.insert(Quad(
		Term::Var(1043),
		Term::Var(1027),
		Term::Ground(36),
		None,
	));
	b.insert(Quad(
		Term::Var(1037),
		Term::Var(1045),
		Term::Var(1046),
		None,
	));
	b.insert(Quad(
		Term::Var(1014),
		Term::Ground(38),
		Term::Ground(34),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Var(1046),
		Term::Var(1036),
		None,
	));
	b.insert(Quad(
		Term::Ground(27),
		Term::Ground(38),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Ground(33),
		Term::Var(1042),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Ground(47),
		Term::Var(1036),
		Term::Var(1028),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Var(1028),
		Term::Var(1019),
		None,
	));
	b.insert(Quad(
		Term::Var(1028),
		Term::Var(1011),
		Term::Var(1013),
		None,
	));
	b.insert(Quad(
		Term::Ground(28),
		Term::Var(1045),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Ground(36),
		Term::Ground(44),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Ground(35),
		Term::Ground(8),
		Term::Var(1049),
		None,
	));
	b.insert(Quad(
		Term::Var(1020),
		Term::Var(1016),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(30),
		Term::Var(1034),
		None,
	));
	b.insert(Quad(
		Term::Ground(46),
		Term::Ground(46),
		Term::Var(1033),
		None,
	));
	b.insert(Quad(
		Term::Ground(43),
		Term::Var(1017),
		Term::Ground(26),
		None,
	));
	b.insert(Quad(
		Term::Ground(11),
		Term::Ground(31),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(41),
		Term::Ground(11),
		Term::Var(1023),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Var(1045),
		Term::Var(1032),
		None,
	));
	b.insert(Quad(
		Term::Var(1014),
		Term::Ground(17),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(18),
		Term::Ground(25),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Var(1018),
		Term::Var(1049),
		Term::Ground(27),
		None,
	));
	b.insert(Quad(
		Term::Var(1010),
		Term::Ground(5),
		Term::Var(1004),
		None,
	));
	b.insert(Quad(
		Term::Ground(7),
		Term::Ground(9),
		Term::Ground(1),
		None,
	));
	b.insert(Quad(
		Term::Ground(31),
		Term::Ground(6),
		Term::Var(1018),
		None,
	));
	b.insert(Quad(
		Term::Var(1048),
		Term::Ground(20),
		Term::Var(1047),
		None,
	));
	b.insert(Quad(
		Term::Ground(12),
		Term::Var(1038),
		Term::Ground(31),
		None,
	));
	b.insert(Quad(
		Term::Var(1016),
		Term::Ground(16),
		Term::Var(1011),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Var(1031),
		Term::Var(1046),
		None,
	));
	b.insert(Quad(
		Term::Var(1027),
		Term::Var(1024),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(37),
		Term::Var(1029),
		Term::Var(1004),
		None,
	));
	b.insert(Quad(
		Term::Var(1049),
		Term::Ground(16),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Var(1022),
		Term::Var(1007),
		Term::Ground(38),
		None,
	));
	b.insert(Quad(
		Term::Var(1049),
		Term::Var(1011),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Var(1002),
		Term::Ground(39),
		None,
	));
	b.insert(Quad(
		Term::Ground(39),
		Term::Var(1017),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Var(1015),
		Term::Ground(19),
		Term::Ground(29),
		None,
	));
	b.insert(Quad(
		Term::Var(1021),
		Term::Var(1049),
		Term::Ground(28),
		None,
	));
	b.insert(Quad(
		Term::Var(1024),
		Term::Var(1031),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Var(1006),
		Term::Ground(24),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Ground(26),
		Term::Var(1011),
		Term::Ground(40),
		None,
	));
	b.insert(Quad(
		Term::Ground(40),
		Term::Ground(45),
		Term::Ground(31),
		None,
	));
	b.insert(Quad(
		Term::Ground(39),
		Term::Ground(16),
		Term::Ground(15),
		None,
	));
	b.insert(Quad(
		Term::Var(1034),
		Term::Var(1031),
		Term::Var(1047),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(2),
		Term::Ground(38),
		None,
	));
	b.insert(Quad(
		Term::Ground(27),
		Term::Ground(41),
		Term::Ground(38),
		None,
	));
	b.insert(Quad(
		Term::Ground(45),
		Term::Ground(11),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Ground(32),
		Term::Var(1000),
		Term::Ground(43),
		None,
	));
	b.insert(Quad(
		Term::Ground(46),
		Term::Ground(48),
		Term::Ground(41),
		None,
	));
	b.insert(Quad(
		Term::Ground(36),
		Term::Var(1014),
		Term::Var(1049),
		None,
	));
	b.insert(Quad(
		Term::Var(1010),
		Term::Ground(4),
		Term::Var(1015),
		None,
	));
	b.insert(Quad(
		Term::Ground(38),
		Term::Ground(13),
		Term::Ground(47),
		None,
	));
	test(a, b)
}
#[test]
fn iso_196() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(
		Term::Ground(36),
		Term::Var(40),
		Term::Ground(30),
		None,
	));
	a.insert(Quad(Term::Ground(48), Term::Var(44), Term::Var(7), None));
	a.insert(Quad(Term::Ground(32), Term::Var(15), Term::Var(47), None));
	a.insert(Quad(
		Term::Ground(26),
		Term::Ground(11),
		Term::Ground(28),
		None,
	));
	a.insert(Quad(Term::Var(29), Term::Ground(29), Term::Var(1), None));
	a.insert(Quad(Term::Var(20), Term::Ground(49), Term::Ground(8), None));
	a.insert(Quad(
		Term::Ground(13),
		Term::Ground(9),
		Term::Ground(16),
		None,
	));
	a.insert(Quad(Term::Var(13), Term::Var(8), Term::Var(1), None));
	a.insert(Quad(
		Term::Ground(22),
		Term::Ground(12),
		Term::Ground(17),
		None,
	));
	a.insert(Quad(Term::Ground(6), Term::Ground(36), Term::Var(33), None));
	a.insert(Quad(Term::Var(31), Term::Ground(7), Term::Var(21), None));
	a.insert(Quad(Term::Var(45), Term::Var(33), Term::Ground(26), None));
	a.insert(Quad(
		Term::Ground(48),
		Term::Ground(28),
		Term::Ground(31),
		None,
	));
	a.insert(Quad(
		Term::Ground(0),
		Term::Ground(31),
		Term::Ground(2),
		None,
	));
	a.insert(Quad(Term::Var(13), Term::Var(26), Term::Ground(27), None));
	a.insert(Quad(Term::Var(46), Term::Var(26), Term::Var(24), None));
	a.insert(Quad(
		Term::Var(12),
		Term::Ground(12),
		Term::Ground(20),
		None,
	));
	a.insert(Quad(Term::Var(29), Term::Var(3), Term::Ground(3), None));
	a.insert(Quad(Term::Ground(3), Term::Var(9), Term::Var(6), None));
	a.insert(Quad(Term::Ground(19), Term::Var(25), Term::Var(42), None));
	a.insert(Quad(Term::Var(23), Term::Ground(29), Term::Var(39), None));
	a.insert(Quad(Term::Ground(1), Term::Var(14), Term::Var(22), None));
	a.insert(Quad(Term::Ground(16), Term::Var(2), Term::Ground(24), None));
	a.insert(Quad(Term::Ground(41), Term::Var(4), Term::Ground(7), None));
	a.insert(Quad(Term::Var(5), Term::Var(6), Term::Ground(12), None));
	a.insert(Quad(Term::Ground(30), Term::Ground(28), Term::Var(1), None));
	a.insert(Quad(Term::Ground(35), Term::Var(3), Term::Var(40), None));
	a.insert(Quad(Term::Ground(41), Term::Var(27), Term::Var(25), None));
	a.insert(Quad(
		Term::Var(15),
		Term::Ground(25),
		Term::Ground(10),
		None,
	));
	a.insert(Quad(Term::Ground(19), Term::Var(0), Term::Var(2), None));
	a.insert(Quad(
		Term::Ground(18),
		Term::Var(16),
		Term::Ground(39),
		None,
	));
	a.insert(Quad(Term::Var(31), Term::Var(45), Term::Var(38), None));
	a.insert(Quad(Term::Ground(12), Term::Var(26), Term::Var(26), None));
	a.insert(Quad(Term::Var(11), Term::Ground(4), Term::Ground(41), None));
	a.insert(Quad(Term::Var(12), Term::Var(34), Term::Ground(9), None));
	a.insert(Quad(
		Term::Ground(40),
		Term::Var(49),
		Term::Ground(49),
		None,
	));
	a.insert(Quad(
		Term::Ground(5),
		Term::Ground(17),
		Term::Ground(48),
		None,
	));
	a.insert(Quad(Term::Var(28), Term::Ground(24), Term::Var(25), None));
	a.insert(Quad(
		Term::Ground(20),
		Term::Ground(44),
		Term::Var(16),
		None,
	));
	a.insert(Quad(
		Term::Ground(23),
		Term::Ground(19),
		Term::Ground(24),
		None,
	));
	a.insert(Quad(
		Term::Ground(48),
		Term::Ground(12),
		Term::Ground(2),
		None,
	));
	a.insert(Quad(
		Term::Ground(36),
		Term::Ground(27),
		Term::Var(12),
		None,
	));
	a.insert(Quad(Term::Var(4), Term::Var(7), Term::Ground(38), None));
	a.insert(Quad(Term::Var(20), Term::Ground(8), Term::Var(30), None));
	a.insert(Quad(Term::Var(30), Term::Var(11), Term::Ground(29), None));
	a.insert(Quad(Term::Var(7), Term::Var(40), Term::Var(41), None));
	a.insert(Quad(Term::Ground(43), Term::Var(12), Term::Var(5), None));
	a.insert(Quad(Term::Var(30), Term::Var(0), Term::Ground(49), None));
	a.insert(Quad(
		Term::Ground(29),
		Term::Ground(19),
		Term::Ground(17),
		None,
	));
	a.insert(Quad(Term::Ground(4), Term::Var(46), Term::Ground(41), None));
	a.insert(Quad(Term::Var(21), Term::Var(34), Term::Var(7), None));
	a.insert(Quad(
		Term::Var(45),
		Term::Ground(33),
		Term::Ground(39),
		None,
	));
	a.insert(Quad(
		Term::Ground(43),
		Term::Var(31),
		Term::Ground(19),
		None,
	));
	a.insert(Quad(
		Term::Ground(43),
		Term::Ground(45),
		Term::Ground(35),
		None,
	));
	a.insert(Quad(
		Term::Ground(10),
		Term::Ground(18),
		Term::Var(28),
		None,
	));
	a.insert(Quad(Term::Ground(5), Term::Var(30), Term::Ground(22), None));
	a.insert(Quad(
		Term::Var(11),
		Term::Ground(36),
		Term::Ground(38),
		None,
	));
	a.insert(Quad(
		Term::Ground(25),
		Term::Ground(15),
		Term::Ground(47),
		None,
	));
	a.insert(Quad(Term::Var(15), Term::Var(29), Term::Var(17), None));
	a.insert(Quad(Term::Ground(31), Term::Ground(1), Term::Var(26), None));
	a.insert(Quad(Term::Var(30), Term::Ground(5), Term::Ground(48), None));
	a.insert(Quad(Term::Var(36), Term::Var(25), Term::Ground(17), None));
	a.insert(Quad(Term::Var(4), Term::Var(45), Term::Ground(30), None));
	a.insert(Quad(Term::Var(28), Term::Var(38), Term::Ground(22), None));
	a.insert(Quad(
		Term::Ground(46),
		Term::Var(34),
		Term::Ground(24),
		None,
	));
	a.insert(Quad(
		Term::Ground(11),
		Term::Var(31),
		Term::Ground(46),
		None,
	));
	a.insert(Quad(
		Term::Ground(26),
		Term::Var(12),
		Term::Ground(22),
		None,
	));
	a.insert(Quad(Term::Ground(37), Term::Var(7), Term::Var(19), None));
	a.insert(Quad(Term::Var(18), Term::Var(25), Term::Var(41), None));
	a.insert(Quad(Term::Var(27), Term::Ground(47), Term::Var(22), None));
	a.insert(Quad(Term::Ground(6), Term::Var(26), Term::Var(41), None));
	a.insert(Quad(Term::Var(25), Term::Ground(36), Term::Var(40), None));
	a.insert(Quad(Term::Ground(35), Term::Var(3), Term::Ground(4), None));
	a.insert(Quad(
		Term::Ground(48),
		Term::Ground(34),
		Term::Ground(15),
		None,
	));
	a.insert(Quad(Term::Var(28), Term::Var(49), Term::Ground(49), None));
	a.insert(Quad(
		Term::Var(17),
		Term::Ground(30),
		Term::Ground(41),
		None,
	));
	a.insert(Quad(Term::Ground(16), Term::Var(32), Term::Var(47), None));
	a.insert(Quad(Term::Var(4), Term::Var(6), Term::Var(27), None));
	a.insert(Quad(Term::Var(2), Term::Var(24), Term::Ground(47), None));
	a.insert(Quad(Term::Ground(13), Term::Var(23), Term::Var(15), None));
	a.insert(Quad(Term::Var(9), Term::Ground(35), Term::Var(32), None));
	a.insert(Quad(Term::Var(15), Term::Var(44), Term::Ground(2), None));
	a.insert(Quad(Term::Var(16), Term::Ground(12), Term::Var(0), None));
	a.insert(Quad(Term::Var(32), Term::Var(34), Term::Var(36), None));
	a.insert(Quad(Term::Var(37), Term::Var(4), Term::Ground(24), None));
	a.insert(Quad(Term::Ground(26), Term::Var(31), Term::Var(22), None));
	a.insert(Quad(Term::Var(22), Term::Var(32), Term::Var(46), None));
	a.insert(Quad(Term::Var(25), Term::Var(3), Term::Var(7), None));
	a.insert(Quad(Term::Var(1), Term::Var(34), Term::Ground(23), None));
	a.insert(Quad(
		Term::Var(23),
		Term::Ground(18),
		Term::Ground(22),
		None,
	));
	a.insert(Quad(
		Term::Ground(28),
		Term::Var(20),
		Term::Ground(12),
		None,
	));
	a.insert(Quad(
		Term::Ground(34),
		Term::Ground(9),
		Term::Ground(2),
		None,
	));
	a.insert(Quad(Term::Var(44), Term::Ground(0), Term::Var(3), None));
	a.insert(Quad(Term::Ground(6), Term::Var(44), Term::Var(9), None));
	a.insert(Quad(Term::Var(0), Term::Ground(17), Term::Ground(34), None));
	a.insert(Quad(Term::Ground(27), Term::Ground(4), Term::Var(43), None));
	a.insert(Quad(
		Term::Ground(40),
		Term::Var(35),
		Term::Ground(41),
		None,
	));
	a.insert(Quad(Term::Var(12), Term::Var(2), Term::Ground(5), None));
	a.insert(Quad(Term::Var(29), Term::Ground(40), Term::Var(48), None));
	a.insert(Quad(Term::Var(11), Term::Ground(8), Term::Ground(2), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(36),
		Term::Var(1040),
		Term::Ground(30),
		None,
	));
	b.insert(Quad(
		Term::Ground(48),
		Term::Var(1044),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Ground(32),
		Term::Var(1015),
		Term::Var(1047),
		None,
	));
	b.insert(Quad(
		Term::Ground(26),
		Term::Ground(11),
		Term::Ground(28),
		None,
	));
	b.insert(Quad(
		Term::Var(1029),
		Term::Ground(29),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Var(1020),
		Term::Ground(49),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Ground(13),
		Term::Ground(9),
		Term::Ground(16),
		None,
	));
	b.insert(Quad(
		Term::Var(1013),
		Term::Var(1008),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(22),
		Term::Ground(12),
		Term::Ground(17),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Ground(36),
		Term::Var(1033),
		None,
	));
	b.insert(Quad(
		Term::Var(1031),
		Term::Ground(7),
		Term::Var(1021),
		None,
	));
	b.insert(Quad(
		Term::Var(1045),
		Term::Var(1033),
		Term::Ground(26),
		None,
	));
	b.insert(Quad(
		Term::Ground(48),
		Term::Ground(28),
		Term::Ground(31),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(31),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Var(1013),
		Term::Var(1026),
		Term::Ground(27),
		None,
	));
	b.insert(Quad(
		Term::Var(1046),
		Term::Var(1026),
		Term::Var(1024),
		None,
	));
	b.insert(Quad(
		Term::Var(1012),
		Term::Ground(12),
		Term::Ground(20),
		None,
	));
	b.insert(Quad(
		Term::Var(1029),
		Term::Var(1003),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Var(1009),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Ground(19),
		Term::Var(1025),
		Term::Var(1042),
		None,
	));
	b.insert(Quad(
		Term::Var(1023),
		Term::Ground(29),
		Term::Var(1039),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Var(1014),
		Term::Var(1022),
		None,
	));
	b.insert(Quad(
		Term::Ground(16),
		Term::Var(1002),
		Term::Ground(24),
		None,
	));
	b.insert(Quad(
		Term::Ground(41),
		Term::Var(1004),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Var(1006),
		Term::Ground(12),
		None,
	));
	b.insert(Quad(
		Term::Ground(30),
		Term::Ground(28),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(35),
		Term::Var(1003),
		Term::Var(1040),
		None,
	));
	b.insert(Quad(
		Term::Ground(41),
		Term::Var(1027),
		Term::Var(1025),
		None,
	));
	b.insert(Quad(
		Term::Var(1015),
		Term::Ground(25),
		Term::Ground(10),
		None,
	));
	b.insert(Quad(
		Term::Ground(19),
		Term::Var(1000),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Ground(18),
		Term::Var(1016),
		Term::Ground(39),
		None,
	));
	b.insert(Quad(
		Term::Var(1031),
		Term::Var(1045),
		Term::Var(1038),
		None,
	));
	b.insert(Quad(
		Term::Ground(12),
		Term::Var(1026),
		Term::Var(1026),
		None,
	));
	b.insert(Quad(
		Term::Var(1011),
		Term::Ground(4),
		Term::Ground(41),
		None,
	));
	b.insert(Quad(
		Term::Var(1012),
		Term::Var(1034),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Ground(40),
		Term::Var(1049),
		Term::Ground(49),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Ground(17),
		Term::Ground(48),
		None,
	));
	b.insert(Quad(
		Term::Var(1028),
		Term::Ground(24),
		Term::Var(1025),
		None,
	));
	b.insert(Quad(
		Term::Ground(20),
		Term::Ground(44),
		Term::Var(1016),
		None,
	));
	b.insert(Quad(
		Term::Ground(23),
		Term::Ground(19),
		Term::Ground(24),
		None,
	));
	b.insert(Quad(
		Term::Ground(48),
		Term::Ground(12),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(36),
		Term::Ground(27),
		Term::Var(1012),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Var(1007),
		Term::Ground(38),
		None,
	));
	b.insert(Quad(
		Term::Var(1020),
		Term::Ground(8),
		Term::Var(1030),
		None,
	));
	b.insert(Quad(
		Term::Var(1030),
		Term::Var(1011),
		Term::Ground(29),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Var(1040),
		Term::Var(1041),
		None,
	));
	b.insert(Quad(
		Term::Ground(43),
		Term::Var(1012),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Var(1030),
		Term::Var(1000),
		Term::Ground(49),
		None,
	));
	b.insert(Quad(
		Term::Ground(29),
		Term::Ground(19),
		Term::Ground(17),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Var(1046),
		Term::Ground(41),
		None,
	));
	b.insert(Quad(
		Term::Var(1021),
		Term::Var(1034),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Var(1045),
		Term::Ground(33),
		Term::Ground(39),
		None,
	));
	b.insert(Quad(
		Term::Ground(43),
		Term::Var(1031),
		Term::Ground(19),
		None,
	));
	b.insert(Quad(
		Term::Ground(43),
		Term::Ground(45),
		Term::Ground(35),
		None,
	));
	b.insert(Quad(
		Term::Ground(10),
		Term::Ground(18),
		Term::Var(1028),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Var(1030),
		Term::Ground(22),
		None,
	));
	b.insert(Quad(
		Term::Var(1011),
		Term::Ground(36),
		Term::Ground(38),
		None,
	));
	b.insert(Quad(
		Term::Ground(25),
		Term::Ground(15),
		Term::Ground(47),
		None,
	));
	b.insert(Quad(
		Term::Var(1015),
		Term::Var(1029),
		Term::Var(1017),
		None,
	));
	b.insert(Quad(
		Term::Ground(31),
		Term::Ground(1),
		Term::Var(1026),
		None,
	));
	b.insert(Quad(
		Term::Var(1030),
		Term::Ground(5),
		Term::Ground(48),
		None,
	));
	b.insert(Quad(
		Term::Var(1036),
		Term::Var(1025),
		Term::Ground(17),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Var(1045),
		Term::Ground(30),
		None,
	));
	b.insert(Quad(
		Term::Var(1028),
		Term::Var(1038),
		Term::Ground(22),
		None,
	));
	b.insert(Quad(
		Term::Ground(46),
		Term::Var(1034),
		Term::Ground(24),
		None,
	));
	b.insert(Quad(
		Term::Ground(11),
		Term::Var(1031),
		Term::Ground(46),
		None,
	));
	b.insert(Quad(
		Term::Ground(26),
		Term::Var(1012),
		Term::Ground(22),
		None,
	));
	b.insert(Quad(
		Term::Ground(37),
		Term::Var(1007),
		Term::Var(1019),
		None,
	));
	b.insert(Quad(
		Term::Var(1018),
		Term::Var(1025),
		Term::Var(1041),
		None,
	));
	b.insert(Quad(
		Term::Var(1027),
		Term::Ground(47),
		Term::Var(1022),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Var(1026),
		Term::Var(1041),
		None,
	));
	b.insert(Quad(
		Term::Var(1025),
		Term::Ground(36),
		Term::Var(1040),
		None,
	));
	b.insert(Quad(
		Term::Ground(35),
		Term::Var(1003),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Ground(48),
		Term::Ground(34),
		Term::Ground(15),
		None,
	));
	b.insert(Quad(
		Term::Var(1028),
		Term::Var(1049),
		Term::Ground(49),
		None,
	));
	b.insert(Quad(
		Term::Var(1017),
		Term::Ground(30),
		Term::Ground(41),
		None,
	));
	b.insert(Quad(
		Term::Ground(16),
		Term::Var(1032),
		Term::Var(1047),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Var(1006),
		Term::Var(1027),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Var(1024),
		Term::Ground(47),
		None,
	));
	b.insert(Quad(
		Term::Ground(13),
		Term::Var(1023),
		Term::Var(1015),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Ground(35),
		Term::Var(1032),
		None,
	));
	b.insert(Quad(
		Term::Var(1015),
		Term::Var(1044),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Var(1016),
		Term::Ground(12),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Var(1032),
		Term::Var(1034),
		Term::Var(1036),
		None,
	));
	b.insert(Quad(
		Term::Var(1037),
		Term::Var(1004),
		Term::Ground(24),
		None,
	));
	b.insert(Quad(
		Term::Ground(26),
		Term::Var(1031),
		Term::Var(1022),
		None,
	));
	b.insert(Quad(
		Term::Var(1022),
		Term::Var(1032),
		Term::Var(1046),
		None,
	));
	b.insert(Quad(
		Term::Var(1025),
		Term::Var(1003),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Var(1034),
		Term::Ground(23),
		None,
	));
	b.insert(Quad(
		Term::Var(1023),
		Term::Ground(18),
		Term::Ground(22),
		None,
	));
	b.insert(Quad(
		Term::Ground(28),
		Term::Var(1020),
		Term::Ground(12),
		None,
	));
	b.insert(Quad(
		Term::Ground(34),
		Term::Ground(9),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Var(1044),
		Term::Ground(0),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Var(1044),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(17),
		Term::Ground(34),
		None,
	));
	b.insert(Quad(
		Term::Ground(27),
		Term::Ground(4),
		Term::Var(1043),
		None,
	));
	b.insert(Quad(
		Term::Ground(40),
		Term::Var(1035),
		Term::Ground(41),
		None,
	));
	b.insert(Quad(
		Term::Var(1012),
		Term::Var(1002),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Var(1029),
		Term::Ground(40),
		Term::Var(1048),
		None,
	));
	b.insert(Quad(
		Term::Var(1011),
		Term::Ground(8),
		Term::Ground(2),
		None,
	));
	test(a, b)
}
#[test]
fn iso_197() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Var(39), Term::Var(0), Term::Ground(8), None));
	a.insert(Quad(Term::Var(25), Term::Var(29), Term::Ground(17), None));
	a.insert(Quad(Term::Ground(27), Term::Ground(40), Term::Var(9), None));
	a.insert(Quad(Term::Var(14), Term::Ground(5), Term::Ground(4), None));
	a.insert(Quad(Term::Ground(2), Term::Ground(49), Term::Var(20), None));
	a.insert(Quad(Term::Ground(1), Term::Var(25), Term::Var(35), None));
	a.insert(Quad(Term::Ground(24), Term::Var(47), Term::Var(49), None));
	a.insert(Quad(
		Term::Var(41),
		Term::Ground(45),
		Term::Ground(32),
		None,
	));
	a.insert(Quad(Term::Ground(6), Term::Var(27), Term::Var(44), None));
	a.insert(Quad(Term::Var(42), Term::Ground(9), Term::Var(47), None));
	a.insert(Quad(Term::Var(18), Term::Ground(3), Term::Var(22), None));
	a.insert(Quad(
		Term::Ground(4),
		Term::Ground(3),
		Term::Ground(21),
		None,
	));
	a.insert(Quad(Term::Ground(8), Term::Ground(41), Term::Var(13), None));
	a.insert(Quad(Term::Var(1), Term::Ground(12), Term::Var(13), None));
	a.insert(Quad(
		Term::Ground(18),
		Term::Ground(29),
		Term::Var(25),
		None,
	));
	a.insert(Quad(Term::Var(38), Term::Ground(10), Term::Var(24), None));
	a.insert(Quad(Term::Var(4), Term::Ground(41), Term::Ground(6), None));
	a.insert(Quad(
		Term::Ground(47),
		Term::Ground(37),
		Term::Ground(36),
		None,
	));
	a.insert(Quad(Term::Ground(47), Term::Ground(3), Term::Var(6), None));
	a.insert(Quad(Term::Var(19), Term::Ground(37), Term::Ground(5), None));
	a.insert(Quad(
		Term::Ground(18),
		Term::Ground(23),
		Term::Var(20),
		None,
	));
	a.insert(Quad(Term::Var(38), Term::Var(49), Term::Var(19), None));
	a.insert(Quad(Term::Var(35), Term::Ground(11), Term::Var(35), None));
	a.insert(Quad(
		Term::Ground(30),
		Term::Ground(0),
		Term::Ground(16),
		None,
	));
	a.insert(Quad(Term::Var(43), Term::Var(27), Term::Var(22), None));
	a.insert(Quad(Term::Var(16), Term::Ground(33), Term::Var(44), None));
	a.insert(Quad(Term::Var(39), Term::Ground(4), Term::Var(29), None));
	a.insert(Quad(Term::Var(26), Term::Var(7), Term::Ground(31), None));
	a.insert(Quad(
		Term::Ground(18),
		Term::Ground(39),
		Term::Ground(4),
		None,
	));
	a.insert(Quad(Term::Var(29), Term::Var(5), Term::Var(42), None));
	a.insert(Quad(Term::Var(38), Term::Ground(5), Term::Ground(23), None));
	a.insert(Quad(Term::Var(15), Term::Ground(3), Term::Ground(7), None));
	a.insert(Quad(Term::Ground(46), Term::Var(16), Term::Var(47), None));
	a.insert(Quad(Term::Ground(34), Term::Ground(6), Term::Var(37), None));
	a.insert(Quad(Term::Var(19), Term::Ground(44), Term::Var(13), None));
	a.insert(Quad(
		Term::Ground(17),
		Term::Var(14),
		Term::Ground(27),
		None,
	));
	a.insert(Quad(Term::Var(48), Term::Var(43), Term::Var(10), None));
	a.insert(Quad(Term::Var(7), Term::Var(24), Term::Ground(49), None));
	a.insert(Quad(Term::Ground(45), Term::Ground(40), Term::Var(7), None));
	a.insert(Quad(Term::Ground(34), Term::Ground(2), Term::Var(34), None));
	a.insert(Quad(
		Term::Ground(23),
		Term::Ground(4),
		Term::Ground(21),
		None,
	));
	a.insert(Quad(Term::Ground(45), Term::Var(6), Term::Ground(43), None));
	a.insert(Quad(Term::Ground(30), Term::Ground(5), Term::Var(16), None));
	a.insert(Quad(Term::Ground(28), Term::Var(2), Term::Ground(21), None));
	a.insert(Quad(Term::Ground(11), Term::Var(0), Term::Var(17), None));
	a.insert(Quad(
		Term::Ground(10),
		Term::Ground(49),
		Term::Ground(8),
		None,
	));
	a.insert(Quad(
		Term::Ground(29),
		Term::Var(43),
		Term::Ground(11),
		None,
	));
	a.insert(Quad(Term::Var(4), Term::Var(42), Term::Ground(42), None));
	a.insert(Quad(Term::Ground(49), Term::Var(26), Term::Var(0), None));
	a.insert(Quad(
		Term::Ground(10),
		Term::Ground(40),
		Term::Ground(27),
		None,
	));
	a.insert(Quad(Term::Var(17), Term::Ground(11), Term::Var(6), None));
	a.insert(Quad(
		Term::Ground(26),
		Term::Ground(24),
		Term::Ground(35),
		None,
	));
	a.insert(Quad(Term::Var(26), Term::Var(46), Term::Ground(45), None));
	a.insert(Quad(Term::Var(24), Term::Var(27), Term::Ground(42), None));
	a.insert(Quad(
		Term::Ground(37),
		Term::Ground(48),
		Term::Ground(36),
		None,
	));
	a.insert(Quad(Term::Var(41), Term::Var(5), Term::Var(35), None));
	a.insert(Quad(Term::Ground(32), Term::Var(10), Term::Ground(5), None));
	a.insert(Quad(Term::Var(19), Term::Ground(0), Term::Var(8), None));
	a.insert(Quad(Term::Ground(1), Term::Ground(11), Term::Var(0), None));
	a.insert(Quad(Term::Ground(45), Term::Ground(24), Term::Var(1), None));
	a.insert(Quad(Term::Var(39), Term::Var(35), Term::Ground(32), None));
	a.insert(Quad(Term::Ground(43), Term::Ground(13), Term::Var(8), None));
	a.insert(Quad(
		Term::Ground(29),
		Term::Var(26),
		Term::Ground(12),
		None,
	));
	a.insert(Quad(
		Term::Ground(47),
		Term::Ground(21),
		Term::Ground(0),
		None,
	));
	a.insert(Quad(
		Term::Ground(16),
		Term::Var(47),
		Term::Ground(37),
		None,
	));
	a.insert(Quad(
		Term::Var(31),
		Term::Ground(41),
		Term::Ground(49),
		None,
	));
	a.insert(Quad(
		Term::Ground(1),
		Term::Ground(46),
		Term::Ground(25),
		None,
	));
	a.insert(Quad(Term::Ground(9), Term::Var(25), Term::Ground(44), None));
	a.insert(Quad(Term::Var(24), Term::Ground(42), Term::Ground(5), None));
	a.insert(Quad(Term::Var(32), Term::Var(37), Term::Var(38), None));
	a.insert(Quad(Term::Var(41), Term::Ground(47), Term::Var(7), None));
	a.insert(Quad(Term::Ground(36), Term::Var(21), Term::Var(44), None));
	a.insert(Quad(Term::Ground(18), Term::Ground(31), Term::Var(8), None));
	a.insert(Quad(Term::Var(47), Term::Ground(17), Term::Var(26), None));
	a.insert(Quad(Term::Ground(33), Term::Var(30), Term::Var(18), None));
	a.insert(Quad(Term::Ground(42), Term::Var(0), Term::Var(27), None));
	a.insert(Quad(
		Term::Ground(8),
		Term::Ground(17),
		Term::Ground(9),
		None,
	));
	a.insert(Quad(
		Term::Ground(36),
		Term::Ground(6),
		Term::Ground(22),
		None,
	));
	a.insert(Quad(
		Term::Ground(22),
		Term::Ground(45),
		Term::Ground(34),
		None,
	));
	a.insert(Quad(Term::Ground(10), Term::Var(22), Term::Var(41), None));
	a.insert(Quad(Term::Var(40), Term::Var(30), Term::Var(10), None));
	a.insert(Quad(
		Term::Ground(19),
		Term::Ground(24),
		Term::Var(34),
		None,
	));
	a.insert(Quad(Term::Var(28), Term::Var(30), Term::Ground(12), None));
	a.insert(Quad(Term::Var(32), Term::Var(39), Term::Ground(44), None));
	a.insert(Quad(
		Term::Ground(21),
		Term::Ground(39),
		Term::Ground(23),
		None,
	));
	a.insert(Quad(
		Term::Ground(38),
		Term::Var(40),
		Term::Ground(45),
		None,
	));
	a.insert(Quad(Term::Var(35), Term::Var(26), Term::Ground(23), None));
	a.insert(Quad(Term::Var(5), Term::Ground(34), Term::Ground(11), None));
	a.insert(Quad(Term::Var(44), Term::Var(15), Term::Ground(15), None));
	a.insert(Quad(
		Term::Ground(0),
		Term::Ground(8),
		Term::Ground(42),
		None,
	));
	a.insert(Quad(Term::Ground(41), Term::Var(28), Term::Ground(9), None));
	a.insert(Quad(
		Term::Ground(37),
		Term::Ground(25),
		Term::Var(44),
		None,
	));
	a.insert(Quad(Term::Var(19), Term::Var(33), Term::Var(12), None));
	a.insert(Quad(
		Term::Var(23),
		Term::Ground(49),
		Term::Ground(46),
		None,
	));
	a.insert(Quad(
		Term::Ground(16),
		Term::Ground(39),
		Term::Ground(26),
		None,
	));
	a.insert(Quad(
		Term::Ground(26),
		Term::Ground(31),
		Term::Ground(18),
		None,
	));
	a.insert(Quad(Term::Var(20), Term::Var(6), Term::Ground(32), None));
	a.insert(Quad(Term::Var(6), Term::Ground(26), Term::Ground(4), None));
	a.insert(Quad(
		Term::Ground(33),
		Term::Ground(13),
		Term::Ground(5),
		None,
	));
	a.insert(Quad(Term::Var(10), Term::Var(1), Term::Var(29), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Var(1039),
		Term::Var(1000),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Var(1025),
		Term::Var(1029),
		Term::Ground(17),
		None,
	));
	b.insert(Quad(
		Term::Ground(27),
		Term::Ground(40),
		Term::Var(1009),
		None,
	));
	b.insert(Quad(
		Term::Var(1014),
		Term::Ground(5),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(49),
		Term::Var(1020),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Var(1025),
		Term::Var(1035),
		None,
	));
	b.insert(Quad(
		Term::Ground(24),
		Term::Var(1047),
		Term::Var(1049),
		None,
	));
	b.insert(Quad(
		Term::Var(1041),
		Term::Ground(45),
		Term::Ground(32),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Var(1027),
		Term::Var(1044),
		None,
	));
	b.insert(Quad(
		Term::Var(1042),
		Term::Ground(9),
		Term::Var(1047),
		None,
	));
	b.insert(Quad(
		Term::Var(1018),
		Term::Ground(3),
		Term::Var(1022),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Ground(3),
		Term::Ground(21),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Ground(41),
		Term::Var(1013),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(12),
		Term::Var(1013),
		None,
	));
	b.insert(Quad(
		Term::Ground(18),
		Term::Ground(29),
		Term::Var(1025),
		None,
	));
	b.insert(Quad(
		Term::Var(1038),
		Term::Ground(10),
		Term::Var(1024),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Ground(41),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Ground(47),
		Term::Ground(37),
		Term::Ground(36),
		None,
	));
	b.insert(Quad(
		Term::Ground(47),
		Term::Ground(3),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Var(1019),
		Term::Ground(37),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Ground(18),
		Term::Ground(23),
		Term::Var(1020),
		None,
	));
	b.insert(Quad(
		Term::Var(1038),
		Term::Var(1049),
		Term::Var(1019),
		None,
	));
	b.insert(Quad(
		Term::Var(1035),
		Term::Ground(11),
		Term::Var(1035),
		None,
	));
	b.insert(Quad(
		Term::Ground(30),
		Term::Ground(0),
		Term::Ground(16),
		None,
	));
	b.insert(Quad(
		Term::Var(1043),
		Term::Var(1027),
		Term::Var(1022),
		None,
	));
	b.insert(Quad(
		Term::Var(1016),
		Term::Ground(33),
		Term::Var(1044),
		None,
	));
	b.insert(Quad(
		Term::Var(1039),
		Term::Ground(4),
		Term::Var(1029),
		None,
	));
	b.insert(Quad(
		Term::Var(1026),
		Term::Var(1007),
		Term::Ground(31),
		None,
	));
	b.insert(Quad(
		Term::Ground(18),
		Term::Ground(39),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Var(1029),
		Term::Var(1005),
		Term::Var(1042),
		None,
	));
	b.insert(Quad(
		Term::Var(1038),
		Term::Ground(5),
		Term::Ground(23),
		None,
	));
	b.insert(Quad(
		Term::Var(1015),
		Term::Ground(3),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Ground(46),
		Term::Var(1016),
		Term::Var(1047),
		None,
	));
	b.insert(Quad(
		Term::Ground(34),
		Term::Ground(6),
		Term::Var(1037),
		None,
	));
	b.insert(Quad(
		Term::Var(1019),
		Term::Ground(44),
		Term::Var(1013),
		None,
	));
	b.insert(Quad(
		Term::Ground(17),
		Term::Var(1014),
		Term::Ground(27),
		None,
	));
	b.insert(Quad(
		Term::Var(1048),
		Term::Var(1043),
		Term::Var(1010),
		None,
	));
	b.insert(Quad(
		Term::Var(1007),
		Term::Var(1024),
		Term::Ground(49),
		None,
	));
	b.insert(Quad(
		Term::Ground(45),
		Term::Ground(40),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Ground(34),
		Term::Ground(2),
		Term::Var(1034),
		None,
	));
	b.insert(Quad(
		Term::Ground(23),
		Term::Ground(4),
		Term::Ground(21),
		None,
	));
	b.insert(Quad(
		Term::Ground(45),
		Term::Var(1006),
		Term::Ground(43),
		None,
	));
	b.insert(Quad(
		Term::Ground(30),
		Term::Ground(5),
		Term::Var(1016),
		None,
	));
	b.insert(Quad(
		Term::Ground(28),
		Term::Var(1002),
		Term::Ground(21),
		None,
	));
	b.insert(Quad(
		Term::Ground(11),
		Term::Var(1000),
		Term::Var(1017),
		None,
	));
	b.insert(Quad(
		Term::Ground(10),
		Term::Ground(49),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Ground(29),
		Term::Var(1043),
		Term::Ground(11),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Var(1042),
		Term::Ground(42),
		None,
	));
	b.insert(Quad(
		Term::Ground(49),
		Term::Var(1026),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Ground(10),
		Term::Ground(40),
		Term::Ground(27),
		None,
	));
	b.insert(Quad(
		Term::Var(1017),
		Term::Ground(11),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Ground(26),
		Term::Ground(24),
		Term::Ground(35),
		None,
	));
	b.insert(Quad(
		Term::Var(1026),
		Term::Var(1046),
		Term::Ground(45),
		None,
	));
	b.insert(Quad(
		Term::Var(1024),
		Term::Var(1027),
		Term::Ground(42),
		None,
	));
	b.insert(Quad(
		Term::Ground(37),
		Term::Ground(48),
		Term::Ground(36),
		None,
	));
	b.insert(Quad(
		Term::Var(1041),
		Term::Var(1005),
		Term::Var(1035),
		None,
	));
	b.insert(Quad(
		Term::Ground(32),
		Term::Var(1010),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Var(1019),
		Term::Ground(0),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(11),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Ground(45),
		Term::Ground(24),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Var(1039),
		Term::Var(1035),
		Term::Ground(32),
		None,
	));
	b.insert(Quad(
		Term::Ground(43),
		Term::Ground(13),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Ground(29),
		Term::Var(1026),
		Term::Ground(12),
		None,
	));
	b.insert(Quad(
		Term::Ground(47),
		Term::Ground(21),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Ground(16),
		Term::Var(1047),
		Term::Ground(37),
		None,
	));
	b.insert(Quad(
		Term::Var(1031),
		Term::Ground(41),
		Term::Ground(49),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(46),
		Term::Ground(25),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Var(1025),
		Term::Ground(44),
		None,
	));
	b.insert(Quad(
		Term::Var(1024),
		Term::Ground(42),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Var(1032),
		Term::Var(1037),
		Term::Var(1038),
		None,
	));
	b.insert(Quad(
		Term::Var(1041),
		Term::Ground(47),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Ground(36),
		Term::Var(1021),
		Term::Var(1044),
		None,
	));
	b.insert(Quad(
		Term::Ground(18),
		Term::Ground(31),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Var(1047),
		Term::Ground(17),
		Term::Var(1026),
		None,
	));
	b.insert(Quad(
		Term::Ground(33),
		Term::Var(1030),
		Term::Var(1018),
		None,
	));
	b.insert(Quad(
		Term::Ground(42),
		Term::Var(1000),
		Term::Var(1027),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Ground(17),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Ground(36),
		Term::Ground(6),
		Term::Ground(22),
		None,
	));
	b.insert(Quad(
		Term::Ground(22),
		Term::Ground(45),
		Term::Ground(34),
		None,
	));
	b.insert(Quad(
		Term::Ground(10),
		Term::Var(1022),
		Term::Var(1041),
		None,
	));
	b.insert(Quad(
		Term::Var(1040),
		Term::Var(1030),
		Term::Var(1010),
		None,
	));
	b.insert(Quad(
		Term::Ground(19),
		Term::Ground(24),
		Term::Var(1034),
		None,
	));
	b.insert(Quad(
		Term::Var(1028),
		Term::Var(1030),
		Term::Ground(12),
		None,
	));
	b.insert(Quad(
		Term::Var(1032),
		Term::Var(1039),
		Term::Ground(44),
		None,
	));
	b.insert(Quad(
		Term::Ground(21),
		Term::Ground(39),
		Term::Ground(23),
		None,
	));
	b.insert(Quad(
		Term::Ground(38),
		Term::Var(1040),
		Term::Ground(45),
		None,
	));
	b.insert(Quad(
		Term::Var(1035),
		Term::Var(1026),
		Term::Ground(23),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Ground(34),
		Term::Ground(11),
		None,
	));
	b.insert(Quad(
		Term::Var(1044),
		Term::Var(1015),
		Term::Ground(15),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Ground(8),
		Term::Ground(42),
		None,
	));
	b.insert(Quad(
		Term::Ground(41),
		Term::Var(1028),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Ground(37),
		Term::Ground(25),
		Term::Var(1044),
		None,
	));
	b.insert(Quad(
		Term::Var(1019),
		Term::Var(1033),
		Term::Var(1012),
		None,
	));
	b.insert(Quad(
		Term::Var(1023),
		Term::Ground(49),
		Term::Ground(46),
		None,
	));
	b.insert(Quad(
		Term::Ground(16),
		Term::Ground(39),
		Term::Ground(26),
		None,
	));
	b.insert(Quad(
		Term::Ground(26),
		Term::Ground(31),
		Term::Ground(18),
		None,
	));
	b.insert(Quad(
		Term::Var(1020),
		Term::Var(1006),
		Term::Ground(32),
		None,
	));
	b.insert(Quad(
		Term::Var(1006),
		Term::Ground(26),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Ground(33),
		Term::Ground(13),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Var(1010),
		Term::Var(1001),
		Term::Var(1029),
		None,
	));
	test(a, b)
}
#[test]
fn iso_198() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Var(31), Term::Var(22), Term::Ground(40), None));
	a.insert(Quad(
		Term::Var(37),
		Term::Ground(42),
		Term::Ground(37),
		None,
	));
	a.insert(Quad(Term::Var(45), Term::Var(25), Term::Ground(38), None));
	a.insert(Quad(Term::Ground(3), Term::Var(34), Term::Var(14), None));
	a.insert(Quad(
		Term::Var(31),
		Term::Ground(26),
		Term::Ground(25),
		None,
	));
	a.insert(Quad(Term::Ground(10), Term::Var(1), Term::Ground(44), None));
	a.insert(Quad(Term::Var(4), Term::Ground(42), Term::Var(24), None));
	a.insert(Quad(Term::Ground(41), Term::Var(49), Term::Var(19), None));
	a.insert(Quad(Term::Var(41), Term::Var(45), Term::Var(17), None));
	a.insert(Quad(Term::Var(28), Term::Var(48), Term::Var(47), None));
	a.insert(Quad(Term::Var(33), Term::Var(4), Term::Ground(13), None));
	a.insert(Quad(Term::Var(45), Term::Ground(46), Term::Var(39), None));
	a.insert(Quad(Term::Var(13), Term::Ground(5), Term::Ground(20), None));
	a.insert(Quad(
		Term::Ground(18),
		Term::Ground(25),
		Term::Ground(38),
		None,
	));
	a.insert(Quad(
		Term::Var(42),
		Term::Ground(35),
		Term::Ground(26),
		None,
	));
	a.insert(Quad(Term::Var(21), Term::Ground(29), Term::Var(15), None));
	a.insert(Quad(Term::Var(46), Term::Ground(17), Term::Ground(6), None));
	a.insert(Quad(Term::Var(31), Term::Ground(24), Term::Var(45), None));
	a.insert(Quad(Term::Var(9), Term::Var(48), Term::Ground(28), None));
	a.insert(Quad(Term::Var(30), Term::Var(43), Term::Var(6), None));
	a.insert(Quad(Term::Ground(9), Term::Var(27), Term::Ground(43), None));
	a.insert(Quad(Term::Ground(7), Term::Ground(22), Term::Var(31), None));
	a.insert(Quad(
		Term::Ground(17),
		Term::Ground(14),
		Term::Ground(9),
		None,
	));
	a.insert(Quad(
		Term::Ground(20),
		Term::Ground(41),
		Term::Ground(22),
		None,
	));
	a.insert(Quad(Term::Ground(42), Term::Ground(2), Term::Var(17), None));
	a.insert(Quad(Term::Var(11), Term::Var(27), Term::Ground(23), None));
	a.insert(Quad(Term::Ground(32), Term::Ground(36), Term::Var(1), None));
	a.insert(Quad(Term::Ground(13), Term::Var(10), Term::Var(14), None));
	a.insert(Quad(Term::Var(32), Term::Var(44), Term::Ground(29), None));
	a.insert(Quad(
		Term::Ground(22),
		Term::Ground(49),
		Term::Ground(12),
		None,
	));
	a.insert(Quad(Term::Ground(4), Term::Var(26), Term::Var(22), None));
	a.insert(Quad(Term::Var(31), Term::Ground(16), Term::Ground(5), None));
	a.insert(Quad(Term::Var(21), Term::Var(38), Term::Var(27), None));
	a.insert(Quad(Term::Var(22), Term::Var(10), Term::Ground(14), None));
	a.insert(Quad(
		Term::Ground(31),
		Term::Ground(23),
		Term::Var(49),
		None,
	));
	a.insert(Quad(Term::Ground(1), Term::Var(24), Term::Ground(8), None));
	a.insert(Quad(
		Term::Ground(4),
		Term::Ground(36),
		Term::Ground(28),
		None,
	));
	a.insert(Quad(Term::Ground(38), Term::Var(3), Term::Ground(6), None));
	a.insert(Quad(Term::Ground(21), Term::Var(38), Term::Var(39), None));
	a.insert(Quad(Term::Ground(6), Term::Var(30), Term::Ground(5), None));
	a.insert(Quad(Term::Ground(27), Term::Var(1), Term::Var(34), None));
	a.insert(Quad(Term::Var(15), Term::Ground(0), Term::Var(13), None));
	a.insert(Quad(
		Term::Ground(8),
		Term::Ground(1),
		Term::Ground(40),
		None,
	));
	a.insert(Quad(
		Term::Ground(43),
		Term::Ground(3),
		Term::Ground(32),
		None,
	));
	a.insert(Quad(Term::Ground(46), Term::Var(26), Term::Var(45), None));
	a.insert(Quad(Term::Var(6), Term::Var(34), Term::Var(8), None));
	a.insert(Quad(Term::Var(6), Term::Var(39), Term::Ground(26), None));
	a.insert(Quad(Term::Ground(4), Term::Var(32), Term::Ground(23), None));
	a.insert(Quad(
		Term::Ground(12),
		Term::Ground(19),
		Term::Ground(47),
		None,
	));
	a.insert(Quad(Term::Var(8), Term::Ground(47), Term::Ground(40), None));
	a.insert(Quad(Term::Var(42), Term::Var(13), Term::Ground(10), None));
	a.insert(Quad(Term::Ground(28), Term::Var(33), Term::Var(19), None));
	a.insert(Quad(
		Term::Ground(20),
		Term::Ground(48),
		Term::Ground(25),
		None,
	));
	a.insert(Quad(Term::Ground(26), Term::Var(43), Term::Var(22), None));
	a.insert(Quad(
		Term::Ground(2),
		Term::Ground(17),
		Term::Ground(37),
		None,
	));
	a.insert(Quad(Term::Var(28), Term::Ground(34), Term::Var(10), None));
	a.insert(Quad(
		Term::Ground(28),
		Term::Ground(1),
		Term::Ground(14),
		None,
	));
	a.insert(Quad(
		Term::Var(29),
		Term::Ground(25),
		Term::Ground(40),
		None,
	));
	a.insert(Quad(
		Term::Ground(19),
		Term::Ground(16),
		Term::Ground(25),
		None,
	));
	a.insert(Quad(Term::Var(18), Term::Var(16), Term::Ground(23), None));
	a.insert(Quad(
		Term::Ground(1),
		Term::Ground(46),
		Term::Ground(0),
		None,
	));
	a.insert(Quad(Term::Var(2), Term::Var(25), Term::Var(27), None));
	a.insert(Quad(Term::Ground(25), Term::Var(6), Term::Var(17), None));
	a.insert(Quad(
		Term::Ground(10),
		Term::Ground(41),
		Term::Var(40),
		None,
	));
	a.insert(Quad(Term::Var(5), Term::Var(8), Term::Var(33), None));
	a.insert(Quad(
		Term::Var(15),
		Term::Ground(32),
		Term::Ground(46),
		None,
	));
	a.insert(Quad(Term::Ground(15), Term::Var(30), Term::Var(24), None));
	a.insert(Quad(Term::Var(37), Term::Var(47), Term::Var(7), None));
	a.insert(Quad(Term::Var(13), Term::Ground(38), Term::Ground(7), None));
	a.insert(Quad(
		Term::Ground(21),
		Term::Var(38),
		Term::Ground(27),
		None,
	));
	a.insert(Quad(Term::Var(0), Term::Ground(45), Term::Ground(5), None));
	a.insert(Quad(Term::Var(5), Term::Var(17), Term::Ground(8), None));
	a.insert(Quad(Term::Var(41), Term::Var(31), Term::Ground(31), None));
	a.insert(Quad(Term::Var(42), Term::Ground(8), Term::Var(27), None));
	a.insert(Quad(Term::Var(33), Term::Ground(24), Term::Var(17), None));
	a.insert(Quad(Term::Var(36), Term::Ground(13), Term::Var(38), None));
	a.insert(Quad(
		Term::Ground(46),
		Term::Ground(13),
		Term::Var(29),
		None,
	));
	a.insert(Quad(
		Term::Ground(11),
		Term::Ground(32),
		Term::Var(45),
		None,
	));
	a.insert(Quad(Term::Ground(36), Term::Var(2), Term::Ground(25), None));
	a.insert(Quad(Term::Var(23), Term::Var(47), Term::Var(20), None));
	a.insert(Quad(
		Term::Ground(14),
		Term::Ground(45),
		Term::Ground(38),
		None,
	));
	a.insert(Quad(Term::Ground(10), Term::Ground(8), Term::Var(27), None));
	a.insert(Quad(Term::Ground(26), Term::Var(12), Term::Ground(4), None));
	a.insert(Quad(Term::Var(13), Term::Ground(10), Term::Var(16), None));
	a.insert(Quad(Term::Ground(10), Term::Var(38), Term::Var(31), None));
	a.insert(Quad(Term::Var(19), Term::Var(36), Term::Var(47), None));
	a.insert(Quad(Term::Var(11), Term::Ground(28), Term::Var(10), None));
	a.insert(Quad(Term::Var(1), Term::Var(20), Term::Ground(23), None));
	a.insert(Quad(
		Term::Ground(37),
		Term::Ground(30),
		Term::Var(27),
		None,
	));
	a.insert(Quad(Term::Var(40), Term::Var(49), Term::Var(22), None));
	a.insert(Quad(Term::Var(42), Term::Var(27), Term::Var(32), None));
	a.insert(Quad(Term::Var(21), Term::Var(13), Term::Ground(37), None));
	a.insert(Quad(Term::Ground(43), Term::Var(18), Term::Var(47), None));
	a.insert(Quad(Term::Ground(20), Term::Var(6), Term::Var(20), None));
	a.insert(Quad(Term::Var(46), Term::Ground(13), Term::Ground(9), None));
	a.insert(Quad(
		Term::Ground(41),
		Term::Ground(46),
		Term::Ground(2),
		None,
	));
	a.insert(Quad(Term::Var(17), Term::Var(35), Term::Var(8), None));
	a.insert(Quad(Term::Ground(49), Term::Var(17), Term::Var(26), None));
	a.insert(Quad(
		Term::Ground(30),
		Term::Ground(37),
		Term::Var(13),
		None,
	));
	a.insert(Quad(
		Term::Ground(48),
		Term::Ground(10),
		Term::Ground(27),
		None,
	));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Var(1031),
		Term::Var(1022),
		Term::Ground(40),
		None,
	));
	b.insert(Quad(
		Term::Var(1037),
		Term::Ground(42),
		Term::Ground(37),
		None,
	));
	b.insert(Quad(
		Term::Var(1045),
		Term::Var(1025),
		Term::Ground(38),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Var(1034),
		Term::Var(1014),
		None,
	));
	b.insert(Quad(
		Term::Var(1031),
		Term::Ground(26),
		Term::Ground(25),
		None,
	));
	b.insert(Quad(
		Term::Ground(10),
		Term::Var(1001),
		Term::Ground(44),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Ground(42),
		Term::Var(1024),
		None,
	));
	b.insert(Quad(
		Term::Ground(41),
		Term::Var(1049),
		Term::Var(1019),
		None,
	));
	b.insert(Quad(
		Term::Var(1041),
		Term::Var(1045),
		Term::Var(1017),
		None,
	));
	b.insert(Quad(
		Term::Var(1028),
		Term::Var(1048),
		Term::Var(1047),
		None,
	));
	b.insert(Quad(
		Term::Var(1033),
		Term::Var(1004),
		Term::Ground(13),
		None,
	));
	b.insert(Quad(
		Term::Var(1045),
		Term::Ground(46),
		Term::Var(1039),
		None,
	));
	b.insert(Quad(
		Term::Var(1013),
		Term::Ground(5),
		Term::Ground(20),
		None,
	));
	b.insert(Quad(
		Term::Ground(18),
		Term::Ground(25),
		Term::Ground(38),
		None,
	));
	b.insert(Quad(
		Term::Var(1042),
		Term::Ground(35),
		Term::Ground(26),
		None,
	));
	b.insert(Quad(
		Term::Var(1021),
		Term::Ground(29),
		Term::Var(1015),
		None,
	));
	b.insert(Quad(
		Term::Var(1046),
		Term::Ground(17),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Var(1031),
		Term::Ground(24),
		Term::Var(1045),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Var(1048),
		Term::Ground(28),
		None,
	));
	b.insert(Quad(
		Term::Var(1030),
		Term::Var(1043),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Var(1027),
		Term::Ground(43),
		None,
	));
	b.insert(Quad(
		Term::Ground(7),
		Term::Ground(22),
		Term::Var(1031),
		None,
	));
	b.insert(Quad(
		Term::Ground(17),
		Term::Ground(14),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Ground(20),
		Term::Ground(41),
		Term::Ground(22),
		None,
	));
	b.insert(Quad(
		Term::Ground(42),
		Term::Ground(2),
		Term::Var(1017),
		None,
	));
	b.insert(Quad(
		Term::Var(1011),
		Term::Var(1027),
		Term::Ground(23),
		None,
	));
	b.insert(Quad(
		Term::Ground(32),
		Term::Ground(36),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(13),
		Term::Var(1010),
		Term::Var(1014),
		None,
	));
	b.insert(Quad(
		Term::Var(1032),
		Term::Var(1044),
		Term::Ground(29),
		None,
	));
	b.insert(Quad(
		Term::Ground(22),
		Term::Ground(49),
		Term::Ground(12),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Var(1026),
		Term::Var(1022),
		None,
	));
	b.insert(Quad(
		Term::Var(1031),
		Term::Ground(16),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Var(1021),
		Term::Var(1038),
		Term::Var(1027),
		None,
	));
	b.insert(Quad(
		Term::Var(1022),
		Term::Var(1010),
		Term::Ground(14),
		None,
	));
	b.insert(Quad(
		Term::Ground(31),
		Term::Ground(23),
		Term::Var(1049),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Var(1024),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Ground(36),
		Term::Ground(28),
		None,
	));
	b.insert(Quad(
		Term::Ground(38),
		Term::Var(1003),
		Term::Ground(6),
		None,
	));
	b.insert(Quad(
		Term::Ground(21),
		Term::Var(1038),
		Term::Var(1039),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Var(1030),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Ground(27),
		Term::Var(1001),
		Term::Var(1034),
		None,
	));
	b.insert(Quad(
		Term::Var(1015),
		Term::Ground(0),
		Term::Var(1013),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Ground(1),
		Term::Ground(40),
		None,
	));
	b.insert(Quad(
		Term::Ground(43),
		Term::Ground(3),
		Term::Ground(32),
		None,
	));
	b.insert(Quad(
		Term::Ground(46),
		Term::Var(1026),
		Term::Var(1045),
		None,
	));
	b.insert(Quad(
		Term::Var(1006),
		Term::Var(1034),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Var(1006),
		Term::Var(1039),
		Term::Ground(26),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Var(1032),
		Term::Ground(23),
		None,
	));
	b.insert(Quad(
		Term::Ground(12),
		Term::Ground(19),
		Term::Ground(47),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Ground(47),
		Term::Ground(40),
		None,
	));
	b.insert(Quad(
		Term::Var(1042),
		Term::Var(1013),
		Term::Ground(10),
		None,
	));
	b.insert(Quad(
		Term::Ground(28),
		Term::Var(1033),
		Term::Var(1019),
		None,
	));
	b.insert(Quad(
		Term::Ground(20),
		Term::Ground(48),
		Term::Ground(25),
		None,
	));
	b.insert(Quad(
		Term::Ground(26),
		Term::Var(1043),
		Term::Var(1022),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(17),
		Term::Ground(37),
		None,
	));
	b.insert(Quad(
		Term::Var(1028),
		Term::Ground(34),
		Term::Var(1010),
		None,
	));
	b.insert(Quad(
		Term::Ground(28),
		Term::Ground(1),
		Term::Ground(14),
		None,
	));
	b.insert(Quad(
		Term::Var(1029),
		Term::Ground(25),
		Term::Ground(40),
		None,
	));
	b.insert(Quad(
		Term::Ground(19),
		Term::Ground(16),
		Term::Ground(25),
		None,
	));
	b.insert(Quad(
		Term::Var(1018),
		Term::Var(1016),
		Term::Ground(23),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(46),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Var(1002),
		Term::Var(1025),
		Term::Var(1027),
		None,
	));
	b.insert(Quad(
		Term::Ground(25),
		Term::Var(1006),
		Term::Var(1017),
		None,
	));
	b.insert(Quad(
		Term::Ground(10),
		Term::Ground(41),
		Term::Var(1040),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Var(1008),
		Term::Var(1033),
		None,
	));
	b.insert(Quad(
		Term::Var(1015),
		Term::Ground(32),
		Term::Ground(46),
		None,
	));
	b.insert(Quad(
		Term::Ground(15),
		Term::Var(1030),
		Term::Var(1024),
		None,
	));
	b.insert(Quad(
		Term::Var(1037),
		Term::Var(1047),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Var(1013),
		Term::Ground(38),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Ground(21),
		Term::Var(1038),
		Term::Ground(27),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Ground(45),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Var(1017),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Var(1041),
		Term::Var(1031),
		Term::Ground(31),
		None,
	));
	b.insert(Quad(
		Term::Var(1042),
		Term::Ground(8),
		Term::Var(1027),
		None,
	));
	b.insert(Quad(
		Term::Var(1033),
		Term::Ground(24),
		Term::Var(1017),
		None,
	));
	b.insert(Quad(
		Term::Var(1036),
		Term::Ground(13),
		Term::Var(1038),
		None,
	));
	b.insert(Quad(
		Term::Ground(46),
		Term::Ground(13),
		Term::Var(1029),
		None,
	));
	b.insert(Quad(
		Term::Ground(11),
		Term::Ground(32),
		Term::Var(1045),
		None,
	));
	b.insert(Quad(
		Term::Ground(36),
		Term::Var(1002),
		Term::Ground(25),
		None,
	));
	b.insert(Quad(
		Term::Var(1023),
		Term::Var(1047),
		Term::Var(1020),
		None,
	));
	b.insert(Quad(
		Term::Ground(14),
		Term::Ground(45),
		Term::Ground(38),
		None,
	));
	b.insert(Quad(
		Term::Ground(10),
		Term::Ground(8),
		Term::Var(1027),
		None,
	));
	b.insert(Quad(
		Term::Ground(26),
		Term::Var(1012),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Var(1013),
		Term::Ground(10),
		Term::Var(1016),
		None,
	));
	b.insert(Quad(
		Term::Ground(10),
		Term::Var(1038),
		Term::Var(1031),
		None,
	));
	b.insert(Quad(
		Term::Var(1019),
		Term::Var(1036),
		Term::Var(1047),
		None,
	));
	b.insert(Quad(
		Term::Var(1011),
		Term::Ground(28),
		Term::Var(1010),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Var(1020),
		Term::Ground(23),
		None,
	));
	b.insert(Quad(
		Term::Ground(37),
		Term::Ground(30),
		Term::Var(1027),
		None,
	));
	b.insert(Quad(
		Term::Var(1040),
		Term::Var(1049),
		Term::Var(1022),
		None,
	));
	b.insert(Quad(
		Term::Var(1042),
		Term::Var(1027),
		Term::Var(1032),
		None,
	));
	b.insert(Quad(
		Term::Var(1021),
		Term::Var(1013),
		Term::Ground(37),
		None,
	));
	b.insert(Quad(
		Term::Ground(43),
		Term::Var(1018),
		Term::Var(1047),
		None,
	));
	b.insert(Quad(
		Term::Ground(20),
		Term::Var(1006),
		Term::Var(1020),
		None,
	));
	b.insert(Quad(
		Term::Var(1046),
		Term::Ground(13),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Ground(41),
		Term::Ground(46),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Var(1017),
		Term::Var(1035),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Ground(49),
		Term::Var(1017),
		Term::Var(1026),
		None,
	));
	b.insert(Quad(
		Term::Ground(30),
		Term::Ground(37),
		Term::Var(1013),
		None,
	));
	b.insert(Quad(
		Term::Ground(48),
		Term::Ground(10),
		Term::Ground(27),
		None,
	));
	test(a, b)
}
#[test]
fn iso_199() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(
		Term::Ground(23),
		Term::Var(43),
		Term::Ground(39),
		None,
	));
	a.insert(Quad(Term::Var(25), Term::Var(6), Term::Ground(28), None));
	a.insert(Quad(Term::Var(28), Term::Ground(41), Term::Ground(4), None));
	a.insert(Quad(Term::Var(48), Term::Var(21), Term::Ground(8), None));
	a.insert(Quad(Term::Var(6), Term::Var(7), Term::Ground(5), None));
	a.insert(Quad(Term::Var(45), Term::Var(17), Term::Var(19), None));
	a.insert(Quad(
		Term::Ground(16),
		Term::Var(20),
		Term::Ground(31),
		None,
	));
	a.insert(Quad(
		Term::Ground(13),
		Term::Ground(7),
		Term::Ground(14),
		None,
	));
	a.insert(Quad(Term::Var(4), Term::Var(30), Term::Ground(9), None));
	a.insert(Quad(
		Term::Var(38),
		Term::Ground(11),
		Term::Ground(41),
		None,
	));
	a.insert(Quad(Term::Var(6), Term::Ground(31), Term::Ground(2), None));
	a.insert(Quad(
		Term::Ground(23),
		Term::Var(31),
		Term::Ground(43),
		None,
	));
	a.insert(Quad(
		Term::Ground(18),
		Term::Ground(26),
		Term::Ground(2),
		None,
	));
	a.insert(Quad(Term::Ground(22), Term::Var(26), Term::Var(10), None));
	a.insert(Quad(Term::Var(8), Term::Var(10), Term::Var(6), None));
	a.insert(Quad(
		Term::Ground(15),
		Term::Ground(24),
		Term::Var(26),
		None,
	));
	a.insert(Quad(Term::Var(44), Term::Var(33), Term::Ground(25), None));
	a.insert(Quad(
		Term::Ground(32),
		Term::Var(34),
		Term::Ground(27),
		None,
	));
	a.insert(Quad(
		Term::Ground(22),
		Term::Ground(40),
		Term::Ground(42),
		None,
	));
	a.insert(Quad(
		Term::Ground(33),
		Term::Var(47),
		Term::Ground(39),
		None,
	));
	a.insert(Quad(Term::Ground(14), Term::Var(32), Term::Var(42), None));
	a.insert(Quad(
		Term::Ground(6),
		Term::Ground(32),
		Term::Ground(8),
		None,
	));
	a.insert(Quad(Term::Var(32), Term::Var(16), Term::Ground(21), None));
	a.insert(Quad(Term::Var(11), Term::Ground(39), Term::Var(3), None));
	a.insert(Quad(Term::Ground(1), Term::Ground(34), Term::Var(12), None));
	a.insert(Quad(
		Term::Ground(49),
		Term::Ground(15),
		Term::Ground(31),
		None,
	));
	a.insert(Quad(
		Term::Var(19),
		Term::Ground(49),
		Term::Ground(25),
		None,
	));
	a.insert(Quad(Term::Ground(4), Term::Var(18), Term::Ground(37), None));
	a.insert(Quad(
		Term::Ground(16),
		Term::Ground(49),
		Term::Var(12),
		None,
	));
	a.insert(Quad(
		Term::Ground(49),
		Term::Ground(14),
		Term::Ground(49),
		None,
	));
	a.insert(Quad(Term::Var(42), Term::Ground(36), Term::Var(3), None));
	a.insert(Quad(Term::Ground(44), Term::Var(49), Term::Ground(8), None));
	a.insert(Quad(
		Term::Ground(46),
		Term::Ground(47),
		Term::Var(17),
		None,
	));
	a.insert(Quad(Term::Ground(24), Term::Var(13), Term::Ground(2), None));
	a.insert(Quad(
		Term::Ground(25),
		Term::Ground(43),
		Term::Var(29),
		None,
	));
	a.insert(Quad(Term::Var(6), Term::Var(21), Term::Var(12), None));
	a.insert(Quad(
		Term::Ground(27),
		Term::Var(49),
		Term::Ground(41),
		None,
	));
	a.insert(Quad(
		Term::Var(36),
		Term::Ground(19),
		Term::Ground(15),
		None,
	));
	a.insert(Quad(
		Term::Var(42),
		Term::Ground(17),
		Term::Ground(11),
		None,
	));
	a.insert(Quad(Term::Var(48), Term::Ground(45), Term::Var(2), None));
	a.insert(Quad(
		Term::Ground(46),
		Term::Ground(24),
		Term::Var(18),
		None,
	));
	a.insert(Quad(Term::Var(35), Term::Ground(18), Term::Var(43), None));
	a.insert(Quad(Term::Var(26), Term::Var(24), Term::Ground(17), None));
	a.insert(Quad(
		Term::Ground(12),
		Term::Var(20),
		Term::Ground(15),
		None,
	));
	a.insert(Quad(
		Term::Var(42),
		Term::Ground(30),
		Term::Ground(29),
		None,
	));
	a.insert(Quad(Term::Ground(5), Term::Ground(13), Term::Var(22), None));
	a.insert(Quad(
		Term::Ground(18),
		Term::Ground(23),
		Term::Var(44),
		None,
	));
	a.insert(Quad(Term::Ground(28), Term::Var(23), Term::Ground(0), None));
	a.insert(Quad(Term::Var(0), Term::Var(15), Term::Ground(13), None));
	a.insert(Quad(Term::Var(10), Term::Var(25), Term::Var(45), None));
	a.insert(Quad(Term::Var(1), Term::Ground(3), Term::Var(3), None));
	a.insert(Quad(
		Term::Ground(35),
		Term::Ground(3),
		Term::Ground(49),
		None,
	));
	a.insert(Quad(Term::Var(42), Term::Var(28), Term::Ground(0), None));
	a.insert(Quad(
		Term::Var(47),
		Term::Ground(26),
		Term::Ground(41),
		None,
	));
	a.insert(Quad(Term::Ground(29), Term::Var(16), Term::Var(22), None));
	a.insert(Quad(
		Term::Ground(22),
		Term::Var(17),
		Term::Ground(13),
		None,
	));
	a.insert(Quad(Term::Var(43), Term::Var(7), Term::Var(20), None));
	a.insert(Quad(Term::Var(4), Term::Var(49), Term::Var(26), None));
	a.insert(Quad(
		Term::Ground(12),
		Term::Var(47),
		Term::Ground(39),
		None,
	));
	a.insert(Quad(Term::Var(4), Term::Var(0), Term::Var(23), None));
	a.insert(Quad(Term::Var(6), Term::Ground(42), Term::Var(31), None));
	a.insert(Quad(Term::Var(22), Term::Var(1), Term::Ground(4), None));
	a.insert(Quad(Term::Ground(8), Term::Ground(20), Term::Var(29), None));
	a.insert(Quad(Term::Ground(43), Term::Var(14), Term::Ground(3), None));
	a.insert(Quad(
		Term::Var(33),
		Term::Ground(19),
		Term::Ground(31),
		None,
	));
	a.insert(Quad(
		Term::Ground(30),
		Term::Var(31),
		Term::Ground(26),
		None,
	));
	a.insert(Quad(Term::Var(28), Term::Var(47), Term::Var(4), None));
	a.insert(Quad(Term::Var(33), Term::Var(7), Term::Var(46), None));
	a.insert(Quad(
		Term::Ground(19),
		Term::Ground(24),
		Term::Ground(4),
		None,
	));
	a.insert(Quad(Term::Var(3), Term::Ground(38), Term::Var(42), None));
	a.insert(Quad(
		Term::Ground(41),
		Term::Ground(43),
		Term::Ground(33),
		None,
	));
	a.insert(Quad(Term::Var(24), Term::Ground(36), Term::Var(23), None));
	a.insert(Quad(
		Term::Ground(40),
		Term::Ground(42),
		Term::Var(24),
		None,
	));
	a.insert(Quad(Term::Var(43), Term::Ground(44), Term::Var(38), None));
	a.insert(Quad(Term::Var(4), Term::Ground(14), Term::Ground(38), None));
	a.insert(Quad(Term::Var(5), Term::Ground(36), Term::Ground(17), None));
	a.insert(Quad(Term::Var(43), Term::Ground(5), Term::Var(33), None));
	a.insert(Quad(Term::Var(10), Term::Var(41), Term::Var(38), None));
	a.insert(Quad(Term::Ground(18), Term::Var(10), Term::Var(16), None));
	a.insert(Quad(
		Term::Ground(27),
		Term::Ground(33),
		Term::Var(35),
		None,
	));
	a.insert(Quad(Term::Var(9), Term::Var(2), Term::Var(7), None));
	a.insert(Quad(Term::Ground(24), Term::Var(47), Term::Var(16), None));
	a.insert(Quad(
		Term::Ground(25),
		Term::Ground(2),
		Term::Ground(41),
		None,
	));
	a.insert(Quad(
		Term::Ground(38),
		Term::Var(11),
		Term::Ground(24),
		None,
	));
	a.insert(Quad(Term::Ground(36), Term::Var(7), Term::Var(36), None));
	a.insert(Quad(Term::Var(38), Term::Ground(8), Term::Ground(7), None));
	a.insert(Quad(Term::Var(27), Term::Var(38), Term::Var(4), None));
	a.insert(Quad(Term::Var(18), Term::Var(2), Term::Var(10), None));
	a.insert(Quad(
		Term::Var(32),
		Term::Ground(34),
		Term::Ground(25),
		None,
	));
	a.insert(Quad(Term::Var(26), Term::Ground(42), Term::Var(44), None));
	a.insert(Quad(
		Term::Ground(1),
		Term::Ground(47),
		Term::Ground(5),
		None,
	));
	a.insert(Quad(
		Term::Ground(19),
		Term::Var(17),
		Term::Ground(26),
		None,
	));
	a.insert(Quad(Term::Var(45), Term::Var(8), Term::Var(21), None));
	a.insert(Quad(Term::Ground(24), Term::Var(25), Term::Var(11), None));
	a.insert(Quad(Term::Ground(13), Term::Var(6), Term::Ground(35), None));
	a.insert(Quad(Term::Ground(8), Term::Ground(18), Term::Var(8), None));
	a.insert(Quad(
		Term::Ground(39),
		Term::Ground(42),
		Term::Ground(30),
		None,
	));
	a.insert(Quad(Term::Ground(9), Term::Var(20), Term::Ground(2), None));
	a.insert(Quad(Term::Ground(7), Term::Var(48), Term::Ground(11), None));
	a.insert(Quad(Term::Ground(7), Term::Ground(19), Term::Var(33), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Ground(23),
		Term::Var(1043),
		Term::Ground(39),
		None,
	));
	b.insert(Quad(
		Term::Var(1025),
		Term::Var(1006),
		Term::Ground(28),
		None,
	));
	b.insert(Quad(
		Term::Var(1028),
		Term::Ground(41),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Var(1048),
		Term::Var(1021),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Var(1006),
		Term::Var(1007),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Var(1045),
		Term::Var(1017),
		Term::Var(1019),
		None,
	));
	b.insert(Quad(
		Term::Ground(16),
		Term::Var(1020),
		Term::Ground(31),
		None,
	));
	b.insert(Quad(
		Term::Ground(13),
		Term::Ground(7),
		Term::Ground(14),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Var(1030),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Var(1038),
		Term::Ground(11),
		Term::Ground(41),
		None,
	));
	b.insert(Quad(
		Term::Var(1006),
		Term::Ground(31),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(23),
		Term::Var(1031),
		Term::Ground(43),
		None,
	));
	b.insert(Quad(
		Term::Ground(18),
		Term::Ground(26),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(22),
		Term::Var(1026),
		Term::Var(1010),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Var(1010),
		Term::Var(1006),
		None,
	));
	b.insert(Quad(
		Term::Ground(15),
		Term::Ground(24),
		Term::Var(1026),
		None,
	));
	b.insert(Quad(
		Term::Var(1044),
		Term::Var(1033),
		Term::Ground(25),
		None,
	));
	b.insert(Quad(
		Term::Ground(32),
		Term::Var(1034),
		Term::Ground(27),
		None,
	));
	b.insert(Quad(
		Term::Ground(22),
		Term::Ground(40),
		Term::Ground(42),
		None,
	));
	b.insert(Quad(
		Term::Ground(33),
		Term::Var(1047),
		Term::Ground(39),
		None,
	));
	b.insert(Quad(
		Term::Ground(14),
		Term::Var(1032),
		Term::Var(1042),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Ground(32),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Var(1032),
		Term::Var(1016),
		Term::Ground(21),
		None,
	));
	b.insert(Quad(
		Term::Var(1011),
		Term::Ground(39),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(34),
		Term::Var(1012),
		None,
	));
	b.insert(Quad(
		Term::Ground(49),
		Term::Ground(15),
		Term::Ground(31),
		None,
	));
	b.insert(Quad(
		Term::Var(1019),
		Term::Ground(49),
		Term::Ground(25),
		None,
	));
	b.insert(Quad(
		Term::Ground(4),
		Term::Var(1018),
		Term::Ground(37),
		None,
	));
	b.insert(Quad(
		Term::Ground(16),
		Term::Ground(49),
		Term::Var(1012),
		None,
	));
	b.insert(Quad(
		Term::Ground(49),
		Term::Ground(14),
		Term::Ground(49),
		None,
	));
	b.insert(Quad(
		Term::Var(1042),
		Term::Ground(36),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Ground(44),
		Term::Var(1049),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Ground(46),
		Term::Ground(47),
		Term::Var(1017),
		None,
	));
	b.insert(Quad(
		Term::Ground(24),
		Term::Var(1013),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(25),
		Term::Ground(43),
		Term::Var(1029),
		None,
	));
	b.insert(Quad(
		Term::Var(1006),
		Term::Var(1021),
		Term::Var(1012),
		None,
	));
	b.insert(Quad(
		Term::Ground(27),
		Term::Var(1049),
		Term::Ground(41),
		None,
	));
	b.insert(Quad(
		Term::Var(1036),
		Term::Ground(19),
		Term::Ground(15),
		None,
	));
	b.insert(Quad(
		Term::Var(1042),
		Term::Ground(17),
		Term::Ground(11),
		None,
	));
	b.insert(Quad(
		Term::Var(1048),
		Term::Ground(45),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Ground(46),
		Term::Ground(24),
		Term::Var(1018),
		None,
	));
	b.insert(Quad(
		Term::Var(1035),
		Term::Ground(18),
		Term::Var(1043),
		None,
	));
	b.insert(Quad(
		Term::Var(1026),
		Term::Var(1024),
		Term::Ground(17),
		None,
	));
	b.insert(Quad(
		Term::Ground(12),
		Term::Var(1020),
		Term::Ground(15),
		None,
	));
	b.insert(Quad(
		Term::Var(1042),
		Term::Ground(30),
		Term::Ground(29),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Ground(13),
		Term::Var(1022),
		None,
	));
	b.insert(Quad(
		Term::Ground(18),
		Term::Ground(23),
		Term::Var(1044),
		None,
	));
	b.insert(Quad(
		Term::Ground(28),
		Term::Var(1023),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Var(1000),
		Term::Var(1015),
		Term::Ground(13),
		None,
	));
	b.insert(Quad(
		Term::Var(1010),
		Term::Var(1025),
		Term::Var(1045),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Ground(3),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Ground(35),
		Term::Ground(3),
		Term::Ground(49),
		None,
	));
	b.insert(Quad(
		Term::Var(1042),
		Term::Var(1028),
		Term::Ground(0),
		None,
	));
	b.insert(Quad(
		Term::Var(1047),
		Term::Ground(26),
		Term::Ground(41),
		None,
	));
	b.insert(Quad(
		Term::Ground(29),
		Term::Var(1016),
		Term::Var(1022),
		None,
	));
	b.insert(Quad(
		Term::Ground(22),
		Term::Var(1017),
		Term::Ground(13),
		None,
	));
	b.insert(Quad(
		Term::Var(1043),
		Term::Var(1007),
		Term::Var(1020),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Var(1049),
		Term::Var(1026),
		None,
	));
	b.insert(Quad(
		Term::Ground(12),
		Term::Var(1047),
		Term::Ground(39),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Var(1000),
		Term::Var(1023),
		None,
	));
	b.insert(Quad(
		Term::Var(1006),
		Term::Ground(42),
		Term::Var(1031),
		None,
	));
	b.insert(Quad(
		Term::Var(1022),
		Term::Var(1001),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Ground(20),
		Term::Var(1029),
		None,
	));
	b.insert(Quad(
		Term::Ground(43),
		Term::Var(1014),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Var(1033),
		Term::Ground(19),
		Term::Ground(31),
		None,
	));
	b.insert(Quad(
		Term::Ground(30),
		Term::Var(1031),
		Term::Ground(26),
		None,
	));
	b.insert(Quad(
		Term::Var(1028),
		Term::Var(1047),
		Term::Var(1004),
		None,
	));
	b.insert(Quad(
		Term::Var(1033),
		Term::Var(1007),
		Term::Var(1046),
		None,
	));
	b.insert(Quad(
		Term::Ground(19),
		Term::Ground(24),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Ground(38),
		Term::Var(1042),
		None,
	));
	b.insert(Quad(
		Term::Ground(41),
		Term::Ground(43),
		Term::Ground(33),
		None,
	));
	b.insert(Quad(
		Term::Var(1024),
		Term::Ground(36),
		Term::Var(1023),
		None,
	));
	b.insert(Quad(
		Term::Ground(40),
		Term::Ground(42),
		Term::Var(1024),
		None,
	));
	b.insert(Quad(
		Term::Var(1043),
		Term::Ground(44),
		Term::Var(1038),
		None,
	));
	b.insert(Quad(
		Term::Var(1004),
		Term::Ground(14),
		Term::Ground(38),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Ground(36),
		Term::Ground(17),
		None,
	));
	b.insert(Quad(
		Term::Var(1043),
		Term::Ground(5),
		Term::Var(1033),
		None,
	));
	b.insert(Quad(
		Term::Var(1010),
		Term::Var(1041),
		Term::Var(1038),
		None,
	));
	b.insert(Quad(
		Term::Ground(18),
		Term::Var(1010),
		Term::Var(1016),
		None,
	));
	b.insert(Quad(
		Term::Ground(27),
		Term::Ground(33),
		Term::Var(1035),
		None,
	));
	b.insert(Quad(
		Term::Var(1009),
		Term::Var(1002),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Ground(24),
		Term::Var(1047),
		Term::Var(1016),
		None,
	));
	b.insert(Quad(
		Term::Ground(25),
		Term::Ground(2),
		Term::Ground(41),
		None,
	));
	b.insert(Quad(
		Term::Ground(38),
		Term::Var(1011),
		Term::Ground(24),
		None,
	));
	b.insert(Quad(
		Term::Ground(36),
		Term::Var(1007),
		Term::Var(1036),
		None,
	));
	b.insert(Quad(
		Term::Var(1038),
		Term::Ground(8),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Var(1027),
		Term::Var(1038),
		Term::Var(1004),
		None,
	));
	b.insert(Quad(
		Term::Var(1018),
		Term::Var(1002),
		Term::Var(1010),
		None,
	));
	b.insert(Quad(
		Term::Var(1032),
		Term::Ground(34),
		Term::Ground(25),
		None,
	));
	b.insert(Quad(
		Term::Var(1026),
		Term::Ground(42),
		Term::Var(1044),
		None,
	));
	b.insert(Quad(
		Term::Ground(1),
		Term::Ground(47),
		Term::Ground(5),
		None,
	));
	b.insert(Quad(
		Term::Ground(19),
		Term::Var(1017),
		Term::Ground(26),
		None,
	));
	b.insert(Quad(
		Term::Var(1045),
		Term::Var(1008),
		Term::Var(1021),
		None,
	));
	b.insert(Quad(
		Term::Ground(24),
		Term::Var(1025),
		Term::Var(1011),
		None,
	));
	b.insert(Quad(
		Term::Ground(13),
		Term::Var(1006),
		Term::Ground(35),
		None,
	));
	b.insert(Quad(
		Term::Ground(8),
		Term::Ground(18),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Ground(39),
		Term::Ground(42),
		Term::Ground(30),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Var(1020),
		Term::Ground(2),
		None,
	));
	b.insert(Quad(
		Term::Ground(7),
		Term::Var(1048),
		Term::Ground(11),
		None,
	));
	b.insert(Quad(
		Term::Ground(7),
		Term::Ground(19),
		Term::Var(1033),
		None,
	));
	test(a, b)
}
#[test]
fn iso_200() {
	let mut a = BTreeDataset::new();
	a.insert(Quad(Term::Var(4), Term::Ground(12), Term::Ground(24), None));
	a.insert(Quad(Term::Var(21), Term::Var(8), Term::Var(18), None));
	a.insert(Quad(Term::Var(22), Term::Ground(36), Term::Var(36), None));
	a.insert(Quad(Term::Ground(46), Term::Var(40), Term::Var(46), None));
	a.insert(Quad(Term::Var(36), Term::Var(17), Term::Ground(34), None));
	a.insert(Quad(Term::Var(32), Term::Ground(38), Term::Var(33), None));
	a.insert(Quad(
		Term::Ground(47),
		Term::Ground(18),
		Term::Var(40),
		None,
	));
	a.insert(Quad(Term::Var(5), Term::Var(37), Term::Ground(47), None));
	a.insert(Quad(Term::Var(29), Term::Var(3), Term::Ground(22), None));
	a.insert(Quad(Term::Var(3), Term::Var(39), Term::Var(15), None));
	a.insert(Quad(
		Term::Ground(35),
		Term::Var(24),
		Term::Ground(47),
		None,
	));
	a.insert(Quad(Term::Ground(11), Term::Var(18), Term::Var(45), None));
	a.insert(Quad(Term::Var(43), Term::Ground(41), Term::Var(31), None));
	a.insert(Quad(
		Term::Ground(16),
		Term::Ground(34),
		Term::Ground(40),
		None,
	));
	a.insert(Quad(
		Term::Ground(44),
		Term::Ground(47),
		Term::Ground(17),
		None,
	));
	a.insert(Quad(
		Term::Ground(17),
		Term::Ground(13),
		Term::Var(15),
		None,
	));
	a.insert(Quad(
		Term::Ground(35),
		Term::Ground(17),
		Term::Var(26),
		None,
	));
	a.insert(Quad(Term::Var(32), Term::Var(40), Term::Var(22), None));
	a.insert(Quad(Term::Var(29), Term::Ground(5), Term::Ground(25), None));
	a.insert(Quad(Term::Ground(31), Term::Ground(40), Term::Var(0), None));
	a.insert(Quad(
		Term::Ground(16),
		Term::Ground(32),
		Term::Ground(35),
		None,
	));
	a.insert(Quad(Term::Ground(9), Term::Var(23), Term::Ground(45), None));
	a.insert(Quad(Term::Var(26), Term::Ground(30), Term::Var(22), None));
	a.insert(Quad(Term::Var(42), Term::Var(25), Term::Var(27), None));
	a.insert(Quad(
		Term::Ground(6),
		Term::Ground(25),
		Term::Ground(33),
		None,
	));
	a.insert(Quad(Term::Var(42), Term::Var(32), Term::Var(41), None));
	a.insert(Quad(Term::Ground(17), Term::Var(43), Term::Var(20), None));
	a.insert(Quad(Term::Var(41), Term::Ground(22), Term::Var(11), None));
	a.insert(Quad(Term::Var(46), Term::Var(46), Term::Ground(11), None));
	a.insert(Quad(
		Term::Ground(12),
		Term::Ground(24),
		Term::Ground(4),
		None,
	));
	a.insert(Quad(
		Term::Ground(27),
		Term::Ground(40),
		Term::Ground(38),
		None,
	));
	a.insert(Quad(Term::Ground(49), Term::Var(22), Term::Var(44), None));
	a.insert(Quad(Term::Var(1), Term::Var(27), Term::Var(43), None));
	a.insert(Quad(Term::Var(17), Term::Ground(27), Term::Var(2), None));
	a.insert(Quad(Term::Ground(23), Term::Var(3), Term::Ground(30), None));
	a.insert(Quad(
		Term::Ground(9),
		Term::Ground(11),
		Term::Ground(3),
		None,
	));
	a.insert(Quad(Term::Ground(12), Term::Var(33), Term::Var(38), None));
	a.insert(Quad(Term::Var(15), Term::Ground(7), Term::Var(17), None));
	a.insert(Quad(Term::Var(49), Term::Var(29), Term::Var(5), None));
	a.insert(Quad(
		Term::Ground(20),
		Term::Ground(44),
		Term::Ground(26),
		None,
	));
	a.insert(Quad(Term::Ground(32), Term::Var(4), Term::Var(8), None));
	a.insert(Quad(Term::Var(14), Term::Ground(25), Term::Var(17), None));
	a.insert(Quad(Term::Var(22), Term::Var(1), Term::Var(4), None));
	a.insert(Quad(Term::Ground(9), Term::Ground(49), Term::Var(41), None));
	a.insert(Quad(Term::Var(8), Term::Ground(13), Term::Ground(19), None));
	a.insert(Quad(Term::Ground(25), Term::Var(4), Term::Var(38), None));
	a.insert(Quad(
		Term::Ground(17),
		Term::Ground(37),
		Term::Var(39),
		None,
	));
	a.insert(Quad(Term::Var(15), Term::Var(21), Term::Var(35), None));
	a.insert(Quad(Term::Ground(0), Term::Var(37), Term::Var(21), None));
	a.insert(Quad(Term::Var(14), Term::Var(22), Term::Ground(45), None));
	a.insert(Quad(
		Term::Ground(11),
		Term::Var(35),
		Term::Ground(11),
		None,
	));
	a.insert(Quad(
		Term::Ground(12),
		Term::Ground(38),
		Term::Var(18),
		None,
	));
	a.insert(Quad(Term::Ground(34), Term::Var(4), Term::Ground(7), None));
	a.insert(Quad(Term::Var(26), Term::Var(27), Term::Var(12), None));
	a.insert(Quad(
		Term::Ground(20),
		Term::Ground(39),
		Term::Ground(31),
		None,
	));
	a.insert(Quad(Term::Var(39), Term::Var(22), Term::Var(20), None));
	a.insert(Quad(Term::Ground(42), Term::Ground(34), Term::Var(0), None));
	a.insert(Quad(
		Term::Ground(18),
		Term::Ground(12),
		Term::Ground(9),
		None,
	));
	a.insert(Quad(Term::Ground(5), Term::Ground(36), Term::Var(40), None));
	a.insert(Quad(
		Term::Ground(2),
		Term::Ground(26),
		Term::Ground(18),
		None,
	));
	a.insert(Quad(Term::Ground(35), Term::Var(25), Term::Var(43), None));
	a.insert(Quad(Term::Var(31), Term::Var(47), Term::Var(1), None));
	a.insert(Quad(Term::Ground(45), Term::Ground(15), Term::Var(3), None));
	a.insert(Quad(Term::Var(36), Term::Var(46), Term::Var(17), None));
	a.insert(Quad(Term::Ground(26), Term::Ground(9), Term::Var(35), None));
	a.insert(Quad(Term::Var(12), Term::Ground(5), Term::Var(39), None));
	a.insert(Quad(Term::Ground(35), Term::Var(25), Term::Ground(8), None));
	a.insert(Quad(
		Term::Ground(46),
		Term::Var(31),
		Term::Ground(36),
		None,
	));
	a.insert(Quad(Term::Ground(17), Term::Var(2), Term::Ground(13), None));
	a.insert(Quad(Term::Ground(14), Term::Var(16), Term::Var(30), None));
	a.insert(Quad(
		Term::Ground(47),
		Term::Var(41),
		Term::Ground(45),
		None,
	));
	a.insert(Quad(
		Term::Var(35),
		Term::Ground(31),
		Term::Ground(34),
		None,
	));
	a.insert(Quad(
		Term::Ground(19),
		Term::Var(41),
		Term::Ground(12),
		None,
	));
	a.insert(Quad(
		Term::Ground(43),
		Term::Ground(32),
		Term::Ground(9),
		None,
	));
	a.insert(Quad(
		Term::Ground(36),
		Term::Var(48),
		Term::Ground(21),
		None,
	));
	a.insert(Quad(
		Term::Ground(41),
		Term::Ground(8),
		Term::Ground(24),
		None,
	));
	a.insert(Quad(
		Term::Ground(35),
		Term::Var(36),
		Term::Ground(39),
		None,
	));
	a.insert(Quad(Term::Var(5), Term::Ground(9), Term::Ground(49), None));
	a.insert(Quad(Term::Ground(12), Term::Var(47), Term::Var(1), None));
	a.insert(Quad(
		Term::Ground(14),
		Term::Ground(12),
		Term::Ground(29),
		None,
	));
	a.insert(Quad(Term::Var(18), Term::Var(13), Term::Var(14), None));
	a.insert(Quad(Term::Var(43), Term::Var(40), Term::Var(44), None));
	a.insert(Quad(Term::Ground(25), Term::Ground(31), Term::Var(7), None));
	a.insert(Quad(
		Term::Ground(21),
		Term::Ground(25),
		Term::Ground(8),
		None,
	));
	a.insert(Quad(Term::Var(6), Term::Var(9), Term::Ground(48), None));
	a.insert(Quad(
		Term::Ground(11),
		Term::Ground(46),
		Term::Var(14),
		None,
	));
	a.insert(Quad(Term::Var(47), Term::Var(40), Term::Ground(14), None));
	a.insert(Quad(Term::Ground(47), Term::Var(44), Term::Var(4), None));
	a.insert(Quad(Term::Var(12), Term::Ground(32), Term::Ground(3), None));
	a.insert(Quad(Term::Ground(37), Term::Var(8), Term::Ground(36), None));
	a.insert(Quad(Term::Var(49), Term::Ground(27), Term::Var(19), None));
	a.insert(Quad(Term::Ground(34), Term::Var(8), Term::Ground(49), None));
	a.insert(Quad(Term::Var(10), Term::Var(28), Term::Ground(25), None));
	a.insert(Quad(Term::Ground(5), Term::Var(39), Term::Ground(13), None));
	a.insert(Quad(
		Term::Ground(37),
		Term::Ground(49),
		Term::Var(16),
		None,
	));
	a.insert(Quad(
		Term::Ground(35),
		Term::Ground(3),
		Term::Ground(29),
		None,
	));
	a.insert(Quad(Term::Ground(3), Term::Ground(20), Term::Var(36), None));
	a.insert(Quad(Term::Var(31), Term::Ground(33), Term::Ground(9), None));
	a.insert(Quad(Term::Var(26), Term::Ground(34), Term::Var(16), None));
	a.insert(Quad(Term::Ground(3), Term::Ground(2), Term::Var(39), None));
	let mut b = BTreeDataset::new();
	b.insert(Quad(
		Term::Var(1004),
		Term::Ground(12),
		Term::Ground(24),
		None,
	));
	b.insert(Quad(
		Term::Var(1021),
		Term::Var(1008),
		Term::Var(1018),
		None,
	));
	b.insert(Quad(
		Term::Var(1022),
		Term::Ground(36),
		Term::Var(1036),
		None,
	));
	b.insert(Quad(
		Term::Ground(46),
		Term::Var(1040),
		Term::Var(1046),
		None,
	));
	b.insert(Quad(
		Term::Var(1036),
		Term::Var(1017),
		Term::Ground(34),
		None,
	));
	b.insert(Quad(
		Term::Var(1032),
		Term::Ground(38),
		Term::Var(1033),
		None,
	));
	b.insert(Quad(
		Term::Ground(47),
		Term::Ground(18),
		Term::Var(1040),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Var(1037),
		Term::Ground(47),
		None,
	));
	b.insert(Quad(
		Term::Var(1029),
		Term::Var(1003),
		Term::Ground(22),
		None,
	));
	b.insert(Quad(
		Term::Var(1003),
		Term::Var(1039),
		Term::Var(1015),
		None,
	));
	b.insert(Quad(
		Term::Ground(35),
		Term::Var(1024),
		Term::Ground(47),
		None,
	));
	b.insert(Quad(
		Term::Ground(11),
		Term::Var(1018),
		Term::Var(1045),
		None,
	));
	b.insert(Quad(
		Term::Var(1043),
		Term::Ground(41),
		Term::Var(1031),
		None,
	));
	b.insert(Quad(
		Term::Ground(16),
		Term::Ground(34),
		Term::Ground(40),
		None,
	));
	b.insert(Quad(
		Term::Ground(44),
		Term::Ground(47),
		Term::Ground(17),
		None,
	));
	b.insert(Quad(
		Term::Ground(17),
		Term::Ground(13),
		Term::Var(1015),
		None,
	));
	b.insert(Quad(
		Term::Ground(35),
		Term::Ground(17),
		Term::Var(1026),
		None,
	));
	b.insert(Quad(
		Term::Var(1032),
		Term::Var(1040),
		Term::Var(1022),
		None,
	));
	b.insert(Quad(
		Term::Var(1029),
		Term::Ground(5),
		Term::Ground(25),
		None,
	));
	b.insert(Quad(
		Term::Ground(31),
		Term::Ground(40),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Ground(16),
		Term::Ground(32),
		Term::Ground(35),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Var(1023),
		Term::Ground(45),
		None,
	));
	b.insert(Quad(
		Term::Var(1026),
		Term::Ground(30),
		Term::Var(1022),
		None,
	));
	b.insert(Quad(
		Term::Var(1042),
		Term::Var(1025),
		Term::Var(1027),
		None,
	));
	b.insert(Quad(
		Term::Ground(6),
		Term::Ground(25),
		Term::Ground(33),
		None,
	));
	b.insert(Quad(
		Term::Var(1042),
		Term::Var(1032),
		Term::Var(1041),
		None,
	));
	b.insert(Quad(
		Term::Ground(17),
		Term::Var(1043),
		Term::Var(1020),
		None,
	));
	b.insert(Quad(
		Term::Var(1041),
		Term::Ground(22),
		Term::Var(1011),
		None,
	));
	b.insert(Quad(
		Term::Var(1046),
		Term::Var(1046),
		Term::Ground(11),
		None,
	));
	b.insert(Quad(
		Term::Ground(12),
		Term::Ground(24),
		Term::Ground(4),
		None,
	));
	b.insert(Quad(
		Term::Ground(27),
		Term::Ground(40),
		Term::Ground(38),
		None,
	));
	b.insert(Quad(
		Term::Ground(49),
		Term::Var(1022),
		Term::Var(1044),
		None,
	));
	b.insert(Quad(
		Term::Var(1001),
		Term::Var(1027),
		Term::Var(1043),
		None,
	));
	b.insert(Quad(
		Term::Var(1017),
		Term::Ground(27),
		Term::Var(1002),
		None,
	));
	b.insert(Quad(
		Term::Ground(23),
		Term::Var(1003),
		Term::Ground(30),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Ground(11),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Ground(12),
		Term::Var(1033),
		Term::Var(1038),
		None,
	));
	b.insert(Quad(
		Term::Var(1015),
		Term::Ground(7),
		Term::Var(1017),
		None,
	));
	b.insert(Quad(
		Term::Var(1049),
		Term::Var(1029),
		Term::Var(1005),
		None,
	));
	b.insert(Quad(
		Term::Ground(20),
		Term::Ground(44),
		Term::Ground(26),
		None,
	));
	b.insert(Quad(
		Term::Ground(32),
		Term::Var(1004),
		Term::Var(1008),
		None,
	));
	b.insert(Quad(
		Term::Var(1014),
		Term::Ground(25),
		Term::Var(1017),
		None,
	));
	b.insert(Quad(
		Term::Var(1022),
		Term::Var(1001),
		Term::Var(1004),
		None,
	));
	b.insert(Quad(
		Term::Ground(9),
		Term::Ground(49),
		Term::Var(1041),
		None,
	));
	b.insert(Quad(
		Term::Var(1008),
		Term::Ground(13),
		Term::Ground(19),
		None,
	));
	b.insert(Quad(
		Term::Ground(25),
		Term::Var(1004),
		Term::Var(1038),
		None,
	));
	b.insert(Quad(
		Term::Ground(17),
		Term::Ground(37),
		Term::Var(1039),
		None,
	));
	b.insert(Quad(
		Term::Var(1015),
		Term::Var(1021),
		Term::Var(1035),
		None,
	));
	b.insert(Quad(
		Term::Ground(0),
		Term::Var(1037),
		Term::Var(1021),
		None,
	));
	b.insert(Quad(
		Term::Var(1014),
		Term::Var(1022),
		Term::Ground(45),
		None,
	));
	b.insert(Quad(
		Term::Ground(11),
		Term::Var(1035),
		Term::Ground(11),
		None,
	));
	b.insert(Quad(
		Term::Ground(12),
		Term::Ground(38),
		Term::Var(1018),
		None,
	));
	b.insert(Quad(
		Term::Ground(34),
		Term::Var(1004),
		Term::Ground(7),
		None,
	));
	b.insert(Quad(
		Term::Var(1026),
		Term::Var(1027),
		Term::Var(1012),
		None,
	));
	b.insert(Quad(
		Term::Ground(20),
		Term::Ground(39),
		Term::Ground(31),
		None,
	));
	b.insert(Quad(
		Term::Var(1039),
		Term::Var(1022),
		Term::Var(1020),
		None,
	));
	b.insert(Quad(
		Term::Ground(42),
		Term::Ground(34),
		Term::Var(1000),
		None,
	));
	b.insert(Quad(
		Term::Ground(18),
		Term::Ground(12),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Ground(36),
		Term::Var(1040),
		None,
	));
	b.insert(Quad(
		Term::Ground(2),
		Term::Ground(26),
		Term::Ground(18),
		None,
	));
	b.insert(Quad(
		Term::Ground(35),
		Term::Var(1025),
		Term::Var(1043),
		None,
	));
	b.insert(Quad(
		Term::Var(1031),
		Term::Var(1047),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(45),
		Term::Ground(15),
		Term::Var(1003),
		None,
	));
	b.insert(Quad(
		Term::Var(1036),
		Term::Var(1046),
		Term::Var(1017),
		None,
	));
	b.insert(Quad(
		Term::Ground(26),
		Term::Ground(9),
		Term::Var(1035),
		None,
	));
	b.insert(Quad(
		Term::Var(1012),
		Term::Ground(5),
		Term::Var(1039),
		None,
	));
	b.insert(Quad(
		Term::Ground(35),
		Term::Var(1025),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Ground(46),
		Term::Var(1031),
		Term::Ground(36),
		None,
	));
	b.insert(Quad(
		Term::Ground(17),
		Term::Var(1002),
		Term::Ground(13),
		None,
	));
	b.insert(Quad(
		Term::Ground(14),
		Term::Var(1016),
		Term::Var(1030),
		None,
	));
	b.insert(Quad(
		Term::Ground(47),
		Term::Var(1041),
		Term::Ground(45),
		None,
	));
	b.insert(Quad(
		Term::Var(1035),
		Term::Ground(31),
		Term::Ground(34),
		None,
	));
	b.insert(Quad(
		Term::Ground(19),
		Term::Var(1041),
		Term::Ground(12),
		None,
	));
	b.insert(Quad(
		Term::Ground(43),
		Term::Ground(32),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Ground(36),
		Term::Var(1048),
		Term::Ground(21),
		None,
	));
	b.insert(Quad(
		Term::Ground(41),
		Term::Ground(8),
		Term::Ground(24),
		None,
	));
	b.insert(Quad(
		Term::Ground(35),
		Term::Var(1036),
		Term::Ground(39),
		None,
	));
	b.insert(Quad(
		Term::Var(1005),
		Term::Ground(9),
		Term::Ground(49),
		None,
	));
	b.insert(Quad(
		Term::Ground(12),
		Term::Var(1047),
		Term::Var(1001),
		None,
	));
	b.insert(Quad(
		Term::Ground(14),
		Term::Ground(12),
		Term::Ground(29),
		None,
	));
	b.insert(Quad(
		Term::Var(1018),
		Term::Var(1013),
		Term::Var(1014),
		None,
	));
	b.insert(Quad(
		Term::Var(1043),
		Term::Var(1040),
		Term::Var(1044),
		None,
	));
	b.insert(Quad(
		Term::Ground(25),
		Term::Ground(31),
		Term::Var(1007),
		None,
	));
	b.insert(Quad(
		Term::Ground(21),
		Term::Ground(25),
		Term::Ground(8),
		None,
	));
	b.insert(Quad(
		Term::Var(1006),
		Term::Var(1009),
		Term::Ground(48),
		None,
	));
	b.insert(Quad(
		Term::Ground(11),
		Term::Ground(46),
		Term::Var(1014),
		None,
	));
	b.insert(Quad(
		Term::Var(1047),
		Term::Var(1040),
		Term::Ground(14),
		None,
	));
	b.insert(Quad(
		Term::Ground(47),
		Term::Var(1044),
		Term::Var(1004),
		None,
	));
	b.insert(Quad(
		Term::Var(1012),
		Term::Ground(32),
		Term::Ground(3),
		None,
	));
	b.insert(Quad(
		Term::Ground(37),
		Term::Var(1008),
		Term::Ground(36),
		None,
	));
	b.insert(Quad(
		Term::Var(1049),
		Term::Ground(27),
		Term::Var(1019),
		None,
	));
	b.insert(Quad(
		Term::Ground(34),
		Term::Var(1008),
		Term::Ground(49),
		None,
	));
	b.insert(Quad(
		Term::Var(1010),
		Term::Var(1028),
		Term::Ground(25),
		None,
	));
	b.insert(Quad(
		Term::Ground(5),
		Term::Var(1039),
		Term::Ground(13),
		None,
	));
	b.insert(Quad(
		Term::Ground(37),
		Term::Ground(49),
		Term::Var(1016),
		None,
	));
	b.insert(Quad(
		Term::Ground(35),
		Term::Ground(3),
		Term::Ground(29),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(20),
		Term::Var(1036),
		None,
	));
	b.insert(Quad(
		Term::Var(1031),
		Term::Ground(33),
		Term::Ground(9),
		None,
	));
	b.insert(Quad(
		Term::Var(1026),
		Term::Ground(34),
		Term::Var(1016),
		None,
	));
	b.insert(Quad(
		Term::Ground(3),
		Term::Ground(2),
		Term::Var(1039),
		None,
	));
	test(a, b)
}
