/// Traits & common functions
pub mod utils;

/// Back-end connectors
/// <br>Those components allows reblued to talk directly to a real or an emulated bluetooth controller
pub mod backends;

/// Front-end connectors
/// <br>Those components allows reblued to interface the I/O of the devices connected via bluetooth with the local system
pub mod frontends;

/// D-Bus APIs module
pub mod dbus;

/// `State` module
/// Contains the `State` struct, which is the core element of the software
pub mod state;

fn main() {}
