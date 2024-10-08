use crate::auth::AccessToken;

use serde::{Deserialize, Serialize};

use super::{Location, PubId};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Request {
	pub access_token: AccessToken,
	pub pub_id: PubId,
	pub with_library: bool,
	pub with_device: bool,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Response(pub Location);
