//! Ready-to-use [`Dataset`](super::Dataset) implementations.
pub mod btree_dataset;
pub mod indexed_btree_dataset;

pub use btree_dataset::BTreeDataset;
pub use indexed_btree_dataset::IndexedBTreeDataset;
