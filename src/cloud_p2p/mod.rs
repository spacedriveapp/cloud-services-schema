use serde::{Deserialize, Serialize};

/// ALPN for the Spacedrive Cloud Services
///
/// Cloud P2P with associated constants for each existing version and an alias for the latest version.
/// This application layer protocol is used when a cloud service needs to devices communicating
/// with each other, like for sending sync keys.
pub struct CloudP2PALPN;

impl CloudP2PALPN {
	pub const LATEST: &'static [u8] = Self::V1;
	pub const V1: &'static [u8] = b"sd-cloud-p2p/v1";
}

crate::declare! {
	rpc = [authorize_new_device_in_sync_group, notify_new_sync_messages, get_thumbnail],
	custom_error = CloudP2PError
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
pub enum CloudP2PError {
	Rejected,
	UnableToConnect,
	TimedOut,
}

pub mod authorize_new_device_in_sync_group;
pub mod get_thumbnail;
pub mod notify_new_sync_messages;
