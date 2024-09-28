use crate::auth::AccessToken;

use serde::{Deserialize, Serialize};

use super::Group;

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
	pub access_token: AccessToken,
	pub with_library: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Response(pub Vec<Group>);
