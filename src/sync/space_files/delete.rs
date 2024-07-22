use crate::{auth::AccessToken, sync::groups};

use serde::{Deserialize, Serialize};

use super::PubId;

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
	pub access_token: AccessToken,
	pub pub_id: PubId,
	pub group_pub_id: groups::PubId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response;
