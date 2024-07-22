use crate::{auth::AccessToken, devices, user_server::UserServerError};

use serde::{Deserialize, Serialize};

use super::PubId;

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
	pub access_token: AccessToken,
	pub pub_id: PubId,
	pub authorized_device_pub_id: devices::PubId,
	pub authorizor_device_pub_id: devices::PubId,
	pub authorizor_response: Result<(), UserServerError>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response;
