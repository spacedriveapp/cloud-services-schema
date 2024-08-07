use crate::auth::AccessToken;

use serde::{Deserialize, Serialize};

use super::PubId;

#[derive(Debug, Serialize, Deserialize, specta::Type)]
#[specta(rename = "LibraryDeleteRequest")]
pub struct Request {
	pub access_token: AccessToken,
	pub pub_id: PubId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response;
