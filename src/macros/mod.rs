mod client;
mod requests;
mod responses;

// FIXME: remove this when quic-rpc fixes their equivalent macro from $service:ident to $service:ty
#[macro_export]
macro_rules! declare_bidi_streaming {
	($service:ty, $m_input:ident, $m_update:ident, $m_output:ident) => {
		impl ::quic_rpc::message::Msg<$service> for $m_input {
			type Pattern = ::quic_rpc::message::BidiStreaming;
		}
		impl ::quic_rpc::message::BidiStreamingMsg<$service> for $m_input {
			type Update = $m_update;
			type Response = $m_output;
		}
	};
}
