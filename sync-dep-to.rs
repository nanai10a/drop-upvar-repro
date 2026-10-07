// #![allow(incomplete_features)]
// #![feature(async_drop)]

fn main() {
    dbg!(std::mem::size_of_val(&dep_by::has_drop_that_has_drop_that_has_drop()));
    dbg!(std::mem::size_of_val(&dep_by::has_drop_that_has_drop()));
    dbg!(std::mem::size_of_val(&dep_by::has_drop()));
    dbg!(std::mem::size_of_val(&dep_by::has_no_drop()));
}
