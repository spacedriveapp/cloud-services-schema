use crate::{auth::AccessToken, sync::groups};

use serde::{Deserialize, Serialize};

use super::SpaceFile;

pub const BATCH_SIZE: usize = 100;

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
	pub access_token: AccessToken,
	pub group_pub_id: groups::PubId,
	pub with_download_link: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response(pub Vec<SpaceFile>);
