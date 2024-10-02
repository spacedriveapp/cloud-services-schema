use crate::auth::AccessToken;

use serde::{Deserialize, Serialize};

use super::{Group, GroupWithDevices, PubId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, specta::Type)]
#[specta(rename = "CloudSyncGroupGetRequestKind")]
pub enum RequestKind {
	WithDevices,
	FullData,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Request {
	pub access_token: AccessToken,
	pub pub_id: PubId,
	pub kind: RequestKind,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResponseKind {
	WithDevices(GroupWithDevices),
	FullData(Group),
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Response(pub ResponseKind);

#[cfg(test)]
mod tests {
	use crate::{devices, libraries, sync::KeyHash};

	use bincode::{deserialize, serialize};
	use iroh_base::key::SecretKey;
	use uuid::Uuid;

	use super::*;

	#[test]
	fn test_serialize_request() {
		let request = Request {
			access_token: AccessToken("fake-access-token".to_owned()),
			pub_id: PubId(Uuid::now_v7()),
			kind: RequestKind::WithDevices,
		};

		let serialized = serialize(&request).unwrap();

		assert_eq!(request, deserialize(&serialized).unwrap());
	}

	#[test]
	fn test_serialize_response() {
		let response = Response(ResponseKind::WithDevices(GroupWithDevices {
			pub_id: PubId(Uuid::now_v7()),
			latest_key_hash: KeyHash("fake-key-hash".to_owned()),
			library: libraries::Library {
				pub_id: libraries::PubId(Uuid::now_v7()),
				name: "fake-library-name".to_owned(),
				original_device: None,
				created_at: chrono::Utc::now(),
				updated_at: chrono::Utc::now(),
			},
			devices: vec![devices::Device {
				pub_id: devices::PubId(Uuid::now_v7()),
				name: "fake-name".to_owned(),
				os: devices::DeviceOS::Linux,
				hardware_model: devices::HardwareModel::Other,
				connection_id: SecretKey::generate().public(),
				created_at: chrono::Utc::now(),
				updated_at: chrono::Utc::now(),
			}],
			created_at: chrono::Utc::now(),
			updated_at: chrono::Utc::now(),
		}));

		let serialized = serialize(&response).unwrap();

		assert_eq!(response, deserialize(&serialized).unwrap());
	}
}
