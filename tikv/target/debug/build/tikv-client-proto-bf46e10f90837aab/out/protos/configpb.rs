#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Status {
    #[prost(enumeration="StatusCode", tag="1")]
    pub code: i32,
    #[prost(string, tag="2")]
    pub message: ::prost::alloc::string::String,
}
/// The version is used to tell the configuration which can be shared
/// or not apart.
/// Global version represents the version of these configuration
/// which can be shared, each kind of component only have one.
/// For local version, every component will have one to represent
/// the version of these configuration which cannot be shared.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Version {
    #[prost(uint64, tag="1")]
    pub local: u64,
    #[prost(uint64, tag="2")]
    pub global: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Local {
    #[prost(string, tag="1")]
    pub component_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Global {
    #[prost(string, tag="1")]
    pub component: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfigKind {
    #[prost(oneof="config_kind::Kind", tags="1, 2")]
    pub kind: ::core::option::Option<config_kind::Kind>,
}
/// Nested message and enum types in `ConfigKind`.
pub mod config_kind {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Kind {
        #[prost(message, tag="1")]
        Local(super::Local),
        #[prost(message, tag="2")]
        Global(super::Global),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfigEntry {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub value: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocalConfig {
    #[prost(message, optional, tag="1")]
    pub version: ::core::option::Option<Version>,
    #[prost(string, tag="2")]
    pub component: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub component_id: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub config: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Header {
    #[prost(uint64, tag="1")]
    pub cluster_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateRequest {
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<Header>,
    #[prost(message, optional, tag="2")]
    pub version: ::core::option::Option<Version>,
    #[prost(string, tag="3")]
    pub component: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub component_id: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub config: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateResponse {
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<Header>,
    #[prost(message, optional, tag="2")]
    pub status: ::core::option::Option<Status>,
    #[prost(message, optional, tag="3")]
    pub version: ::core::option::Option<Version>,
    #[prost(string, tag="4")]
    pub config: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAllRequest {
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<Header>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAllResponse {
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<Header>,
    #[prost(message, optional, tag="2")]
    pub status: ::core::option::Option<Status>,
    #[prost(message, repeated, tag="3")]
    pub local_configs: ::prost::alloc::vec::Vec<LocalConfig>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRequest {
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<Header>,
    #[prost(message, optional, tag="2")]
    pub version: ::core::option::Option<Version>,
    #[prost(string, tag="3")]
    pub component: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub component_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetResponse {
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<Header>,
    #[prost(message, optional, tag="2")]
    pub status: ::core::option::Option<Status>,
    #[prost(message, optional, tag="3")]
    pub version: ::core::option::Option<Version>,
    #[prost(string, tag="4")]
    pub config: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateRequest {
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<Header>,
    #[prost(message, optional, tag="2")]
    pub version: ::core::option::Option<Version>,
    #[prost(message, optional, tag="3")]
    pub kind: ::core::option::Option<ConfigKind>,
    #[prost(message, repeated, tag="4")]
    pub entries: ::prost::alloc::vec::Vec<ConfigEntry>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateResponse {
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<Header>,
    #[prost(message, optional, tag="2")]
    pub status: ::core::option::Option<Status>,
    #[prost(message, optional, tag="3")]
    pub version: ::core::option::Option<Version>,
    #[prost(string, tag="4")]
    pub config: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteRequest {
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<Header>,
    #[prost(message, optional, tag="2")]
    pub version: ::core::option::Option<Version>,
    #[prost(message, optional, tag="3")]
    pub kind: ::core::option::Option<ConfigKind>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteResponse {
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<Header>,
    #[prost(message, optional, tag="2")]
    pub status: ::core::option::Option<Status>,
    #[prost(message, optional, tag="3")]
    pub version: ::core::option::Option<Version>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum StatusCode {
    Unknown = 0,
    Ok = 1,
    WrongVersion = 2,
    NotChange = 3,
    ComponentNotFound = 4,
    ComponentIdNotFound = 5,
}
const METHOD_CONFIG_CREATE: ::grpcio::Method<CreateRequest, CreateResponse> = ::grpcio::Method{ty: ::grpcio::MethodType::Unary, name: "/configpb.Config/Create", req_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, };
const METHOD_CONFIG_GET_ALL: ::grpcio::Method<GetAllRequest, GetAllResponse> = ::grpcio::Method{ty: ::grpcio::MethodType::Unary, name: "/configpb.Config/GetAll", req_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, };
const METHOD_CONFIG_GET: ::grpcio::Method<GetRequest, GetResponse> = ::grpcio::Method{ty: ::grpcio::MethodType::Unary, name: "/configpb.Config/Get", req_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, };
const METHOD_CONFIG_UPDATE: ::grpcio::Method<UpdateRequest, UpdateResponse> = ::grpcio::Method{ty: ::grpcio::MethodType::Unary, name: "/configpb.Config/Update", req_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, };
const METHOD_CONFIG_DELETE: ::grpcio::Method<DeleteRequest, DeleteResponse> = ::grpcio::Method{ty: ::grpcio::MethodType::Unary, name: "/configpb.Config/Delete", req_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, };
#[derive(Clone)]
pub struct ConfigClient { client: ::grpcio::Client }
impl ConfigClient {
pub fn new(channel: ::grpcio::Channel) -> Self { ConfigClient { client: ::grpcio::Client::new(channel) }}
pub fn create_opt(&self, req: &CreateRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<CreateResponse,> { self.client.unary_call(&METHOD_CONFIG_CREATE, req, opt) }
pub fn create(&self, req: &CreateRequest) -> ::grpcio::Result<CreateResponse,> { self.create_opt(req, ::grpcio::CallOption::default()) }
pub fn create_async_opt(&self, req: &CreateRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<CreateResponse>,> { self.client.unary_call_async(&METHOD_CONFIG_CREATE, req, opt) }
pub fn create_async(&self, req: &CreateRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<CreateResponse>,> { self.create_async_opt(req, ::grpcio::CallOption::default()) }
pub fn get_all_opt(&self, req: &GetAllRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<GetAllResponse,> { self.client.unary_call(&METHOD_CONFIG_GET_ALL, req, opt) }
pub fn get_all(&self, req: &GetAllRequest) -> ::grpcio::Result<GetAllResponse,> { self.get_all_opt(req, ::grpcio::CallOption::default()) }
pub fn get_all_async_opt(&self, req: &GetAllRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<GetAllResponse>,> { self.client.unary_call_async(&METHOD_CONFIG_GET_ALL, req, opt) }
pub fn get_all_async(&self, req: &GetAllRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<GetAllResponse>,> { self.get_all_async_opt(req, ::grpcio::CallOption::default()) }
pub fn get_opt(&self, req: &GetRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<GetResponse,> { self.client.unary_call(&METHOD_CONFIG_GET, req, opt) }
pub fn get(&self, req: &GetRequest) -> ::grpcio::Result<GetResponse,> { self.get_opt(req, ::grpcio::CallOption::default()) }
pub fn get_async_opt(&self, req: &GetRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<GetResponse>,> { self.client.unary_call_async(&METHOD_CONFIG_GET, req, opt) }
pub fn get_async(&self, req: &GetRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<GetResponse>,> { self.get_async_opt(req, ::grpcio::CallOption::default()) }
pub fn update_opt(&self, req: &UpdateRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<UpdateResponse,> { self.client.unary_call(&METHOD_CONFIG_UPDATE, req, opt) }
pub fn update(&self, req: &UpdateRequest) -> ::grpcio::Result<UpdateResponse,> { self.update_opt(req, ::grpcio::CallOption::default()) }
pub fn update_async_opt(&self, req: &UpdateRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<UpdateResponse>,> { self.client.unary_call_async(&METHOD_CONFIG_UPDATE, req, opt) }
pub fn update_async(&self, req: &UpdateRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<UpdateResponse>,> { self.update_async_opt(req, ::grpcio::CallOption::default()) }
pub fn delete_opt(&self, req: &DeleteRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<DeleteResponse,> { self.client.unary_call(&METHOD_CONFIG_DELETE, req, opt) }
pub fn delete(&self, req: &DeleteRequest) -> ::grpcio::Result<DeleteResponse,> { self.delete_opt(req, ::grpcio::CallOption::default()) }
pub fn delete_async_opt(&self, req: &DeleteRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<DeleteResponse>,> { self.client.unary_call_async(&METHOD_CONFIG_DELETE, req, opt) }
pub fn delete_async(&self, req: &DeleteRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<DeleteResponse>,> { self.delete_async_opt(req, ::grpcio::CallOption::default()) }
pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Output = ()> + Send + 'static {self.client.spawn(f)}
}
pub trait Config {
fn create(&mut self, ctx: ::grpcio::RpcContext, _req: CreateRequest, sink: ::grpcio::UnarySink<CreateResponse>) { grpcio::unimplemented_call!(ctx, sink) }
fn get_all(&mut self, ctx: ::grpcio::RpcContext, _req: GetAllRequest, sink: ::grpcio::UnarySink<GetAllResponse>) { grpcio::unimplemented_call!(ctx, sink) }
fn get(&mut self, ctx: ::grpcio::RpcContext, _req: GetRequest, sink: ::grpcio::UnarySink<GetResponse>) { grpcio::unimplemented_call!(ctx, sink) }
fn update(&mut self, ctx: ::grpcio::RpcContext, _req: UpdateRequest, sink: ::grpcio::UnarySink<UpdateResponse>) { grpcio::unimplemented_call!(ctx, sink) }
fn delete(&mut self, ctx: ::grpcio::RpcContext, _req: DeleteRequest, sink: ::grpcio::UnarySink<DeleteResponse>) { grpcio::unimplemented_call!(ctx, sink) }
}
pub fn create_config<S: Config + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
let mut builder = ::grpcio::ServiceBuilder::new();
let mut instance = s.clone();
builder = builder.add_unary_handler(&METHOD_CONFIG_CREATE, move |ctx, req, resp| instance.create(ctx, req, resp));
let mut instance = s.clone();
builder = builder.add_unary_handler(&METHOD_CONFIG_GET_ALL, move |ctx, req, resp| instance.get_all(ctx, req, resp));
let mut instance = s.clone();
builder = builder.add_unary_handler(&METHOD_CONFIG_GET, move |ctx, req, resp| instance.get(ctx, req, resp));
let mut instance = s.clone();
builder = builder.add_unary_handler(&METHOD_CONFIG_UPDATE, move |ctx, req, resp| instance.update(ctx, req, resp));
let mut instance = s;
builder = builder.add_unary_handler(&METHOD_CONFIG_DELETE, move |ctx, req, resp| instance.delete(ctx, req, resp));
builder.build()
}
