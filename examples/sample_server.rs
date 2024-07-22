#![allow(clippy::unused_async, clippy::unwrap_used)]

use std::sync::Arc;

use quic_rpc::{
	RpcClient,
	RpcServer,
	server::{RpcChannel, RpcServerError},
	ServiceEndpoint, transport::ConnectionErrors, transport::flume,
};
use tokio::{spawn, task::JoinHandle};
use uuid::Uuid;

use sd_cloud_schema::{auth::AccessToken, Client, Error, Request, Service, sync};

#[derive(Default)]
pub struct App {
	pub sync: SyncHandler,
}

#[derive(Clone, Copy, Default)]
pub struct SyncHandler {
	groups: SyncGroupsHandler,
}

impl SyncHandler {
	async fn sync_key(
		self,
		sync::get_sync_key::Request {
			access_token: _,
			group_id: _,
		}: sync::get_sync_key::Request,
	) -> Result<sync::get_sync_key::Response, Error> {
		Ok(sync::get_sync_key::Response {
			root_aes_key: vec![],
			encrypted_sync_aes_key: vec![],
		})
	}

	async fn push_sync(
		self,
		sync::messages::push::Request {
			access_token: _,
			group_id: _,
			encrypted_sync_messages: _,
			start_timestamp: _,
			end_timestamp: _,
		}: sync::messages::push::Request,
	) -> Result<sync::messages::push::Response, Error> {
		Ok(sync::messages::push::Response)
	}

	async fn instance_hello(
		self,
		sync::instance_hello::Request {
			access_token: _,
			sd_instance_id: _,
		}: sync::instance_hello::Request,
	) -> Result<sync::instance_hello::Response, Error> {
		Ok(sync::instance_hello::Response)
	}

	async fn handle_rpc_request<S, E>(
		self,
		req: sync::Request,
		chan: RpcChannel<S, E, sync::Service>,
	) -> Result<(), RpcServerError<impl ConnectionErrors>>
	where
		S: quic_rpc::Service,
		E: ServiceEndpoint<S>,
	{
		match req {
			sync::Request::Groups(req) => match req {
				sync::groups::Request::Create(req) => {
					chan.rpc(req, self.groups, SyncGroupsHandler::create_group)
						.await
				}
				_ => unimplemented!(),
			},
			sync::Request::GetSyncKey(req) => chan.rpc(req, self, Self::sync_key).await,
			sync::Request::Messages(req) => match req {
				sync::messages::Request::Push(req) => chan.rpc(req, self, Self::push_sync).await,
			},
			sync::Request::InstanceHello(req) => chan.rpc(req, self, Self::instance_hello).await,
		}
	}
}

#[derive(Clone, Copy, Default)]
pub struct SyncGroupsHandler;

impl SyncGroupsHandler {
	async fn create_group(
		self,
		sync::groups::create::Request {
			access_token: AccessToken(access_token),
			sd_instance_id: _,
			pgp_public_key: _,
			encrypted_sync_aes_key: _,
		}: sync::groups::create::Request,
	) -> Result<sync::groups::create::Response, Error> {
		if access_token == "unauthorized" {
			return Err(Error::Unauthorized);
		}

		Ok(sync::groups::create::Response {
			group_id: Uuid::default(),
			root_aes_key: vec![],
		})
	}
}

impl App {
	pub async fn handle_rpc_request<E: ServiceEndpoint<Service>>(
		&self,
		req: Request,
		chan: RpcChannel<Service, E>,
	) -> Result<(), RpcServerError<impl ConnectionErrors>> {
		match req {
			Request::Sync(req) => self.sync.handle_rpc_request(req, chan.map()).await?,
			_ => unimplemented!(),
		};
		Ok(())
	}
}

pub fn dispatch_server<C: ServiceEndpoint<Service>>(
	server_conn: C,
	handler: App,
) -> JoinHandle<()> {
	let server = RpcServer::new(server_conn);
	let handler = Arc::new(handler);

	spawn(async move {
		loop {
			let Ok(accepting) = server.accept().await else {
				continue;
			};
			match accepting.read_first().await {
				Err(e) => eprintln!("server accept failed: {e:#?}"),
				Ok((req, chan)) => {
					println!("Got request: {req:?}");
					let handler = Arc::clone(&handler);
					spawn(async move {
						if let Err(err) = handler.handle_rpc_request(req, chan).await {
							eprintln!("internal rpc error: {err:#?}");
						}
					});
				}
			}
		}
	})
}

#[tokio::main]
async fn main() {
	let server = App::default();

	let (server_conn, client_conn) = flume::connection::<Service>(1);

	let server_handle = dispatch_server(server_conn, server);

	let client = Client::new(RpcClient::new(client_conn));

	let response = client
		.sync()
		.groups()
		.create(sync::groups::create::Request {
			access_token: AccessToken("unauthorized".to_string()),
			sd_instance_id: Uuid::default(),
			pgp_public_key: vec![],
			encrypted_sync_aes_key: vec![],
		})
		.await
		.unwrap();

	println!("Received response: {response:?}");

	assert!(matches!(response, Err(Error::Unauthorized)));

	server_handle.abort();

	assert!(server_handle.await.is_err_and(|e| e.is_cancelled()));
}
