#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Header {
    #[prost(uint64, tag="1")]
    pub cluster_id: u64,
    #[prost(string, tag="2")]
    pub ticdc_version: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DuplicateRequest {
    #[prost(uint64, tag="1")]
    pub region_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Compatibility {
    #[prost(string, tag="1")]
    pub required_version: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Error {
    #[prost(message, optional, tag="1")]
    pub not_leader: ::core::option::Option<super::errorpb::NotLeader>,
    #[prost(message, optional, tag="2")]
    pub region_not_found: ::core::option::Option<super::errorpb::RegionNotFound>,
    #[prost(message, optional, tag="3")]
    pub epoch_not_match: ::core::option::Option<super::errorpb::EpochNotMatch>,
    #[prost(message, optional, tag="4")]
    pub duplicate_request: ::core::option::Option<DuplicateRequest>,
    #[prost(message, optional, tag="5")]
    pub compatibility: ::core::option::Option<Compatibility>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxnInfo {
    #[prost(uint64, tag="1")]
    pub start_ts: u64,
    #[prost(bytes="vec", tag="2")]
    pub primary: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxnStatus {
    #[prost(uint64, tag="1")]
    pub start_ts: u64,
    #[prost(uint64, tag="2")]
    pub min_commit_ts: u64,
    #[prost(uint64, tag="3")]
    pub commit_ts: u64,
    #[prost(bool, tag="4")]
    pub is_rolled_back: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Event {
    #[prost(uint64, tag="1")]
    pub region_id: u64,
    #[prost(uint64, tag="2")]
    pub index: u64,
    #[prost(uint64, tag="7")]
    pub request_id: u64,
    #[prost(oneof="event::Event", tags="3, 4, 5, 6, 8")]
    pub event: ::core::option::Option<event::Event>,
}
/// Nested message and enum types in `Event`.
pub mod event {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Row {
        #[prost(uint64, tag="1")]
        pub start_ts: u64,
        #[prost(uint64, tag="2")]
        pub commit_ts: u64,
        #[prost(enumeration="LogType", tag="3")]
        pub r#type: i32,
        #[prost(enumeration="row::OpType", tag="4")]
        pub op_type: i32,
        #[prost(bytes="vec", tag="5")]
        pub key: ::prost::alloc::vec::Vec<u8>,
        #[prost(bytes="vec", tag="6")]
        pub value: ::prost::alloc::vec::Vec<u8>,
        #[prost(bytes="vec", tag="7")]
        pub old_value: ::prost::alloc::vec::Vec<u8>,
    }
    /// Nested message and enum types in `Row`.
    pub mod row {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum OpType {
            Unknown = 0,
            Put = 1,
            Delete = 2,
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Entries {
        #[prost(message, repeated, tag="1")]
        pub entries: ::prost::alloc::vec::Vec<Row>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Admin {
        #[prost(message, optional, tag="1")]
        pub admin_request: ::core::option::Option<super::super::raft_cmdpb::AdminRequest>,
        #[prost(message, optional, tag="2")]
        pub admin_response: ::core::option::Option<super::super::raft_cmdpb::AdminResponse>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct LongTxn {
        #[prost(message, repeated, tag="1")]
        pub txn_info: ::prost::alloc::vec::Vec<super::TxnInfo>,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum LogType {
        Unknown = 0,
        Prewrite = 1,
        Commit = 2,
        Rollback = 3,
        Committed = 4,
        Initialized = 5,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Event {
        #[prost(message, tag="3")]
        Entries(Entries),
        #[prost(message, tag="4")]
        Admin(Admin),
        #[prost(message, tag="5")]
        Error(super::Error),
        #[prost(uint64, tag="6")]
        ResolvedTs(u64),
        /// Note that field 7 is taken by request_id.
        ///
        /// More region level events ...
        #[prost(message, tag="8")]
        LongTxn(LongTxn),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeDataEvent {
    #[prost(message, repeated, tag="1")]
    pub events: ::prost::alloc::vec::Vec<Event>,
    /// More store level events ...
    #[prost(message, optional, tag="2")]
    pub resolved_ts: ::core::option::Option<ResolvedTs>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResolvedTs {
    #[prost(uint64, repeated, tag="1")]
    pub regions: ::prost::alloc::vec::Vec<u64>,
    #[prost(uint64, tag="2")]
    pub ts: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeDataRequest {
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<Header>,
    #[prost(uint64, tag="2")]
    pub region_id: u64,
    #[prost(message, optional, tag="3")]
    pub region_epoch: ::core::option::Option<super::metapb::RegionEpoch>,
    #[prost(uint64, tag="4")]
    pub checkpoint_ts: u64,
    #[prost(bytes="vec", tag="5")]
    pub start_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="6")]
    pub end_key: ::prost::alloc::vec::Vec<u8>,
    /// Used for CDC to identify events corresponding to different requests.
    #[prost(uint64, tag="7")]
    pub request_id: u64,
    #[prost(enumeration="super::kvrpcpb::ExtraOp", tag="8")]
    pub extra_op: i32,
    #[prost(oneof="change_data_request::Request", tags="9, 10")]
    pub request: ::core::option::Option<change_data_request::Request>,
}
/// Nested message and enum types in `ChangeDataRequest`.
pub mod change_data_request {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Register {
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct NotifyTxnStatus {
        #[prost(message, repeated, tag="1")]
        pub txn_status: ::prost::alloc::vec::Vec<super::TxnStatus>,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Request {
        /// A normal request that trying to register change data feed on a region.
        #[prost(message, tag="9")]
        Register(Register),
        /// Notify the region that some of the running transactions on the region has a pushed
        /// min_commit_ts so that the resolved_ts can be advanced.
        #[prost(message, tag="10")]
        NotifyTxnStatus(NotifyTxnStatus),
    }
}
const METHOD_CHANGE_DATA_EVENT_FEED: ::grpcio::Method<ChangeDataRequest, ChangeDataEvent> = ::grpcio::Method{ty: ::grpcio::MethodType::Duplex, name: "/cdcpb.ChangeData/EventFeed", req_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pr_ser, de: ::grpcio::pr_de }, };
#[derive(Clone)]
pub struct ChangeDataClient { client: ::grpcio::Client }
impl ChangeDataClient {
pub fn new(channel: ::grpcio::Channel) -> Self { ChangeDataClient { client: ::grpcio::Client::new(channel) }}
pub fn event_feed_opt(&self, opt: ::grpcio::CallOption) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<ChangeDataRequest>,::grpcio::ClientDuplexReceiver<ChangeDataEvent>,)> { self.client.duplex_streaming(&METHOD_CHANGE_DATA_EVENT_FEED, opt) }
pub fn event_feed(&self) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<ChangeDataRequest>,::grpcio::ClientDuplexReceiver<ChangeDataEvent>,)> { self.event_feed_opt(::grpcio::CallOption::default()) }
pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Output = ()> + Send + 'static {self.client.spawn(f)}
}
pub trait ChangeData {
fn event_feed(&mut self, ctx: ::grpcio::RpcContext, _stream: ::grpcio::RequestStream<ChangeDataRequest>, sink: ::grpcio::DuplexSink<ChangeDataEvent>) { grpcio::unimplemented_call!(ctx, sink) }
}
pub fn create_change_data<S: ChangeData + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
let mut builder = ::grpcio::ServiceBuilder::new();
let mut instance = s;
builder = builder.add_duplex_streaming_handler(&METHOD_CHANGE_DATA_EVENT_FEED, move |ctx, req, resp| instance.event_feed(ctx, req, resp));
builder.build()
}
