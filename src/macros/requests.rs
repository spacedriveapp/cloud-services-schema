#[doc(hidden)]
#[macro_export]
macro_rules! __declare_requests {
	(
		parent -> ($($parent:ty)?),
		children = [$($children_module:tt),* $(,)?],
		rpc = [$($rpc_module:tt),* $(,)?],
		client_stream = [$($client_stream_module:tt),* $(,)?],
		server_stream = [$($server_stream_module:tt),* $(,)?],
		bidirectional_stream = [$($bidirectional_stream_module:tt),* $(,)?] $(,)?
	) => {
		::paste::paste!{
			#[derive(
				::std::fmt::Debug,
				::serde::Serialize,
				::serde::Deserialize
			)]
			pub enum Request {
				$( [<$children_module:camel>]($children_module::Request),)*
				$( [<$rpc_module:camel>]($rpc_module::Request),)*
				$(
					[<$client_stream_module:camel>]($client_stream_module::Request),
					[<$client_stream_module:camel Update>]($client_stream_module::RequestUpdate),
				)*
				$( [<$server_stream_module:camel>]($server_stream_module::Request),)*
				$(
					[<$bidirectional_stream_module:camel>]($bidirectional_stream_module::Request),
					[<$bidirectional_stream_module:camel Update>]($bidirectional_stream_module::RequestUpdate),
				)*
			}

			$($crate::__requests_conversion!($children_module);)*
			$($crate::__requests_conversion!($rpc_module);)*
			$($crate::__requests_conversion!($client_stream_module, client_stream);)*
			$($crate::__requests_conversion!($server_stream_module);)*
			$($crate::__requests_conversion!($bidirectional_stream_module, client_stream);)*

			$crate::__internal_requests_conversion_with_parent!(
				$(parent -> $parent,)?
				$($children_module,)* $($rpc_module,)* $($server_stream_module,)*
			);
			$crate::__internal_requests_conversion_with_parent!(
				client_stream,
				$($parent)?,
				$($client_stream_module,)*  $($bidirectional_stream_module,)*
			);
		}
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __requests_conversion {
	($module:tt) => {
		::paste::paste! {
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

	($s_module:tt, client_stream) => {
		::paste::paste! {
			impl From<$s_module::RequestUpdate> for Request {
				#[allow(unreachable_patterns)]
				fn from(req: $s_module::RequestUpdate) -> Self {
					Self::[<$s_module:camel Update>](req)
				}
			}

			impl TryFrom<Request> for $s_module::RequestUpdate {
				type Error = Request;

				#[allow(unreachable_patterns)]
				fn try_from(req: Request) -> Result<Self, Self::Error> {
					match req {
						Request::[<$s_module:camel Update>](req) => Ok(req),
						_ => Err(req),
					}
				}
			}
		}

		$crate::__requests_conversion!($s_module);
	};

	($module:tt, $parent:ty) => {
		::paste::paste! {
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

	($s_module:tt, $parent:ty, client_stream) => {
		::paste::paste! {
		   impl From<$s_module::RequestUpdate> for $parent {
			   fn from(req: $s_module::RequestUpdate) -> Self {
				   Request::from(req).into()
			   }
		   }

		   impl TryFrom<$parent> for $s_module::RequestUpdate {
			   type Error = $parent;

			   #[allow(unreachable_patterns)]
			   fn try_from(req: $parent) -> Result<Self, Self::Error> {
				   match Request::try_from(req) {
					   Ok(Request::[<$s_module:camel Update>](req)) => Ok(req),
					   Ok(x) => Err($parent::from(x)),
					   Err(req) => Err(req),
				   }
			   }
		   }
		}

		$crate::__requests_conversion!($s_module, $parent);
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __internal_requests_conversion_with_parent {
	(parent -> $parent:ty, $($module:tt),* $(,)?) => {
		$($crate::__requests_conversion!($module, $parent);)*
	};

	// No Parent we're good
	($($module:tt),* $(,)?) => {};

	(client_stream, $parent:ty, $($module:tt),* $(,)?) => {
		$($crate::__requests_conversion!($module, $parent, client_stream);)*
	};
}
