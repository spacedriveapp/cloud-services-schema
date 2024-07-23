// Re-exporting the opaque-ke crate
pub use iroh_base::ticket::{NodeTicket, Ticket};
pub use opaque_ke;
pub use ed25519_dalek::{VerifyingKey, Verifier};

/// Exporting the error type
pub use error::Error;

/* API modules */

/// Requests and responses for dealing with device's data
pub mod devices;

/// Requests and responses for dealing with user's libraries
pub mod libraries;

/// Requests and responses for dealing with user's locations
pub mod locations;

/// Sync service and it's related requests and responses
pub mod sync;

/// Service running as a server in the user's side
pub mod user_server;

/// Requests and responses for dealing with user's data
pub mod users;

/* API modules END */

/// Auth helper module with a new type for access tokens
pub mod auth;

mod error;
mod macros;

declare!(
	nested_services = [sync],
	children_clients = [devices, libraries, locations, users],
);

/// The cipher suite for e2e encryption with OPAQUE used by Spacedrive Cloud Services
pub struct SpacedriveCipherSuite;

impl opaque_ke::CipherSuite for SpacedriveCipherSuite {
	type OprfCs = opaque_ke::Ristretto255;
	type KeGroup = opaque_ke::Ristretto255;
	type KeyExchange = opaque_ke::key_exchange::tripledh::TripleDh;
	type Ksf = argon2::Argon2<'static>;
}
