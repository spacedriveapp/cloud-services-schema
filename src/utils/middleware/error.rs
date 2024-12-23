use crate::Error;

use std::{future::Future, pin::pin};

use async_stream::stream;
use futures_lite::{Stream, StreamExt};
use quic_rpc::{
	message::{BidiStreamingMsg, ClientStreamingMsg, RpcMsg, ServerStreamingMsg},
	server::UpdateStream,
};
use tracing::error;

/// Error handling middleware for RPC requests.
pub async fn rpc<Args, Message, Service, Route, RouteFut, Response>(
	(app, route): (Args, Route),
	req: Message,
) -> Message::Response
where
	Args: Send,
	Message: RpcMsg<Service>,
	Route: FnOnce(Args, Message) -> RouteFut + Send,
	RouteFut: Future<Output = eyre::Result<Message::Response>> + Send,
	Service: quic_rpc::Service,
	Message::Response: From<Result<Response, Error>>,
{
	route(app, req).await.unwrap_or_else(|e| {
		error!(?e);
		Message::Response::from(Err(Error::Server))
	})
}

/// Error handling middleware for Client Streaming requests.
pub async fn client_streaming<Args, Message, Service, Listener, Route, RouteFut, Response>(
	(app, route): (Args, Route),
	req: Message,
	update_stream: UpdateStream<Listener, Message::Update>,
) -> Message::Response
where
	Args: Send,
	Message: ClientStreamingMsg<Service>,
	Route: FnOnce(Args, Message, UpdateStream<Listener, Message::Update>) -> RouteFut + Send,
	RouteFut: Future<Output = eyre::Result<Message::Response>> + Send,
	Service: quic_rpc::Service,
	Listener: quic_rpc::transport::StreamTypes<In = Service::Req, Out = Service::Res>,
	Message::Response: From<Result<Response, Error>>,
{
	route(app, req, update_stream).await.unwrap_or_else(|e| {
		error!(?e);
		Message::Response::from(Err(Error::Server))
	})
}

/// Error handling middleware for Server Streaming requests.
pub fn server_streaming<Args, Message, Service, Route, RouteStream, Response>(
	(args, route): (Args, Route),
	req: Message,
) -> impl Stream<Item = Message::Response> + Send
where
	Args: Send,
	Message: ServerStreamingMsg<Service>,
	Route: FnOnce(Args, Message) -> RouteStream + Send,
	RouteStream: Stream<Item = eyre::Result<Message::Response>> + Send,
	Service: quic_rpc::Service,
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
pub fn bidi_streaming<Args, Message, Service, Listener, Route, RouteStream, Response>(
	(args, route): (Args, Route),
	req: Message,
	update_stream: UpdateStream<Listener, Message::Update>,
) -> impl Stream<Item = Message::Response> + Send
where
	Args: Send,
	Message: BidiStreamingMsg<Service>,
	Route: FnOnce(Args, Message, UpdateStream<Listener, Message::Update>) -> RouteStream + Send,
	RouteStream: Stream<Item = eyre::Result<Message::Response>> + Send,
	Service: quic_rpc::Service,
	Listener: quic_rpc::transport::StreamTypes<In = Service::Req, Out = Service::Res>,
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
