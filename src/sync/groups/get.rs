use crate::auth::AccessToken;

use serde::{Deserialize, Serialize};

use super::{Group, GroupWithDevices, PubId};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, specta::Type)]
#[repr(u8)]
#[specta(rename = "CloudSyncGroupGetRequestKind")]
pub enum RequestKind {
	WithDevices = 0,
	FullData = 1,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
	pub access_token: AccessToken,
	pub pub_id: PubId,
	pub kind: RequestKind,
}

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[serde(tag = "kind", content = "data")]
#[specta(rename = "CloudSyncGroupGetResponseKind")]
pub enum ResponseKind {
	WithDevices(GroupWithDevices),
	FullData(Group),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Response(pub ResponseKind);
