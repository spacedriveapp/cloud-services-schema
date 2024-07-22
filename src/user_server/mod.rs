use serde::{Deserialize, Serialize};

crate::declare!(
	rpc = [authorize_new_device_in_sync_group, reply_sync_group_join],
	custom_error = UserServerError
);

#[derive(Debug, Serialize, Deserialize)]
pub enum UserServerError {
	Rejected,
	UnableToConnect,
}

pub mod authorize_new_device_in_sync_group;
pub mod p2p_authorize_new_device_in_sync_group;
pub mod reply_sync_group_join;
