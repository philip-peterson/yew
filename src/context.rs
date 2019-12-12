//! This module contains data about the different temporal contexts that Yew lifecycle events
//! may occur in.
use std::cell::RefCell;

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

thread_local! {
    /// Specifies whether a render is occuring during the static rendering phase or
    /// the dynamic runtime phase.
    static CURRENT_CONTEXT: RefCell<RenderingContext> = RefCell::new(RenderingContext::Runtime);
}

/// Allows Yew applications to get the CURRENT_CONTEXT
pub fn get_current_rendering_context() -> RenderingContext {
    return CURRENT_CONTEXT.with(|ctx| ctx.borrow().clone());
}

/// Allows Yew to set the CURRENT_CONTEXT
pub(crate) fn set_current_rendering_context(next_ctx: RenderingContext) {
    return CURRENT_CONTEXT.with(|ctx| {
        let mut ctx_ref = ctx.borrow_mut();
        *ctx_ref = next_ctx;
    });
}