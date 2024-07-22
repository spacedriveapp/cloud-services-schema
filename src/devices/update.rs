use crate::auth::AccessToken;

use iroh_base::ticket::NodeTicket;
use serde::{Deserialize, Serialize};

use super::PubId;

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
	pub access_token: AccessToken,
	pub pub_id: PubId,
	pub name: String,
	pub storage_size: u64,
	pub connection_id: NodeTicket,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response;
