use crate::{auth::AccessToken, sync::groups};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Request {
	pub access_token: AccessToken,
	pub group_pub_id: groups::PubId,
	pub max_allowed_end_time: DateTime<Utc>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Response;
