// Copyright 2018 Chainpool

#![cfg_attr(not(feature = "std"), no_std)]

extern crate byteorder;
extern crate primitives;
#[allow(unused_imports)]
#[macro_use]
extern crate sr_std as rstd;

mod compact_integer;
mod impls;
mod list;
mod reader;
mod stream;

pub use primitives::{hash, bytes, compact, io::Error};

pub use compact_integer::CompactInteger;
pub use list::List;
pub use reader::{Reader, Deserializable, deserialize, deserialize_iterator, ReadIterator};
pub use stream::{
	Stream, Serializable, serialize, serialize_with_flags, serialize_list, serialized_list_size,
	serialized_list_size_with_flags, SERIALIZE_TRANSACTION_WITNESS,
};

