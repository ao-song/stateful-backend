#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RaftMessage {
    #[prost(uint64, tag="1")]
    pub region_id: u64,
    #[prost(message, optional, tag="2")]
    pub from_peer: ::core::option::Option<super::metapb::Peer>,
    #[prost(message, optional, tag="3")]
    pub to_peer: ::core::option::Option<super::metapb::Peer>,
    #[prost(message, optional, tag="4")]
    pub message: ::core::option::Option<super::eraftpb::Message>,
    #[prost(message, optional, tag="5")]
    pub region_epoch: ::core::option::Option<super::metapb::RegionEpoch>,
    /// true means to_peer is a tombstone peer and it should remove itself.
    #[prost(bool, tag="6")]
    pub is_tombstone: bool,
    /// Region key range [start_key, end_key).
    #[prost(bytes="vec", tag="7")]
    pub start_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="8")]
    pub end_key: ::prost::alloc::vec::Vec<u8>,
    /// If it has value, to_peer should be removed if merge is never going to complete.
    #[prost(message, optional, tag="9")]
    pub merge_target: ::core::option::Option<super::metapb::Region>,
    #[prost(message, optional, tag="10")]
    pub extra_msg: ::core::option::Option<ExtraMessage>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RaftTruncatedState {
    #[prost(uint64, tag="1")]
    pub index: u64,
    #[prost(uint64, tag="2")]
    pub term: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SnapshotCfFile {
    #[prost(string, tag="1")]
    pub cf: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub size: u64,
    #[prost(uint32, tag="3")]
    pub checksum: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SnapshotMeta {
    #[prost(message, repeated, tag="1")]
    pub cf_files: ::prost::alloc::vec::Vec<SnapshotCfFile>,
    /// true means this snapshot is triggered for load balance
    #[prost(bool, tag="2")]
    pub for_balance: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SnapshotChunk {
    #[prost(message, optional, tag="1")]
    pub message: ::core::option::Option<RaftMessage>,
    #[prost(bytes="vec", tag="2")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Done {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeyValue {
    #[prost(bytes="vec", tag="1")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub value: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RaftSnapshotData {
    #[prost(message, optional, tag="1")]
    pub region: ::core::option::Option<super::metapb::Region>,
    #[prost(uint64, tag="2")]
    pub file_size: u64,
    #[prost(message, repeated, tag="3")]
    pub data: ::prost::alloc::vec::Vec<KeyValue>,
    #[prost(uint64, tag="4")]
    pub version: u64,
    #[prost(message, optional, tag="5")]
    pub meta: ::core::option::Option<SnapshotMeta>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StoreIdent {
    #[prost(uint64, tag="1")]
    pub cluster_id: u64,
    #[prost(uint64, tag="2")]
    pub store_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RaftLocalState {
    #[prost(message, optional, tag="1")]
    pub hard_state: ::core::option::Option<super::eraftpb::HardState>,
    #[prost(uint64, tag="2")]
    pub last_index: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RaftApplyState {
    #[prost(uint64, tag="1")]
    pub applied_index: u64,
    #[prost(uint64, tag="3")]
    pub last_commit_index: u64,
    #[prost(uint64, tag="4")]
    pub commit_index: u64,
    #[prost(uint64, tag="5")]
    pub commit_term: u64,
    #[prost(message, optional, tag="2")]
    pub truncated_state: ::core::option::Option<RaftTruncatedState>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MergeState {
    #[prost(uint64, tag="1")]
    pub min_index: u64,
    #[prost(message, optional, tag="2")]
    pub target: ::core::option::Option<super::metapb::Region>,
    #[prost(uint64, tag="3")]
    pub commit: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegionLocalState {
    #[prost(enumeration="PeerState", tag="1")]
    pub state: i32,
    #[prost(message, optional, tag="2")]
    pub region: ::core::option::Option<super::metapb::Region>,
    #[prost(message, optional, tag="3")]
    pub merge_state: ::core::option::Option<MergeState>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExtraMessage {
    #[prost(enumeration="ExtraMessageType", tag="1")]
    pub r#type: i32,
    #[prost(uint64, tag="2")]
    pub premerge_commit: u64,
    #[prost(message, repeated, tag="3")]
    pub check_peers: ::prost::alloc::vec::Vec<super::metapb::Peer>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PeerState {
    Normal = 0,
    Applying = 1,
    Tombstone = 2,
    Merging = 3,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ExtraMessageType {
    MsgRegionWakeUp = 0,
    MsgWantRollbackMerge = 1,
    MsgCheckStalePeer = 2,
    MsgCheckStalePeerResponse = 3,
    /// If leader is going to sleep, it will send requests to all its followers
    /// to make sure they all agree to sleep.
    MsgHibernateRequest = 4,
    MsgHibernateResponse = 5,
}
