use crate::vm_state::VMState;
use std::{
    error::Error,
    fmt,
    fmt::{Display, Formatter},
};

/// Represents errors during VM execution.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum VMError {
    /// Trying to exceed invocation stack size limit.
    InvocationStackOverflow(String),

    /// Trying to exceed try nesting limit.
    TryNestingOverflow(String),

    /// Trying to exceed maximum stack size.
    StackOverflow(String),

    /// Trying to create a single item that exceeds size limit.
    ItemTooLarge(String),

    /// Invalid opcode encountered.
    InvalidOpcode(String),

    /// Trying to divide by zero.
    DivisionByZero(String),

    /// Invalid jump offset or pointer.
    InvalidJump(String),

    /// Unsupported token encountered.
    InvalidToken(String),

    /// Invalid parameter for operation.
    InvalidParameter(String),

    /// Item not found in collection.
    ItemNotFound(String),

    /// Type mismatch for operation.
    InvalidType(String),

    /// Custom error with message.
    Custom(String),
}

impl Display for VMError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvocationStackOverflow(msg) => write!(f, "Invocation stack size limit exceeded: {}", msg),
            Self::TryNestingOverflow(msg) => write!(f, "Try nesting depth limit exceeded: {}", msg),
            Self::StackOverflow(msg) => write!(f, "Stack size limit exceeded: {}", msg),
            Self::ItemTooLarge(msg) => write!(f, "Item size exceeds limit: {}", msg),
            Self::InvalidOpcode(msg) => write!(f, "Encountered invalid opcode: {}", msg),
            Self::DivisionByZero(msg) => write!(f, "Tried to divide by zero: {}", msg),
            Self::InvalidJump(msg) => write!(f, "Invalid jump offset or pointer: {}", msg),
            Self::InvalidToken(msg) => write!(f, "Invalid token encountered: {}", msg),
            Self::InvalidParameter(msg) => write!(f, "Invalid parameter for operation: {}", msg),
            Self::ItemNotFound(msg) => write!(f, "Item not found in collection: {}", msg),
            Self::InvalidType(msg) => write!(f, "Type mismatch for operation: {}", msg),
            Self::Custom(msg) => write!(f, "Custom VM error: {}", msg),
        }
    }
}

impl Error for VMError {}

// The commented-out implementation is no longer needed as it's replaced by the above implementation.
