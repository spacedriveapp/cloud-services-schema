#[doc(hidden)]
#[macro_export]
macro_rules! __declare_communication {
	(
		$error:ty,
		$service:ty,
		rpc = [$($rpc_module:tt),* $(,)?],
		client_stream = [$($client_stream_module:tt),* $(,)?],
		server_stream = [$($server_stream_module:tt),* $(,)?],
		bidirectional_stream = [$($bidirectional_stream_module:tt),* $(,)?] $(,)?
	) => {
		$($crate::__rpc!($service, $rpc_module, $error);)*
		$($crate::__client_stream!($service, $client_stream_module, $error);)*
		$($crate::__server_stream!($service, $server_stream_module, $error);)*
		$($crate::__bidirectional_stream!($service, $bidirectional_stream_module, $error);)*
	};
	(
		$service:ty,
		rpc = [$($rpc_module:tt),* $(,)?],
		client_stream = [$($client_stream_module:tt),* $(,)?],
		server_stream = [$($server_stream_module:tt),* $(,)?],
		bidirectional_stream = [$($bidirectional_stream_module:tt),* $(,)?] $(,)?
	) => {
		$crate::__declare_communication!(
			$crate::Error,
			$service,
			rpc = [$($rpc_module,)*],
			client_stream = [$($client_stream_module,)*],
			server_stream = [$($server_stream_module,)*],
			bidirectional_stream = [$($bidirectional_stream_module,)*],
		);
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __rpc {
	($service:ty, $module:tt, $error:ty) => {
		impl ::quic_rpc::message::RpcMsg<$service> for $module::Request {
			type Response = Result<$module::Response, $error>;
		}
	};

	($service:ty, $module:tt) => {
		$crate::__rpc!($service, $module, $crate::Error);
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __client_stream {
	($service:ty, $module:tt, $error:ty) => {
		impl ::quic_rpc::message::Msg<$service> for $module::Request {
			type Pattern = ::quic_rpc::message::ClientStreaming;
		}

		impl ::quic_rpc::message::ClientStreamingMsg<$service> for $module::Request {
			type Update = $module::RequestUpdate;
			type Response = Result<$module::Response, $error>;
		}
	};

	($service:ty, $module:tt) => {
		$crate::__client_stream!($service, $module, $crate::Error);
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __server_stream {
	($service:ty, $module:tt, $error:ty) => {
		impl ::quic_rpc::message::Msg<$service> for $module::Request {
			type Pattern = ::quic_rpc::message::ServerStreaming;
		}

		impl ::quic_rpc::message::ServerStreamingMsg<$service> for $module::Request {
			type Response = Result<$module::Response, $error>;
		}
	};

	($service:ty, $module:tt) => {
		$crate::__server_stream!($service, $module, $crate::Error);
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __bidirectional_stream {
	($service:ty, $module:tt, $error:ty) => {
		impl ::quic_rpc::message::Msg<$service> for $module::Request {
			type Pattern = ::quic_rpc::message::BidiStreaming;
		}

		impl ::quic_rpc::message::BidiStreamingMsg<$service> for $module::Request {
			type Update = $module::RequestUpdate;
			type Response = Result<$module::Response, $error>;
		}
	};

	($service:ty, $module:tt) => {
		$crate::__bidirectional_stream!($service, $module, $crate::Error);
	};
}
