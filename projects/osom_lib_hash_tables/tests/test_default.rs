#![cfg(feature = "std")]
mod common;

use osom_lib_hash_tables::defaults::StdDefaultHashTable;

common::build_tests!(StdDefaultHashTable);
