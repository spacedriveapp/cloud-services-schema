#![allow(clippy::unused_async, clippy::unwrap_used)]

use sd_cloud_schema::{
	auth::AccessToken, devices, error::ClientSideError, libraries, sync, Client, Error, Request,
	Response, Service,
};

use std::sync::Arc;

use quic_rpc::{
	server::{RpcChannel, RpcServerError},
	transport::{flume, mapped::MappedStreamTypes, ConnectionErrors},
	Listener, RpcClient, RpcServer,
};
use tokio::{spawn, task::JoinHandle};
use uuid::Uuid;

#[derive(Default)]
pub struct App {
	pub sync: SyncHandler,
}

#[derive(Clone, Copy, Default)]
pub struct SyncHandler {
	groups: SyncGroupsHandler,
}

impl SyncHandler {
	async fn push_sync(
		self,
		sync::messages::push::Request { .. }: sync::messages::push::Request,
	) -> Result<sync::messages::push::Response, Error> {
		Ok(sync::messages::push::Response)
	}

	async fn handle_rpc_request(
		self,
		req: sync::Request,
		chan: RpcChannel<
			sync::Service,
			MappedStreamTypes<sync::Request, sync::Response, impl Listener<Service>>,
		>,
	) -> Result<(), RpcServerError<impl ConnectionErrors>> {
		match req {
			sync::Request::Groups(req) => match req {
				sync::groups::Request::Create(req) => {
					chan.rpc(req, self.groups, SyncGroupsHandler::create_group)
						.await
				}
				sync::groups::Request::Delete(_) => todo!(),
				sync::groups::Request::Get(_) => todo!(),
				sync::groups::Request::Leave(_) => todo!(),
				sync::groups::Request::List(_) => todo!(),
				sync::groups::Request::RemoveDevice(_) => todo!(),
				sync::groups::Request::ReplyJoinRequest(_) => todo!(),
				sync::groups::Request::RequestJoin(_) => todo!(),
			},
			sync::Request::Messages(req) => match req {
				sync::messages::Request::Push(req) => chan.rpc(req, self, Self::push_sync).await,
				sync::messages::Request::DeleteOlder(_) => todo!(),
				sync::messages::Request::GetLatestTime(_) => todo!(),
				sync::messages::Request::Pull(_) => todo!(),
			},
			sync::Request::SpaceFiles(_) => todo!(),
			sync::Request::FindKeyOwners(_) => todo!(),
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
			..
		}: sync::groups::create::Request,
	) -> Result<sync::groups::create::Response, Error> {
		if access_token == "unauthorized" {
			return Err(Error::Client(ClientSideError::Unauthorized));
		}

		Ok(sync::groups::create::Response(sync::groups::PubId(
			Uuid::default(),
		)))
	}
}

impl App {
	pub async fn handle_rpc_request(
		&self,
		req: Request,
		chan: RpcChannel<Service, impl Listener<Service>>,
	) -> Result<(), RpcServerError<impl ConnectionErrors>> {
		match req {
			Request::Sync(req) => {
				let map = chan.map();
				self.sync.handle_rpc_request(req, map).await?
			}
			_ => unimplemented!(),
		};

		Ok(())
	}
}

pub fn dispatch_server(listener: impl Listener<Service>, handler: App) -> JoinHandle<()> {
	let server = RpcServer::new(listener);
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

	let (server_conn, client_conn) = flume::channel::<Request, Response>(1);

	let server_handle = dispatch_server(server_conn, server);

	let client = Client::new(RpcClient::new(client_conn));

	let response = client
		.sync()
		.groups()
		.create(sync::groups::create::Request {
			access_token: AccessToken("unauthorized".to_string()),
			key_hash: sync::KeyHash(String::new()),
			library_pub_id: libraries::PubId(Uuid::default()),
			device_pub_id: devices::PubId(Uuid::default()),
		})
		.await
		.unwrap();

	println!("Received response: {response:?}");

	assert!(matches!(
		response,
		Err(Error::Client(ClientSideError::Unauthorized))
	));

	server_handle.abort();

	assert!(server_handle.await.is_err_and(|e| e.is_cancelled()));
}
