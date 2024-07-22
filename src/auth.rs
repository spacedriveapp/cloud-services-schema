use std::fmt;

use serde::{Deserialize, Serialize};
use zeroize::ZeroizeOnDrop;

#[derive(Serialize, Deserialize)]
#[serde(transparent)]
pub struct AccessToken(pub String);

#[derive(Serialize, Deserialize)]
#[serde(transparent)]
pub struct P2PSignedToken(pub String);

#[derive(Serialize, Deserialize)]
#[serde(transparent)]
pub struct ServerSignedToken(pub String);

#[derive(Serialize, Deserialize)]
#[serde(transparent)]
pub struct DevicePublicKey(pub Vec<u8>);

#[derive(Serialize, Deserialize, ZeroizeOnDrop)]
#[serde(transparent)]
pub struct ServerSecretKey(pub Vec<u8>);

impl fmt::Debug for AccessToken {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "AccessToken(<REDACTED>)")
	}
}

impl fmt::Debug for P2PSignedToken {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "P2PSignedToken(<REDACTED>)")
	}
}

impl fmt::Debug for ServerSignedToken {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "ServerSignedToken(<REDACTED>)")
	}
}

impl fmt::Debug for DevicePublicKey {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "DevicePublicKey(<REDACTED>)")
	}
}

impl fmt::Debug for ServerSecretKey {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "ServerSecretKey(<REDACTED>)")
	}
}
