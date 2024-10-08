use crate::Error;

use std::{future::Future, pin::pin};

use async_stream::stream;
use futures_lite::{Stream, StreamExt};
use quic_rpc::{
	message::{BidiStreamingMsg, ClientStreamingMsg, RpcMsg, ServerStreamingMsg},
	server::UpdateStream,
	ServiceEndpoint,
};
use tracing::error;

/// Error handling middleware for RPC requests.
pub async fn rpc<Args, Message, Service, Endpoint, InnerService, Route, RouteFut, Response>(
	(app, route): (Args, Route),
	req: Message,
) -> Message::Response
where
	Args: Send,
	Message: RpcMsg<InnerService>,
	Route: FnOnce(Args, Message) -> RouteFut + Send,
	RouteFut: Future<Output = eyre::Result<Message::Response>> + Send,
	Service: quic_rpc::Service,
	Endpoint: ServiceEndpoint<Service>,
	InnerService: quic_rpc::Service,
	Message::Response: From<Result<Response, Error>>,
{
	route(app, req).await.unwrap_or_else(|e| {
		error!(?e);
		Message::Response::from(Err(Error::Server))
	})
}

/// Error handling middleware for Client Streaming requests.
pub async fn client_streaming<
	Args,
	Message,
	Service,
	Endpoint,
	InnerService,
	Route,
	RouteFut,
	Response,
>(
	(app, route): (Args, Route),
	req: Message,
	update_stream: UpdateStream<Service, Endpoint, Message::Update, InnerService>,
) -> Message::Response
where
	Args: Send,
	Message: ClientStreamingMsg<InnerService>,
	Route: FnOnce(
			Args,
			Message,
			UpdateStream<Service, Endpoint, Message::Update, InnerService>,
		) -> RouteFut
		+ Send,
	RouteFut: Future<Output = eyre::Result<Message::Response>> + Send,
	Service: quic_rpc::Service,
	Endpoint: ServiceEndpoint<Service>,
	InnerService: quic_rpc::Service,
	Message::Response: From<Result<Response, Error>>,
{
	route(app, req, update_stream).await.unwrap_or_else(|e| {
		error!(?e);
		Message::Response::from(Err(Error::Server))
	})
}

/// Error handling middleware for Server Streaming requests.
pub fn server_streaming<
	Args,
	Message,
	Service,
	Endpoint,
	InnerService,
	Route,
	RouteStream,
	Response,
>(
	(args, route): (Args, Route),
	req: Message,
) -> impl Stream<Item = Message::Response> + Send
where
	Args: Send,
	Message: ServerStreamingMsg<InnerService>,
	Route: FnOnce(Args, Message) -> RouteStream + Send,
	RouteStream: Stream<Item = eyre::Result<Message::Response>> + Send,
	Service: quic_rpc::Service,
	Endpoint: ServiceEndpoint<Service>,
	InnerService: quic_rpc::Service,
	Message::Response: From<Result<Response, Error>>,
{
	stream! {
		let mut stream = pin!(route(args, req));
		while let Some(item) = stream.next().await {
			match item {
				Ok(item) => yield item,
				Err(e) => {
					error!(?e);
					yield Message::Response::from(Err(Error::Server));
					break;
				}
			}
		}
	}
}

/// Error handling middleware for Bidirectional Streaming requests.
pub fn bidi_streaming<
	Args,
	Message,
	Service,
	Endpoint,
	InnerService,
	Route,
	RouteStream,
	Response,
>(
	(args, route): (Args, Route),
	req: Message,
	update_stream: UpdateStream<Service, Endpoint, Message::Update, InnerService>,
) -> impl Stream<Item = Message::Response> + Send
where
	Args: Send,
	Message: BidiStreamingMsg<InnerService>,
	Route: FnOnce(
			Args,
			Message,
			UpdateStream<Service, Endpoint, Message::Update, InnerService>,
		) -> RouteStream
		+ Send,
	RouteStream: Stream<Item = eyre::Result<Message::Response>> + Send,
	Service: quic_rpc::Service,
	Endpoint: ServiceEndpoint<Service>,
	InnerService: quic_rpc::Service,
	Message::Response: From<Result<Response, Error>>,
{
	stream! {
		let mut stream = pin!(route(args, req, update_stream));
		while let Some(item) = stream.next().await {
			match item {
				Ok(item) => yield item,
				Err(e) => {
					error!(?e);
					yield Message::Response::from(Err(Error::Server));
					break;
				}
			}
		}
	}
}
