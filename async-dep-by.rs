#![allow(incomplete_features)]
#![feature(async_drop)]

pub struct HasNoDrop { _u64: u64 }

pub struct IsDrop;
impl Drop for IsDrop { fn drop(&mut self) {/* stub */} }

pub struct HasDrop { _is_drop: IsDrop }
pub struct HasHasDrop { _has_drop: HasDrop }
pub struct HasHasHasDrop { _has_has_drop: HasHasDrop }

pub async fn has_drop_that_has_drop_that_has_drop() {
    let _has_has_has_drop = HasHasHasDrop { _has_has_drop: HasHasDrop { _has_drop: HasDrop { _is_drop: IsDrop } } };
}

pub async fn has_drop_that_has_drop() {
    let _has_has_drop = HasHasDrop { _has_drop: HasDrop { _is_drop: IsDrop } };
}

pub async fn has_drop() {
    let _has_drop = HasDrop { _is_drop: IsDrop };
}

pub async fn has_no_drop() {
    let _has_no_drop = HasNoDrop { _u64: 0 };
}
