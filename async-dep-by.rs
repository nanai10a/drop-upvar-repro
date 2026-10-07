#![allow(incomplete_features)]
#![feature(async_drop)]

pub async fn coroutine() -> Result<(), ()> {
    let _os = std::ffi::OsString::new();
    let Some(_v) = stub().await? else { return Ok(()) };
    Ok(())
}

async fn stub() -> Result<Option<Vec<u8>>, ()> {
    std::future::pending::<()>().await;
    unreachable!()
}
