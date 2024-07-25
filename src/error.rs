use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{devices, libraries, locations, sync};

#[derive(thiserror::Error, Debug, Serialize, Deserialize)]
pub enum Error {
	#[error(transparent)]
	Client(#[from] ClientSideError),
	#[error("Internal server error")]
	Server,
}

#[derive(thiserror::Error, Debug, Serialize, Deserialize)]
pub enum ClientSideError {
	#[error("Bad request")]
	BadRequest,
	#[error("Unauthorized")]
	Unauthorized,
	#[error("Forbidden")]
	Forbidden,
	#[error(transparent)]
	NotFound(#[from] NotFoundError),
	#[error(transparent)]
	Conflict(#[from] ConflictError),
	#[error(transparent)]
	UserQuotaExceeded(#[from] UserQuotaExceededError),
}

#[derive(thiserror::Error, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub enum NotFoundError {
	#[error("User not found")]
	User(Uuid),
	#[error("Device not found")]
	Device(devices::PubId),
	#[error("Sync group not found")]
	SyncGroup(sync::groups::PubId),
	#[error("Library not found")]
	Library(libraries::PubId),
	#[error("Location not found")]
	Location(locations::PubId),
	#[error("SpaceFile not found")]
	SpaceFile {
		object_pub_id: sync::space_files::ObjectPubId,
		file_path_pub_id: sync::space_files::FilePathPubId,
	},
}

#[derive(thiserror::Error, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub enum ConflictError {
	#[error("User already exists: {0}")]
	User(Uuid),
	#[error("Device already registered: {0}")]
	Device(devices::PubId),
	#[error("Library already exists: {0}")]
	Library(libraries::PubId),
	#[error("Location already exists: {0}")]
	Location(locations::PubId),
	#[error("Sync group already exists: {0}")]
	SyncGroup(sync::groups::PubId),
}

#[derive(thiserror::Error, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub enum UserQuotaExceededError {
	#[error("Not enough storage space: {available} bytes available, {required} required")]
	NotEnoughStorageSpace { available: u64, required: u64 },
}
