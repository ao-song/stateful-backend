/// The replication status sync from PD to TiKV.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplicationStatus {
    #[prost(enumeration="ReplicationMode", tag="1")]
    pub mode: i32,
    #[prost(message, optional, tag="2")]
    pub dr_auto_sync: ::core::option::Option<DrAutoSync>,
}
/// The status of dr-autosync mode.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DrAutoSync {
    /// The key of the label that used for distinguish different DC.
    #[prost(string, tag="1")]
    pub label_key: ::prost::alloc::string::String,
    #[prost(enumeration="DrAutoSyncState", tag="2")]
    pub state: i32,
    /// Unique ID of the state, it increases after each state transfer.
    #[prost(uint64, tag="3")]
    pub state_id: u64,
    /// Duration to wait before switching to SYNC by force (in seconds)
    #[prost(int32, tag="4")]
    pub wait_sync_timeout_hint: i32,
}
/// The replication status sync from TiKV to PD.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegionReplicationStatus {
    #[prost(enumeration="RegionReplicationState", tag="1")]
    pub state: i32,
    /// Unique ID of the state, it increases after each state transfer.
    #[prost(uint64, tag="2")]
    pub state_id: u64,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ReplicationMode {
    /// The standard mode. Replicate logs to majority peer.
    Majority = 0,
    /// DR mode. Replicate logs among 2 DCs.
    DrAutoSync = 1,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DrAutoSyncState {
    /// Raft logs need to sync between different DCs
    Sync = 0,
    /// Raft logs need to sync to majority peers
    Async = 1,
    /// Switching from ASYNC to SYNC mode
    SyncRecover = 2,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RegionReplicationState {
    /// The region's state is unknown
    Unknown = 0,
    /// Logs sync to majority peers
    SimpleMajority = 1,
    /// Logs sync to different DCs
    IntegrityOverLabel = 2,
}
