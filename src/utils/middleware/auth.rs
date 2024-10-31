use crate::{
	auth::{AuthError, AuthableApp, NeedAuth},
	error::{ClientSideError, Error},
};

use std::{any::type_name, fmt, future::Future, pin::pin};

use async_stream::stream;
use futures_lite::{Stream, StreamExt};
use quic_rpc::{
	message::{BidiStreamingMsg, ClientStreamingMsg, RpcMsg, ServerStreamingMsg},
	server::UpdateStream,
	ServiceEndpoint,
};
use tracing::error;

/// Auth middleware for RPC routes.
pub async fn rpc<
	App,
	Message,
	Service,
	Route,
	RouteFut,
	Response,
	UnauthorizedError,
	InternalError,
>(
	(app, route): (App, Route),
	req: Message,
) -> eyre::Result<Message::Response>
where
	App: AuthableApp<UnauthorizedError, InternalError>,
	Message: RpcMsg<Service> + NeedAuth,
	Route: FnOnce((App, App::Claims), Message) -> RouteFut + Send,
	RouteFut: Future<Output = eyre::Result<Message::Response>> + Send,
	Service: quic_rpc::Service,
	Message::Response: From<Result<Response, Error>>,
	UnauthorizedError: std::error::Error + Send + Sync + fmt::Debug + 'static,
	InternalError: std::error::Error + Send + Sync + fmt::Debug + 'static,
{
	match app.decode(req.access_token()).await {
		Ok(claims) => route((app, claims), req).await,

		Err(AuthError::Unauthorized(e)) => {
			error!(?e, "Unauthorized error on {} rpc", type_name::<Message>());
			Ok(Err(Error::Client(ClientSideError::Unauthorized)).into())
		}

		Err(AuthError::Internal(e)) => {
			error!(
				?e,
				"Internal error processing auth on {} rpc",
				type_name::<Message>()
			);
			Ok(Err(Error::Server).into())
		}
	}
}

/// Auth middleware for Client Streaming routes.
pub async fn client_streaming<
	App,
	Message,
	Service,
	Endpoint,
	ParentService,
	Route,
	RouteFut,
	Response,
	UnauthorizedError,
	InternalError,
>(
	(app, route): (App, Route),
	req: Message,
	update_stream: UpdateStream<ParentService, Endpoint, Message::Update, Service>,
) -> eyre::Result<Message::Response>
where
	App: AuthableApp<UnauthorizedError, InternalError>,
	Message: ClientStreamingMsg<Service> + NeedAuth,
	Route: FnOnce(
			(App, App::Claims),
			Message,
			UpdateStream<ParentService, Endpoint, Message::Update, Service>,
		) -> RouteFut
		+ Send,
	RouteFut: Future<Output = eyre::Result<Message::Response>> + Send,
	Service: quic_rpc::Service,
	ParentService: quic_rpc::Service,
	Endpoint: ServiceEndpoint<ParentService>,
	Message::Response: From<Result<Response, Error>>,
	UnauthorizedError: std::error::Error + Send + Sync + fmt::Debug + 'static,
	InternalError: std::error::Error + Send + Sync + fmt::Debug + 'static,
{
	match app.decode(req.access_token()).await {
		Ok(claims) => route((app, claims), req, update_stream).await,

		Err(AuthError::Unauthorized(e)) => {
			error!(
				?e,
				"Unauthorized error on {} client streaming",
				type_name::<Message>()
			);
			Ok(Err(Error::Client(ClientSideError::Unauthorized)).into())
		}

		Err(AuthError::Internal(e)) => {
			error!(
				?e,
				"Internal error processing auth on {} client streaming",
				type_name::<Message>()
			);
			Ok(Err(Error::Server).into())
		}
	}
}

/// Auth middleware for Server Streaming routes.
pub fn server_streaming<
	App,
	Message,
	Service,
	Route,
	RouteStream,
	Response,
	UnauthorizedError,
	InternalError,
>(
	(app, route): (App, Route),
	req: Message,
) -> impl Stream<Item = eyre::Result<Message::Response>> + Send
where
	App: AuthableApp<UnauthorizedError, InternalError>,
	Message: ServerStreamingMsg<Service> + NeedAuth,
	Route: FnOnce((App, App::Claims), Message) -> RouteStream + Send,
	RouteStream: Stream<Item = eyre::Result<Message::Response>> + Send,
	Service: quic_rpc::Service,
	Message::Response: From<Result<Response, Error>>,
	UnauthorizedError: std::error::Error + Send + Sync + fmt::Debug + 'static,
	InternalError: std::error::Error + Send + Sync + fmt::Debug + 'static,
{
	stream! {
		match app.decode(req.access_token()).await {
			Ok(claims) =>  {
				let mut stream = pin!(route((app, claims), req));
				while let Some(item) = stream.next().await {
					yield item;
				}
			},

			Err(AuthError::Unauthorized(e)) => {
				error!(
					?e,
					"Unauthorized error on {} server streaming",
					type_name::<Message>()
				);
				yield Ok(Err(Error::Client(ClientSideError::Unauthorized)).into())
			},

			Err(AuthError::Internal(e)) => {
				error!(
					?e,
					"Internal error processing auth on {} server streaming",
					type_name::<Message>()
				);
				yield Ok(Err(Error::Server).into())
			}
		}
	}
}

/// Auth middleware for Bidirectional Streaming routes.
pub fn bidi_streaming<
	App,
	Message,
	Service,
	Endpoint,
	ParentService,
	Route,
	RouteStream,
	Response,
	UnauthorizedError,
	InternalError,
>(
	(app, route): (App, Route),
	req: Message,
	update_stream: UpdateStream<ParentService, Endpoint, Message::Update, Service>,
) -> impl Stream<Item = eyre::Result<Message::Response>> + Send
where
	App: AuthableApp<UnauthorizedError, InternalError>,
	Message: BidiStreamingMsg<Service> + NeedAuth,
	Route: FnOnce(
			(App, App::Claims),
			Message,
			UpdateStream<ParentService, Endpoint, Message::Update, Service>,
		) -> RouteStream
		+ Send,
	RouteStream: Stream<Item = eyre::Result<Message::Response>> + Send,
	Service: quic_rpc::Service,
	ParentService: quic_rpc::Service,
	Endpoint: ServiceEndpoint<ParentService>,
	Message::Response: From<Result<Response, Error>>,
	UnauthorizedError: std::error::Error + Send + Sync + fmt::Debug + 'static,
	InternalError: std::error::Error + Send + Sync + fmt::Debug + 'static,
{
	stream! {
		match app.decode(req.access_token()).await {
			Ok(claims) =>  {
				let mut stream = pin!(route((app, claims), req, update_stream));
				while let Some(item) = stream.next().await {
					yield item;
				}
			},

			Err(AuthError::Unauthorized(e)) => {
				error!(
					?e,
					"Unauthorized error on {} bidirectional streaming",
					type_name::<Message>()
				);
				yield Ok(Err(Error::Client(ClientSideError::Unauthorized)).into())
			},

			Err(AuthError::Internal(e)) => {
				error!(
					?e,
					"Internal error processing auth on {} bidirectional streaming",
					type_name::<Message>()
				);
				yield Ok(Err(Error::Server).into())
			}
		}
	}
}
