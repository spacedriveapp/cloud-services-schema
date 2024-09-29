use crate::auth::AccessToken;

use serde::{Deserialize, Serialize};

use super::{Group, GroupWithDevices, PubId};

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[specta(rename = "CloudSyncGroupGetRequestKind")]
pub enum RequestKind {
	WithDevices,
	FullData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
	pub access_token: AccessToken,
	pub pub_id: PubId,
	pub kind: RequestKind,
}

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
#[specta(rename = "CloudSyncGroupGetResponseKind")]
pub enum ResponseKind {
	WithDevices(GroupWithDevices),
	FullData(Group),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Response(pub ResponseKind);
