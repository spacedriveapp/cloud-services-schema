use crate::{auth::AccessToken, devices, libraries};

use serde::{Deserialize, Serialize};

use super::{KeyHash, PubId};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Request {
	pub access_token: AccessToken,
	pub key_hash: KeyHash,
	pub library_pub_id: libraries::PubId,
	pub device_pub_id: devices::PubId,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Response(pub PubId);
