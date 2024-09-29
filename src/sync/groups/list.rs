use crate::auth::AccessToken;

use serde::{Deserialize, Serialize};

use super::GroupBaseData;

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
	pub access_token: AccessToken,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Response(pub Vec<GroupBaseData>);
