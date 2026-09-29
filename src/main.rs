use async_drop_unsoundness::Link;

fn check() {
    let mut link = Link { _n: 0 };
    let addr = &link as *const _ as usize;
    let fut = link.tick();
    let size = std::mem::size_of_val(&fut);
    eprintln!("tick future size = {size}");
    let bytes: &[u8] =
        unsafe { std::slice::from_raw_parts(&fut as *const _ as *const u8, size) };
    let le = (addr as u64).to_le_bytes();
    let found = bytes.windows(le.len()).any(|w| w == le);
    std::mem::forget(fut);
    std::mem::forget(link);
    assert!(found, "link address {addr:#x} not found in {size}-byte future");
}

fn main() {
    check();
}

#[cfg(test)]
mod tests {
    #[test]
    fn tick_future_contains_link_addr() {
        super::check();
    }
}
