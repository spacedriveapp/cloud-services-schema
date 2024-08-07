use crate::auth::AccessToken;

use serde::{Deserialize, Serialize};

use super::{Library, PubId};

#[derive(Debug, Serialize, Deserialize, specta::Type)]
#[specta(rename = "LibraryGetRequest")]
pub struct Request {
	pub access_token: AccessToken,
	pub pub_id: PubId,
	pub with_device: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Response(pub Library);
