// Re-exporting the opaque-ke crate
pub use opaque_ke;

pub use error::Error;

pub mod auth;
pub mod devices;
mod error;
mod macros;
pub mod sync;
pub mod user_side;
pub mod users;

declare!(
	nested_services = [sync],
	children_clients = [users, devices]
);

/// The cipher suite for e2e encryption with OPAQUE used by Spacedrive Cloud Services
pub struct SpacedriveCipherSuite;

impl opaque_ke::CipherSuite for SpacedriveCipherSuite {
	type OprfCs = opaque_ke::Ristretto255;
	type KeGroup = opaque_ke::Ristretto255;
	type KeyExchange = opaque_ke::key_exchange::tripledh::TripleDh;
	type Ksf = argon2::Argon2<'static>;
}
