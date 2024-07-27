use crate::{auth::ServerSignedToken, devices, sync::groups};

use serde::{Deserialize, Serialize};

use super::UserServerError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
	pub server_signature_token: ServerSignedToken,
	pub sync_group: groups::Group,
	pub response: Result<devices::PubId, UserServerError>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response;
