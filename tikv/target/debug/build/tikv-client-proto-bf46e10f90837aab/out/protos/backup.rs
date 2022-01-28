/// The message save the metadata of a backup.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BackupMeta {
    /// ID and version of backuped cluster.
    #[prost(uint64, tag="1")]
    pub cluster_id: u64,
    #[prost(string, tag="2")]
    pub cluster_version: ::prost::alloc::string::String,
    /// A set of files that compose a backup.
    #[prost(message, repeated, tag="4")]
    pub files: ::prost::alloc::vec::Vec<File>,
    /// A pair of timestamp specifies a time range of a backup.
    /// For full backup, the start_version equals to the end_version,
    /// it means point in time.
    /// For incremental backup, the time range is specified as
    /// (start_version, end_version].
    #[prost(uint64, tag="5")]
    pub start_version: u64,
    #[prost(uint64, tag="6")]
    pub end_version: u64,
    /// Additional metadata describes database and table info.
    #[prost(message, repeated, tag="7")]
    pub schemas: ::prost::alloc::vec::Vec<Schema>,
    /// If in raw kv mode, `start_versions`, `end_versions` and `schemas` will be ignored, and the
    /// backup data's range is represented by raw_ranges.
    #[prost(bool, tag="8")]
    pub is_raw_kv: bool,
    #[prost(message, repeated, tag="9")]
    pub raw_ranges: ::prost::alloc::vec::Vec<RawRange>,
    /// In incremental backup, DDLs which are completed in (lastBackupTS, backupTS] will be stored here.
    #[prost(bytes="vec", tag="10")]
    pub ddls: ::prost::alloc::vec::Vec<u8>,
    /// Save the version of BR running backup jobs.
    #[prost(string, tag="11")]
    pub br_version: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct File {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="2")]
    pub sha256: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="3")]
    pub start_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="4")]
    pub end_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="5")]
    pub start_version: u64,
    #[prost(uint64, tag="6")]
    pub end_version: u64,
    #[prost(uint64, tag="7")]
    pub crc64xor: u64,
    #[prost(uint64, tag="8")]
    pub total_kvs: u64,
    #[prost(uint64, tag="9")]
    pub total_bytes: u64,
    #[prost(string, tag="10")]
    pub cf: ::prost::alloc::string::String,
    #[prost(uint64, tag="11")]
    pub size: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Schema {
    #[prost(bytes="vec", tag="1")]
    pub db: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub table: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="3")]
    pub crc64xor: u64,
    #[prost(uint64, tag="4")]
    pub total_kvs: u64,
    #[prost(uint64, tag="5")]
    pub total_bytes: u64,
    #[prost(uint32, tag="6")]
    pub tiflash_replicas: u32,
    /// stats represents the dump stats for a analyzed table, which generate by DumpStatsToJSON
    /// https://github.com/pingcap/tidb/blob/e136429d8dc5d70f43cd3f94179b0b9f47595097/statistics/handle/dump.go#L116
    #[prost(bytes="vec", tag="7")]
    pub stats: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RawRange {
    #[prost(bytes="vec", tag="1")]
    pub start_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub end_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="3")]
    pub cf: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClusterIdError {
    #[prost(uint64, tag="1")]
    pub current: u64,
    #[prost(uint64, tag="2")]
    pub request: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Error {
    #[prost(string, tag="1")]
    pub msg: ::prost::alloc::string::String,
    #[prost(oneof="error::Detail", tags="3, 4, 5")]
    pub detail: ::core::option::Option<error::Detail>,
}
/// Nested message and enum types in `Error`.
pub mod error {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Detail {
        #[prost(message, tag="3")]
        ClusterIdError(super::ClusterIdError),
        #[prost(message, tag="4")]
        KvError(super::super::kvrpcpb::KeyError),
        #[prost(message, tag="5")]
        RegionError(super::super::errorpb::Error),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BackupRequest {
    #[prost(uint64, tag="1")]
    pub cluster_id: u64,
    #[prost(bytes="vec", tag="2")]
    pub start_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="3")]
    pub end_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="4")]
    pub start_version: u64,
    #[prost(uint64, tag="5")]
    pub end_version: u64,
    /// The I/O rate limit for backup request.
    #[prost(uint64, tag="7")]
    pub rate_limit: u64,
    /// The concurrency for executing the backup request in every tikv node.
    #[prost(uint32, tag="8")]
    pub concurrency: u32,
    #[prost(message, optional, tag="9")]
    pub storage_backend: ::core::option::Option<StorageBackend>,
    /// If raw kv mode is enabled, `start_version` and `end_version` will be ignored, and `cf`
    /// specifies which cf to backup.
    #[prost(bool, tag="10")]
    pub is_raw_kv: bool,
    #[prost(string, tag="11")]
    pub cf: ::prost::alloc::string::String,
    /// algorithm used for compress sst files
    #[prost(enumeration="CompressionType", tag="12")]
    pub compression_type: i32,
    /// sst compression level, some algorithms support negative compression levels
    #[prost(int32, tag="13")]
    pub compression_level: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StorageBackend {
    #[prost(oneof="storage_backend::Backend", tags="1, 2, 3, 4, 5")]
    pub backend: ::core::option::Option<storage_backend::Backend>,
}
/// Nested message and enum types in `StorageBackend`.
pub mod storage_backend {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Backend {
        #[prost(message, tag="1")]
        Noop(super::Noop),
        #[prost(message, tag="2")]
        Local(super::Local),
        #[prost(message, tag="3")]
        S3(super::S3),
        #[prost(message, tag="4")]
        Gcs(super::Gcs),
        #[prost(message, tag="5")]
        CloudDynamic(super::CloudDynamic),
    }
}
/// Noop storage backend saves files into void.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Noop {
}
/// Local storage backend saves files into local disk
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Local {
    #[prost(string, tag="1")]
    pub path: ::prost::alloc::string::String,
}
/// S3 storage backend saves files into S3 compatible storages
/// For non-aws providers, endpoint must be provided
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct S3 {
    #[prost(string, tag="1")]
    pub endpoint: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub region: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub bucket: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub prefix: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub storage_class: ::prost::alloc::string::String,
    /// server side encryption
    #[prost(string, tag="6")]
    pub sse: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub acl: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub access_key: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub secret_access_key: ::prost::alloc::string::String,
    #[prost(bool, tag="10")]
    pub force_path_style: bool,
    #[prost(string, tag="11")]
    pub sse_kms_key_id: ::prost::alloc::string::String,
}
/// GCS storage backend saves files into google cloud storage.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Gcs {
    #[prost(string, tag="1")]
    pub endpoint: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub bucket: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub prefix: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub storage_class: ::prost::alloc::string::String,
    /// If not empty, applies a predefined set of access controls.
    /// See https://cloud.google.com/storage/docs/access-control/lists#predefined-acl
    /// for valid values.
    #[prost(string, tag="5")]
    pub predefined_acl: ::prost::alloc::string::String,
    /// Service Account Credentials JSON blob
    /// You can get one from https://console.cloud.google.com/apis/credentials, and
    /// copy the content, set it as string here.
    #[prost(string, tag="6")]
    pub credentials_blob: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Bucket {
    #[prost(string, tag="1")]
    pub endpoint: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub region: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub bucket: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub prefix: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub storage_class: ::prost::alloc::string::String,
}
/// CloudDynamic allows testing new cloud providers and new fields without changing protobuf definitions
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloudDynamic {
    #[prost(message, optional, tag="1")]
    pub bucket: ::core::option::Option<Bucket>,
    /// s3 and gcs are supported
    #[prost(string, tag="2")]
    pub provider_name: ::prost::alloc::string::String,
    #[prost(map="string, string", tag="3")]
    pub attrs: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BackupResponse {
    #[prost(message, optional, tag="1")]
    pub error: ::core::option::Option<Error>,
    #[prost(bytes="vec", tag="2")]
    pub start_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="3")]
    pub end_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, repeated, tag="4")]
    pub files: ::prost::alloc::vec::Vec<File>,
}
/// sst files compression algorithm
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CompressionType {
    Unknown = 0,
    Lz4 = 1,
    Snappy = 2,
    Zstd = 3,
}
const METHOD_BACKUP_BACKUP: ::grpcio::Method<BackupRequest, BackupResponse> = ::grpcio::Method{ty: ::grpcio::MethodType::ServerStreaming, name: "/backup.Backup/backup", req_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, };
#[derive(Clone)]
pub struct BackupClient { client: ::grpcio::Client }
impl BackupClient {
pub fn new(channel: ::grpcio::Channel) -> Self { BackupClient { client: ::grpcio::Client::new(channel) }}
pub fn backup_opt(&self, req: &BackupRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<BackupResponse>,> { self.client.server_streaming(&METHOD_BACKUP_BACKUP, req, opt) }
pub fn backup(&self, req: &BackupRequest) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<BackupResponse>,> { self.backup_opt(req, ::grpcio::CallOption::default()) }
pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Output = ()> + Send + 'static {self.client.spawn(f)}
}
pub trait Backup {
fn backup(&mut self, ctx: ::grpcio::RpcContext, _req: BackupRequest, sink: ::grpcio::ServerStreamingSink<BackupResponse>) { grpcio::unimplemented_call!(ctx, sink) }
}
pub fn create_backup<S: Backup + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
let mut builder = ::grpcio::ServiceBuilder::new();
let mut instance = s;
builder = builder.add_server_streaming_handler(&METHOD_BACKUP_BACKUP, move |ctx, req, resp| instance.backup(ctx, req, resp));
builder.build()
}
