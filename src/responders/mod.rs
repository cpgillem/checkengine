// Resource routes for registers (accounts).
pub mod register;

/// Provides responders for treating users like a resource, mostly
/// for admin activity.
pub mod member;

/// Provides the common routes for login/signup/reset.
pub mod auth;