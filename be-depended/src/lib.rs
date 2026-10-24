#![cfg_attr(feature = "async-drop", feature(async_drop))]

pub struct HasNoDrop { _u64: u64 }

pub struct IsDrop(IsAnotherDrop);
impl Drop for IsDrop {
    fn drop(&mut self) { print!("{{<IsDrop as Drop>::drop}} "); }
}

pub struct IsAnotherDrop;
impl Drop for IsAnotherDrop {
    fn drop(&mut self) { print!("{{<IsAnotherDrop as Drop>::drop}} "); }
}

pub struct HasDrop { _is_drop: IsDrop }
pub struct HasHasDrop { _has_drop: HasDrop }
pub struct HasHasHasDrop { _has_has_drop: HasHasDrop }
#[cfg(feature = "extra")]
pub struct HasHasHasHasDrop { _has_has_has_drop: HasHasHasDrop }

#[cfg(feature = "extra")]
pub static HAS_DROP_THAT_HAS_DROP_THAT_HAS_DROP_THAT_HAS_DROP: std::sync::LazyLock<usize> = std::sync::LazyLock::new(|| std::mem::size_of_val(&std::mem::ManuallyDrop::new(has_drop_that_has_drop_that_has_drop_that_has_drop()))) ;

#[cfg(feature = "extra")]
pub async fn has_drop_that_has_drop_that_has_drop_that_has_drop() {
    let _has_has_has_has_drop = HasHasHasHasDrop { _has_has_has_drop: HasHasHasDrop { _has_has_drop: HasHasDrop { _has_drop: HasDrop { _is_drop: IsDrop(IsAnotherDrop) } } } };
}

pub static HAS_DROP_THAT_HAS_DROP_THAT_HAS_DROP: std::sync::LazyLock<usize> = std::sync::LazyLock::new(|| std::mem::size_of_val(&std::mem::ManuallyDrop::new(has_drop_that_has_drop_that_has_drop()))) ;

pub async fn has_drop_that_has_drop_that_has_drop() {
    let _has_has_has_drop = HasHasHasDrop { _has_has_drop: HasHasDrop { _has_drop: HasDrop { _is_drop: IsDrop(IsAnotherDrop) } } };
}

pub static HAS_DROP_THAT_HAS_DROP: std::sync::LazyLock<usize> = std::sync::LazyLock::new(|| std::mem::size_of_val(&std::mem::ManuallyDrop::new(has_drop_that_has_drop()))) ;

pub async fn has_drop_that_has_drop() {
    let _has_has_drop = HasHasDrop { _has_drop: HasDrop { _is_drop: IsDrop(IsAnotherDrop) } };
}

pub static HAS_DROP: std::sync::LazyLock<usize> = std::sync::LazyLock::new(|| std::mem::size_of_val(&std::mem::ManuallyDrop::new(has_drop()))) ;

pub async fn has_drop() {
    let _has_drop = HasDrop { _is_drop: IsDrop(IsAnotherDrop) };
}

pub static HAS_NO_DROP: std::sync::LazyLock<usize> = std::sync::LazyLock::new(|| std::mem::size_of_val(&std::mem::ManuallyDrop::new(has_no_drop()))) ;

pub async fn has_no_drop() {
    let _has_no_drop = HasNoDrop { _u64: 0 };
}
