#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRequest {
    #[prost(enumeration="Db", tag="1")]
    pub db: i32,
    #[prost(string, tag="2")]
    pub cf: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="3")]
    pub key: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetResponse {
    #[prost(bytes="vec", tag="1")]
    pub value: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RaftLogRequest {
    #[prost(uint64, tag="1")]
    pub region_id: u64,
    #[prost(uint64, tag="2")]
    pub log_index: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RaftLogResponse {
    #[prost(message, optional, tag="1")]
    pub entry: ::core::option::Option<super::eraftpb::Entry>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegionInfoRequest {
    #[prost(uint64, tag="1")]
    pub region_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegionInfoResponse {
    #[prost(message, optional, tag="1")]
    pub raft_local_state: ::core::option::Option<super::raft_serverpb::RaftLocalState>,
    #[prost(message, optional, tag="2")]
    pub raft_apply_state: ::core::option::Option<super::raft_serverpb::RaftApplyState>,
    #[prost(message, optional, tag="3")]
    pub region_local_state: ::core::option::Option<super::raft_serverpb::RegionLocalState>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegionSizeRequest {
    #[prost(uint64, tag="1")]
    pub region_id: u64,
    #[prost(string, repeated, tag="2")]
    pub cfs: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegionSizeResponse {
    #[prost(message, repeated, tag="1")]
    pub entries: ::prost::alloc::vec::Vec<region_size_response::Entry>,
}
/// Nested message and enum types in `RegionSizeResponse`.
pub mod region_size_response {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Entry {
        #[prost(string, tag="1")]
        pub cf: ::prost::alloc::string::String,
        #[prost(uint64, tag="2")]
        pub size: u64,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScanMvccRequest {
    #[prost(bytes="vec", tag="1")]
    pub from_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub to_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="3")]
    pub limit: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScanMvccResponse {
    #[prost(bytes="vec", tag="1")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="2")]
    pub info: ::core::option::Option<super::kvrpcpb::MvccInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompactRequest {
    #[prost(enumeration="Db", tag="1")]
    pub db: i32,
    #[prost(string, tag="2")]
    pub cf: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="3")]
    pub from_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="4")]
    pub to_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, tag="5")]
    pub threads: u32,
    #[prost(enumeration="BottommostLevelCompaction", tag="6")]
    pub bottommost_level_compaction: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompactResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InjectFailPointRequest {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub actions: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InjectFailPointResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecoverFailPointRequest {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecoverFailPointResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListFailPointsRequest {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListFailPointsResponse {
    #[prost(message, repeated, tag="1")]
    pub entries: ::prost::alloc::vec::Vec<list_fail_points_response::Entry>,
}
/// Nested message and enum types in `ListFailPointsResponse`.
pub mod list_fail_points_response {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Entry {
        #[prost(string, tag="1")]
        pub name: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub actions: ::prost::alloc::string::String,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMetricsRequest {
    #[prost(bool, tag="1")]
    pub all: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMetricsResponse {
    #[prost(string, tag="1")]
    pub prometheus: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub rocksdb_kv: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub rocksdb_raft: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub jemalloc: ::prost::alloc::string::String,
    #[prost(uint64, tag="5")]
    pub store_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegionConsistencyCheckRequest {
    #[prost(uint64, tag="1")]
    pub region_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegionConsistencyCheckResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModifyTikvConfigRequest {
    #[prost(enumeration="Module", tag="1")]
    pub module: i32,
    #[prost(string, tag="2")]
    pub config_name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub config_value: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModifyTikvConfigResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Property {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub value: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRegionPropertiesRequest {
    #[prost(uint64, tag="1")]
    pub region_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRegionPropertiesResponse {
    #[prost(message, repeated, tag="1")]
    pub props: ::prost::alloc::vec::Vec<Property>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetStoreInfoRequest {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetStoreInfoResponse {
    #[prost(uint64, tag="1")]
    pub store_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetClusterInfoRequest {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetClusterInfoResponse {
    #[prost(uint64, tag="1")]
    pub cluster_id: u64,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Db {
    Invalid = 0,
    Kv = 1,
    Raft = 2,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Module {
    Unused = 0,
    Kvdb = 1,
    Raftdb = 2,
    Readpool = 3,
    Server = 4,
    Storage = 5,
    Pd = 6,
    Metric = 7,
    Coprocessor = 8,
    Security = 9,
    Import = 10,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum BottommostLevelCompaction {
    /// Skip bottommost level compaction
    Skip = 0,
    /// Force bottommost level compaction
    Force = 1,
    /// Compact bottommost level if there is a compaction filter.
    IfHaveCompactionFilter = 2,
}
const METHOD_DEBUG_GET: ::grpcio::Method<GetRequest, GetResponse> = ::grpcio::Method{ty: ::grpcio::MethodType::Unary, name: "/debugpb.Debug/Get", req_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, };
const METHOD_DEBUG_RAFT_LOG: ::grpcio::Method<RaftLogRequest, RaftLogResponse> = ::grpcio::Method{ty: ::grpcio::MethodType::Unary, name: "/debugpb.Debug/RaftLog", req_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, };
const METHOD_DEBUG_REGION_INFO: ::grpcio::Method<RegionInfoRequest, RegionInfoResponse> = ::grpcio::Method{ty: ::grpcio::MethodType::Unary, name: "/debugpb.Debug/RegionInfo", req_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, };
const METHOD_DEBUG_REGION_SIZE: ::grpcio::Method<RegionSizeRequest, RegionSizeResponse> = ::grpcio::Method{ty: ::grpcio::MethodType::Unary, name: "/debugpb.Debug/RegionSize", req_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, };
const METHOD_DEBUG_SCAN_MVCC: ::grpcio::Method<ScanMvccRequest, ScanMvccResponse> = ::grpcio::Method{ty: ::grpcio::MethodType::ServerStreaming, name: "/debugpb.Debug/ScanMvcc", req_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, };
const METHOD_DEBUG_COMPACT: ::grpcio::Method<CompactRequest, CompactResponse> = ::grpcio::Method{ty: ::grpcio::MethodType::Unary, name: "/debugpb.Debug/Compact", req_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, };
const METHOD_DEBUG_INJECT_FAIL_POINT: ::grpcio::Method<InjectFailPointRequest, InjectFailPointResponse> = ::grpcio::Method{ty: ::grpcio::MethodType::Unary, name: "/debugpb.Debug/InjectFailPoint", req_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, };
const METHOD_DEBUG_RECOVER_FAIL_POINT: ::grpcio::Method<RecoverFailPointRequest, RecoverFailPointResponse> = ::grpcio::Method{ty: ::grpcio::MethodType::Unary, name: "/debugpb.Debug/RecoverFailPoint", req_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, };
const METHOD_DEBUG_LIST_FAIL_POINTS: ::grpcio::Method<ListFailPointsRequest, ListFailPointsResponse> = ::grpcio::Method{ty: ::grpcio::MethodType::Unary, name: "/debugpb.Debug/ListFailPoints", req_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, };
const METHOD_DEBUG_GET_METRICS: ::grpcio::Method<GetMetricsRequest, GetMetricsResponse> = ::grpcio::Method{ty: ::grpcio::MethodType::Unary, name: "/debugpb.Debug/GetMetrics", req_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, };
const METHOD_DEBUG_CHECK_REGION_CONSISTENCY: ::grpcio::Method<RegionConsistencyCheckRequest, RegionConsistencyCheckResponse> = ::grpcio::Method{ty: ::grpcio::MethodType::Unary, name: "/debugpb.Debug/CheckRegionConsistency", req_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, };
const METHOD_DEBUG_MODIFY_TIKV_CONFIG: ::grpcio::Method<ModifyTikvConfigRequest, ModifyTikvConfigResponse> = ::grpcio::Method{ty: ::grpcio::MethodType::Unary, name: "/debugpb.Debug/ModifyTikvConfig", req_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, };
const METHOD_DEBUG_GET_REGION_PROPERTIES: ::grpcio::Method<GetRegionPropertiesRequest, GetRegionPropertiesResponse> = ::grpcio::Method{ty: ::grpcio::MethodType::Unary, name: "/debugpb.Debug/GetRegionProperties", req_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, };
const METHOD_DEBUG_GET_STORE_INFO: ::grpcio::Method<GetStoreInfoRequest, GetStoreInfoResponse> = ::grpcio::Method{ty: ::grpcio::MethodType::Unary, name: "/debugpb.Debug/GetStoreInfo", req_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, };
const METHOD_DEBUG_GET_CLUSTER_INFO: ::grpcio::Method<GetClusterInfoRequest, GetClusterInfoResponse> = ::grpcio::Method{ty: ::grpcio::MethodType::Unary, name: "/debugpb.Debug/GetClusterInfo", req_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, };
#[derive(Clone)]
pub struct DebugClient { client: ::grpcio::Client }
impl DebugClient {
pub fn new(channel: ::grpcio::Channel) -> Self { DebugClient { client: ::grpcio::Client::new(channel) }}
pub fn get_opt(&self, req: &GetRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<GetResponse,> { self.client.unary_call(&METHOD_DEBUG_GET, req, opt) }
pub fn get(&self, req: &GetRequest) -> ::grpcio::Result<GetResponse,> { self.get_opt(req, ::grpcio::CallOption::default()) }
pub fn get_async_opt(&self, req: &GetRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<GetResponse>,> { self.client.unary_call_async(&METHOD_DEBUG_GET, req, opt) }
pub fn get_async(&self, req: &GetRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<GetResponse>,> { self.get_async_opt(req, ::grpcio::CallOption::default()) }
pub fn raft_log_opt(&self, req: &RaftLogRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<RaftLogResponse,> { self.client.unary_call(&METHOD_DEBUG_RAFT_LOG, req, opt) }
pub fn raft_log(&self, req: &RaftLogRequest) -> ::grpcio::Result<RaftLogResponse,> { self.raft_log_opt(req, ::grpcio::CallOption::default()) }
pub fn raft_log_async_opt(&self, req: &RaftLogRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<RaftLogResponse>,> { self.client.unary_call_async(&METHOD_DEBUG_RAFT_LOG, req, opt) }
pub fn raft_log_async(&self, req: &RaftLogRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<RaftLogResponse>,> { self.raft_log_async_opt(req, ::grpcio::CallOption::default()) }
pub fn region_info_opt(&self, req: &RegionInfoRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<RegionInfoResponse,> { self.client.unary_call(&METHOD_DEBUG_REGION_INFO, req, opt) }
pub fn region_info(&self, req: &RegionInfoRequest) -> ::grpcio::Result<RegionInfoResponse,> { self.region_info_opt(req, ::grpcio::CallOption::default()) }
pub fn region_info_async_opt(&self, req: &RegionInfoRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<RegionInfoResponse>,> { self.client.unary_call_async(&METHOD_DEBUG_REGION_INFO, req, opt) }
pub fn region_info_async(&self, req: &RegionInfoRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<RegionInfoResponse>,> { self.region_info_async_opt(req, ::grpcio::CallOption::default()) }
pub fn region_size_opt(&self, req: &RegionSizeRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<RegionSizeResponse,> { self.client.unary_call(&METHOD_DEBUG_REGION_SIZE, req, opt) }
pub fn region_size(&self, req: &RegionSizeRequest) -> ::grpcio::Result<RegionSizeResponse,> { self.region_size_opt(req, ::grpcio::CallOption::default()) }
pub fn region_size_async_opt(&self, req: &RegionSizeRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<RegionSizeResponse>,> { self.client.unary_call_async(&METHOD_DEBUG_REGION_SIZE, req, opt) }
pub fn region_size_async(&self, req: &RegionSizeRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<RegionSizeResponse>,> { self.region_size_async_opt(req, ::grpcio::CallOption::default()) }
pub fn scan_mvcc_opt(&self, req: &ScanMvccRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<ScanMvccResponse>,> { self.client.server_streaming(&METHOD_DEBUG_SCAN_MVCC, req, opt) }
pub fn scan_mvcc(&self, req: &ScanMvccRequest) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<ScanMvccResponse>,> { self.scan_mvcc_opt(req, ::grpcio::CallOption::default()) }
pub fn compact_opt(&self, req: &CompactRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<CompactResponse,> { self.client.unary_call(&METHOD_DEBUG_COMPACT, req, opt) }
pub fn compact(&self, req: &CompactRequest) -> ::grpcio::Result<CompactResponse,> { self.compact_opt(req, ::grpcio::CallOption::default()) }
pub fn compact_async_opt(&self, req: &CompactRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<CompactResponse>,> { self.client.unary_call_async(&METHOD_DEBUG_COMPACT, req, opt) }
pub fn compact_async(&self, req: &CompactRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<CompactResponse>,> { self.compact_async_opt(req, ::grpcio::CallOption::default()) }
pub fn inject_fail_point_opt(&self, req: &InjectFailPointRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<InjectFailPointResponse,> { self.client.unary_call(&METHOD_DEBUG_INJECT_FAIL_POINT, req, opt) }
pub fn inject_fail_point(&self, req: &InjectFailPointRequest) -> ::grpcio::Result<InjectFailPointResponse,> { self.inject_fail_point_opt(req, ::grpcio::CallOption::default()) }
pub fn inject_fail_point_async_opt(&self, req: &InjectFailPointRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<InjectFailPointResponse>,> { self.client.unary_call_async(&METHOD_DEBUG_INJECT_FAIL_POINT, req, opt) }
pub fn inject_fail_point_async(&self, req: &InjectFailPointRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<InjectFailPointResponse>,> { self.inject_fail_point_async_opt(req, ::grpcio::CallOption::default()) }
pub fn recover_fail_point_opt(&self, req: &RecoverFailPointRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<RecoverFailPointResponse,> { self.client.unary_call(&METHOD_DEBUG_RECOVER_FAIL_POINT, req, opt) }
pub fn recover_fail_point(&self, req: &RecoverFailPointRequest) -> ::grpcio::Result<RecoverFailPointResponse,> { self.recover_fail_point_opt(req, ::grpcio::CallOption::default()) }
pub fn recover_fail_point_async_opt(&self, req: &RecoverFailPointRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<RecoverFailPointResponse>,> { self.client.unary_call_async(&METHOD_DEBUG_RECOVER_FAIL_POINT, req, opt) }
pub fn recover_fail_point_async(&self, req: &RecoverFailPointRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<RecoverFailPointResponse>,> { self.recover_fail_point_async_opt(req, ::grpcio::CallOption::default()) }
pub fn list_fail_points_opt(&self, req: &ListFailPointsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<ListFailPointsResponse,> { self.client.unary_call(&METHOD_DEBUG_LIST_FAIL_POINTS, req, opt) }
pub fn list_fail_points(&self, req: &ListFailPointsRequest) -> ::grpcio::Result<ListFailPointsResponse,> { self.list_fail_points_opt(req, ::grpcio::CallOption::default()) }
pub fn list_fail_points_async_opt(&self, req: &ListFailPointsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<ListFailPointsResponse>,> { self.client.unary_call_async(&METHOD_DEBUG_LIST_FAIL_POINTS, req, opt) }
pub fn list_fail_points_async(&self, req: &ListFailPointsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<ListFailPointsResponse>,> { self.list_fail_points_async_opt(req, ::grpcio::CallOption::default()) }
pub fn get_metrics_opt(&self, req: &GetMetricsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<GetMetricsResponse,> { self.client.unary_call(&METHOD_DEBUG_GET_METRICS, req, opt) }
pub fn get_metrics(&self, req: &GetMetricsRequest) -> ::grpcio::Result<GetMetricsResponse,> { self.get_metrics_opt(req, ::grpcio::CallOption::default()) }
pub fn get_metrics_async_opt(&self, req: &GetMetricsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<GetMetricsResponse>,> { self.client.unary_call_async(&METHOD_DEBUG_GET_METRICS, req, opt) }
pub fn get_metrics_async(&self, req: &GetMetricsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<GetMetricsResponse>,> { self.get_metrics_async_opt(req, ::grpcio::CallOption::default()) }
pub fn check_region_consistency_opt(&self, req: &RegionConsistencyCheckRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<RegionConsistencyCheckResponse,> { self.client.unary_call(&METHOD_DEBUG_CHECK_REGION_CONSISTENCY, req, opt) }
pub fn check_region_consistency(&self, req: &RegionConsistencyCheckRequest) -> ::grpcio::Result<RegionConsistencyCheckResponse,> { self.check_region_consistency_opt(req, ::grpcio::CallOption::default()) }
pub fn check_region_consistency_async_opt(&self, req: &RegionConsistencyCheckRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<RegionConsistencyCheckResponse>,> { self.client.unary_call_async(&METHOD_DEBUG_CHECK_REGION_CONSISTENCY, req, opt) }
pub fn check_region_consistency_async(&self, req: &RegionConsistencyCheckRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<RegionConsistencyCheckResponse>,> { self.check_region_consistency_async_opt(req, ::grpcio::CallOption::default()) }
pub fn modify_tikv_config_opt(&self, req: &ModifyTikvConfigRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<ModifyTikvConfigResponse,> { self.client.unary_call(&METHOD_DEBUG_MODIFY_TIKV_CONFIG, req, opt) }
pub fn modify_tikv_config(&self, req: &ModifyTikvConfigRequest) -> ::grpcio::Result<ModifyTikvConfigResponse,> { self.modify_tikv_config_opt(req, ::grpcio::CallOption::default()) }
pub fn modify_tikv_config_async_opt(&self, req: &ModifyTikvConfigRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<ModifyTikvConfigResponse>,> { self.client.unary_call_async(&METHOD_DEBUG_MODIFY_TIKV_CONFIG, req, opt) }
pub fn modify_tikv_config_async(&self, req: &ModifyTikvConfigRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<ModifyTikvConfigResponse>,> { self.modify_tikv_config_async_opt(req, ::grpcio::CallOption::default()) }
pub fn get_region_properties_opt(&self, req: &GetRegionPropertiesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<GetRegionPropertiesResponse,> { self.client.unary_call(&METHOD_DEBUG_GET_REGION_PROPERTIES, req, opt) }
pub fn get_region_properties(&self, req: &GetRegionPropertiesRequest) -> ::grpcio::Result<GetRegionPropertiesResponse,> { self.get_region_properties_opt(req, ::grpcio::CallOption::default()) }
pub fn get_region_properties_async_opt(&self, req: &GetRegionPropertiesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<GetRegionPropertiesResponse>,> { self.client.unary_call_async(&METHOD_DEBUG_GET_REGION_PROPERTIES, req, opt) }
pub fn get_region_properties_async(&self, req: &GetRegionPropertiesRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<GetRegionPropertiesResponse>,> { self.get_region_properties_async_opt(req, ::grpcio::CallOption::default()) }
pub fn get_store_info_opt(&self, req: &GetStoreInfoRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<GetStoreInfoResponse,> { self.client.unary_call(&METHOD_DEBUG_GET_STORE_INFO, req, opt) }
pub fn get_store_info(&self, req: &GetStoreInfoRequest) -> ::grpcio::Result<GetStoreInfoResponse,> { self.get_store_info_opt(req, ::grpcio::CallOption::default()) }
pub fn get_store_info_async_opt(&self, req: &GetStoreInfoRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<GetStoreInfoResponse>,> { self.client.unary_call_async(&METHOD_DEBUG_GET_STORE_INFO, req, opt) }
pub fn get_store_info_async(&self, req: &GetStoreInfoRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<GetStoreInfoResponse>,> { self.get_store_info_async_opt(req, ::grpcio::CallOption::default()) }
pub fn get_cluster_info_opt(&self, req: &GetClusterInfoRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<GetClusterInfoResponse,> { self.client.unary_call(&METHOD_DEBUG_GET_CLUSTER_INFO, req, opt) }
pub fn get_cluster_info(&self, req: &GetClusterInfoRequest) -> ::grpcio::Result<GetClusterInfoResponse,> { self.get_cluster_info_opt(req, ::grpcio::CallOption::default()) }
pub fn get_cluster_info_async_opt(&self, req: &GetClusterInfoRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<GetClusterInfoResponse>,> { self.client.unary_call_async(&METHOD_DEBUG_GET_CLUSTER_INFO, req, opt) }
pub fn get_cluster_info_async(&self, req: &GetClusterInfoRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<GetClusterInfoResponse>,> { self.get_cluster_info_async_opt(req, ::grpcio::CallOption::default()) }
pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Output = ()> + Send + 'static {self.client.spawn(f)}
}
pub trait Debug {
fn get(&mut self, ctx: ::grpcio::RpcContext, _req: GetRequest, sink: ::grpcio::UnarySink<GetResponse>) { grpcio::unimplemented_call!(ctx, sink) }
fn raft_log(&mut self, ctx: ::grpcio::RpcContext, _req: RaftLogRequest, sink: ::grpcio::UnarySink<RaftLogResponse>) { grpcio::unimplemented_call!(ctx, sink) }
fn region_info(&mut self, ctx: ::grpcio::RpcContext, _req: RegionInfoRequest, sink: ::grpcio::UnarySink<RegionInfoResponse>) { grpcio::unimplemented_call!(ctx, sink) }
fn region_size(&mut self, ctx: ::grpcio::RpcContext, _req: RegionSizeRequest, sink: ::grpcio::UnarySink<RegionSizeResponse>) { grpcio::unimplemented_call!(ctx, sink) }
fn scan_mvcc(&mut self, ctx: ::grpcio::RpcContext, _req: ScanMvccRequest, sink: ::grpcio::ServerStreamingSink<ScanMvccResponse>) { grpcio::unimplemented_call!(ctx, sink) }
fn compact(&mut self, ctx: ::grpcio::RpcContext, _req: CompactRequest, sink: ::grpcio::UnarySink<CompactResponse>) { grpcio::unimplemented_call!(ctx, sink) }
fn inject_fail_point(&mut self, ctx: ::grpcio::RpcContext, _req: InjectFailPointRequest, sink: ::grpcio::UnarySink<InjectFailPointResponse>) { grpcio::unimplemented_call!(ctx, sink) }
fn recover_fail_point(&mut self, ctx: ::grpcio::RpcContext, _req: RecoverFailPointRequest, sink: ::grpcio::UnarySink<RecoverFailPointResponse>) { grpcio::unimplemented_call!(ctx, sink) }
fn list_fail_points(&mut self, ctx: ::grpcio::RpcContext, _req: ListFailPointsRequest, sink: ::grpcio::UnarySink<ListFailPointsResponse>) { grpcio::unimplemented_call!(ctx, sink) }
fn get_metrics(&mut self, ctx: ::grpcio::RpcContext, _req: GetMetricsRequest, sink: ::grpcio::UnarySink<GetMetricsResponse>) { grpcio::unimplemented_call!(ctx, sink) }
fn check_region_consistency(&mut self, ctx: ::grpcio::RpcContext, _req: RegionConsistencyCheckRequest, sink: ::grpcio::UnarySink<RegionConsistencyCheckResponse>) { grpcio::unimplemented_call!(ctx, sink) }
fn modify_tikv_config(&mut self, ctx: ::grpcio::RpcContext, _req: ModifyTikvConfigRequest, sink: ::grpcio::UnarySink<ModifyTikvConfigResponse>) { grpcio::unimplemented_call!(ctx, sink) }
fn get_region_properties(&mut self, ctx: ::grpcio::RpcContext, _req: GetRegionPropertiesRequest, sink: ::grpcio::UnarySink<GetRegionPropertiesResponse>) { grpcio::unimplemented_call!(ctx, sink) }
fn get_store_info(&mut self, ctx: ::grpcio::RpcContext, _req: GetStoreInfoRequest, sink: ::grpcio::UnarySink<GetStoreInfoResponse>) { grpcio::unimplemented_call!(ctx, sink) }
fn get_cluster_info(&mut self, ctx: ::grpcio::RpcContext, _req: GetClusterInfoRequest, sink: ::grpcio::UnarySink<GetClusterInfoResponse>) { grpcio::unimplemented_call!(ctx, sink) }
}
pub fn create_debug<S: Debug + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
let mut builder = ::grpcio::ServiceBuilder::new();
let mut instance = s.clone();
builder = builder.add_unary_handler(&METHOD_DEBUG_GET, move |ctx, req, resp| instance.get(ctx, req, resp));
let mut instance = s.clone();
builder = builder.add_unary_handler(&METHOD_DEBUG_RAFT_LOG, move |ctx, req, resp| instance.raft_log(ctx, req, resp));
let mut instance = s.clone();
builder = builder.add_unary_handler(&METHOD_DEBUG_REGION_INFO, move |ctx, req, resp| instance.region_info(ctx, req, resp));
let mut instance = s.clone();
builder = builder.add_unary_handler(&METHOD_DEBUG_REGION_SIZE, move |ctx, req, resp| instance.region_size(ctx, req, resp));
let mut instance = s.clone();
builder = builder.add_server_streaming_handler(&METHOD_DEBUG_SCAN_MVCC, move |ctx, req, resp| instance.scan_mvcc(ctx, req, resp));
let mut instance = s.clone();
builder = builder.add_unary_handler(&METHOD_DEBUG_COMPACT, move |ctx, req, resp| instance.compact(ctx, req, resp));
let mut instance = s.clone();
builder = builder.add_unary_handler(&METHOD_DEBUG_INJECT_FAIL_POINT, move |ctx, req, resp| instance.inject_fail_point(ctx, req, resp));
let mut instance = s.clone();
builder = builder.add_unary_handler(&METHOD_DEBUG_RECOVER_FAIL_POINT, move |ctx, req, resp| instance.recover_fail_point(ctx, req, resp));
let mut instance = s.clone();
builder = builder.add_unary_handler(&METHOD_DEBUG_LIST_FAIL_POINTS, move |ctx, req, resp| instance.list_fail_points(ctx, req, resp));
let mut instance = s.clone();
builder = builder.add_unary_handler(&METHOD_DEBUG_GET_METRICS, move |ctx, req, resp| instance.get_metrics(ctx, req, resp));
let mut instance = s.clone();
builder = builder.add_unary_handler(&METHOD_DEBUG_CHECK_REGION_CONSISTENCY, move |ctx, req, resp| instance.check_region_consistency(ctx, req, resp));
let mut instance = s.clone();
builder = builder.add_unary_handler(&METHOD_DEBUG_MODIFY_TIKV_CONFIG, move |ctx, req, resp| instance.modify_tikv_config(ctx, req, resp));
let mut instance = s.clone();
builder = builder.add_unary_handler(&METHOD_DEBUG_GET_REGION_PROPERTIES, move |ctx, req, resp| instance.get_region_properties(ctx, req, resp));
let mut instance = s.clone();
builder = builder.add_unary_handler(&METHOD_DEBUG_GET_STORE_INFO, move |ctx, req, resp| instance.get_store_info(ctx, req, resp));
let mut instance = s;
builder = builder.add_unary_handler(&METHOD_DEBUG_GET_CLUSTER_INFO, move |ctx, req, resp| instance.get_cluster_info(ctx, req, resp));
builder.build()
}
