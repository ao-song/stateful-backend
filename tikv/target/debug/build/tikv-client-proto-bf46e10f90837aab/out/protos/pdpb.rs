#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestHeader {
    /// cluster_id is the ID of the cluster which be sent to.
    #[prost(uint64, tag="1")]
    pub cluster_id: u64,
    /// sender_id is the ID of the sender server, also member ID or etcd ID.
    #[prost(uint64, tag="2")]
    pub sender_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseHeader {
    /// cluster_id is the ID of the cluster which sent the response.
    #[prost(uint64, tag="1")]
    pub cluster_id: u64,
    #[prost(message, optional, tag="2")]
    pub error: ::core::option::Option<Error>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Error {
    #[prost(enumeration="ErrorType", tag="1")]
    pub r#type: i32,
    #[prost(string, tag="2")]
    pub message: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TsoRequest {
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<RequestHeader>,
    #[prost(uint32, tag="2")]
    pub count: u32,
    #[prost(string, tag="3")]
    pub dc_location: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Timestamp {
    #[prost(int64, tag="1")]
    pub physical: i64,
    #[prost(int64, tag="2")]
    pub logical: i64,
    /// Number of suffix bits used for global distinction,
    /// PD client will use this to compute a TSO's logical part.
    #[prost(uint32, tag="3")]
    pub suffix_bits: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TsoResponse {
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<ResponseHeader>,
    #[prost(uint32, tag="2")]
    pub count: u32,
    #[prost(message, optional, tag="3")]
    pub timestamp: ::core::option::Option<Timestamp>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BootstrapRequest {
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<RequestHeader>,
    #[prost(message, optional, tag="2")]
    pub store: ::core::option::Option<super::metapb::Store>,
    #[prost(message, optional, tag="3")]
    pub region: ::core::option::Option<super::metapb::Region>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BootstrapResponse {
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<ResponseHeader>,
    #[prost(message, optional, tag="2")]
    pub replication_status: ::core::option::Option<super::replication_modepb::ReplicationStatus>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IsBootstrappedRequest {
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<RequestHeader>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IsBootstrappedResponse {
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<ResponseHeader>,
    #[prost(bool, tag="2")]
    pub bootstrapped: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AllocIdRequest {
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<RequestHeader>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AllocIdResponse {
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<ResponseHeader>,
    #[prost(uint64, tag="2")]
    pub id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetStoreRequest {
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<RequestHeader>,
    #[prost(uint64, tag="2")]
    pub store_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetStoreResponse {
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<ResponseHeader>,
    #[prost(message, optional, tag="2")]
    pub store: ::core::option::Option<super::metapb::Store>,
    #[prost(message, optional, tag="3")]
    pub stats: ::core::option::Option<StoreStats>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PutStoreRequest {
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<RequestHeader>,
    #[prost(message, optional, tag="2")]
    pub store: ::core::option::Option<super::metapb::Store>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PutStoreResponse {
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<ResponseHeader>,
    #[prost(message, optional, tag="2")]
    pub replication_status: ::core::option::Option<super::replication_modepb::ReplicationStatus>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAllStoresRequest {
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<RequestHeader>,
    /// Do NOT return tombstone stores if set to true.
    #[prost(bool, tag="2")]
    pub exclude_tombstone_stores: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAllStoresResponse {
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<ResponseHeader>,
    #[prost(message, repeated, tag="2")]
    pub stores: ::prost::alloc::vec::Vec<super::metapb::Store>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRegionRequest {
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<RequestHeader>,
    #[prost(bytes="vec", tag="2")]
    pub region_key: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRegionResponse {
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<ResponseHeader>,
    #[prost(message, optional, tag="2")]
    pub region: ::core::option::Option<super::metapb::Region>,
    #[prost(message, optional, tag="3")]
    pub leader: ::core::option::Option<super::metapb::Peer>,
    /// Leader considers that these peers are down.
    #[prost(message, repeated, tag="5")]
    pub down_peers: ::prost::alloc::vec::Vec<PeerStats>,
    /// Pending peers are the peers that the leader can't consider as
    /// working followers.
    #[prost(message, repeated, tag="6")]
    pub pending_peers: ::prost::alloc::vec::Vec<super::metapb::Peer>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRegionByIdRequest {
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<RequestHeader>,
    #[prost(uint64, tag="2")]
    pub region_id: u64,
}
// Use GetRegionResponse as the response of GetRegionByIDRequest.

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScanRegionsRequest {
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<RequestHeader>,
    #[prost(bytes="vec", tag="2")]
    pub start_key: ::prost::alloc::vec::Vec<u8>,
    /// no limit when limit <= 0.
    #[prost(int32, tag="3")]
    pub limit: i32,
    /// end_key is +inf when it is empty.
    #[prost(bytes="vec", tag="4")]
    pub end_key: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Region {
    #[prost(message, optional, tag="1")]
    pub region: ::core::option::Option<super::metapb::Region>,
    #[prost(message, optional, tag="2")]
    pub leader: ::core::option::Option<super::metapb::Peer>,
    /// Leader considers that these peers are down.
    #[prost(message, repeated, tag="3")]
    pub down_peers: ::prost::alloc::vec::Vec<PeerStats>,
    /// Pending peers are the peers that the leader can't consider as
    /// working followers.
    #[prost(message, repeated, tag="4")]
    pub pending_peers: ::prost::alloc::vec::Vec<super::metapb::Peer>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScanRegionsResponse {
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<ResponseHeader>,
    /// Keep for backword compatibability.
    #[prost(message, repeated, tag="2")]
    pub region_metas: ::prost::alloc::vec::Vec<super::metapb::Region>,
    #[prost(message, repeated, tag="3")]
    pub leaders: ::prost::alloc::vec::Vec<super::metapb::Peer>,
    /// Extended region info with down/pending peers.
    #[prost(message, repeated, tag="4")]
    pub regions: ::prost::alloc::vec::Vec<Region>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetClusterConfigRequest {
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<RequestHeader>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetClusterConfigResponse {
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<ResponseHeader>,
    #[prost(message, optional, tag="2")]
    pub cluster: ::core::option::Option<super::metapb::Cluster>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PutClusterConfigRequest {
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<RequestHeader>,
    #[prost(message, optional, tag="2")]
    pub cluster: ::core::option::Option<super::metapb::Cluster>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PutClusterConfigResponse {
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<ResponseHeader>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Member {
    /// name is the name of the PD member.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// member_id is the unique id of the PD member.
    #[prost(uint64, tag="2")]
    pub member_id: u64,
    #[prost(string, repeated, tag="3")]
    pub peer_urls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="4")]
    pub client_urls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(int32, tag="5")]
    pub leader_priority: i32,
    #[prost(string, tag="6")]
    pub deploy_path: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub binary_version: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub git_hash: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub dc_location: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMembersRequest {
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<RequestHeader>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMembersResponse {
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<ResponseHeader>,
    #[prost(message, repeated, tag="2")]
    pub members: ::prost::alloc::vec::Vec<Member>,
    #[prost(message, optional, tag="3")]
    pub leader: ::core::option::Option<Member>,
    #[prost(message, optional, tag="4")]
    pub etcd_leader: ::core::option::Option<Member>,
    #[prost(map="string, message", tag="5")]
    pub tso_allocator_leaders: ::std::collections::HashMap<::prost::alloc::string::String, Member>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PeerStats {
    #[prost(message, optional, tag="1")]
    pub peer: ::core::option::Option<super::metapb::Peer>,
    #[prost(uint64, tag="2")]
    pub down_seconds: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegionHeartbeatRequest {
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<RequestHeader>,
    #[prost(message, optional, tag="2")]
    pub region: ::core::option::Option<super::metapb::Region>,
    /// Leader Peer sending the heartbeat.
    #[prost(message, optional, tag="3")]
    pub leader: ::core::option::Option<super::metapb::Peer>,
    /// Leader considers that these peers are down.
    #[prost(message, repeated, tag="4")]
    pub down_peers: ::prost::alloc::vec::Vec<PeerStats>,
    /// Pending peers are the peers that the leader can't consider as
    /// working followers.
    #[prost(message, repeated, tag="5")]
    pub pending_peers: ::prost::alloc::vec::Vec<super::metapb::Peer>,
    /// Bytes read/written during this period.
    #[prost(uint64, tag="6")]
    pub bytes_written: u64,
    #[prost(uint64, tag="7")]
    pub bytes_read: u64,
    /// Keys read/written during this period.
    #[prost(uint64, tag="8")]
    pub keys_written: u64,
    #[prost(uint64, tag="9")]
    pub keys_read: u64,
    /// Approximate region size.
    #[prost(uint64, tag="10")]
    pub approximate_size: u64,
    /// Actually reported time interval
    #[prost(message, optional, tag="12")]
    pub interval: ::core::option::Option<TimeInterval>,
    /// Approximate number of keys.
    #[prost(uint64, tag="13")]
    pub approximate_keys: u64,
    /// Term is the term of raft group.
    #[prost(uint64, tag="14")]
    pub term: u64,
    #[prost(message, optional, tag="15")]
    pub replication_status: ::core::option::Option<super::replication_modepb::RegionReplicationStatus>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangePeer {
    #[prost(message, optional, tag="1")]
    pub peer: ::core::option::Option<super::metapb::Peer>,
    #[prost(enumeration="super::eraftpb::ConfChangeType", tag="2")]
    pub change_type: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangePeerV2 {
    /// If changes is empty, it means that to exit joint state.
    #[prost(message, repeated, tag="1")]
    pub changes: ::prost::alloc::vec::Vec<ChangePeer>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferLeader {
    #[prost(message, optional, tag="1")]
    pub peer: ::core::option::Option<super::metapb::Peer>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Merge {
    #[prost(message, optional, tag="1")]
    pub target: ::core::option::Option<super::metapb::Region>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SplitRegion {
    #[prost(enumeration="CheckPolicy", tag="1")]
    pub policy: i32,
    #[prost(bytes="vec", repeated, tag="2")]
    pub keys: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegionHeartbeatResponse {
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<ResponseHeader>,
    /// Notice, Pd only allows handling reported epoch >= current pd's.
    /// Leader peer reports region status with RegionHeartbeatRequest
    /// to pd regularly, pd will determine whether this region
    /// should do ChangePeer or not.
    /// E,g, max peer number is 3, region A, first only peer 1 in A.
    /// 1. Pd region state -> Peers (1), ConfVer (1).
    /// 2. Leader peer 1 reports region state to pd, pd finds the
    /// peer number is < 3, so first changes its current region
    /// state -> Peers (1, 2), ConfVer (1), and returns ChangePeer Adding 2.
    /// 3. Leader does ChangePeer, then reports Peers (1, 2), ConfVer (2),
    /// pd updates its state -> Peers (1, 2), ConfVer (2).
    /// 4. Leader may report old Peers (1), ConfVer (1) to pd before ConfChange
    /// finished, pd stills responses ChangePeer Adding 2, of course, we must
    /// guarantee the second ChangePeer can't be applied in TiKV.
    #[prost(message, optional, tag="2")]
    pub change_peer: ::core::option::Option<ChangePeer>,
    /// Pd can return transfer_leader to let TiKV does leader transfer itself.
    #[prost(message, optional, tag="3")]
    pub transfer_leader: ::core::option::Option<TransferLeader>,
    /// ID of the region
    #[prost(uint64, tag="4")]
    pub region_id: u64,
    #[prost(message, optional, tag="5")]
    pub region_epoch: ::core::option::Option<super::metapb::RegionEpoch>,
    /// Leader of the region at the moment of the corresponding request was made.
    #[prost(message, optional, tag="6")]
    pub target_peer: ::core::option::Option<super::metapb::Peer>,
    #[prost(message, optional, tag="7")]
    pub merge: ::core::option::Option<Merge>,
    /// PD sends split_region to let TiKV split a region into two regions.
    #[prost(message, optional, tag="8")]
    pub split_region: ::core::option::Option<SplitRegion>,
    /// Multiple change peer operations atomically.
    /// Note: PD can use both ChangePeer and ChangePeerV2 at the same time
    ///       (not in the same RegionHeartbeatResponse).
    ///       Now, PD use ChangePeerV2 only for replacing peers.
    #[prost(message, optional, tag="9")]
    pub change_peer_v2: ::core::option::Option<ChangePeerV2>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AskSplitRequest {
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<RequestHeader>,
    #[prost(message, optional, tag="2")]
    pub region: ::core::option::Option<super::metapb::Region>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AskSplitResponse {
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<ResponseHeader>,
    /// We split the region into two, first uses the origin
    /// parent region id, and the second uses the new_region_id.
    /// We must guarantee that the new_region_id is global unique.
    #[prost(uint64, tag="2")]
    pub new_region_id: u64,
    /// The peer ids for the new split region.
    #[prost(uint64, repeated, tag="3")]
    pub new_peer_ids: ::prost::alloc::vec::Vec<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReportSplitRequest {
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<RequestHeader>,
    #[prost(message, optional, tag="2")]
    pub left: ::core::option::Option<super::metapb::Region>,
    #[prost(message, optional, tag="3")]
    pub right: ::core::option::Option<super::metapb::Region>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReportSplitResponse {
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<ResponseHeader>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AskBatchSplitRequest {
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<RequestHeader>,
    #[prost(message, optional, tag="2")]
    pub region: ::core::option::Option<super::metapb::Region>,
    #[prost(uint32, tag="3")]
    pub split_count: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SplitId {
    #[prost(uint64, tag="1")]
    pub new_region_id: u64,
    #[prost(uint64, repeated, tag="2")]
    pub new_peer_ids: ::prost::alloc::vec::Vec<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AskBatchSplitResponse {
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<ResponseHeader>,
    #[prost(message, repeated, tag="2")]
    pub ids: ::prost::alloc::vec::Vec<SplitId>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReportBatchSplitRequest {
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<RequestHeader>,
    #[prost(message, repeated, tag="2")]
    pub regions: ::prost::alloc::vec::Vec<super::metapb::Region>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReportBatchSplitResponse {
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<ResponseHeader>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimeInterval {
    /// The unix timestamp in seconds of the start of this period.
    #[prost(uint64, tag="1")]
    pub start_timestamp: u64,
    /// The unix timestamp in seconds of the end of this period.
    #[prost(uint64, tag="2")]
    pub end_timestamp: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecordPair {
    #[prost(string, tag="1")]
    pub key: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub value: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StoreStats {
    #[prost(uint64, tag="1")]
    pub store_id: u64,
    /// Capacity for the store.
    #[prost(uint64, tag="2")]
    pub capacity: u64,
    /// Available size for the store.
    #[prost(uint64, tag="3")]
    pub available: u64,
    /// Total region count in this store.
    #[prost(uint32, tag="4")]
    pub region_count: u32,
    /// Current sending snapshot count.
    #[prost(uint32, tag="5")]
    pub sending_snap_count: u32,
    /// Current receiving snapshot count.
    #[prost(uint32, tag="6")]
    pub receiving_snap_count: u32,
    /// When the store is started (unix timestamp in seconds).
    #[prost(uint32, tag="7")]
    pub start_time: u32,
    /// How many region is applying snapshot.
    #[prost(uint32, tag="8")]
    pub applying_snap_count: u32,
    /// If the store is busy
    #[prost(bool, tag="9")]
    pub is_busy: bool,
    /// Actually used space by db
    #[prost(uint64, tag="10")]
    pub used_size: u64,
    /// Bytes written for the store during this period.
    #[prost(uint64, tag="11")]
    pub bytes_written: u64,
    /// Keys written for the store during this period.
    #[prost(uint64, tag="12")]
    pub keys_written: u64,
    /// Bytes read for the store during this period.
    #[prost(uint64, tag="13")]
    pub bytes_read: u64,
    /// Keys read for the store during this period.
    #[prost(uint64, tag="14")]
    pub keys_read: u64,
    /// Actually reported time interval
    #[prost(message, optional, tag="15")]
    pub interval: ::core::option::Option<TimeInterval>,
    /// Threads' CPU usages in the store
    #[prost(message, repeated, tag="16")]
    pub cpu_usages: ::prost::alloc::vec::Vec<RecordPair>,
    /// Threads' read disk I/O rates in the store
    #[prost(message, repeated, tag="17")]
    pub read_io_rates: ::prost::alloc::vec::Vec<RecordPair>,
    /// Threads' write disk I/O rates in the store
    #[prost(message, repeated, tag="18")]
    pub write_io_rates: ::prost::alloc::vec::Vec<RecordPair>,
    /// Operations' latencies in the store
    #[prost(message, repeated, tag="19")]
    pub op_latencies: ::prost::alloc::vec::Vec<RecordPair>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StoreHeartbeatRequest {
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<RequestHeader>,
    #[prost(message, optional, tag="2")]
    pub stats: ::core::option::Option<StoreStats>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StoreHeartbeatResponse {
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<ResponseHeader>,
    #[prost(message, optional, tag="2")]
    pub replication_status: ::core::option::Option<super::replication_modepb::ReplicationStatus>,
    #[prost(string, tag="3")]
    pub cluster_version: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScatterRegionRequest {
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<RequestHeader>,
    #[deprecated]
    #[prost(uint64, tag="2")]
    pub region_id: u64,
    /// PD will use these region information if it can't find the region.
    /// For example, the region is just split and hasn't report to PD yet.
    #[prost(message, optional, tag="3")]
    pub region: ::core::option::Option<super::metapb::Region>,
    #[prost(message, optional, tag="4")]
    pub leader: ::core::option::Option<super::metapb::Peer>,
    /// If group is defined, the regions with the same group would be scattered as a whole group.
    /// If not defined, the regions would be scattered in a cluster level.
    #[prost(string, tag="5")]
    pub group: ::prost::alloc::string::String,
    /// If regions_id is defined, the region_id would be ignored.
    #[prost(uint64, repeated, tag="6")]
    pub regions_id: ::prost::alloc::vec::Vec<u64>,
    #[prost(uint64, tag="7")]
    pub retry_limit: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScatterRegionResponse {
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<ResponseHeader>,
    #[prost(uint64, tag="2")]
    pub finished_percentage: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetGcSafePointRequest {
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<RequestHeader>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetGcSafePointResponse {
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<ResponseHeader>,
    #[prost(uint64, tag="2")]
    pub safe_point: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateGcSafePointRequest {
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<RequestHeader>,
    #[prost(uint64, tag="2")]
    pub safe_point: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateGcSafePointResponse {
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<ResponseHeader>,
    #[prost(uint64, tag="2")]
    pub new_safe_point: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateServiceGcSafePointRequest {
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<RequestHeader>,
    #[prost(bytes="vec", tag="2")]
    pub service_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="3")]
    pub ttl: i64,
    #[prost(uint64, tag="4")]
    pub safe_point: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateServiceGcSafePointResponse {
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<ResponseHeader>,
    #[prost(bytes="vec", tag="2")]
    pub service_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="3")]
    pub ttl: i64,
    #[prost(uint64, tag="4")]
    pub min_safe_point: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegionStat {
    /// Bytes read/written during this period.
    #[prost(uint64, tag="1")]
    pub bytes_written: u64,
    #[prost(uint64, tag="2")]
    pub bytes_read: u64,
    /// Keys read/written during this period.
    #[prost(uint64, tag="3")]
    pub keys_written: u64,
    #[prost(uint64, tag="4")]
    pub keys_read: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SyncRegionRequest {
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<RequestHeader>,
    #[prost(message, optional, tag="2")]
    pub member: ::core::option::Option<Member>,
    /// the follower PD will use the start index to locate historical changes
    /// that require synchronization.
    #[prost(uint64, tag="3")]
    pub start_index: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SyncRegionResponse {
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<ResponseHeader>,
    /// the leader PD will send the repsonds include
    /// changed regions records and the index of the first record.
    #[prost(message, repeated, tag="2")]
    pub regions: ::prost::alloc::vec::Vec<super::metapb::Region>,
    #[prost(uint64, tag="3")]
    pub start_index: u64,
    #[prost(message, repeated, tag="4")]
    pub region_stats: ::prost::alloc::vec::Vec<RegionStat>,
    #[prost(message, repeated, tag="5")]
    pub region_leaders: ::prost::alloc::vec::Vec<super::metapb::Peer>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOperatorRequest {
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<RequestHeader>,
    #[prost(uint64, tag="2")]
    pub region_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOperatorResponse {
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<ResponseHeader>,
    #[prost(uint64, tag="2")]
    pub region_id: u64,
    #[prost(bytes="vec", tag="3")]
    pub desc: ::prost::alloc::vec::Vec<u8>,
    #[prost(enumeration="OperatorStatus", tag="4")]
    pub status: i32,
    #[prost(bytes="vec", tag="5")]
    pub kind: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SyncMaxTsRequest {
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<RequestHeader>,
    #[prost(message, optional, tag="2")]
    pub max_ts: ::core::option::Option<Timestamp>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SyncMaxTsResponse {
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<ResponseHeader>,
    #[prost(message, optional, tag="2")]
    pub max_local_ts: ::core::option::Option<Timestamp>,
    #[prost(string, repeated, tag="3")]
    pub dcs: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SplitRegionsRequest {
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<RequestHeader>,
    #[prost(bytes="vec", repeated, tag="2")]
    pub split_keys: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint64, tag="3")]
    pub retry_limit: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SplitRegionsResponse {
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<ResponseHeader>,
    #[prost(uint64, tag="2")]
    pub finished_percentage: u64,
    #[prost(uint64, repeated, tag="3")]
    pub regions_id: ::prost::alloc::vec::Vec<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDcLocationInfoRequest {
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<RequestHeader>,
    #[prost(string, tag="2")]
    pub dc_location: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDcLocationInfoResponse {
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<ResponseHeader>,
    /// suffix sign
    #[prost(int32, tag="2")]
    pub suffix: i32,
    /// max_ts will be included into this response if PD leader think the receiver needs,
    /// which it's set when the number of the max suffix bits changes.
    #[prost(message, optional, tag="3")]
    pub max_ts: ::core::option::Option<Timestamp>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ErrorType {
    Ok = 0,
    Unknown = 1,
    NotBootstrapped = 2,
    StoreTombstone = 3,
    AlreadyBootstrapped = 4,
    IncompatibleVersion = 5,
    RegionNotFound = 6,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CheckPolicy {
    Scan = 0,
    Approximate = 1,
    Usekey = 2,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OperatorStatus {
    Success = 0,
    Timeout = 1,
    Cancel = 2,
    Replace = 3,
    Running = 4,
}
const METHOD_PD_GET_MEMBERS: ::grpcio::Method<GetMembersRequest, GetMembersResponse> = ::grpcio::Method{ty: ::grpcio::MethodType::Unary, name: "/pdpb.PD/GetMembers", req_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, };
const METHOD_PD_TSO: ::grpcio::Method<TsoRequest, TsoResponse> = ::grpcio::Method{ty: ::grpcio::MethodType::Duplex, name: "/pdpb.PD/Tso", req_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, };
const METHOD_PD_BOOTSTRAP: ::grpcio::Method<BootstrapRequest, BootstrapResponse> = ::grpcio::Method{ty: ::grpcio::MethodType::Unary, name: "/pdpb.PD/Bootstrap", req_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, };
const METHOD_PD_IS_BOOTSTRAPPED: ::grpcio::Method<IsBootstrappedRequest, IsBootstrappedResponse> = ::grpcio::Method{ty: ::grpcio::MethodType::Unary, name: "/pdpb.PD/IsBootstrapped", req_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, };
const METHOD_PD_ALLOC_ID: ::grpcio::Method<AllocIdRequest, AllocIdResponse> = ::grpcio::Method{ty: ::grpcio::MethodType::Unary, name: "/pdpb.PD/AllocID", req_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, };
const METHOD_PD_GET_STORE: ::grpcio::Method<GetStoreRequest, GetStoreResponse> = ::grpcio::Method{ty: ::grpcio::MethodType::Unary, name: "/pdpb.PD/GetStore", req_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, };
const METHOD_PD_PUT_STORE: ::grpcio::Method<PutStoreRequest, PutStoreResponse> = ::grpcio::Method{ty: ::grpcio::MethodType::Unary, name: "/pdpb.PD/PutStore", req_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, };
const METHOD_PD_GET_ALL_STORES: ::grpcio::Method<GetAllStoresRequest, GetAllStoresResponse> = ::grpcio::Method{ty: ::grpcio::MethodType::Unary, name: "/pdpb.PD/GetAllStores", req_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, };
const METHOD_PD_STORE_HEARTBEAT: ::grpcio::Method<StoreHeartbeatRequest, StoreHeartbeatResponse> = ::grpcio::Method{ty: ::grpcio::MethodType::Unary, name: "/pdpb.PD/StoreHeartbeat", req_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, };
const METHOD_PD_REGION_HEARTBEAT: ::grpcio::Method<RegionHeartbeatRequest, RegionHeartbeatResponse> = ::grpcio::Method{ty: ::grpcio::MethodType::Duplex, name: "/pdpb.PD/RegionHeartbeat", req_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, };
const METHOD_PD_GET_REGION: ::grpcio::Method<GetRegionRequest, GetRegionResponse> = ::grpcio::Method{ty: ::grpcio::MethodType::Unary, name: "/pdpb.PD/GetRegion", req_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, };
const METHOD_PD_GET_PREV_REGION: ::grpcio::Method<GetRegionRequest, GetRegionResponse> = ::grpcio::Method{ty: ::grpcio::MethodType::Unary, name: "/pdpb.PD/GetPrevRegion", req_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, };
const METHOD_PD_GET_REGION_BY_ID: ::grpcio::Method<GetRegionByIdRequest, GetRegionResponse> = ::grpcio::Method{ty: ::grpcio::MethodType::Unary, name: "/pdpb.PD/GetRegionByID", req_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, };
const METHOD_PD_SCAN_REGIONS: ::grpcio::Method<ScanRegionsRequest, ScanRegionsResponse> = ::grpcio::Method{ty: ::grpcio::MethodType::Unary, name: "/pdpb.PD/ScanRegions", req_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, };
const METHOD_PD_ASK_SPLIT: ::grpcio::Method<AskSplitRequest, AskSplitResponse> = ::grpcio::Method{ty: ::grpcio::MethodType::Unary, name: "/pdpb.PD/AskSplit", req_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, };
const METHOD_PD_REPORT_SPLIT: ::grpcio::Method<ReportSplitRequest, ReportSplitResponse> = ::grpcio::Method{ty: ::grpcio::MethodType::Unary, name: "/pdpb.PD/ReportSplit", req_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, };
const METHOD_PD_ASK_BATCH_SPLIT: ::grpcio::Method<AskBatchSplitRequest, AskBatchSplitResponse> = ::grpcio::Method{ty: ::grpcio::MethodType::Unary, name: "/pdpb.PD/AskBatchSplit", req_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, };
const METHOD_PD_REPORT_BATCH_SPLIT: ::grpcio::Method<ReportBatchSplitRequest, ReportBatchSplitResponse> = ::grpcio::Method{ty: ::grpcio::MethodType::Unary, name: "/pdpb.PD/ReportBatchSplit", req_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, };
const METHOD_PD_GET_CLUSTER_CONFIG: ::grpcio::Method<GetClusterConfigRequest, GetClusterConfigResponse> = ::grpcio::Method{ty: ::grpcio::MethodType::Unary, name: "/pdpb.PD/GetClusterConfig", req_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, };
const METHOD_PD_PUT_CLUSTER_CONFIG: ::grpcio::Method<PutClusterConfigRequest, PutClusterConfigResponse> = ::grpcio::Method{ty: ::grpcio::MethodType::Unary, name: "/pdpb.PD/PutClusterConfig", req_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, };
const METHOD_PD_SCATTER_REGION: ::grpcio::Method<ScatterRegionRequest, ScatterRegionResponse> = ::grpcio::Method{ty: ::grpcio::MethodType::Unary, name: "/pdpb.PD/ScatterRegion", req_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, };
const METHOD_PD_GET_GC_SAFE_POINT: ::grpcio::Method<GetGcSafePointRequest, GetGcSafePointResponse> = ::grpcio::Method{ty: ::grpcio::MethodType::Unary, name: "/pdpb.PD/GetGCSafePoint", req_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, };
const METHOD_PD_UPDATE_GC_SAFE_POINT: ::grpcio::Method<UpdateGcSafePointRequest, UpdateGcSafePointResponse> = ::grpcio::Method{ty: ::grpcio::MethodType::Unary, name: "/pdpb.PD/UpdateGCSafePoint", req_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, };
const METHOD_PD_UPDATE_SERVICE_GC_SAFE_POINT: ::grpcio::Method<UpdateServiceGcSafePointRequest, UpdateServiceGcSafePointResponse> = ::grpcio::Method{ty: ::grpcio::MethodType::Unary, name: "/pdpb.PD/UpdateServiceGCSafePoint", req_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, };
const METHOD_PD_SYNC_REGIONS: ::grpcio::Method<SyncRegionRequest, SyncRegionResponse> = ::grpcio::Method{ty: ::grpcio::MethodType::Duplex, name: "/pdpb.PD/SyncRegions", req_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, };
const METHOD_PD_GET_OPERATOR: ::grpcio::Method<GetOperatorRequest, GetOperatorResponse> = ::grpcio::Method{ty: ::grpcio::MethodType::Unary, name: "/pdpb.PD/GetOperator", req_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, };
const METHOD_PD_SYNC_MAX_TS: ::grpcio::Method<SyncMaxTsRequest, SyncMaxTsResponse> = ::grpcio::Method{ty: ::grpcio::MethodType::Unary, name: "/pdpb.PD/SyncMaxTS", req_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, };
const METHOD_PD_SPLIT_REGIONS: ::grpcio::Method<SplitRegionsRequest, SplitRegionsResponse> = ::grpcio::Method{ty: ::grpcio::MethodType::Unary, name: "/pdpb.PD/SplitRegions", req_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, };
const METHOD_PD_GET_DC_LOCATION_INFO: ::grpcio::Method<GetDcLocationInfoRequest, GetDcLocationInfoResponse> = ::grpcio::Method{ty: ::grpcio::MethodType::Unary, name: "/pdpb.PD/GetDCLocationInfo", req_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, };
#[derive(Clone)]
pub struct PdClient { client: ::grpcio::Client }
impl PdClient {
pub fn new(channel: ::grpcio::Channel) -> Self { PdClient { client: ::grpcio::Client::new(channel) }}
pub fn get_members_opt(&self, req: &GetMembersRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<GetMembersResponse,> { self.client.unary_call(&METHOD_PD_GET_MEMBERS, req, opt) }
pub fn get_members(&self, req: &GetMembersRequest) -> ::grpcio::Result<GetMembersResponse,> { self.get_members_opt(req, ::grpcio::CallOption::default()) }
pub fn get_members_async_opt(&self, req: &GetMembersRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<GetMembersResponse>,> { self.client.unary_call_async(&METHOD_PD_GET_MEMBERS, req, opt) }
pub fn get_members_async(&self, req: &GetMembersRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<GetMembersResponse>,> { self.get_members_async_opt(req, ::grpcio::CallOption::default()) }
pub fn tso_opt(&self, opt: ::grpcio::CallOption) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<TsoRequest>,::grpcio::ClientDuplexReceiver<TsoResponse>,)> { self.client.duplex_streaming(&METHOD_PD_TSO, opt) }
pub fn tso(&self) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<TsoRequest>,::grpcio::ClientDuplexReceiver<TsoResponse>,)> { self.tso_opt(::grpcio::CallOption::default()) }
pub fn bootstrap_opt(&self, req: &BootstrapRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<BootstrapResponse,> { self.client.unary_call(&METHOD_PD_BOOTSTRAP, req, opt) }
pub fn bootstrap(&self, req: &BootstrapRequest) -> ::grpcio::Result<BootstrapResponse,> { self.bootstrap_opt(req, ::grpcio::CallOption::default()) }
pub fn bootstrap_async_opt(&self, req: &BootstrapRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<BootstrapResponse>,> { self.client.unary_call_async(&METHOD_PD_BOOTSTRAP, req, opt) }
pub fn bootstrap_async(&self, req: &BootstrapRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<BootstrapResponse>,> { self.bootstrap_async_opt(req, ::grpcio::CallOption::default()) }
pub fn is_bootstrapped_opt(&self, req: &IsBootstrappedRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<IsBootstrappedResponse,> { self.client.unary_call(&METHOD_PD_IS_BOOTSTRAPPED, req, opt) }
pub fn is_bootstrapped(&self, req: &IsBootstrappedRequest) -> ::grpcio::Result<IsBootstrappedResponse,> { self.is_bootstrapped_opt(req, ::grpcio::CallOption::default()) }
pub fn is_bootstrapped_async_opt(&self, req: &IsBootstrappedRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<IsBootstrappedResponse>,> { self.client.unary_call_async(&METHOD_PD_IS_BOOTSTRAPPED, req, opt) }
pub fn is_bootstrapped_async(&self, req: &IsBootstrappedRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<IsBootstrappedResponse>,> { self.is_bootstrapped_async_opt(req, ::grpcio::CallOption::default()) }
pub fn alloc_id_opt(&self, req: &AllocIdRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<AllocIdResponse,> { self.client.unary_call(&METHOD_PD_ALLOC_ID, req, opt) }
pub fn alloc_id(&self, req: &AllocIdRequest) -> ::grpcio::Result<AllocIdResponse,> { self.alloc_id_opt(req, ::grpcio::CallOption::default()) }
pub fn alloc_id_async_opt(&self, req: &AllocIdRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<AllocIdResponse>,> { self.client.unary_call_async(&METHOD_PD_ALLOC_ID, req, opt) }
pub fn alloc_id_async(&self, req: &AllocIdRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<AllocIdResponse>,> { self.alloc_id_async_opt(req, ::grpcio::CallOption::default()) }
pub fn get_store_opt(&self, req: &GetStoreRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<GetStoreResponse,> { self.client.unary_call(&METHOD_PD_GET_STORE, req, opt) }
pub fn get_store(&self, req: &GetStoreRequest) -> ::grpcio::Result<GetStoreResponse,> { self.get_store_opt(req, ::grpcio::CallOption::default()) }
pub fn get_store_async_opt(&self, req: &GetStoreRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<GetStoreResponse>,> { self.client.unary_call_async(&METHOD_PD_GET_STORE, req, opt) }
pub fn get_store_async(&self, req: &GetStoreRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<GetStoreResponse>,> { self.get_store_async_opt(req, ::grpcio::CallOption::default()) }
pub fn put_store_opt(&self, req: &PutStoreRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<PutStoreResponse,> { self.client.unary_call(&METHOD_PD_PUT_STORE, req, opt) }
pub fn put_store(&self, req: &PutStoreRequest) -> ::grpcio::Result<PutStoreResponse,> { self.put_store_opt(req, ::grpcio::CallOption::default()) }
pub fn put_store_async_opt(&self, req: &PutStoreRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<PutStoreResponse>,> { self.client.unary_call_async(&METHOD_PD_PUT_STORE, req, opt) }
pub fn put_store_async(&self, req: &PutStoreRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<PutStoreResponse>,> { self.put_store_async_opt(req, ::grpcio::CallOption::default()) }
pub fn get_all_stores_opt(&self, req: &GetAllStoresRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<GetAllStoresResponse,> { self.client.unary_call(&METHOD_PD_GET_ALL_STORES, req, opt) }
pub fn get_all_stores(&self, req: &GetAllStoresRequest) -> ::grpcio::Result<GetAllStoresResponse,> { self.get_all_stores_opt(req, ::grpcio::CallOption::default()) }
pub fn get_all_stores_async_opt(&self, req: &GetAllStoresRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<GetAllStoresResponse>,> { self.client.unary_call_async(&METHOD_PD_GET_ALL_STORES, req, opt) }
pub fn get_all_stores_async(&self, req: &GetAllStoresRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<GetAllStoresResponse>,> { self.get_all_stores_async_opt(req, ::grpcio::CallOption::default()) }
pub fn store_heartbeat_opt(&self, req: &StoreHeartbeatRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<StoreHeartbeatResponse,> { self.client.unary_call(&METHOD_PD_STORE_HEARTBEAT, req, opt) }
pub fn store_heartbeat(&self, req: &StoreHeartbeatRequest) -> ::grpcio::Result<StoreHeartbeatResponse,> { self.store_heartbeat_opt(req, ::grpcio::CallOption::default()) }
pub fn store_heartbeat_async_opt(&self, req: &StoreHeartbeatRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<StoreHeartbeatResponse>,> { self.client.unary_call_async(&METHOD_PD_STORE_HEARTBEAT, req, opt) }
pub fn store_heartbeat_async(&self, req: &StoreHeartbeatRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<StoreHeartbeatResponse>,> { self.store_heartbeat_async_opt(req, ::grpcio::CallOption::default()) }
pub fn region_heartbeat_opt(&self, opt: ::grpcio::CallOption) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<RegionHeartbeatRequest>,::grpcio::ClientDuplexReceiver<RegionHeartbeatResponse>,)> { self.client.duplex_streaming(&METHOD_PD_REGION_HEARTBEAT, opt) }
pub fn region_heartbeat(&self) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<RegionHeartbeatRequest>,::grpcio::ClientDuplexReceiver<RegionHeartbeatResponse>,)> { self.region_heartbeat_opt(::grpcio::CallOption::default()) }
pub fn get_region_opt(&self, req: &GetRegionRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<GetRegionResponse,> { self.client.unary_call(&METHOD_PD_GET_REGION, req, opt) }
pub fn get_region(&self, req: &GetRegionRequest) -> ::grpcio::Result<GetRegionResponse,> { self.get_region_opt(req, ::grpcio::CallOption::default()) }
pub fn get_region_async_opt(&self, req: &GetRegionRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<GetRegionResponse>,> { self.client.unary_call_async(&METHOD_PD_GET_REGION, req, opt) }
pub fn get_region_async(&self, req: &GetRegionRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<GetRegionResponse>,> { self.get_region_async_opt(req, ::grpcio::CallOption::default()) }
pub fn get_prev_region_opt(&self, req: &GetRegionRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<GetRegionResponse,> { self.client.unary_call(&METHOD_PD_GET_PREV_REGION, req, opt) }
pub fn get_prev_region(&self, req: &GetRegionRequest) -> ::grpcio::Result<GetRegionResponse,> { self.get_prev_region_opt(req, ::grpcio::CallOption::default()) }
pub fn get_prev_region_async_opt(&self, req: &GetRegionRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<GetRegionResponse>,> { self.client.unary_call_async(&METHOD_PD_GET_PREV_REGION, req, opt) }
pub fn get_prev_region_async(&self, req: &GetRegionRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<GetRegionResponse>,> { self.get_prev_region_async_opt(req, ::grpcio::CallOption::default()) }
pub fn get_region_by_id_opt(&self, req: &GetRegionByIdRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<GetRegionResponse,> { self.client.unary_call(&METHOD_PD_GET_REGION_BY_ID, req, opt) }
pub fn get_region_by_id(&self, req: &GetRegionByIdRequest) -> ::grpcio::Result<GetRegionResponse,> { self.get_region_by_id_opt(req, ::grpcio::CallOption::default()) }
pub fn get_region_by_id_async_opt(&self, req: &GetRegionByIdRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<GetRegionResponse>,> { self.client.unary_call_async(&METHOD_PD_GET_REGION_BY_ID, req, opt) }
pub fn get_region_by_id_async(&self, req: &GetRegionByIdRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<GetRegionResponse>,> { self.get_region_by_id_async_opt(req, ::grpcio::CallOption::default()) }
pub fn scan_regions_opt(&self, req: &ScanRegionsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<ScanRegionsResponse,> { self.client.unary_call(&METHOD_PD_SCAN_REGIONS, req, opt) }
pub fn scan_regions(&self, req: &ScanRegionsRequest) -> ::grpcio::Result<ScanRegionsResponse,> { self.scan_regions_opt(req, ::grpcio::CallOption::default()) }
pub fn scan_regions_async_opt(&self, req: &ScanRegionsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<ScanRegionsResponse>,> { self.client.unary_call_async(&METHOD_PD_SCAN_REGIONS, req, opt) }
pub fn scan_regions_async(&self, req: &ScanRegionsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<ScanRegionsResponse>,> { self.scan_regions_async_opt(req, ::grpcio::CallOption::default()) }
pub fn ask_split_opt(&self, req: &AskSplitRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<AskSplitResponse,> { self.client.unary_call(&METHOD_PD_ASK_SPLIT, req, opt) }
pub fn ask_split(&self, req: &AskSplitRequest) -> ::grpcio::Result<AskSplitResponse,> { self.ask_split_opt(req, ::grpcio::CallOption::default()) }
pub fn ask_split_async_opt(&self, req: &AskSplitRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<AskSplitResponse>,> { self.client.unary_call_async(&METHOD_PD_ASK_SPLIT, req, opt) }
pub fn ask_split_async(&self, req: &AskSplitRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<AskSplitResponse>,> { self.ask_split_async_opt(req, ::grpcio::CallOption::default()) }
pub fn report_split_opt(&self, req: &ReportSplitRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<ReportSplitResponse,> { self.client.unary_call(&METHOD_PD_REPORT_SPLIT, req, opt) }
pub fn report_split(&self, req: &ReportSplitRequest) -> ::grpcio::Result<ReportSplitResponse,> { self.report_split_opt(req, ::grpcio::CallOption::default()) }
pub fn report_split_async_opt(&self, req: &ReportSplitRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<ReportSplitResponse>,> { self.client.unary_call_async(&METHOD_PD_REPORT_SPLIT, req, opt) }
pub fn report_split_async(&self, req: &ReportSplitRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<ReportSplitResponse>,> { self.report_split_async_opt(req, ::grpcio::CallOption::default()) }
pub fn ask_batch_split_opt(&self, req: &AskBatchSplitRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<AskBatchSplitResponse,> { self.client.unary_call(&METHOD_PD_ASK_BATCH_SPLIT, req, opt) }
pub fn ask_batch_split(&self, req: &AskBatchSplitRequest) -> ::grpcio::Result<AskBatchSplitResponse,> { self.ask_batch_split_opt(req, ::grpcio::CallOption::default()) }
pub fn ask_batch_split_async_opt(&self, req: &AskBatchSplitRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<AskBatchSplitResponse>,> { self.client.unary_call_async(&METHOD_PD_ASK_BATCH_SPLIT, req, opt) }
pub fn ask_batch_split_async(&self, req: &AskBatchSplitRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<AskBatchSplitResponse>,> { self.ask_batch_split_async_opt(req, ::grpcio::CallOption::default()) }
pub fn report_batch_split_opt(&self, req: &ReportBatchSplitRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<ReportBatchSplitResponse,> { self.client.unary_call(&METHOD_PD_REPORT_BATCH_SPLIT, req, opt) }
pub fn report_batch_split(&self, req: &ReportBatchSplitRequest) -> ::grpcio::Result<ReportBatchSplitResponse,> { self.report_batch_split_opt(req, ::grpcio::CallOption::default()) }
pub fn report_batch_split_async_opt(&self, req: &ReportBatchSplitRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<ReportBatchSplitResponse>,> { self.client.unary_call_async(&METHOD_PD_REPORT_BATCH_SPLIT, req, opt) }
pub fn report_batch_split_async(&self, req: &ReportBatchSplitRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<ReportBatchSplitResponse>,> { self.report_batch_split_async_opt(req, ::grpcio::CallOption::default()) }
pub fn get_cluster_config_opt(&self, req: &GetClusterConfigRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<GetClusterConfigResponse,> { self.client.unary_call(&METHOD_PD_GET_CLUSTER_CONFIG, req, opt) }
pub fn get_cluster_config(&self, req: &GetClusterConfigRequest) -> ::grpcio::Result<GetClusterConfigResponse,> { self.get_cluster_config_opt(req, ::grpcio::CallOption::default()) }
pub fn get_cluster_config_async_opt(&self, req: &GetClusterConfigRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<GetClusterConfigResponse>,> { self.client.unary_call_async(&METHOD_PD_GET_CLUSTER_CONFIG, req, opt) }
pub fn get_cluster_config_async(&self, req: &GetClusterConfigRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<GetClusterConfigResponse>,> { self.get_cluster_config_async_opt(req, ::grpcio::CallOption::default()) }
pub fn put_cluster_config_opt(&self, req: &PutClusterConfigRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<PutClusterConfigResponse,> { self.client.unary_call(&METHOD_PD_PUT_CLUSTER_CONFIG, req, opt) }
pub fn put_cluster_config(&self, req: &PutClusterConfigRequest) -> ::grpcio::Result<PutClusterConfigResponse,> { self.put_cluster_config_opt(req, ::grpcio::CallOption::default()) }
pub fn put_cluster_config_async_opt(&self, req: &PutClusterConfigRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<PutClusterConfigResponse>,> { self.client.unary_call_async(&METHOD_PD_PUT_CLUSTER_CONFIG, req, opt) }
pub fn put_cluster_config_async(&self, req: &PutClusterConfigRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<PutClusterConfigResponse>,> { self.put_cluster_config_async_opt(req, ::grpcio::CallOption::default()) }
pub fn scatter_region_opt(&self, req: &ScatterRegionRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<ScatterRegionResponse,> { self.client.unary_call(&METHOD_PD_SCATTER_REGION, req, opt) }
pub fn scatter_region(&self, req: &ScatterRegionRequest) -> ::grpcio::Result<ScatterRegionResponse,> { self.scatter_region_opt(req, ::grpcio::CallOption::default()) }
pub fn scatter_region_async_opt(&self, req: &ScatterRegionRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<ScatterRegionResponse>,> { self.client.unary_call_async(&METHOD_PD_SCATTER_REGION, req, opt) }
pub fn scatter_region_async(&self, req: &ScatterRegionRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<ScatterRegionResponse>,> { self.scatter_region_async_opt(req, ::grpcio::CallOption::default()) }
pub fn get_gc_safe_point_opt(&self, req: &GetGcSafePointRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<GetGcSafePointResponse,> { self.client.unary_call(&METHOD_PD_GET_GC_SAFE_POINT, req, opt) }
pub fn get_gc_safe_point(&self, req: &GetGcSafePointRequest) -> ::grpcio::Result<GetGcSafePointResponse,> { self.get_gc_safe_point_opt(req, ::grpcio::CallOption::default()) }
pub fn get_gc_safe_point_async_opt(&self, req: &GetGcSafePointRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<GetGcSafePointResponse>,> { self.client.unary_call_async(&METHOD_PD_GET_GC_SAFE_POINT, req, opt) }
pub fn get_gc_safe_point_async(&self, req: &GetGcSafePointRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<GetGcSafePointResponse>,> { self.get_gc_safe_point_async_opt(req, ::grpcio::CallOption::default()) }
pub fn update_gc_safe_point_opt(&self, req: &UpdateGcSafePointRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<UpdateGcSafePointResponse,> { self.client.unary_call(&METHOD_PD_UPDATE_GC_SAFE_POINT, req, opt) }
pub fn update_gc_safe_point(&self, req: &UpdateGcSafePointRequest) -> ::grpcio::Result<UpdateGcSafePointResponse,> { self.update_gc_safe_point_opt(req, ::grpcio::CallOption::default()) }
pub fn update_gc_safe_point_async_opt(&self, req: &UpdateGcSafePointRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<UpdateGcSafePointResponse>,> { self.client.unary_call_async(&METHOD_PD_UPDATE_GC_SAFE_POINT, req, opt) }
pub fn update_gc_safe_point_async(&self, req: &UpdateGcSafePointRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<UpdateGcSafePointResponse>,> { self.update_gc_safe_point_async_opt(req, ::grpcio::CallOption::default()) }
pub fn update_service_gc_safe_point_opt(&self, req: &UpdateServiceGcSafePointRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<UpdateServiceGcSafePointResponse,> { self.client.unary_call(&METHOD_PD_UPDATE_SERVICE_GC_SAFE_POINT, req, opt) }
pub fn update_service_gc_safe_point(&self, req: &UpdateServiceGcSafePointRequest) -> ::grpcio::Result<UpdateServiceGcSafePointResponse,> { self.update_service_gc_safe_point_opt(req, ::grpcio::CallOption::default()) }
pub fn update_service_gc_safe_point_async_opt(&self, req: &UpdateServiceGcSafePointRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<UpdateServiceGcSafePointResponse>,> { self.client.unary_call_async(&METHOD_PD_UPDATE_SERVICE_GC_SAFE_POINT, req, opt) }
pub fn update_service_gc_safe_point_async(&self, req: &UpdateServiceGcSafePointRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<UpdateServiceGcSafePointResponse>,> { self.update_service_gc_safe_point_async_opt(req, ::grpcio::CallOption::default()) }
pub fn sync_regions_opt(&self, opt: ::grpcio::CallOption) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<SyncRegionRequest>,::grpcio::ClientDuplexReceiver<SyncRegionResponse>,)> { self.client.duplex_streaming(&METHOD_PD_SYNC_REGIONS, opt) }
pub fn sync_regions(&self) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<SyncRegionRequest>,::grpcio::ClientDuplexReceiver<SyncRegionResponse>,)> { self.sync_regions_opt(::grpcio::CallOption::default()) }
pub fn get_operator_opt(&self, req: &GetOperatorRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<GetOperatorResponse,> { self.client.unary_call(&METHOD_PD_GET_OPERATOR, req, opt) }
pub fn get_operator(&self, req: &GetOperatorRequest) -> ::grpcio::Result<GetOperatorResponse,> { self.get_operator_opt(req, ::grpcio::CallOption::default()) }
pub fn get_operator_async_opt(&self, req: &GetOperatorRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<GetOperatorResponse>,> { self.client.unary_call_async(&METHOD_PD_GET_OPERATOR, req, opt) }
pub fn get_operator_async(&self, req: &GetOperatorRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<GetOperatorResponse>,> { self.get_operator_async_opt(req, ::grpcio::CallOption::default()) }
pub fn sync_max_ts_opt(&self, req: &SyncMaxTsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<SyncMaxTsResponse,> { self.client.unary_call(&METHOD_PD_SYNC_MAX_TS, req, opt) }
pub fn sync_max_ts(&self, req: &SyncMaxTsRequest) -> ::grpcio::Result<SyncMaxTsResponse,> { self.sync_max_ts_opt(req, ::grpcio::CallOption::default()) }
pub fn sync_max_ts_async_opt(&self, req: &SyncMaxTsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<SyncMaxTsResponse>,> { self.client.unary_call_async(&METHOD_PD_SYNC_MAX_TS, req, opt) }
pub fn sync_max_ts_async(&self, req: &SyncMaxTsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<SyncMaxTsResponse>,> { self.sync_max_ts_async_opt(req, ::grpcio::CallOption::default()) }
pub fn split_regions_opt(&self, req: &SplitRegionsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<SplitRegionsResponse,> { self.client.unary_call(&METHOD_PD_SPLIT_REGIONS, req, opt) }
pub fn split_regions(&self, req: &SplitRegionsRequest) -> ::grpcio::Result<SplitRegionsResponse,> { self.split_regions_opt(req, ::grpcio::CallOption::default()) }
pub fn split_regions_async_opt(&self, req: &SplitRegionsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<SplitRegionsResponse>,> { self.client.unary_call_async(&METHOD_PD_SPLIT_REGIONS, req, opt) }
pub fn split_regions_async(&self, req: &SplitRegionsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<SplitRegionsResponse>,> { self.split_regions_async_opt(req, ::grpcio::CallOption::default()) }
pub fn get_dc_location_info_opt(&self, req: &GetDcLocationInfoRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<GetDcLocationInfoResponse,> { self.client.unary_call(&METHOD_PD_GET_DC_LOCATION_INFO, req, opt) }
pub fn get_dc_location_info(&self, req: &GetDcLocationInfoRequest) -> ::grpcio::Result<GetDcLocationInfoResponse,> { self.get_dc_location_info_opt(req, ::grpcio::CallOption::default()) }
pub fn get_dc_location_info_async_opt(&self, req: &GetDcLocationInfoRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<GetDcLocationInfoResponse>,> { self.client.unary_call_async(&METHOD_PD_GET_DC_LOCATION_INFO, req, opt) }
pub fn get_dc_location_info_async(&self, req: &GetDcLocationInfoRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<GetDcLocationInfoResponse>,> { self.get_dc_location_info_async_opt(req, ::grpcio::CallOption::default()) }
pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Output = ()> + Send + 'static {self.client.spawn(f)}
}
pub trait Pd {
fn get_members(&mut self, ctx: ::grpcio::RpcContext, _req: GetMembersRequest, sink: ::grpcio::UnarySink<GetMembersResponse>) { grpcio::unimplemented_call!(ctx, sink) }
fn tso(&mut self, ctx: ::grpcio::RpcContext, _stream: ::grpcio::RequestStream<TsoRequest>, sink: ::grpcio::DuplexSink<TsoResponse>) { grpcio::unimplemented_call!(ctx, sink) }
fn bootstrap(&mut self, ctx: ::grpcio::RpcContext, _req: BootstrapRequest, sink: ::grpcio::UnarySink<BootstrapResponse>) { grpcio::unimplemented_call!(ctx, sink) }
fn is_bootstrapped(&mut self, ctx: ::grpcio::RpcContext, _req: IsBootstrappedRequest, sink: ::grpcio::UnarySink<IsBootstrappedResponse>) { grpcio::unimplemented_call!(ctx, sink) }
fn alloc_id(&mut self, ctx: ::grpcio::RpcContext, _req: AllocIdRequest, sink: ::grpcio::UnarySink<AllocIdResponse>) { grpcio::unimplemented_call!(ctx, sink) }
fn get_store(&mut self, ctx: ::grpcio::RpcContext, _req: GetStoreRequest, sink: ::grpcio::UnarySink<GetStoreResponse>) { grpcio::unimplemented_call!(ctx, sink) }
fn put_store(&mut self, ctx: ::grpcio::RpcContext, _req: PutStoreRequest, sink: ::grpcio::UnarySink<PutStoreResponse>) { grpcio::unimplemented_call!(ctx, sink) }
fn get_all_stores(&mut self, ctx: ::grpcio::RpcContext, _req: GetAllStoresRequest, sink: ::grpcio::UnarySink<GetAllStoresResponse>) { grpcio::unimplemented_call!(ctx, sink) }
fn store_heartbeat(&mut self, ctx: ::grpcio::RpcContext, _req: StoreHeartbeatRequest, sink: ::grpcio::UnarySink<StoreHeartbeatResponse>) { grpcio::unimplemented_call!(ctx, sink) }
fn region_heartbeat(&mut self, ctx: ::grpcio::RpcContext, _stream: ::grpcio::RequestStream<RegionHeartbeatRequest>, sink: ::grpcio::DuplexSink<RegionHeartbeatResponse>) { grpcio::unimplemented_call!(ctx, sink) }
fn get_region(&mut self, ctx: ::grpcio::RpcContext, _req: GetRegionRequest, sink: ::grpcio::UnarySink<GetRegionResponse>) { grpcio::unimplemented_call!(ctx, sink) }
fn get_prev_region(&mut self, ctx: ::grpcio::RpcContext, _req: GetRegionRequest, sink: ::grpcio::UnarySink<GetRegionResponse>) { grpcio::unimplemented_call!(ctx, sink) }
fn get_region_by_id(&mut self, ctx: ::grpcio::RpcContext, _req: GetRegionByIdRequest, sink: ::grpcio::UnarySink<GetRegionResponse>) { grpcio::unimplemented_call!(ctx, sink) }
fn scan_regions(&mut self, ctx: ::grpcio::RpcContext, _req: ScanRegionsRequest, sink: ::grpcio::UnarySink<ScanRegionsResponse>) { grpcio::unimplemented_call!(ctx, sink) }
fn ask_split(&mut self, ctx: ::grpcio::RpcContext, _req: AskSplitRequest, sink: ::grpcio::UnarySink<AskSplitResponse>) { grpcio::unimplemented_call!(ctx, sink) }
fn report_split(&mut self, ctx: ::grpcio::RpcContext, _req: ReportSplitRequest, sink: ::grpcio::UnarySink<ReportSplitResponse>) { grpcio::unimplemented_call!(ctx, sink) }
fn ask_batch_split(&mut self, ctx: ::grpcio::RpcContext, _req: AskBatchSplitRequest, sink: ::grpcio::UnarySink<AskBatchSplitResponse>) { grpcio::unimplemented_call!(ctx, sink) }
fn report_batch_split(&mut self, ctx: ::grpcio::RpcContext, _req: ReportBatchSplitRequest, sink: ::grpcio::UnarySink<ReportBatchSplitResponse>) { grpcio::unimplemented_call!(ctx, sink) }
fn get_cluster_config(&mut self, ctx: ::grpcio::RpcContext, _req: GetClusterConfigRequest, sink: ::grpcio::UnarySink<GetClusterConfigResponse>) { grpcio::unimplemented_call!(ctx, sink) }
fn put_cluster_config(&mut self, ctx: ::grpcio::RpcContext, _req: PutClusterConfigRequest, sink: ::grpcio::UnarySink<PutClusterConfigResponse>) { grpcio::unimplemented_call!(ctx, sink) }
fn scatter_region(&mut self, ctx: ::grpcio::RpcContext, _req: ScatterRegionRequest, sink: ::grpcio::UnarySink<ScatterRegionResponse>) { grpcio::unimplemented_call!(ctx, sink) }
fn get_gc_safe_point(&mut self, ctx: ::grpcio::RpcContext, _req: GetGcSafePointRequest, sink: ::grpcio::UnarySink<GetGcSafePointResponse>) { grpcio::unimplemented_call!(ctx, sink) }
fn update_gc_safe_point(&mut self, ctx: ::grpcio::RpcContext, _req: UpdateGcSafePointRequest, sink: ::grpcio::UnarySink<UpdateGcSafePointResponse>) { grpcio::unimplemented_call!(ctx, sink) }
fn update_service_gc_safe_point(&mut self, ctx: ::grpcio::RpcContext, _req: UpdateServiceGcSafePointRequest, sink: ::grpcio::UnarySink<UpdateServiceGcSafePointResponse>) { grpcio::unimplemented_call!(ctx, sink) }
fn sync_regions(&mut self, ctx: ::grpcio::RpcContext, _stream: ::grpcio::RequestStream<SyncRegionRequest>, sink: ::grpcio::DuplexSink<SyncRegionResponse>) { grpcio::unimplemented_call!(ctx, sink) }
fn get_operator(&mut self, ctx: ::grpcio::RpcContext, _req: GetOperatorRequest, sink: ::grpcio::UnarySink<GetOperatorResponse>) { grpcio::unimplemented_call!(ctx, sink) }
fn sync_max_ts(&mut self, ctx: ::grpcio::RpcContext, _req: SyncMaxTsRequest, sink: ::grpcio::UnarySink<SyncMaxTsResponse>) { grpcio::unimplemented_call!(ctx, sink) }
fn split_regions(&mut self, ctx: ::grpcio::RpcContext, _req: SplitRegionsRequest, sink: ::grpcio::UnarySink<SplitRegionsResponse>) { grpcio::unimplemented_call!(ctx, sink) }
fn get_dc_location_info(&mut self, ctx: ::grpcio::RpcContext, _req: GetDcLocationInfoRequest, sink: ::grpcio::UnarySink<GetDcLocationInfoResponse>) { grpcio::unimplemented_call!(ctx, sink) }
}
pub fn create_pd<S: Pd + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
let mut builder = ::grpcio::ServiceBuilder::new();
let mut instance = s.clone();
builder = builder.add_unary_handler(&METHOD_PD_GET_MEMBERS, move |ctx, req, resp| instance.get_members(ctx, req, resp));
let mut instance = s.clone();
builder = builder.add_duplex_streaming_handler(&METHOD_PD_TSO, move |ctx, req, resp| instance.tso(ctx, req, resp));
let mut instance = s.clone();
builder = builder.add_unary_handler(&METHOD_PD_BOOTSTRAP, move |ctx, req, resp| instance.bootstrap(ctx, req, resp));
let mut instance = s.clone();
builder = builder.add_unary_handler(&METHOD_PD_IS_BOOTSTRAPPED, move |ctx, req, resp| instance.is_bootstrapped(ctx, req, resp));
let mut instance = s.clone();
builder = builder.add_unary_handler(&METHOD_PD_ALLOC_ID, move |ctx, req, resp| instance.alloc_id(ctx, req, resp));
let mut instance = s.clone();
builder = builder.add_unary_handler(&METHOD_PD_GET_STORE, move |ctx, req, resp| instance.get_store(ctx, req, resp));
let mut instance = s.clone();
builder = builder.add_unary_handler(&METHOD_PD_PUT_STORE, move |ctx, req, resp| instance.put_store(ctx, req, resp));
let mut instance = s.clone();
builder = builder.add_unary_handler(&METHOD_PD_GET_ALL_STORES, move |ctx, req, resp| instance.get_all_stores(ctx, req, resp));
let mut instance = s.clone();
builder = builder.add_unary_handler(&METHOD_PD_STORE_HEARTBEAT, move |ctx, req, resp| instance.store_heartbeat(ctx, req, resp));
let mut instance = s.clone();
builder = builder.add_duplex_streaming_handler(&METHOD_PD_REGION_HEARTBEAT, move |ctx, req, resp| instance.region_heartbeat(ctx, req, resp));
let mut instance = s.clone();
builder = builder.add_unary_handler(&METHOD_PD_GET_REGION, move |ctx, req, resp| instance.get_region(ctx, req, resp));
let mut instance = s.clone();
builder = builder.add_unary_handler(&METHOD_PD_GET_PREV_REGION, move |ctx, req, resp| instance.get_prev_region(ctx, req, resp));
let mut instance = s.clone();
builder = builder.add_unary_handler(&METHOD_PD_GET_REGION_BY_ID, move |ctx, req, resp| instance.get_region_by_id(ctx, req, resp));
let mut instance = s.clone();
builder = builder.add_unary_handler(&METHOD_PD_SCAN_REGIONS, move |ctx, req, resp| instance.scan_regions(ctx, req, resp));
let mut instance = s.clone();
builder = builder.add_unary_handler(&METHOD_PD_ASK_SPLIT, move |ctx, req, resp| instance.ask_split(ctx, req, resp));
let mut instance = s.clone();
builder = builder.add_unary_handler(&METHOD_PD_REPORT_SPLIT, move |ctx, req, resp| instance.report_split(ctx, req, resp));
let mut instance = s.clone();
builder = builder.add_unary_handler(&METHOD_PD_ASK_BATCH_SPLIT, move |ctx, req, resp| instance.ask_batch_split(ctx, req, resp));
let mut instance = s.clone();
builder = builder.add_unary_handler(&METHOD_PD_REPORT_BATCH_SPLIT, move |ctx, req, resp| instance.report_batch_split(ctx, req, resp));
let mut instance = s.clone();
builder = builder.add_unary_handler(&METHOD_PD_GET_CLUSTER_CONFIG, move |ctx, req, resp| instance.get_cluster_config(ctx, req, resp));
let mut instance = s.clone();
builder = builder.add_unary_handler(&METHOD_PD_PUT_CLUSTER_CONFIG, move |ctx, req, resp| instance.put_cluster_config(ctx, req, resp));
let mut instance = s.clone();
builder = builder.add_unary_handler(&METHOD_PD_SCATTER_REGION, move |ctx, req, resp| instance.scatter_region(ctx, req, resp));
let mut instance = s.clone();
builder = builder.add_unary_handler(&METHOD_PD_GET_GC_SAFE_POINT, move |ctx, req, resp| instance.get_gc_safe_point(ctx, req, resp));
let mut instance = s.clone();
builder = builder.add_unary_handler(&METHOD_PD_UPDATE_GC_SAFE_POINT, move |ctx, req, resp| instance.update_gc_safe_point(ctx, req, resp));
let mut instance = s.clone();
builder = builder.add_unary_handler(&METHOD_PD_UPDATE_SERVICE_GC_SAFE_POINT, move |ctx, req, resp| instance.update_service_gc_safe_point(ctx, req, resp));
let mut instance = s.clone();
builder = builder.add_duplex_streaming_handler(&METHOD_PD_SYNC_REGIONS, move |ctx, req, resp| instance.sync_regions(ctx, req, resp));
let mut instance = s.clone();
builder = builder.add_unary_handler(&METHOD_PD_GET_OPERATOR, move |ctx, req, resp| instance.get_operator(ctx, req, resp));
let mut instance = s.clone();
builder = builder.add_unary_handler(&METHOD_PD_SYNC_MAX_TS, move |ctx, req, resp| instance.sync_max_ts(ctx, req, resp));
let mut instance = s.clone();
builder = builder.add_unary_handler(&METHOD_PD_SPLIT_REGIONS, move |ctx, req, resp| instance.split_regions(ctx, req, resp));
let mut instance = s;
builder = builder.add_unary_handler(&METHOD_PD_GET_DC_LOCATION_INFO, move |ctx, req, resp| instance.get_dc_location_info(ctx, req, resp));
builder.build()
}
