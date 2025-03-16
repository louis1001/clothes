// Taken from https://docs.rs/map-macro/latest/src/map_macro/lib.rs.html#140-144
#[macro_export]
macro_rules! dictionary {
    {$($v: expr),* $(,)?} => {
        std::collections::HashSet::from([$($v,)*])
    };
}

pub mod layout;
pub mod graphics;
// pub mod animation;
pub mod rendering;
pub mod output;
pub mod fonts;