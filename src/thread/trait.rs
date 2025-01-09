/// Trait alias for functions that can be executed in a recoverable context.
///
/// - Functions implementing this trait must satisfy `Fn() + Send + Sync + 'static`.
pub trait RecoverableFunction: Fn() + Send + Sync + 'static {}

impl<T> RecoverableFunction for T where T: Fn() + Send + Sync + 'static {}

/// Trait alias for error-handling functions used in a recoverable context.
///
/// - Functions implementing this trait must accept a `&str` as an error message
///   and satisfy `Fn(&str) + Send + Sync + 'static`.
pub trait ErrorHandlerFunction: Fn(&str) + Send + Sync + 'static {}

impl<T> ErrorHandlerFunction for T where T: Fn(&str) + Send + Sync + 'static {}
