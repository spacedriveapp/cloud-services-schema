use crate::auth::AccessToken;

use serde::{Deserialize, Serialize};

use super::Library;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Request {
	pub access_token: AccessToken,
	pub with_device: bool,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Response(pub Vec<Library>);
