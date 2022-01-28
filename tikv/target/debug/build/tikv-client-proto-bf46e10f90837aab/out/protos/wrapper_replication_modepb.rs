// Generated file, please don't edit manually.

impl ReplicationStatus {
pub fn new_() -> ReplicationStatus { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn clear_mode(&mut self) { self.mode = 0 }
#[inline] pub fn get_mode(&self) -> ReplicationMode { match ReplicationMode::from_i32(self.mode) {Some(e) => e,
                        None => panic!("Unknown enum variant: {}", self.mode),
                    } }
#[inline] pub fn has_dr_auto_sync(&self) -> bool { self.dr_auto_sync.is_some() }
#[inline] pub fn clear_dr_auto_sync(&mut self) { self.dr_auto_sync = ::std::option::Option::None }
#[inline] pub fn set_dr_auto_sync(&mut self, v: DrAutoSync) { self.dr_auto_sync = ::std::option::Option::Some(v); }
#[inline] pub fn get_dr_auto_sync(&self) -> &DrAutoSync { match self.dr_auto_sync.as_ref() {
                            Some(v) => v,
                            None => DrAutoSync::default_ref(),
                        } }
#[inline] pub fn mut_dr_auto_sync(&mut self) -> &mut DrAutoSync { if self.dr_auto_sync.is_none() {
                                self.dr_auto_sync = ::std::option::Option::Some(DrAutoSync::default());
                            }
                            self.dr_auto_sync.as_mut().unwrap() } 
#[inline] pub fn take_dr_auto_sync(&mut self) -> DrAutoSync { self.dr_auto_sync.take().unwrap_or_else(DrAutoSync::default) }
}
impl ::protobuf::Clear for ReplicationStatus {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for ReplicationStatus {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static ReplicationStatus {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: ReplicationStatus = ReplicationStatus::default();
        }
        &*INSTANCE
    }
fn is_initialized(&self) -> bool { true }
fn write_to_with_cached_sizes(&self, _os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> { unimplemented!(); }
fn merge_from(&mut self, _is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> { unimplemented!(); }
fn get_unknown_fields(&self) -> &::protobuf::UnknownFields { unimplemented!(); }
fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields { unimplemented!(); }
fn write_to_bytes(&self) -> ::protobuf::ProtobufResult<Vec<u8>> {
            let mut buf = Vec::new();
            if ::prost::Message::encode(self, &mut buf).is_err() {
                return Err(::protobuf::ProtobufError::WireError(::protobuf::error::WireError::Other));
            }
            Ok(buf)
        }
fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
            if ::prost::Message::merge(self, bytes).is_err() {
                return Err(::protobuf::ProtobufError::WireError(::protobuf::error::WireError::Other));
            }
            Ok(())
        }
}
impl DrAutoSync {
pub fn new_() -> DrAutoSync { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn clear_label_key(&mut self) { self.label_key.clear(); }
#[inline] pub fn set_label_key(&mut self, v: :: prost :: alloc :: string :: String) { self.label_key = v; }
#[inline] pub fn get_label_key(&self) -> &str { &self.label_key }
#[inline] pub fn mut_label_key(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.label_key }
#[inline] pub fn take_label_key(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.label_key, Default::default()) }
#[inline] pub fn clear_state(&mut self) { self.state = 0 }
#[inline] pub fn get_state(&self) -> DrAutoSyncState { match DrAutoSyncState::from_i32(self.state) {Some(e) => e,
                        None => panic!("Unknown enum variant: {}", self.state),
                    } }
#[inline] pub fn clear_state_id(&mut self) { self.state_id = 0 }
#[inline] pub fn set_state_id(&mut self, v: u64) { self.state_id = v; }
#[inline] pub fn get_state_id(&self) -> u64 { self.state_id }
#[inline] pub fn clear_wait_sync_timeout_hint(&mut self) { self.wait_sync_timeout_hint = 0 }
#[inline] pub fn set_wait_sync_timeout_hint(&mut self, v: i32) { self.wait_sync_timeout_hint = v; }
#[inline] pub fn get_wait_sync_timeout_hint(&self) -> i32 { self.wait_sync_timeout_hint }
}
impl ::protobuf::Clear for DrAutoSync {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for DrAutoSync {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static DrAutoSync {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: DrAutoSync = DrAutoSync::default();
        }
        &*INSTANCE
    }
fn is_initialized(&self) -> bool { true }
fn write_to_with_cached_sizes(&self, _os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> { unimplemented!(); }
fn merge_from(&mut self, _is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> { unimplemented!(); }
fn get_unknown_fields(&self) -> &::protobuf::UnknownFields { unimplemented!(); }
fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields { unimplemented!(); }
fn write_to_bytes(&self) -> ::protobuf::ProtobufResult<Vec<u8>> {
            let mut buf = Vec::new();
            if ::prost::Message::encode(self, &mut buf).is_err() {
                return Err(::protobuf::ProtobufError::WireError(::protobuf::error::WireError::Other));
            }
            Ok(buf)
        }
fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
            if ::prost::Message::merge(self, bytes).is_err() {
                return Err(::protobuf::ProtobufError::WireError(::protobuf::error::WireError::Other));
            }
            Ok(())
        }
}
impl RegionReplicationStatus {
pub fn new_() -> RegionReplicationStatus { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn clear_state(&mut self) { self.state = 0 }
#[inline] pub fn get_state(&self) -> RegionReplicationState { match RegionReplicationState::from_i32(self.state) {Some(e) => e,
                        None => panic!("Unknown enum variant: {}", self.state),
                    } }
#[inline] pub fn clear_state_id(&mut self) { self.state_id = 0 }
#[inline] pub fn set_state_id(&mut self, v: u64) { self.state_id = v; }
#[inline] pub fn get_state_id(&self) -> u64 { self.state_id }
}
impl ::protobuf::Clear for RegionReplicationStatus {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for RegionReplicationStatus {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static RegionReplicationStatus {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: RegionReplicationStatus = RegionReplicationStatus::default();
        }
        &*INSTANCE
    }
fn is_initialized(&self) -> bool { true }
fn write_to_with_cached_sizes(&self, _os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> { unimplemented!(); }
fn merge_from(&mut self, _is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> { unimplemented!(); }
fn get_unknown_fields(&self) -> &::protobuf::UnknownFields { unimplemented!(); }
fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields { unimplemented!(); }
fn write_to_bytes(&self) -> ::protobuf::ProtobufResult<Vec<u8>> {
            let mut buf = Vec::new();
            if ::prost::Message::encode(self, &mut buf).is_err() {
                return Err(::protobuf::ProtobufError::WireError(::protobuf::error::WireError::Other));
            }
            Ok(buf)
        }
fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
            if ::prost::Message::merge(self, bytes).is_err() {
                return Err(::protobuf::ProtobufError::WireError(::protobuf::error::WireError::Other));
            }
            Ok(())
        }
}
impl ReplicationMode {
pub fn values() -> &'static [Self] {
static VALUES: &'static [ReplicationMode] = &[
ReplicationMode::Majority,
ReplicationMode::DrAutoSync,
];
VALUES
}
}
impl DrAutoSyncState {
pub fn values() -> &'static [Self] {
static VALUES: &'static [DrAutoSyncState] = &[
DrAutoSyncState::Sync,
DrAutoSyncState::Async,
DrAutoSyncState::SyncRecover,
];
VALUES
}
}
impl RegionReplicationState {
pub fn values() -> &'static [Self] {
static VALUES: &'static [RegionReplicationState] = &[
RegionReplicationState::Unknown,
RegionReplicationState::SimpleMajority,
RegionReplicationState::IntegrityOverLabel,
];
VALUES
}
}
