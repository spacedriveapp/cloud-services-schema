use serde::{Deserialize, Serialize};

/// ALPN for the Spacedrive Cloud Services - User Server with associated
/// constants for each existing version and an alias for the latest version.
/// This application layer protocol is used when a cloud service needs to communicate back with
/// a user's device, or to devices communicate with each other for a cloud service, like for
/// sending sync keys.
pub struct UserServerALPN;

impl UserServerALPN {
	pub const LATEST: &'static [u8] = Self::V1;
	pub const V1: &'static [u8] = b"sd-user-server/v1";
}

crate::declare!(
	rpc = [authorize_new_device_in_sync_group, reply_sync_group_join],
	custom_error = UserServerError
);

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum UserServerError {
	Rejected,
	UnableToConnect,
}

pub mod authorize_new_device_in_sync_group;
pub mod p2p_authorize_new_device_in_sync_group;
pub mod reply_sync_group_join;
