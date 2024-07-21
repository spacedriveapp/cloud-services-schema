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
