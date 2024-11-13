/// Macro to generate the route function for a given router module, with middlewares for error
/// handling and authentication if needed
#[allow(clippy::crate_in_macro_def)]
#[macro_export]
macro_rules! router {
	(
		$app:ty,
		service: $service:ty,
        router: $router_module:tt,
		$(authed_routes = {$($auth_route:tt -> $auth_rpc_kind:tt),+ $(,)?} $(,)?)?
        $(routes = {$($route:tt -> $rpc_kind:tt),+ $(,)?} $(,)?)?
    ) => {
		$crate::paste::paste! {
			#[allow(unreachable_patterns, unused)]
			pub async fn route<Listener>(
				app: $app,
				request: $router_module::Request,
				chan: ::quic_rpc::server::RpcChannel<$service, Listener>,
			) -> ::eyre::Result<()>
				where Listener: ::quic_rpc
						::transport
						::StreamTypes<
							In = <$service as ::quic_rpc::Service>::Req,
							Out = <$service as ::quic_rpc::Service>::Res
						>,
			{
				use ::eyre::WrapErr;

				match request {
					$($(
						$router_module::Request::[<$route:camel>](req) => chan
							.$rpc_kind(
								req,
								(app, [<$route:snake>]),
								$crate::utils
									::middleware
									::error
									::$rpc_kind,
							)
							.await
							.wrap_err(concat!(
								"Failed to handle ",
								stringify!([<$route:snake>]),
								" ",
								stringify!([<$router_module:snake>]),
								" request",
							)),
					)+)?
					$($(
						$router_module::Request::[<$auth_route:camel>](req) => {
							chan
								.$auth_rpc_kind(
									req,
									(
										(app, [<$auth_route:snake>]),
										$crate::utils
											::middleware
											::auth
											::$auth_rpc_kind,
									),
									$crate::utils
										::middleware
										::error
										::$auth_rpc_kind,
								)
									.await
									.wrap_err(concat!(
										"Failed to handle authed ",
										stringify!([<$auth_route:snake>]),
										" ",
										stringify!([<$router_module:snake>]),
										" request",
									))
						},
					)+)?
					req => {
						::tracing::warn!(
							request_type = ::std::any::type_name_of_val(&req),
							"Update requests are not allowed as first message of a request"
						);

						Ok(())
					}
				}
			}
		}
	};
}
