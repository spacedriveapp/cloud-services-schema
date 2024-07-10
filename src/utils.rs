#[macro_export]
macro_rules! declare_client {
	(
		$service:ty
		$(, child_clients = [$($child_module:tt),* $(,)?] $(,)?)?) => {
		#[derive(::std::fmt::Debug, ::std::clone::Clone)]
		pub struct Client<C, S = $service> {
			client: ::quic_rpc::RpcClient<S, C, $service>,
		}

		impl<C, S> Client<C, S>
		where
			C: ::quic_rpc::ServiceConnection<S>,
			S: ::quic_rpc::Service,
		{
			pub const fn new(client: ::quic_rpc::RpcClient<S, C, $service>) -> Self {
				Self { client }
			}


			$($(pub const fn $child_module(&self) -> $child_module::Client<'_, C, S>
			{
				$child_module::Client::new(&self.client)
			})*)?
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
			$($(pub async fn $rpc_module(
				&self,
				req: $rpc_module::Request,
			) -> Result<Result<$rpc_module::Response, $error>, ::quic_rpc::pattern::rpc::Error<C>>
			{
				self.client.rpc(req).await
			})*)?
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

		impl<'c,C, S> Client<'c, C, S>
		where
			C: ::quic_rpc::ServiceConnection<S>,
			S: ::quic_rpc::Service,
		{
			pub const fn new(client: &'c ::quic_rpc::RpcClient<S, C, $service>) -> Self {
				Self { client }
			}

			$(
				$(
					pub async fn $rpc_module(
						&self,
						req: $rpc_module::Request,
					) -> Result<
						Result<$rpc_module::Response, $crate::Error>,
						::quic_rpc::pattern::rpc::Error<C>
					>
					{
						self.client.rpc(req).await
					}
				)+
			)?
		}
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __requests_conversion {
	($module:tt) => {
		paste::paste! {
			impl From<$module::Request> for Request {
				#[allow(unreachable_patterns)]
				fn from(req: $module::Request) -> Self {
					Self::[<$module:camel>](req)
				}
			}

			impl TryFrom<Request> for $module::Request {
				type Error = Request;

				#[allow(unreachable_patterns)]
				fn try_from(req: Request) -> Result<Self, Self::Error> {
					match req {
						Request::[<$module:camel>](req) => Ok(req),
						_ => Err(req),
					}
				}
			}
		}
	};

	($module:tt, $parent:ty) => {
		paste::paste! {
			impl From<$module::Request> for $parent {
				fn from(req: $module::Request) -> Self {
					Request::from(req).into()
				}
			}

			impl TryFrom<$parent> for $module::Request {
				type Error = $parent;

				#[allow(unreachable_patterns)]
				fn try_from(req: $parent) -> Result<Self, Self::Error> {
					match Request::try_from(req) {
						Ok(Request::[<$module:camel>](req)) => Ok(req),
						Ok(x) => Err($parent::from(x)),
						Err(req) => Err(req),
					}
				}
			}
		}
	};
}

#[macro_export]
macro_rules! declare_requests {
	($($module:tt),+ $(,)?) => {
		paste::paste!{
			#[derive(::std::fmt::Debug, ::serde::Serialize, ::serde::Deserialize)]
			pub enum Request {
				$( [<$module:camel>]($module::Request),)*
			}
		}

		$(
			$crate::__requests_conversion!($module);
		)+
	};

	(parent -> $parent:ty, $($module:tt),+ $(,)?) => {
		paste::paste!{
			#[derive(
				::std::fmt::Debug,
				::serde::Serialize,
				::serde::Deserialize
			)]
			pub enum Request {
				$( [<$module:camel>]($module::Request),)*
			}

			$(
				$crate::__requests_conversion!($module);
				$crate::__requests_conversion!($module, $parent);
			)+
		}
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __responses_conversion {
	($module:tt, $error:ty) => {
		paste::paste! {
			impl From<Result<$module::Response, $error>> for Response {
				fn from(res: Result<$module::Response, $error>) -> Self {
					Self::[<$module:camel>](res)
				}
			}

			impl TryFrom<Response> for Result<$module::Response, $error> {
				type Error = Response;

				#[allow(unreachable_patterns)]
				fn try_from(res: Response) -> Result<Self, Self::Error> {
					match res {
						Response::[<$module:camel>](res) => Ok(res),
						_ => Err(res),
					}
				}
			}
		}
	};

	($module:tt, $error:ty, $parent:ty) => {
		paste::paste! {
			impl From<Result<$module::Response, $crate::Error>> for $parent {
				fn from(res: Result<$module::Response, $crate::Error>) -> Self {
					Response::from(res).into()
				}
			}

			impl TryFrom<$parent> for Result<$module::Response, $crate::Error> {
				type Error = $parent;

				#[allow(unreachable_patterns)]
				fn try_from(res: $parent) -> Result<Self, Self::Error> {
					match Response::try_from(res) {
						Ok(Response::[<$module:camel>](res)) => Ok(res),
						Ok(x) => Err($parent::from(x)),
						Err(res) => Err(res),
					}
				}
			}
		}
	};

	(children -> $children_module:tt) => {
		paste::paste! {
			impl From<$children_module::Response> for Response {
				fn from(res: $children_module::Response) -> Self {
					Self::[<$children_module:camel>](res)
				}
			}

			impl TryFrom<Response> for $children_module::Response {
				type Error = Response;

				#[allow(unreachable_patterns)]
				fn try_from(res: Response) -> Result<Self, Self::Error> {
					match res {
						Response::[<$children_module:camel>](res) => Ok(res),
						_ => Err(res),
					}
				}
			}
		}
	};
}

#[macro_export]
macro_rules! declare_responses {
	(
		children_responses = [$($children_module:tt),* $(,)?],
		$($module:tt),* $(,)?
	) => {
		paste::paste!{
			#[derive(::std::fmt::Debug, ::serde::Serialize, ::serde::Deserialize)]
			pub enum Response {
				$( [<$module:camel>](Result<$module::Response, $crate::Error>),)*
				$( [<$children_module:camel>]($children_module::Response),)*
			}

			$(
				$crate::__responses_conversion!($module, $crate::Error);
			)*

			$(
				$crate::__responses_conversion!(children -> $children_module);
			)*
		}
	};

	(
		children_responses = [$($children_module:tt),* $(,)?]
	) => {
		paste::paste!{
			#[derive(::std::fmt::Debug, ::serde::Serialize, ::serde::Deserialize)]
			pub enum Response {
				$( [<$children_module:camel>]($children_module::Response),)*
			}

			$(
				$crate::__responses_conversion!(children -> $children_module);
			)*
		}
	};

	(
		$error:ty,
		$($module:tt),+ $(,)?
	) => {
		paste::paste!{
			#[derive(::std::fmt::Debug, ::serde::Serialize, ::serde::Deserialize)]
			pub enum Response {
				$( [<$module:camel>](Result<$module::Response, $error>),)+
			}

			$(
				$crate::__responses_conversion!($module, $error);
			)*
		}
	};


	(parent -> $parent:ty, $($module:tt),+ $(,)?) => {
		paste::paste!{
			#[derive(::std::fmt::Debug, ::serde::Serialize, ::serde::Deserialize)]
			pub enum Response {
				$( [<$module:camel>](Result<$module::Response, $crate::Error>),)*
			}

			$(
				$crate::__responses_conversion!($module, $crate::Error);
				$crate::__responses_conversion!($module, $crate::Error, $parent);
			)+
		}
	};
}
