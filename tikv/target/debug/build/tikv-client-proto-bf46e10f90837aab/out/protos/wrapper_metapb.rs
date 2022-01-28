// Generated file, please don't edit manually.

impl Cluster {
pub fn new_() -> Cluster { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn clear_id(&mut self) { self.id = 0 }
#[inline] pub fn set_id(&mut self, v: u64) { self.id = v; }
#[inline] pub fn get_id(&self) -> u64 { self.id }
#[inline] pub fn clear_max_peer_count(&mut self) { self.max_peer_count = 0 }
#[inline] pub fn set_max_peer_count(&mut self, v: u32) { self.max_peer_count = v; }
#[inline] pub fn get_max_peer_count(&self) -> u32 { self.max_peer_count }
}
impl ::protobuf::Clear for Cluster {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for Cluster {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static Cluster {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: Cluster = Cluster::default();
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
impl StoreLabel {
pub fn new_() -> StoreLabel { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn clear_key(&mut self) { self.key.clear(); }
#[inline] pub fn set_key(&mut self, v: :: prost :: alloc :: string :: String) { self.key = v; }
#[inline] pub fn get_key(&self) -> &str { &self.key }
#[inline] pub fn mut_key(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.key }
#[inline] pub fn take_key(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.key, Default::default()) }
#[inline] pub fn clear_value(&mut self) { self.value.clear(); }
#[inline] pub fn set_value(&mut self, v: :: prost :: alloc :: string :: String) { self.value = v; }
#[inline] pub fn get_value(&self) -> &str { &self.value }
#[inline] pub fn mut_value(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.value }
#[inline] pub fn take_value(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.value, Default::default()) }
}
impl ::protobuf::Clear for StoreLabel {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for StoreLabel {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static StoreLabel {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: StoreLabel = StoreLabel::default();
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
impl Store {
pub fn new_() -> Store { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn clear_id(&mut self) { self.id = 0 }
#[inline] pub fn set_id(&mut self, v: u64) { self.id = v; }
#[inline] pub fn get_id(&self) -> u64 { self.id }
#[inline] pub fn clear_address(&mut self) { self.address.clear(); }
#[inline] pub fn set_address(&mut self, v: :: prost :: alloc :: string :: String) { self.address = v; }
#[inline] pub fn get_address(&self) -> &str { &self.address }
#[inline] pub fn mut_address(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.address }
#[inline] pub fn take_address(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.address, Default::default()) }
#[inline] pub fn clear_state(&mut self) { self.state = 0 }
#[inline] pub fn get_state(&self) -> StoreState { match StoreState::from_i32(self.state) {Some(e) => e,
                        None => panic!("Unknown enum variant: {}", self.state),
                    } }
#[inline] pub fn clear_labels(&mut self) { self.labels.clear(); }
#[inline] pub fn set_labels(&mut self, v: ::std::vec::Vec<StoreLabel>) { self.labels = v; }
#[inline] pub fn get_labels(&self) -> &[StoreLabel] { &self.labels }
#[inline] pub fn mut_labels(&mut self) -> &mut ::std::vec::Vec<StoreLabel> { &mut self.labels }
#[inline] pub fn take_labels(&mut self) -> ::std::vec::Vec<StoreLabel> { ::std::mem::replace(&mut self.labels, ::std::vec::Vec::new()) }
#[inline] pub fn clear_version(&mut self) { self.version.clear(); }
#[inline] pub fn set_version(&mut self, v: :: prost :: alloc :: string :: String) { self.version = v; }
#[inline] pub fn get_version(&self) -> &str { &self.version }
#[inline] pub fn mut_version(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.version }
#[inline] pub fn take_version(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.version, Default::default()) }
#[inline] pub fn clear_peer_address(&mut self) { self.peer_address.clear(); }
#[inline] pub fn set_peer_address(&mut self, v: :: prost :: alloc :: string :: String) { self.peer_address = v; }
#[inline] pub fn get_peer_address(&self) -> &str { &self.peer_address }
#[inline] pub fn mut_peer_address(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.peer_address }
#[inline] pub fn take_peer_address(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.peer_address, Default::default()) }
#[inline] pub fn clear_status_address(&mut self) { self.status_address.clear(); }
#[inline] pub fn set_status_address(&mut self, v: :: prost :: alloc :: string :: String) { self.status_address = v; }
#[inline] pub fn get_status_address(&self) -> &str { &self.status_address }
#[inline] pub fn mut_status_address(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.status_address }
#[inline] pub fn take_status_address(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.status_address, Default::default()) }
#[inline] pub fn clear_git_hash(&mut self) { self.git_hash.clear(); }
#[inline] pub fn set_git_hash(&mut self, v: :: prost :: alloc :: string :: String) { self.git_hash = v; }
#[inline] pub fn get_git_hash(&self) -> &str { &self.git_hash }
#[inline] pub fn mut_git_hash(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.git_hash }
#[inline] pub fn take_git_hash(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.git_hash, Default::default()) }
#[inline] pub fn clear_start_timestamp(&mut self) { self.start_timestamp = 0 }
#[inline] pub fn set_start_timestamp(&mut self, v: i64) { self.start_timestamp = v; }
#[inline] pub fn get_start_timestamp(&self) -> i64 { self.start_timestamp }
#[inline] pub fn clear_deploy_path(&mut self) { self.deploy_path.clear(); }
#[inline] pub fn set_deploy_path(&mut self, v: :: prost :: alloc :: string :: String) { self.deploy_path = v; }
#[inline] pub fn get_deploy_path(&self) -> &str { &self.deploy_path }
#[inline] pub fn mut_deploy_path(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.deploy_path }
#[inline] pub fn take_deploy_path(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.deploy_path, Default::default()) }
#[inline] pub fn clear_last_heartbeat(&mut self) { self.last_heartbeat = 0 }
#[inline] pub fn set_last_heartbeat(&mut self, v: i64) { self.last_heartbeat = v; }
#[inline] pub fn get_last_heartbeat(&self) -> i64 { self.last_heartbeat }
#[inline] pub fn clear_physically_destroyed(&mut self) { self.physically_destroyed = false }
#[inline] pub fn set_physically_destroyed(&mut self, v: bool) { self.physically_destroyed = v; }
#[inline] pub fn get_physically_destroyed(&self) -> bool { self.physically_destroyed }
}
impl ::protobuf::Clear for Store {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for Store {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static Store {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: Store = Store::default();
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
impl RegionEpoch {
pub fn new_() -> RegionEpoch { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn clear_conf_ver(&mut self) { self.conf_ver = 0 }
#[inline] pub fn set_conf_ver(&mut self, v: u64) { self.conf_ver = v; }
#[inline] pub fn get_conf_ver(&self) -> u64 { self.conf_ver }
#[inline] pub fn clear_version(&mut self) { self.version = 0 }
#[inline] pub fn set_version(&mut self, v: u64) { self.version = v; }
#[inline] pub fn get_version(&self) -> u64 { self.version }
}
impl ::protobuf::Clear for RegionEpoch {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for RegionEpoch {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static RegionEpoch {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: RegionEpoch = RegionEpoch::default();
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
impl Region {
pub fn new_() -> Region { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn clear_id(&mut self) { self.id = 0 }
#[inline] pub fn set_id(&mut self, v: u64) { self.id = v; }
#[inline] pub fn get_id(&self) -> u64 { self.id }
#[inline] pub fn clear_start_key(&mut self) { self.start_key.clear(); }
#[inline] pub fn set_start_key(&mut self, v: :: prost :: alloc :: vec :: Vec < u8 >) { self.start_key = v; }
#[inline] pub fn get_start_key(&self) -> &[u8] { &self.start_key }
#[inline] pub fn mut_start_key(&mut self) -> &mut :: prost :: alloc :: vec :: Vec < u8 > { &mut self.start_key }
#[inline] pub fn take_start_key(&mut self) -> :: prost :: alloc :: vec :: Vec < u8 > { ::std::mem::replace(&mut self.start_key, Default::default()) }
#[inline] pub fn clear_end_key(&mut self) { self.end_key.clear(); }
#[inline] pub fn set_end_key(&mut self, v: :: prost :: alloc :: vec :: Vec < u8 >) { self.end_key = v; }
#[inline] pub fn get_end_key(&self) -> &[u8] { &self.end_key }
#[inline] pub fn mut_end_key(&mut self) -> &mut :: prost :: alloc :: vec :: Vec < u8 > { &mut self.end_key }
#[inline] pub fn take_end_key(&mut self) -> :: prost :: alloc :: vec :: Vec < u8 > { ::std::mem::replace(&mut self.end_key, Default::default()) }
#[inline] pub fn has_region_epoch(&self) -> bool { self.region_epoch.is_some() }
#[inline] pub fn clear_region_epoch(&mut self) { self.region_epoch = ::std::option::Option::None }
#[inline] pub fn set_region_epoch(&mut self, v: RegionEpoch) { self.region_epoch = ::std::option::Option::Some(v); }
#[inline] pub fn get_region_epoch(&self) -> &RegionEpoch { match self.region_epoch.as_ref() {
                            Some(v) => v,
                            None => RegionEpoch::default_ref(),
                        } }
#[inline] pub fn mut_region_epoch(&mut self) -> &mut RegionEpoch { if self.region_epoch.is_none() {
                                self.region_epoch = ::std::option::Option::Some(RegionEpoch::default());
                            }
                            self.region_epoch.as_mut().unwrap() } 
#[inline] pub fn take_region_epoch(&mut self) -> RegionEpoch { self.region_epoch.take().unwrap_or_else(RegionEpoch::default) }
#[inline] pub fn clear_peers(&mut self) { self.peers.clear(); }
#[inline] pub fn set_peers(&mut self, v: ::std::vec::Vec<Peer>) { self.peers = v; }
#[inline] pub fn get_peers(&self) -> &[Peer] { &self.peers }
#[inline] pub fn mut_peers(&mut self) -> &mut ::std::vec::Vec<Peer> { &mut self.peers }
#[inline] pub fn take_peers(&mut self) -> ::std::vec::Vec<Peer> { ::std::mem::replace(&mut self.peers, ::std::vec::Vec::new()) }
#[inline] pub fn has_encryption_meta(&self) -> bool { self.encryption_meta.is_some() }
#[inline] pub fn clear_encryption_meta(&mut self) { self.encryption_meta = ::std::option::Option::None }
#[inline] pub fn set_encryption_meta(&mut self, v: super :: encryptionpb :: EncryptionMeta) { self.encryption_meta = ::std::option::Option::Some(v); }
#[inline] pub fn get_encryption_meta(&self) -> &super :: encryptionpb :: EncryptionMeta { match self.encryption_meta.as_ref() {
                            Some(v) => v,
                            None => super :: encryptionpb :: EncryptionMeta::default_ref(),
                        } }
#[inline] pub fn mut_encryption_meta(&mut self) -> &mut super :: encryptionpb :: EncryptionMeta { if self.encryption_meta.is_none() {
                                self.encryption_meta = ::std::option::Option::Some(super :: encryptionpb :: EncryptionMeta::default());
                            }
                            self.encryption_meta.as_mut().unwrap() } 
#[inline] pub fn take_encryption_meta(&mut self) -> super :: encryptionpb :: EncryptionMeta { self.encryption_meta.take().unwrap_or_else(super :: encryptionpb :: EncryptionMeta::default) }
}
impl ::protobuf::Clear for Region {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for Region {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static Region {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: Region = Region::default();
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
impl Peer {
pub fn new_() -> Peer { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn clear_id(&mut self) { self.id = 0 }
#[inline] pub fn set_id(&mut self, v: u64) { self.id = v; }
#[inline] pub fn get_id(&self) -> u64 { self.id }
#[inline] pub fn clear_store_id(&mut self) { self.store_id = 0 }
#[inline] pub fn set_store_id(&mut self, v: u64) { self.store_id = v; }
#[inline] pub fn get_store_id(&self) -> u64 { self.store_id }
#[inline] pub fn clear_role(&mut self) { self.role = 0 }
#[inline] pub fn get_role(&self) -> PeerRole { match PeerRole::from_i32(self.role) {Some(e) => e,
                        None => panic!("Unknown enum variant: {}", self.role),
                    } }
}
impl ::protobuf::Clear for Peer {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for Peer {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static Peer {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: Peer = Peer::default();
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
impl StoreState {
pub fn values() -> &'static [Self] {
static VALUES: &'static [StoreState] = &[
StoreState::Up,
StoreState::Offline,
StoreState::Tombstone,
];
VALUES
}
}
impl PeerRole {
pub fn values() -> &'static [Self] {
static VALUES: &'static [PeerRole] = &[
PeerRole::Voter,
PeerRole::Learner,
PeerRole::IncomingVoter,
PeerRole::DemotingVoter,
];
VALUES
}
}
