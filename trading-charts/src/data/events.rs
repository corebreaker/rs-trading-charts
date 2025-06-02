//! `events` Module
//!
//! This module provides thread-safe event management through the `EventManager` struct.
//! `EventManager` allows adding event listeners and emitting events to all registered listeners.
//! It is designed for use in multithreaded environments while ensuring thread safety using `Arc` and `Mutex`.
use crate::error::JsError;
use std::sync::{Arc, Mutex};

/// `EventManager` is a thread-safe structure for managing events.
///
/// # Features
/// - Allows adding event listeners.
/// - Emits events to all registered listeners.
/// - Uses `Arc` and `Mutex` to ensure thread safety.
///
/// # Example Usage
/// ```rust
/// use trading_charts::data::events::EventManager;
///
/// let manager = EventManager::new();
///
/// manager
///     .add_listener(|event: String| {
///         println!("Event received: {}", event);
///     })
///     .unwrap();
///
/// manager.emit(String::from("Test Event")).unwrap();
/// ```
///
/// # Thread-Safety Guarantees
/// - Listeners are stored in a `Mutex`, ensuring exclusive access.
/// - The structure can be cloned and shared across multiple threads using `Arc`.
/// - Listeners must be thread-safe functions (`Send` and `Sync`).
#[derive(Clone)]
pub struct EventManager<T: Clone + Send + Sync + 'static> {
    listeners: Arc<Mutex<Vec<Box<dyn FnMut(T) + Send + Sync + 'static>>>>,
}

impl<T: Clone + Send + Sync + 'static> EventManager<T> {
    /// Creates a new instance of `EventManager`.
    ///
    /// # Returns
    /// A new, empty instance of `EventManager` ready to register listeners.
    pub fn new() -> Self {
        Self {
            listeners: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Adds an event listener.
    ///
    /// # Arguments
    /// - `listener`: A function or closure that will be called when the event is emitted.
    ///
    /// # Returns
    /// - `Ok(())` if the listener was successfully added.
    /// - `Err(JsError)` if access to the `Mutex` failed.
    ///
    /// # Example
    /// ```rust
    /// let manager = trading_charts::data::events::EventManager::new();
    /// manager
    ///     .add_listener(|event: String| {
    ///         println!("Event received: {}", event);
    ///     })
    ///     .unwrap();
    /// ```
    pub fn add_listener<F: FnMut(T) + Send + Sync + 'static>(&self, listener: F) -> Result<(), JsError> {
        let mut listeners = self
            .listeners
            .lock()
            .map_err(|err| JsError::new_from_str(&err.to_string()))?;

        listeners.push(Box::new(listener));
        Ok(())
    }

    /// Emits an event to all registered listeners.
    ///
    /// # Arguments
    /// - `event_arg`: The event argument that will be passed to each listener.
    ///
    /// # Returns
    /// - `Ok(())` if the event was successfully emitted.
    /// - `Err(JsError)` if access to the `Mutex` failed.
    ///
    /// # Example
    /// ```rust
    /// let manager = trading_charts::data::events::EventManager::new();
    /// manager.emit(String::from("Test Event")).unwrap();
    /// ```
    pub fn emit(&self, event_arg: T) -> Result<(), JsError> {
        let mut listeners = self
            .listeners
            .lock()
            .map_err(|err| JsError::new_from_str(&err.to_string()))?;

        for listener in listeners.iter_mut() {
            listener(event_arg.clone());
        }

        Ok(())
    }
}
