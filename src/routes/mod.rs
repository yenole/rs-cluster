mod act_voice;
mod router;
mod state;

pub use router::setup_routing;
pub use state::RspAny;

pub type Result<T> = std::result::Result<RspAny<T>, crate::Error>;

#[macro_export]
macro_rules! Ok {
    ($v:expr) => {
        Ok(crate::routes::RspAny($v))
    };
}
