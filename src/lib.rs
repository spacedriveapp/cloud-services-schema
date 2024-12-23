// Re-exporting the opaque-ke crate and NodeId and SecretKey from iroh-base
pub use iroh_base::key::{NodeId, SecretKey};
pub use opaque_ke;

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

/// Service running on the user's side, for cloud assisted p2p communication
pub mod cloud_p2p;

/// Requests and responses for dealing with user's data
pub mod users;

/* API modules END */

/// Auth helper module with a new type for access tokens
pub mod auth;

/// Exporting errors module
pub mod error;
mod macros;

/// A collection of utilities for working with the Spacedrive Cloud Services
///
/// Although they are not part of the API, they can help a lot when working with the services,
/// like routing and middlewares for authentication and error handling
pub mod utils;

/// Re-exporting the paste crate as the router macro needs it
pub use paste;

/// ALPN for the Spacedrive Cloud Services with associated constants for each existing version
/// and an alias for the latest version
pub struct ServicesALPN;

impl ServicesALPN {
	pub const LATEST: &'static [u8] = Self::V1;
	pub const V1: &'static [u8] = b"sd-cloud-services/v1";
}

declare!(
	nested_services = [sync],
	children_clients = [devices, libraries, locations, users],
);

/// A trait abstracting away the declared root clients in this schema
pub trait DeclaredClient<Connection, Service>
where
	Connection: quic_rpc::Connector<Service>,
	Service: quic_rpc::Service,
{
	fn new(client: quic_rpc::RpcClient<Service, Connection>) -> Self;
}

/// The cipher suite for e2e encryption with OPAQUE used by Spacedrive Cloud Services
pub struct SpacedriveCipherSuite;

impl opaque_ke::CipherSuite for SpacedriveCipherSuite {
	type OprfCs = opaque_ke::Ristretto255;
	type KeGroup = opaque_ke::Ristretto255;
	type KeyExchange = opaque_ke::key_exchange::tripledh::TripleDh;
	type Ksf = argon2::Argon2<'static>;
}
