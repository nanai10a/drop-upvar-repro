#![cfg_attr(feature = "async-drop", feature(async_drop))]

fn main() {
    dbg!(*be_depended::HAS_DROP_THAT_HAS_DROP_THAT_HAS_DROP);
    dbg!(std::mem::size_of_val(&be_depended::has_drop_that_has_drop_that_has_drop()));
    dbg!(*be_depended::HAS_DROP_THAT_HAS_DROP);
    dbg!(std::mem::size_of_val(&be_depended::has_drop_that_has_drop()));
    dbg!(*be_depended::HAS_DROP);
    dbg!(std::mem::size_of_val(&be_depended::has_drop()));
    dbg!(*be_depended::HAS_NO_DROP);
    dbg!(std::mem::size_of_val(&be_depended::has_no_drop()));
}
