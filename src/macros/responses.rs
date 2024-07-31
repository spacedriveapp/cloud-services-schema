#[doc(hidden)]
#[macro_export]
macro_rules! __declare_responses {
	(
		$error:ty,
		parent -> ($($parent:ty)?),
		children = [$($children_module:tt),* $(,)?],
		$($module:tt),* $(,)?
	) => {
		paste::paste!{
			#[derive(::std::fmt::Debug, ::serde::Serialize, ::serde::Deserialize)]
			pub enum Response {
				$( [<$module:camel>](Box<Result<$module::Response, $error>>),)*
				$( [<$children_module:camel>]($children_module::Response),)*
			}

			$crate::__responses_conversion!(
				$error,
				$(parent -> $parent,)?
				$($module,)*
			);

			$(
				$crate::__responses_conversion!(children -> $children_module);
			)*
		}
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __responses_conversion {
	($error:ty, $($module:tt),* $(,)?) => {
		$(
			$crate::__responses_conversion!(@end, $error, $module);
		)*
	};

	(@end, $error:ty, $module:tt $(,)?) => {
		paste::paste! {
			impl From<Result<$module::Response, $error>> for Response {
				fn from(res: Result<$module::Response, $error>) -> Self {
					Self::[<$module:camel>](Box::new(res))
				}
			}

			impl TryFrom<Response> for Result<$module::Response, $error> {
				type Error = Response;

				#[allow(unreachable_patterns)]
				fn try_from(res: Response) -> Result<Self, Self::Error> {
					match res {
						Response::[<$module:camel>](res) => Ok(*res),
						_ => Err(res),
					}
				}
			}
		}
	};

	($error:ty, parent -> $parent:ty, $($module:tt),* $(,)?) => {
		$(
			$crate::__responses_conversion!(@end, $error, parent -> $parent, $module);
		)*
	};

	(@end, $error:ty, parent -> $parent:ty, $module:tt $(,)?) => {
		paste::paste! {
			$crate::__responses_conversion!(@end, $error, $module);

			impl From<Result<$module::Response, $error>> for $parent {
				fn from(res: Result<$module::Response, $error>) -> Self {
					Response::from(res).into()
				}
			}

			impl TryFrom<$parent> for Result<$module::Response, $error> {
				type Error = $parent;

				#[allow(unreachable_patterns)]
				fn try_from(res: $parent) -> Result<Self, Self::Error> {
					match Response::try_from(res) {
						Ok(Response::[<$module:camel>](res)) => Ok(*res),
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
