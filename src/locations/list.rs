use crate::{auth::AccessToken, libraries};

use serde::{Deserialize, Serialize};

use super::Location;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Request {
	pub access_token: AccessToken,
	pub library_pub_id: libraries::PubId,
	pub with_library: bool,
	pub with_device: bool,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Response(pub Vec<Location>);
