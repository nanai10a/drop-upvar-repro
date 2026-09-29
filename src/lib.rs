#![feature(async_drop)]

async fn pend_res() -> Result<Option<Vec<u8>>, ()> {
    core::future::pending::<()>().await;
    unreachable!()
}

pub struct Link {
    pub _n: u64,
}

impl Link {
    pub async fn tick(&mut self) -> Result<(), ()> {
        let _pb = std::ffi::OsString::from("x");
        let Some(_iov0) = pend_res().await? else { return Ok(()) };

        Ok(())
    }
}

#[cfg(test)]
mod layout_probe {
    use super::*;

    #[test]
    fn tick_future_contains_link_addr_lib_side() {
        let mut link = Link { _n: 0 };
        let addr = &link as *const _ as usize;
        let fut = link.tick();
        let size = std::mem::size_of_val(&fut);
        eprintln!("lib-side tick future size = {size}");
        let bytes: &[u8] =
            unsafe { std::slice::from_raw_parts(&fut as *const _ as *const u8, size) };
        let le = (addr as u64).to_le_bytes();
        let found = bytes.windows(le.len()).any(|w| w == le);
        std::mem::forget(fut);
        std::mem::forget(link);
        assert!(found, "lib-side: link address {addr:#x} not found in {size}-byte future");
    }
}
