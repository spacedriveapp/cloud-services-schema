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
		paste::paste! {
			#[allow(unreachable_patterns, unused)]
			pub async fn route<S, E>(
				app: $app,
				request: $router_module::Request,
				chan: ::quic_rpc::server::RpcChannel<S, E, $service>,
			) -> ::eyre::Result<()>
				where S: ::quic_rpc::Service,
					  E: ::quic_rpc::ServiceEndpoint<S>,
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
									::$rpc_kind::<_, _, S, E, $service, _, _, _>,
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
											::$auth_rpc_kind::<_, _, S, E, $service, _, _, _, _, _>,
									),
									$crate::utils
										::middleware
										::error
										::$auth_rpc_kind::<_, _, S, E, $service, _, _, _>,
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
					_ => {
						unreachable!("Request updates MUST never come first");
					}
				}
			}
		}
	};
}
