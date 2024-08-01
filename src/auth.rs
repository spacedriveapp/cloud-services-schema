use std::{fmt, future::Future};

use serde::{de::DeserializeOwned, Deserialize, Serialize};

/// Trait to abstract over the need of authentication for a request
pub trait NeedAuth {
	fn access_token(&self) -> &AccessToken;
}

/// Newtype wrapper for the access token
#[derive(Serialize, Clone, Deserialize, Debug)]
#[serde(transparent)]
pub struct AccessToken(pub String);

/// Enum to wrap the possible errors that can happen during authentication, used by
/// the auth middleware
pub enum AuthError<Unauthorized, Internal>
where
	Unauthorized: std::error::Error + Send + Sync + fmt::Debug + 'static,
	Internal: std::error::Error + Send + Sync + fmt::Debug + 'static,
{
	Unauthorized(Unauthorized),
	Internal(Internal),
}

/// Apps that need authentication should implement this trait to be able to use the auth middleware
pub trait AuthableApp<Unauthorized, Internal>: Send
where
	Unauthorized: std::error::Error + Send + Sync + fmt::Debug + 'static,
	Internal: std::error::Error + Send + Sync + fmt::Debug + 'static,
{
	type Claims: DeserializeOwned + Send;
	fn decode(
		&self,
		token: &AccessToken,
	) -> impl Future<Output = Result<Self::Claims, AuthError<Unauthorized, Internal>>> + Send;
}
