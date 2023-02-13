#[cfg(feature = "explicit")]
mod explicit_middleware;
#[cfg(feature = "implicit")]
mod ext;
#[cfg(feature = "implicit")]
mod implicit_middleware;

#[cfg(any(feature = "explicit", feature = "implicit",))]
pub type ArcTxn = std::sync::Arc<sea_orm::DatabaseTransaction>;
#[cfg(feature = "explicit")]
pub use explicit_middleware::*;
#[cfg(feature = "implicit")]
pub use ext::*;
#[cfg(feature = "implicit")]
pub use implicit_middleware::*;
