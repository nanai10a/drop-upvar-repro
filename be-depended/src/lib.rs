#![cfg_attr(feature = "async-drop", feature(async_drop))]

pub struct HasNoDrop { _u64: u64 }

pub struct IsDrop(pub String);

impl IsDrop {
    fn new() -> Self {
        // some random text (brought from Google Fonts)
        Self(String::from("Everyone has the right to freedom of thought, conscience and religion; this right includes freedom to change his religion or belief, and freedom, either alone or in community with others and in public or private, to manifest his religion or belief in teaching, practice, worship and observance. Everyone has the right to freedom of opinion and expression; this right includes freedom to hold opinions without interference and to seek, receive and impart information and ideas through any media and regardless of frontiers. Everyone has the right to rest and leisure, including reasonable limitation of working hours and periodic holidays with pay."))
    }
}

pub struct HasDrop { _is_drop: IsDrop }
pub struct HasHasDrop { _has_drop: HasDrop }
pub struct HasHasHasDrop { _has_has_drop: HasHasDrop }

pub static HAS_DROP_THAT_HAS_DROP_THAT_HAS_DROP: std::sync::LazyLock<usize> = std::sync::LazyLock::new(|| std::mem::size_of_val(&std::mem::ManuallyDrop::new(has_drop_that_has_drop_that_has_drop()))) ;

pub async fn has_drop_that_has_drop_that_has_drop() {
    let _has_has_has_drop = HasHasHasDrop { _has_has_drop: HasHasDrop { _has_drop: HasDrop { _is_drop: IsDrop::new() } } };
}

pub static HAS_DROP_THAT_HAS_DROP: std::sync::LazyLock<usize> = std::sync::LazyLock::new(|| std::mem::size_of_val(&std::mem::ManuallyDrop::new(has_drop_that_has_drop()))) ;

pub async fn has_drop_that_has_drop() {
    let _has_has_drop = HasHasDrop { _has_drop: HasDrop { _is_drop: IsDrop::new() } };
}

pub static HAS_DROP: std::sync::LazyLock<usize> = std::sync::LazyLock::new(|| std::mem::size_of_val(&std::mem::ManuallyDrop::new(has_drop()))) ;

pub async fn has_drop() {
    let _has_drop = HasDrop { _is_drop: IsDrop::new() };
}

pub static HAS_NO_DROP: std::sync::LazyLock<usize> = std::sync::LazyLock::new(|| std::mem::size_of_val(&std::mem::ManuallyDrop::new(has_no_drop()))) ;

pub async fn has_no_drop() {
    let _has_no_drop = HasNoDrop { _u64: 0 };
}
