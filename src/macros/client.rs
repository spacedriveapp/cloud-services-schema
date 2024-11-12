#[doc(hidden)]
#[macro_export]
macro_rules! __declare_client {
	(
		$service:ty,
		nested_services = [$($nested_module:tt),* $(,)?],
		child_clients = [$($child_module:tt),* $(,)?] $(,)?
	) => {
		#[derive(::std::fmt::Debug, ::std::clone::Clone)]
		pub struct Client<C> {
			$($nested_module: $nested_module::Client<
				::quic_rpc::transport::mapped::MappedConnector<
					<$nested_module::Service as ::quic_rpc::Service>::Res,
					<$nested_module::Service as ::quic_rpc::Service>::Req,
					C,
				>,
			>,)*
			client: ::quic_rpc::RpcClient<$service, C>,
		}

		impl<C> Client<C>
		where
			C: ::quic_rpc::Connector<$service>,
		{
			$crate::__internal_client_new_fn!($service, nested = [$($nested_module),*]);

			$(pub const fn $nested_module(&self) -> &$nested_module::Client<
				::quic_rpc::transport::mapped::MappedConnector<
					<$nested_module::Service as ::quic_rpc::Service>::Res,
					<$nested_module::Service as ::quic_rpc::Service>::Req,
					C,
				>,>
			{
				&self.$nested_module
			})*

			$(pub const fn $child_module(&self) -> $child_module::Client<'_, C>
			{
				$child_module::Client::new(&self.client)
			})*
		}

		impl<C> $crate::DeclaredClient<C, $service> for Client<C>
		where
			C: ::quic_rpc::Connector<$service>,
		{
			fn new(client: ::quic_rpc::RpcClient<$service, C>) -> Self {
				Self::new(client)
			}
		}
	};

	(
		inner,
		$service:ty,
		child_clients = [$($child_module:tt),* $(,)?] $(,)?
	) => {
		#[derive(::std::fmt::Debug, ::std::clone::Clone)]
		pub struct Client<'c, C> {
			client: &'c ::quic_rpc::RpcClient<$service, C>,
		}

		impl<'c, C> Client<'c, C>
		where
			C: ::quic_rpc::Connector<$service>,
		{
			$crate::__internal_client_new_fn!(inner -> $service);

			$(pub const fn $child_module(&self) -> $child_module::Client<'_, C>
			{
				$child_module::Client::new(&self.client)
			})*
		}
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __declare_client_rpc_calls {
	(
		rpc = [$($rpc_module:tt),* $(,)?],
		client_stream = [$($client_stream_module:tt),* $(,)?],
		server_stream = [$($server_stream_module:tt),* $(,)?],
		bidirectional_stream = [$($bidirectional_stream_module:tt),* $(,)?] $(,)?
	) => {
		$crate::__declare_client_rpc_calls!(
			$crate::Error,
			rpc = [$($rpc_module,)*],
			client_stream = [$($client_stream_module,)*],
			server_stream = [$($server_stream_module,)*],
			bidirectional_stream = [$($bidirectional_stream_module,)*],
		);
	};

	(
		parent -> ($parent:tt),
		rpc = [$($rpc_module:tt),* $(,)?],
		client_stream = [$($client_stream_module:tt),* $(,)?],
		server_stream = [$($server_stream_module:tt),* $(,)?],
		bidirectional_stream = [$($bidirectional_stream_module:tt),* $(,)?] $(,)?
	) => {
		$crate::__declare_client_rpc_calls!(
			$crate::Error,
			$parent,
			rpc = [$($rpc_module,)*],
			client_stream = [$($client_stream_module,)*],
			server_stream = [$($server_stream_module,)*],
			bidirectional_stream = [$($bidirectional_stream_module,)*],
		);
	};

	(
		$error:ty,
		$service:ty,
		rpc = [$($rpc_module:tt),* $(,)?],
		client_stream = [$($client_stream_module:tt),* $(,)?],
		server_stream = [$($server_stream_module:tt),* $(,)?],
		bidirectional_stream = [$($bidirectional_stream_module:tt),* $(,)?] $(,)?
	) => {
		impl<C> Client<C>
		where
			C: ::quic_rpc::Connector<$service>,
		{
			$crate::__internal_client_communication_methods!(
				$error,
				S,
				rpc = [$($rpc_module),*],
				client_stream = [$($client_stream_module),*],
				server_stream = [$($server_stream_module),*],
				bidirectional_stream = [$($bidirectional_stream_module),*],
			);
		}
	};

	(
		$error:ty,
		parent -> ($parent:tt),
		$(rpc = [$($rpc_module:tt),* $(,)?] $(,)?)?
		$(client_stream = [$($client_stream_module:tt),* $(,)?] $(,)?)?
		$(server_stream = [$($server_stream_module:tt),* $(,)?] $(,)?)?
		$(bidirectional_stream = [$($bidirectional_stream_module:tt),* $(,)?] $(,)?)?
	) => {

		impl<'c, C> Client<'c, C>
		where
			C: ::quic_rpc::Connector<$parent::Service>,
		{
			$crate::__internal_client_communication_methods!(
				$error,
				$parent::Service,
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
macro_rules! __internal_client_communication_methods {
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
						::quic_rpc::client::UpdateSink<C, $client_stream_module::RequestUpdate>,
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
							C,
							$bidirectional_stream_module::RequestUpdate,
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

#[doc(hidden)]
#[macro_export]
macro_rules! __internal_client_new_fn {
	(inner -> $service:ty) => {
		pub const fn new(client: &'c ::quic_rpc::RpcClient<$service, C>) -> Self {
			Self {
				client,
			}
		}
	};

	($service:ty, nested = [$($nested_module:tt),+ $(,)?]) => {
		pub fn new(client: ::quic_rpc::RpcClient<$service, C>) -> Self {
			Self {
				$($nested_module: $nested_module::Client::new(client.clone().map()),)+
				client,
			}
		}
	};

	// Without nested modules, we can have a `const fn new`
	($service:ty, nested = []) => {
		pub const fn new(client: ::quic_rpc::RpcClient<$service, C>) -> Self {
			Self { client }
		}
	};
}
