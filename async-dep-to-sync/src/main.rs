#![allow(incomplete_features)]
#![feature(async_drop)]

fn main() {
    dbg!(*dep_by::HAS_DROP_THAT_HAS_DROP_THAT_HAS_DROP);
    dbg!(std::mem::size_of_val(&dep_by::has_drop_that_has_drop_that_has_drop()));
    dbg!(*dep_by::HAS_DROP_THAT_HAS_DROP);
    dbg!(std::mem::size_of_val(&dep_by::has_drop_that_has_drop()));
    dbg!(*dep_by::HAS_DROP);
    dbg!(std::mem::size_of_val(&dep_by::has_drop()));
    dbg!(*dep_by::HAS_NO_DROP);
    dbg!(std::mem::size_of_val(&dep_by::has_no_drop()));
}
