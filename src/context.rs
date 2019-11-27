//! This module contains data about the different temporal contexts that Yew lifecycle events
//! may occur in.

#[derive(Clone, Debug, Copy)]
/// The temporal context that a Component or Service is being instantiated/rendered in.
pub enum RenderingContext {
    /// StaticRenderingPhase is the temporal context representing the SSR aka Static Rendering
    /// rendering target.
    StaticRenderingPhase,

    /// Runtime is the temporal context representing the dynamic (aka non-static)
    /// rendering target.
    Runtime,
}