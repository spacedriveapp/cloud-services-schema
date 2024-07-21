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
use quic_rpc::{RpcClient, ServiceConnection};

pub use error::Error;

pub mod auth;
mod error;
pub mod sync;
pub mod user_side;
mod utils;

declare_requests!(sync);
declare_responses!(children_responses = [sync]);

pub struct SpacedriveCipherSuite<'sd> {
	phantom: std::marker::PhantomData<&'sd ()>,
}
impl<'sd> opaque_ke::CipherSuite for SpacedriveCipherSuite<'sd> {
	type OprfCs = opaque_ke::Ristretto255;
	type KeGroup = opaque_ke::Ristretto255;
	type KeyExchange = opaque_ke::key_exchange::tripledh::TripleDh;
	type Ksf = argon2::Argon2<'sd>;
}

#[derive(Copy, Clone, Debug)]
pub struct Service;
impl quic_rpc::Service for Service {
	type Req = Request;
	type Res = Response;
}

#[derive(Debug, Clone)]
pub struct Client<S: quic_rpc::Service, C: ServiceConnection<S>> {
	sync: sync::Client<C, S>,
}

impl<S, C> Client<S, C>
where
	S: quic_rpc::Service,
	C: ServiceConnection<S>,
{
	pub fn new(client: RpcClient<S, C, Service>) -> Self {
		Self {
			sync: sync::Client::new(client.map()),
		}
	}

	pub const fn sync(&self) -> &sync::Client<C, S> {
		&self.sync
	}
}
