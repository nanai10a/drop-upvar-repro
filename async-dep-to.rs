#![allow(incomplete_features)]
#![feature(async_drop)]

fn main() {
    let f = dep_by::coroutine();
    print!("{}", std::mem::size_of_val(&f));
}
