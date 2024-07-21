#![warn(
	clippy::all,
	clippy::pedantic,
	clippy::correctness,
	clippy::perf,
	clippy::style,
	clippy::suspicious,
	clippy::complexity,
	clippy::nursery,
	clippy::unwrap_used,
	unused_qualifications,
	rust_2018_idioms,
	trivial_casts,
	trivial_numeric_casts,
	unused_allocation,
	clippy::unnecessary_cast,
	clippy::cast_lossless,
	clippy::cast_possible_truncation,
	clippy::cast_possible_wrap,
	clippy::cast_precision_loss,
	clippy::cast_sign_loss,
	clippy::dbg_macro,
	clippy::deprecated_cfg_attr,
	clippy::separated_literal_suffix,
	deprecated
)]
#![forbid(deprecated_in_future)]
#![allow(clippy::missing_errors_doc, clippy::module_name_repetitions)]

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

declare_requests!(rpc = [sync, users, devices]);
declare_responses!(children_responses = [sync, users, devices]);

pub struct SpacedriveCipherSuite;

impl opaque_ke::CipherSuite for SpacedriveCipherSuite {
	type OprfCs = opaque_ke::Ristretto255;
	type KeGroup = opaque_ke::Ristretto255;
	type KeyExchange = opaque_ke::key_exchange::tripledh::TripleDh;
	type Ksf = argon2::Argon2<'static>;
}

#[derive(Copy, Clone, Debug)]
pub struct Service;
impl quic_rpc::Service for Service {
	type Req = Request;
	type Res = Response;
}

declare_client!(
	Service,
	nested_service_clients = [sync],
	child_clients = [users, devices]
);
