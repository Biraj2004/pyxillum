//! # State Listener Interface
//!
//! Provides the observer callback interface through which front ends receive coalesced
//! updates of immutable application state snapshots.

use crate::state::AppState;

/// Observer trait implemented by UI adapters to receive state updates.
pub trait StateListener: Send + Sync + 'static {
    /// Invoked whenever the coalesced application state changes.
    fn on_state_changed(&self, state: &AppState);
}

/// Simple closure-based listener wrapper.
pub struct CallbackListener<F: Fn(&AppState) + Send + Sync + 'static>(pub F);

impl<F: Fn(&AppState) + Send + Sync + 'static> StateListener for CallbackListener<F> {
    fn on_state_changed(&self, state: &AppState) {
        (self.0)(state);
    }
}
