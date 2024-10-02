use crate::auth::AccessToken;

use serde::{Deserialize, Serialize};

use super::{Device, PubId};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Request {
	pub access_token: AccessToken,
	pub pub_id: PubId,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Response(pub Device);
