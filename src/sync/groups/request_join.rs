use crate::{auth::AccessToken, devices};

use iroh_base::key::NodeId;
use serde::{Deserialize, Serialize};

use super::PubId;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Request {
	pub access_token: AccessToken,
	pub group_pub_id: PubId,
	pub current_device_pub_id: devices::PubId,
}

/// This response represents all devices in the sync group, along with their [`NodeId`]s.
///
/// So the asking device can ask for himself to join the sync group, sending a P2P message to
/// each device in the sync group.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Response(pub Vec<(devices::PubId, NodeId)>);
