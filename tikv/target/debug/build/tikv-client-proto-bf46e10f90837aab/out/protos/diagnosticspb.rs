#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchLogRequest {
    #[prost(int64, tag="1")]
    pub start_time: i64,
    #[prost(int64, tag="2")]
    pub end_time: i64,
    #[prost(enumeration="LogLevel", repeated, tag="3")]
    pub levels: ::prost::alloc::vec::Vec<i32>,
    /// We use a string array to represent multiple CNF pattern sceniaor like:
    /// SELECT * FROM t WHERE c LIKE '%s%' and c REGEXP '.*a.*' because
    /// Golang and Rust don't support perl-like (?=re1)(?=re2)
    #[prost(string, repeated, tag="4")]
    pub patterns: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(enumeration="search_log_request::Target", tag="5")]
    pub target: i32,
}
/// Nested message and enum types in `SearchLogRequest`.
pub mod search_log_request {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Target {
        Normal = 0,
        Slow = 1,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchLogResponse {
    #[prost(message, repeated, tag="1")]
    pub messages: ::prost::alloc::vec::Vec<LogMessage>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogMessage {
    #[prost(int64, tag="1")]
    pub time: i64,
    #[prost(enumeration="LogLevel", tag="2")]
    pub level: i32,
    #[prost(string, tag="3")]
    pub message: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerInfoRequest {
    #[prost(enumeration="ServerInfoType", tag="1")]
    pub tp: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerInfoPair {
    #[prost(string, tag="1")]
    pub key: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub value: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerInfoItem {
    /// cpu, memory, disk, network ...
    #[prost(string, tag="1")]
    pub tp: ::prost::alloc::string::String,
    /// eg. network: lo1/eth0, cpu: core1/core2, disk: sda1/sda2 
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    /// all key-value pairs for specified item, e.g:
    /// ServerInfoItem {
    ///     tp = "network"
    ///     name = "eth0"
    ///     paris = [
    ///         ServerInfoPair { key = "readbytes", value = "4k"},
    ///         ServerInfoPair { key = "writebytes", value = "1k"},
    ///     ]
    /// }
    #[prost(message, repeated, tag="3")]
    pub pairs: ::prost::alloc::vec::Vec<ServerInfoPair>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerInfoResponse {
    #[prost(message, repeated, tag="1")]
    pub items: ::prost::alloc::vec::Vec<ServerInfoItem>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LogLevel {
    Unknown = 0,
    Debug = 1,
    Info = 2,
    Warn = 3,
    Trace = 4,
    Critical = 5,
    Error = 6,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ServerInfoType {
    All = 0,
    HardwareInfo = 1,
    SystemInfo = 2,
    LoadInfo = 3,
}
const METHOD_DIAGNOSTICS_SEARCH_LOG: ::grpcio::Method<SearchLogRequest, SearchLogResponse> = ::grpcio::Method{ty: ::grpcio::MethodType::ServerStreaming, name: "/diagnosticspb.Diagnostics/search_log", req_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, };
const METHOD_DIAGNOSTICS_SERVER_INFO: ::grpcio::Method<ServerInfoRequest, ServerInfoResponse> = ::grpcio::Method{ty: ::grpcio::MethodType::Unary, name: "/diagnosticspb.Diagnostics/server_info", req_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, };
#[derive(Clone)]
pub struct DiagnosticsClient { client: ::grpcio::Client }
impl DiagnosticsClient {
pub fn new(channel: ::grpcio::Channel) -> Self { DiagnosticsClient { client: ::grpcio::Client::new(channel) }}
pub fn search_log_opt(&self, req: &SearchLogRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<SearchLogResponse>,> { self.client.server_streaming(&METHOD_DIAGNOSTICS_SEARCH_LOG, req, opt) }
pub fn search_log(&self, req: &SearchLogRequest) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<SearchLogResponse>,> { self.search_log_opt(req, ::grpcio::CallOption::default()) }
pub fn server_info_opt(&self, req: &ServerInfoRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<ServerInfoResponse,> { self.client.unary_call(&METHOD_DIAGNOSTICS_SERVER_INFO, req, opt) }
pub fn server_info(&self, req: &ServerInfoRequest) -> ::grpcio::Result<ServerInfoResponse,> { self.server_info_opt(req, ::grpcio::CallOption::default()) }
pub fn server_info_async_opt(&self, req: &ServerInfoRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<ServerInfoResponse>,> { self.client.unary_call_async(&METHOD_DIAGNOSTICS_SERVER_INFO, req, opt) }
pub fn server_info_async(&self, req: &ServerInfoRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<ServerInfoResponse>,> { self.server_info_async_opt(req, ::grpcio::CallOption::default()) }
pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Output = ()> + Send + 'static {self.client.spawn(f)}
}
pub trait Diagnostics {
fn search_log(&mut self, ctx: ::grpcio::RpcContext, _req: SearchLogRequest, sink: ::grpcio::ServerStreamingSink<SearchLogResponse>) { grpcio::unimplemented_call!(ctx, sink) }
fn server_info(&mut self, ctx: ::grpcio::RpcContext, _req: ServerInfoRequest, sink: ::grpcio::UnarySink<ServerInfoResponse>) { grpcio::unimplemented_call!(ctx, sink) }
}
pub fn create_diagnostics<S: Diagnostics + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
let mut builder = ::grpcio::ServiceBuilder::new();
let mut instance = s.clone();
builder = builder.add_server_streaming_handler(&METHOD_DIAGNOSTICS_SEARCH_LOG, move |ctx, req, resp| instance.search_log(ctx, req, resp));
let mut instance = s;
builder = builder.add_unary_handler(&METHOD_DIAGNOSTICS_SERVER_INFO, move |ctx, req, resp| instance.server_info(ctx, req, resp));
builder.build()
}
