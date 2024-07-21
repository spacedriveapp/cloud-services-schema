#[macro_export]
macro_rules! declare_client {
	(
		$service:ty,
		nested_service_clients = [$($nested_module:tt),* $(,)?] $(,)?
		child_clients = [$($child_module:tt),* $(,)?] $(,)?) => {
		#[derive(::std::fmt::Debug, ::std::clone::Clone)]
		pub struct Client<C, S = $service> {
			$($nested_module: $nested_module::Client<C, S>,)*
			client: ::quic_rpc::RpcClient<S, C, $service>,
		}

		impl<C, S> Client<C, S>
		where
			C: ::quic_rpc::ServiceConnection<S>,
			S: ::quic_rpc::Service,
		{
			$crate::__client_new_fn!($service, $($nested_module),*);

			$(pub const fn $nested_module(&self) -> &$nested_module::Client< C, S>
			{
				&self.$nested_module
			})*

			$(pub const fn $child_module(&self) -> $child_module::Client<'_, C, S>
			{
				$child_module::Client::new(&self.client)
			})*
		}
	};

	($service:ty) => {
		declare_client!($service, nested_service_clients = [], child_clients = []);
	};
	($service:ty, nested_clients = [$($nested_module:tt),* $(,)?] $(,)?) => {
		declare_client!($service, nested_service_clients = [$($nested_module),*], child_clients = []);
	};
	($service:ty, child_clients = [$($child_module:tt),* $(,)?] $(,)?) => {
		declare_client!($service, nested_service_clients = [], child_clients = [$($child_module),*]);
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __client_new_fn {
	($service:ty,) => {
		pub const fn new(client: ::quic_rpc::RpcClient<S, C, $service>) -> Self {
			Self {
				client,
			}
		}
	};

	(inner -> $service:ty) => {
		pub const fn new(client: &'c ::quic_rpc::RpcClient<S, C, $service>) -> Self {
			Self { client }
		}
	};

	($service:ty, $($nested_module:tt),* $(,)?) => {
		pub fn new(client: ::quic_rpc::RpcClient<S, C, $service>) -> Self {
			Self {
				$($nested_module: $nested_module::Client::new(client.clone().map()),)*
				client,
			}
		}
	};
}

#[macro_export]
macro_rules! declare_client_rpc_calls {
	(
		$client:ty,
		$(rpc = [$($rpc_module:tt),* $(,)?] $(,)?)?
		$(client_stream = [$($client_stream_module:tt),* $(,)?] $(,)?)?
		$(server_stream = [$($server_stream_module:tt),* $(,)?] $(,)?)?
		$(bidirectional_stream = [$($bidirectional_stream_module:tt),* $(,)?] $(,)?)?
	) => {
		declare_client_rpc_calls!(
			$client,
			$crate::Error,
			rpc = [$($($rpc_module,)?)+],
			client_stream = [$($($client_stream_module)?)+],
			server_stream = [$($($server_stream_module)?)+],
			bidirectional_stream = [$($($bidirectional_stream_module)?)+],
		);
	};

	(
		$client:ty,
		$error:ty,
		$(rpc = [$($rpc_module:tt),* $(,)?] $(,)?)?
		$(client_stream = [$($client_stream_module:tt),* $(,)?] $(,)?)?
		$(server_stream = [$($server_stream_module:tt),* $(,)?] $(,)?)?
		$(bidirectional_stream = [$($bidirectional_stream_module:tt),* $(,)?] $(,)?)?
	) => {
		impl<C, S> $client
		where
			C: ::quic_rpc::ServiceConnection<S>,
			S: ::quic_rpc::Service,
		{
			$crate::__client_communication_methods!(
				$error,
				S,
				$(rpc = [$($rpc_module),*])?
				$(client_stream = [$($client_stream_module),*])?
				$(server_stream = [$($server_stream_module),*])?
				$(bidirectional_stream = [$($bidirectional_stream_module),*])?
			);
		}
	};
}

#[macro_export]
macro_rules! declare_inner_client_rpc_calls {
	(
		$service:ty,
		$(rpc = [$($rpc_module:tt),* $(,)?] $(,)?)?
		$(client_stream = [$($client_stream_module:tt),* $(,)?] $(,)?)?
		$(server_stream = [$($server_stream_module:tt),* $(,)?] $(,)?)?
		$(bidirectional_stream = [$($bidirectional_stream_module:tt),* $(,)?] $(,)?)?
	) => {
		#[derive(::std::fmt::Debug, ::std::clone::Clone)]
		pub struct Client<'c, C, S = $service> {
			client: &'c ::quic_rpc::RpcClient<S, C, $service>,
		}

		impl<'c, C, S> Client<'c, C, S>
		where
			C: ::quic_rpc::ServiceConnection<S>,
			S: ::quic_rpc::Service,
		{
			$crate::__client_new_fn!(inner -> $service);

			$crate::__client_communication_methods!(
				$crate::Error,
				$service,
				$(rpc = [$($rpc_module),*])?
				$(client_stream = [$($client_stream_module),*])?
				$(server_stream = [$($server_stream_module),*])?
				$(bidirectional_stream = [$($bidirectional_stream_module),*])?
			);
		}
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __client_communication_methods {
	(
		$error:ty,
		$service:ty,
		$(rpc = [$($rpc_module:tt),* $(,)?] $(,)?)?
		$(client_stream = [$($client_stream_module:tt),* $(,)?] $(,)?)?
		$(server_stream = [$($server_stream_module:tt),* $(,)?] $(,)?)?
		$(bidirectional_stream = [$($bidirectional_stream_module:tt),* $(,)?] $(,)?)?
	) => {
		$(
				$(
					pub async fn $rpc_module(
						&self,
						req: $rpc_module::Request,
					) -> Result<
						Result<$rpc_module::Response, $error>,
						::quic_rpc::pattern::rpc::Error<C>
					>
					{
						self.client.rpc(req).await
					}
				)*
			)?

			$(
				$(
					pub async fn $client_stream_module(
						&self,
						req: $client_stream_module::Request,
					) -> Result<
						(
							::quic_rpc::client::UpdateSink<S, C, $client_stream_module::RequestUpdate, $service>,
							::futures_lite::future::Boxed<
								Result<
									Result<$client_stream_module::Response, $error>,
									::quic_rpc::pattern::client_streaming::ItemError<C>,
								>,
							>,
						),
						::quic_rpc::pattern::client_streaming::Error<C>,
					> {
						self.client.client_streaming(req).await
					}
				)*
			)?

			$(
				$(
					pub async fn $server_stream_module(
						&self,
						req: $server_stream_module::Request,
					) -> Result<
						::quic_rpc::client::BoxStreamSync<
							'static,
							Result<
								Result<$server_stream_module::Response, $error>,
								::quic_rpc::pattern::server_streaming::ItemError<C>,
							>,
						>,
						quic_rpc::pattern::server_streaming::Error<C>,
					> {
						self.client.server_streaming(req).await
					}
				)*
			)?

			$(
				$(
					pub async fn $bidirectional_stream_module(
						&self,
						req: $bidirectional_stream_module::Request,
					) -> Result<
						(
							::quic_rpc::client::UpdateSink<
								S,
								C,
								$bidirectional_stream_module::RequestUpdate, $service
							>,
							::quic_rpc::client::BoxStreamSync<
								'static,
								Result<
									Result<$bidirectional_stream_module::Response, $error>,
									::quic_rpc::pattern::bidi_streaming::ItemError<C>,
								>,
							>,
						),
						::quic_rpc::pattern::bidi_streaming::Error<C>,
					> {
						self.client.bidi(req).await
					}
				)*
			)?
	};
}
