//! This module contains data and traits useful to the representation of Services.
//! 
//! Services should be instantiated within a component's Component::create() body.

use crate::context::{RenderingContext};

/// Abstract trait representing a Service
pub trait ServiceSpec<T> {
    /// Abstract trait representing a Service's behavior
    fn new(rendering_context: Option<RenderingContext>, params: T) -> Self;
}

/// Represents a Service that behaves differently when rendering with SSR (Sseerver-Side Rendering / Static Rendering)
pub trait SsrAwareServiceBuilder<'a, T = ()> {
    /// The service spec defines how a service behaves after it has been instantiated.
    type ServiceSpec: ServiceSpec<T>;

    /// Creates a Service given a known Rendering Context
    fn create(rendering_context: RenderingContext, params: T) -> Option<Self::ServiceSpec>;
}

/// Service that provides a default create() function, to automate common case where the
/// RenderingContext is irrelevant.
pub trait CommonServiceBuilder<'a, T = ()> {
    /// The service spec defines how a service behaves after it has been instantiated.
    type ServiceSpec: ServiceSpec<T>;

    /// Creates a Service explicitly ignorant of Rendering Context
    fn create(params: T) -> Option<Self::ServiceSpec> {
        // Default behavior is to provide the service regardless of whether rendering
        // on the client side or the server side.

        // we explicitly set the rendering context to be None, since it is acknowledged to
        // be irrelevant here.
        let rc: Option<RenderingContext> = None;
        
        let service = Self::ServiceSpec::new(rc, params);

        return Some(service);
    }
}