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
        let _s = 0u8;
        let _p = &_s;
        let _pb = std::ffi::OsString::from("x");
        let Some(_iov0) = pend_res().await? else { return Ok(()) };

        Ok(())
    }
}
