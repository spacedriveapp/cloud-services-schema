use crate::auth::AccessToken;

use serde::{Deserialize, Serialize};

use super::Device;

#[derive(Debug, Serialize, Deserialize, specta::Type)]
#[specta(rename = "DeviceListRequest")]
pub struct Request {
	pub access_token: AccessToken,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Response(pub Vec<Device>);
