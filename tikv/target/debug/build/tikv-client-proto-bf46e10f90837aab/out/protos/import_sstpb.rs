#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SwitchModeRequest {
    #[prost(enumeration="SwitchMode", tag="1")]
    pub mode: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SwitchModeResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Range {
    #[prost(bytes="vec", tag="1")]
    pub start: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub end: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SstMeta {
    #[prost(bytes="vec", tag="1")]
    pub uuid: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="2")]
    pub range: ::core::option::Option<Range>,
    #[prost(uint32, tag="3")]
    pub crc32: u32,
    #[prost(uint64, tag="4")]
    pub length: u64,
    #[prost(string, tag="5")]
    pub cf_name: ::prost::alloc::string::String,
    #[prost(uint64, tag="6")]
    pub region_id: u64,
    #[prost(message, optional, tag="7")]
    pub region_epoch: ::core::option::Option<super::metapb::RegionEpoch>,
    #[prost(bool, tag="8")]
    pub end_key_exclusive: bool,
    /// total_kvs and total_bytes is equivalent to PD's approximate_keys and approximate_size
    /// set these values can save time from tikv upload keys and size to PD through Heartbeat.
    #[prost(uint64, tag="9")]
    pub total_kvs: u64,
    #[prost(uint64, tag="10")]
    pub total_bytes: u64,
}
/// A rewrite rule is applied on the *encoded* keys (the internal storage
/// representation).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RewriteRule {
    #[prost(bytes="vec", tag="1")]
    pub old_key_prefix: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub new_key_prefix: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="3")]
    pub new_timestamp: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UploadRequest {
    #[prost(oneof="upload_request::Chunk", tags="1, 2")]
    pub chunk: ::core::option::Option<upload_request::Chunk>,
}
/// Nested message and enum types in `UploadRequest`.
pub mod upload_request {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Chunk {
        #[prost(message, tag="1")]
        Meta(super::SstMeta),
        #[prost(bytes, tag="2")]
        Data(::prost::alloc::vec::Vec<u8>),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UploadResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IngestRequest {
    #[prost(message, optional, tag="1")]
    pub context: ::core::option::Option<super::kvrpcpb::Context>,
    #[prost(message, optional, tag="2")]
    pub sst: ::core::option::Option<SstMeta>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IngestResponse {
    #[prost(message, optional, tag="1")]
    pub error: ::core::option::Option<super::errorpb::Error>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompactRequest {
    /// Compact files in the range and above the output level.
    /// Compact all files if the range is not specified.
    /// Compact all files to the bottommost level if the output level is -1.
    #[prost(message, optional, tag="1")]
    pub range: ::core::option::Option<Range>,
    #[prost(int32, tag="2")]
    pub output_level: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompactResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DownloadRequest {
    /// The SST meta used to identify the downloaded file.
    /// Must be the same among all nodes in the same Raft group.
    /// Note: the "crc32" and "cf_name" fields are ignored in this request,
    /// and the "range" field represents the closed key range after rewrite
    /// (as origin keys in encoded representation).
    #[prost(message, optional, tag="2")]
    pub sst: ::core::option::Option<SstMeta>,
    /// The file name of the SST file.
    #[prost(string, tag="9")]
    pub name: ::prost::alloc::string::String,
    /// Performs a key prefix rewrite after downloading the SST file.
    /// All keys in the SST will be rewritten as:
    ///
    ///  new_key = new_key_prefix + old_key[len(old_key_prefix)..]
    ///
    /// When used for TiDB, rewriting the prefix changes the table ID. Please
    /// note that key-rewrite is applied on the origin keys in encoded
    /// representation (the SST itself should still use data keys in encoded
    /// representation).
    ///
    /// You need to ensure that the keys before and after rewriting are in the
    /// same order, otherwise the RPC request will fail.
    #[prost(message, optional, tag="13")]
    pub rewrite_rule: ::core::option::Option<RewriteRule>,
    #[prost(message, optional, tag="14")]
    pub storage_backend: ::core::option::Option<super::backup::StorageBackend>,
    #[prost(bool, tag="15")]
    pub is_raw_kv: bool,
}
/// For now it is just used for distinguishing the error of the request with the error
/// of gRPC, add more concrete types if it is necessary later.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Error {
    #[prost(string, tag="1")]
    pub message: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DownloadResponse {
    /// The actual key range (after rewrite) of the downloaded SST. The range is
    /// inclusive in both ends.
    #[prost(message, optional, tag="1")]
    pub range: ::core::option::Option<Range>,
    /// Whether the SST is empty. An empty SST is prohibited in TiKV, do not
    /// ingest if this field is true.
    /// (Deprecated, should be replaced by checking `length == 0` in the future)
    #[prost(bool, tag="2")]
    pub is_empty: bool,
    #[prost(message, optional, tag="3")]
    pub error: ::core::option::Option<Error>,
    /// The CRC32 checksum of the rewritten SST file (implementation can return
    /// zero, indicating the CRC32 was not calculated).
    #[prost(uint32, tag="4")]
    pub crc32: u32,
    /// The actual length of the rewritten SST file.
    #[prost(uint64, tag="5")]
    pub length: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetDownloadSpeedLimitRequest {
    /// The download speed limit (bytes/second). Set to 0 for unlimited speed.
    #[prost(uint64, tag="1")]
    pub speed_limit: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetDownloadSpeedLimitResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pair {
    #[prost(bytes="vec", tag="1")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub value: ::prost::alloc::vec::Vec<u8>,
    #[prost(enumeration="pair::Op", tag="3")]
    pub op: i32,
}
/// Nested message and enum types in `Pair`.
pub mod pair {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Op {
        Put = 0,
        Delete = 1,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WriteBatch {
    #[prost(uint64, tag="1")]
    pub commit_ts: u64,
    #[prost(message, repeated, tag="2")]
    pub pairs: ::prost::alloc::vec::Vec<Pair>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WriteRequest {
    #[prost(oneof="write_request::Chunk", tags="1, 2")]
    pub chunk: ::core::option::Option<write_request::Chunk>,
}
/// Nested message and enum types in `WriteRequest`.
pub mod write_request {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Chunk {
        #[prost(message, tag="1")]
        Meta(super::SstMeta),
        #[prost(message, tag="2")]
        Batch(super::WriteBatch),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WriteResponse {
    #[prost(message, optional, tag="1")]
    pub error: ::core::option::Option<Error>,
    #[prost(message, repeated, tag="2")]
    pub metas: ::prost::alloc::vec::Vec<SstMeta>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SwitchMode {
    Normal = 0,
    Import = 1,
}
const METHOD_IMPORT_SST_SWITCH_MODE: ::grpcio::Method<SwitchModeRequest, SwitchModeResponse> = ::grpcio::Method{ty: ::grpcio::MethodType::Unary, name: "/import_sstpb.ImportSST/SwitchMode", req_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, };
const METHOD_IMPORT_SST_UPLOAD: ::grpcio::Method<UploadRequest, UploadResponse> = ::grpcio::Method{ty: ::grpcio::MethodType::ClientStreaming, name: "/import_sstpb.ImportSST/Upload", req_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, };
const METHOD_IMPORT_SST_INGEST: ::grpcio::Method<IngestRequest, IngestResponse> = ::grpcio::Method{ty: ::grpcio::MethodType::Unary, name: "/import_sstpb.ImportSST/Ingest", req_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, };
const METHOD_IMPORT_SST_COMPACT: ::grpcio::Method<CompactRequest, CompactResponse> = ::grpcio::Method{ty: ::grpcio::MethodType::Unary, name: "/import_sstpb.ImportSST/Compact", req_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, };
const METHOD_IMPORT_SST_SET_DOWNLOAD_SPEED_LIMIT: ::grpcio::Method<SetDownloadSpeedLimitRequest, SetDownloadSpeedLimitResponse> = ::grpcio::Method{ty: ::grpcio::MethodType::Unary, name: "/import_sstpb.ImportSST/SetDownloadSpeedLimit", req_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, };
const METHOD_IMPORT_SST_DOWNLOAD: ::grpcio::Method<DownloadRequest, DownloadResponse> = ::grpcio::Method{ty: ::grpcio::MethodType::Unary, name: "/import_sstpb.ImportSST/Download", req_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, };
const METHOD_IMPORT_SST_WRITE: ::grpcio::Method<WriteRequest, WriteResponse> = ::grpcio::Method{ty: ::grpcio::MethodType::ClientStreaming, name: "/import_sstpb.ImportSST/Write", req_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, };
#[derive(Clone)]
pub struct ImportSstClient { client: ::grpcio::Client }
impl ImportSstClient {
pub fn new(channel: ::grpcio::Channel) -> Self { ImportSstClient { client: ::grpcio::Client::new(channel) }}
pub fn switch_mode_opt(&self, req: &SwitchModeRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<SwitchModeResponse,> { self.client.unary_call(&METHOD_IMPORT_SST_SWITCH_MODE, req, opt) }
pub fn switch_mode(&self, req: &SwitchModeRequest) -> ::grpcio::Result<SwitchModeResponse,> { self.switch_mode_opt(req, ::grpcio::CallOption::default()) }
pub fn switch_mode_async_opt(&self, req: &SwitchModeRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<SwitchModeResponse>,> { self.client.unary_call_async(&METHOD_IMPORT_SST_SWITCH_MODE, req, opt) }
pub fn switch_mode_async(&self, req: &SwitchModeRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<SwitchModeResponse>,> { self.switch_mode_async_opt(req, ::grpcio::CallOption::default()) }
pub fn upload_opt(&self, opt: ::grpcio::CallOption) -> ::grpcio::Result<(::grpcio::ClientCStreamSender<UploadRequest>,::grpcio::ClientCStreamReceiver<UploadResponse>,)> { self.client.client_streaming(&METHOD_IMPORT_SST_UPLOAD, opt) }
pub fn upload(&self) -> ::grpcio::Result<(::grpcio::ClientCStreamSender<UploadRequest>,::grpcio::ClientCStreamReceiver<UploadResponse>,)> { self.upload_opt(::grpcio::CallOption::default()) }
pub fn ingest_opt(&self, req: &IngestRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<IngestResponse,> { self.client.unary_call(&METHOD_IMPORT_SST_INGEST, req, opt) }
pub fn ingest(&self, req: &IngestRequest) -> ::grpcio::Result<IngestResponse,> { self.ingest_opt(req, ::grpcio::CallOption::default()) }
pub fn ingest_async_opt(&self, req: &IngestRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<IngestResponse>,> { self.client.unary_call_async(&METHOD_IMPORT_SST_INGEST, req, opt) }
pub fn ingest_async(&self, req: &IngestRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<IngestResponse>,> { self.ingest_async_opt(req, ::grpcio::CallOption::default()) }
pub fn compact_opt(&self, req: &CompactRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<CompactResponse,> { self.client.unary_call(&METHOD_IMPORT_SST_COMPACT, req, opt) }
pub fn compact(&self, req: &CompactRequest) -> ::grpcio::Result<CompactResponse,> { self.compact_opt(req, ::grpcio::CallOption::default()) }
pub fn compact_async_opt(&self, req: &CompactRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<CompactResponse>,> { self.client.unary_call_async(&METHOD_IMPORT_SST_COMPACT, req, opt) }
pub fn compact_async(&self, req: &CompactRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<CompactResponse>,> { self.compact_async_opt(req, ::grpcio::CallOption::default()) }
pub fn set_download_speed_limit_opt(&self, req: &SetDownloadSpeedLimitRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<SetDownloadSpeedLimitResponse,> { self.client.unary_call(&METHOD_IMPORT_SST_SET_DOWNLOAD_SPEED_LIMIT, req, opt) }
pub fn set_download_speed_limit(&self, req: &SetDownloadSpeedLimitRequest) -> ::grpcio::Result<SetDownloadSpeedLimitResponse,> { self.set_download_speed_limit_opt(req, ::grpcio::CallOption::default()) }
pub fn set_download_speed_limit_async_opt(&self, req: &SetDownloadSpeedLimitRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<SetDownloadSpeedLimitResponse>,> { self.client.unary_call_async(&METHOD_IMPORT_SST_SET_DOWNLOAD_SPEED_LIMIT, req, opt) }
pub fn set_download_speed_limit_async(&self, req: &SetDownloadSpeedLimitRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<SetDownloadSpeedLimitResponse>,> { self.set_download_speed_limit_async_opt(req, ::grpcio::CallOption::default()) }
pub fn download_opt(&self, req: &DownloadRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<DownloadResponse,> { self.client.unary_call(&METHOD_IMPORT_SST_DOWNLOAD, req, opt) }
pub fn download(&self, req: &DownloadRequest) -> ::grpcio::Result<DownloadResponse,> { self.download_opt(req, ::grpcio::CallOption::default()) }
pub fn download_async_opt(&self, req: &DownloadRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<DownloadResponse>,> { self.client.unary_call_async(&METHOD_IMPORT_SST_DOWNLOAD, req, opt) }
pub fn download_async(&self, req: &DownloadRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<DownloadResponse>,> { self.download_async_opt(req, ::grpcio::CallOption::default()) }
pub fn write_opt(&self, opt: ::grpcio::CallOption) -> ::grpcio::Result<(::grpcio::ClientCStreamSender<WriteRequest>,::grpcio::ClientCStreamReceiver<WriteResponse>,)> { self.client.client_streaming(&METHOD_IMPORT_SST_WRITE, opt) }
pub fn write(&self) -> ::grpcio::Result<(::grpcio::ClientCStreamSender<WriteRequest>,::grpcio::ClientCStreamReceiver<WriteResponse>,)> { self.write_opt(::grpcio::CallOption::default()) }
pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Output = ()> + Send + 'static {self.client.spawn(f)}
}
pub trait ImportSst {
fn switch_mode(&mut self, ctx: ::grpcio::RpcContext, _req: SwitchModeRequest, sink: ::grpcio::UnarySink<SwitchModeResponse>) { grpcio::unimplemented_call!(ctx, sink) }
fn upload(&mut self, ctx: ::grpcio::RpcContext, _stream: ::grpcio::RequestStream<UploadRequest>, sink: ::grpcio::ClientStreamingSink<UploadResponse>) { grpcio::unimplemented_call!(ctx, sink) }
fn ingest(&mut self, ctx: ::grpcio::RpcContext, _req: IngestRequest, sink: ::grpcio::UnarySink<IngestResponse>) { grpcio::unimplemented_call!(ctx, sink) }
fn compact(&mut self, ctx: ::grpcio::RpcContext, _req: CompactRequest, sink: ::grpcio::UnarySink<CompactResponse>) { grpcio::unimplemented_call!(ctx, sink) }
fn set_download_speed_limit(&mut self, ctx: ::grpcio::RpcContext, _req: SetDownloadSpeedLimitRequest, sink: ::grpcio::UnarySink<SetDownloadSpeedLimitResponse>) { grpcio::unimplemented_call!(ctx, sink) }
fn download(&mut self, ctx: ::grpcio::RpcContext, _req: DownloadRequest, sink: ::grpcio::UnarySink<DownloadResponse>) { grpcio::unimplemented_call!(ctx, sink) }
fn write(&mut self, ctx: ::grpcio::RpcContext, _stream: ::grpcio::RequestStream<WriteRequest>, sink: ::grpcio::ClientStreamingSink<WriteResponse>) { grpcio::unimplemented_call!(ctx, sink) }
}
pub fn create_import_sst<S: ImportSst + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
let mut builder = ::grpcio::ServiceBuilder::new();
let mut instance = s.clone();
builder = builder.add_unary_handler(&METHOD_IMPORT_SST_SWITCH_MODE, move |ctx, req, resp| instance.switch_mode(ctx, req, resp));
let mut instance = s.clone();
builder = builder.add_client_streaming_handler(&METHOD_IMPORT_SST_UPLOAD, move |ctx, req, resp| instance.upload(ctx, req, resp));
let mut instance = s.clone();
builder = builder.add_unary_handler(&METHOD_IMPORT_SST_INGEST, move |ctx, req, resp| instance.ingest(ctx, req, resp));
let mut instance = s.clone();
builder = builder.add_unary_handler(&METHOD_IMPORT_SST_COMPACT, move |ctx, req, resp| instance.compact(ctx, req, resp));
let mut instance = s.clone();
builder = builder.add_unary_handler(&METHOD_IMPORT_SST_SET_DOWNLOAD_SPEED_LIMIT, move |ctx, req, resp| instance.set_download_speed_limit(ctx, req, resp));
let mut instance = s.clone();
builder = builder.add_unary_handler(&METHOD_IMPORT_SST_DOWNLOAD, move |ctx, req, resp| instance.download(ctx, req, resp));
let mut instance = s;
builder = builder.add_client_streaming_handler(&METHOD_IMPORT_SST_WRITE, move |ctx, req, resp| instance.write(ctx, req, resp));
builder.build()
}
