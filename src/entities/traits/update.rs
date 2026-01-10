//! Contains the `Update` entity trait

/// Fires the `update` method, every frame.
pub trait Update {
    /// Fires, every frame.
    /// # Arguements
    /// - `delta`: the time between the last frame and the second to last frame
    fn update(delta: f32);
}
