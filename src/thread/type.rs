use crate::*;
use std::{any::Any, sync::Arc};

/// Type alias for a boxed dynamic type that implements `Any` and `Send`.
///
/// - Represents a boxed value that can be used for dynamic type checking (`Any`)
///   and safely transferred across threads (`Send`).
pub type BoxAnySend = Box<dyn Any + Send>;

/// Type alias for the result type returned by spawnable functions.
///
/// - `Ok(())`: Indicates successful execution of the function.
/// - `Err(BoxAnySend)`: Contains a boxed error value in case of a panic or failure.
pub type SpawnResult = Result<(), BoxAnySend>;

/// Alias for an `Arc`-wrapped recoverable function.
///
/// - This type represents an `Arc`-wrapped version of any function implementing the `RecoverableFunction` trait.
/// - Enables shared ownership and thread-safe usage of recoverable functions in concurrent environments.
pub type BoxRecoverableFunction = Arc<dyn RecoverableFunction>;

/// Alias for an `Arc`-wrapped error handler function.
///
/// - This type represents an `Arc`-wrapped version of any function implementing the `ErrorHandlerFunction` trait.
/// - Allows shared ownership and thread-safe handling of errors with custom logic across multiple threads.
pub type BoxErrorHandlerFunction = Arc<dyn ErrorHandlerFunction>;
