#![feature(allocator_api)]
#![feature(dropck_eyepatch)]
#![feature(generic_param_attrs)]
#![feature(hashmap_internals)]
#![feature(placement_in_syntax)]
#![feature(placement_new_protocol)]
#![feature(ptr_internals)]
#![feature(sip_hash_13)]
#![feature(test)]

extern crate rand;

mod bench;
mod table;
mod map;

pub use map::*;
pub use table::{make_hash, SafeHash};

trait Recover<Q: ?Sized> {
    type Key;

    fn get(&self, key: &Q) -> Option<&Self::Key>;
    fn take(&mut self, key: &Q) -> Option<Self::Key>;
    fn replace(&mut self, key: Self::Key) -> Option<Self::Key>;
}
