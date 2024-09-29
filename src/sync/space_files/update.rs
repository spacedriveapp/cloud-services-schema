use crate::{auth::AccessToken, devices, sync::groups};

use serde::{Deserialize, Serialize};
use url::Url;

use super::{FilePathPubId, ObjectPubId};

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
	pub access_token: AccessToken,
	pub object_pub_id: ObjectPubId,
	pub file_path_pub_id: FilePathPubId,
	pub device_pub_id: devices::PubId,
	pub group_pub_id: groups::PubId,
	pub size: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum UpdateKind {
	Ping,
	End,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestUpdate(pub UpdateKind);

#[derive(Debug, Serialize, Deserialize)]
pub enum ResponseKind {
	SinglePresignedUrl(Url),
	ManyPresignedUrls(Vec<Url>),
	Pong,
	End,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response(pub ResponseKind);
