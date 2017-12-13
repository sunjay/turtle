
/// An opaque timestamp, useful only for calculating durations between instances.
/// Basically, an abstraction similar to Instant.
pub trait Timestamp {
    /// How long has it been since this timestamp was taken, in fractional milliseconds
    fn elapsed(&self) -> f64;
}

/// A monotonic clock useful for calculating elapsed time.
///
/// This allows us to replace uses of Instant (which doesn't exist in wasm) with something that
/// does, when on that platform.
pub trait Clock {
    /// An opaque timestamp
    type Timestamp: Timestamp;

    fn now() -> Self::Timestamp;
}

