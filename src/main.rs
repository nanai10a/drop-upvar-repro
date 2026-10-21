#![cfg_attr(feature = "async-drop", feature(async_drop))]

use std::pin::pin;

macro_rules! print_compared_sizes {
    ($label:literal ... $expect:expr, $actual:expr) => {
        let expect_size = *$expect;
        let actual_size = std::mem::size_of_val($actual);
        let is_same_size = expect_size == actual_size;
        if is_same_size {
            println!("[{}] ok - size `{actual_size}`", $label);
        } else {
            println!("[{}] ng - inconsistent size, expects `{expect_size}` but actually is `{actual_size}`", $label);
        }
    };
}

#[cfg(not(feature = "minimal"))]
fn main() {
    let f = be_depended::has_no_drop();
    print_compared_sizes!("  no drop   " ... be_depended::HAS_NO_DROP, &f);
    let f = pin!(f);
    auxiliary::block_on(f);

    let f = be_depended::has_drop();
    print_compared_sizes!(" has drop   " ... be_depended::HAS_DROP, &f);
    let f = pin!(f);
    auxiliary::block_on(f);

    let f = be_depended::has_drop_that_has_drop();
    print_compared_sizes!("(has drop)^2" ... be_depended::HAS_DROP_THAT_HAS_DROP, &f);
    let f = pin!(f);
    auxiliary::block_on(f);

    let f = be_depended::has_drop_that_has_drop_that_has_drop();
    print_compared_sizes!("(has drop)^3" ... be_depended::HAS_DROP_THAT_HAS_DROP_THAT_HAS_DROP, &f);
    let f = pin!(f);
    auxiliary::block_on(f);
}

#[cfg(feature = "minimal")]
fn main() {
    let f = pin!(be_depended::has_drop_that_has_drop());
    auxiliary::block_on(f);

    let f = pin!(be_depended::has_drop_that_has_drop());
    auxiliary::block_on(f);
}
