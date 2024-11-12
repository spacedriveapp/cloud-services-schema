mod client;
mod communication;
mod requests;
mod responses;

/// THE ONE MACRO TO RULE THEM ALL!
#[macro_export]
macro_rules! declare {
	// Use this one for Standalone Services
	(
		$(nested_services = [$($nested_service:tt),* $(,)?] $(,)?)?
		$(children_clients = [$($children_client:tt),* $(,)?] $(,)?)?
		$(rpc = [$($rpc_module:tt),* $(,)?] $(,)?)?
		$(client_stream = [$($client_stream_module:tt),* $(,)?] $(,)?)?
		$(server_stream = [$($server_stream_module:tt),* $(,)?] $(,)?)?
		$(bidirectional_stream = [$($bidirectional_stream_module:tt),* $(,)?] $(,)?)?
	) => {
		$crate::__declare_impl!(
			@root_service,
			$crate::Error,
			nested_services = [$($($nested_service,)*)?],
			children_clients = [$($($children_client,)*)?],
			rpc = [$($($rpc_module,)*)?],
			client_stream = [$($($client_stream_module,)*)?],
			server_stream = [$($($server_stream_module,)*)?],
			bidirectional_stream = [$($($bidirectional_stream_module,)*)?]
		);
	};

	// Use this one for Standalone Services with a custom error
	(
		$(nested_services = [$($nested_service:tt),* $(,)?] $(,)?)?
		$(children_clients = [$($children_client:tt),* $(,)?] $(,)?)?
		$(rpc = [$($rpc_module:tt),* $(,)?] $(,)?)?
		$(client_stream = [$($client_stream_module:tt),* $(,)?] $(,)?)?
		$(server_stream = [$($server_stream_module:tt),* $(,)?] $(,)?)?
		$(bidirectional_stream = [$($bidirectional_stream_module:tt),* $(,)?] $(,)?)?
		custom_error = $error:ty $(,)?
	) => {
		$crate::__declare_impl!(
			@root_service,
			$error,
			nested_services = [$($($nested_service,)*)?],
			children_clients = [$($($children_client,)*)?],
			rpc = [$($($rpc_module,)*)?],
			client_stream = [$($($client_stream_module,)*)?],
			server_stream = [$($($server_stream_module,)*)?],
			bidirectional_stream = [$($($bidirectional_stream_module,)*)?]
		);
	};

	// Use this one for children requests using an existing root service
	(
		parent = $parent:tt,
		$(children_clients = [$($children_client:tt),* $(,)?] $(,)?)?
		$(rpc = [$($rpc_module:tt),* $(,)?] $(,)?)?
		$(client_stream = [$($client_stream_module:tt),* $(,)?] $(,)?)?
		$(server_stream = [$($server_stream_module:tt),* $(,)?] $(,)?)?
		$(bidirectional_stream = [$($bidirectional_stream_module:tt),* $(,)?] $(,)?)?
	) => {
		$crate::__declare_impl!(
			@children,
			$parent,
			$crate::Error,
			children_clients = [$($($children_client,)*)?],
			rpc = [$($($rpc_module,)*)?],
			client_stream = [$($($client_stream_module,)*)?],
			server_stream = [$($($server_stream_module,)*)?],
			bidirectional_stream = [$($($bidirectional_stream_module,)*)?]
		);
	};

	// Use this one for children requests using an existing root service with a custom error
	(
		parent = $parent:tt,
		$(children_clients = [$($children_client:tt),* $(,)?] $(,)?)?
		$(rpc = [$($rpc_module:tt),* $(,)?] $(,)?)?
		$(client_stream = [$($client_stream_module:tt),* $(,)?] $(,)?)?
		$(server_stream = [$($server_stream_module:tt),* $(,)?] $(,)?)?
		$(bidirectional_stream = [$($bidirectional_stream_module:tt),* $(,)?] $(,)?)?
		custom_error = $error:ty $(,)?
	) => {
		$crate::__declare_impl!(
			@children,
			$parent,
			$error,
			children_clients = [$($($children_client,)*)?],
			rpc = [$($($rpc_module,)*)?],
			client_stream = [$($($client_stream_module,)*)?],
			server_stream = [$($($server_stream_module,)*)?],
			bidirectional_stream = [$($($bidirectional_stream_module,)*)?]
		);
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __declare_impl {
	(
		@root_service,
		$error:ty,
		nested_services = [$($nested_service:tt),* $(,)?],
		children_clients = [$($children_client:tt),* $(,)?],
		rpc = [$($rpc_module:tt),* $(,)?],
		client_stream = [$($client_stream_module:tt),* $(,)?],
		server_stream = [$($server_stream_module:tt),* $(,)?],
		bidirectional_stream = [$($bidirectional_stream_module:tt),* $(,)?]
	) => {
		#[derive(Copy, Clone, Debug)]
		pub struct Service;
		impl quic_rpc::Service for Service {
			type Req = Request;
			type Res = Response;
		}

		$crate::__declare_client!(
			Service,
			nested_services = [$($nested_service,)*],
			child_clients = [$($children_client,)*],
		);

		$crate::__declare_requests!(
			parent -> (),
			children = [$($nested_service,)* $($children_client,)*],
			rpc = [$($rpc_module,)*],
			client_stream = [$($client_stream_module,)*],
			server_stream = [$($server_stream_module,)*],
			bidirectional_stream = [$($bidirectional_stream_module,)*],
		);
		$crate::__declare_responses!(
			$error,
			parent -> (),
			children = [$($nested_service,)* $($children_client,)*],
			$($rpc_module,)*
			$($client_stream_module,)*
			$($server_stream_module,)*
			$($bidirectional_stream_module,)*
		);

		$crate::__declare_client_rpc_calls!(
			$error,
			Service,
			rpc = [$($rpc_module,)*],
			client_stream = [$($client_stream_module,)*],
			server_stream = [$($server_stream_module,)*],
			bidirectional_stream = [$($bidirectional_stream_module,)*]
		);

		$crate::__declare_communication!(
			$error,
			Service,
			rpc = [$($rpc_module,)*],
			client_stream = [$($client_stream_module,)*],
			server_stream = [$($server_stream_module,)*],
			bidirectional_stream = [$($bidirectional_stream_module,)*]
		);
	};

	(
		@children,
		$parent:tt,
		$error:ty,
		children_clients = [$($children_client:tt),* $(,)?],
		rpc = [$($rpc_module:tt),* $(,)?],
		client_stream = [$($client_stream_module:tt),* $(,)?],
		server_stream = [$($server_stream_module:tt),* $(,)?],
		bidirectional_stream = [$($bidirectional_stream_module:tt),* $(,)?]
	) => {
		$crate::__declare_client!(
			inner,
			$parent::Service,
			child_clients = [$($children_client,)*]
		);

		$crate::__declare_requests!(
			parent -> ($parent::Request),
			children = [$($children_client,)*],
			rpc = [$($rpc_module,)*],
			client_stream = [$($client_stream_module,)*],
			server_stream = [$($server_stream_module,)*],
			bidirectional_stream = [$($bidirectional_stream_module,)*],
		);
		$crate::__declare_responses!(
			$error,
			parent -> ($parent::Response),
			children = [$($children_client,)*],
			$($rpc_module,)*
			$($client_stream_module,)*
			$($server_stream_module,)*
			$($bidirectional_stream_module,)*
		);

		$crate::__declare_client_rpc_calls!(
			$error,
			parent ->($parent),
			rpc = [$($rpc_module,)*],
			client_stream = [$($client_stream_module,)*],
			server_stream = [$($server_stream_module,)*],
			bidirectional_stream = [$($bidirectional_stream_module,)*]
		);

		$crate::__declare_communication!(
			$error,
			$parent::Service,
			rpc = [$($rpc_module,)*],
			client_stream = [$($client_stream_module,)*],
			server_stream = [$($server_stream_module,)*],
			bidirectional_stream = [$($bidirectional_stream_module,)*]
		);
	};
}

/// Helper macro to implement the [`NeedAuth`] trait for all the requests that need authentication
#[macro_export]
macro_rules! need_auth {
	($($need_auth_mods:tt),+ $(,)?) => {
		$(
			impl $crate::auth::NeedAuth for $need_auth_mods::Request {
				fn access_token(&self) -> &$crate::auth::AccessToken {
					&self.access_token
				}
			}
		)+
	};
}
