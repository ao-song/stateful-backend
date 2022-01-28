// Generated file, please don't edit manually.

impl Header {
pub fn new_() -> Header { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn clear_cluster_id(&mut self) { self.cluster_id = 0 }
#[inline] pub fn set_cluster_id(&mut self, v: u64) { self.cluster_id = v; }
#[inline] pub fn get_cluster_id(&self) -> u64 { self.cluster_id }
#[inline] pub fn clear_ticdc_version(&mut self) { self.ticdc_version.clear(); }
#[inline] pub fn set_ticdc_version(&mut self, v: :: prost :: alloc :: string :: String) { self.ticdc_version = v; }
#[inline] pub fn get_ticdc_version(&self) -> &str { &self.ticdc_version }
#[inline] pub fn mut_ticdc_version(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.ticdc_version }
#[inline] pub fn take_ticdc_version(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.ticdc_version, Default::default()) }
}
impl ::protobuf::Clear for Header {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for Header {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static Header {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: Header = Header::default();
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
impl DuplicateRequest {
pub fn new_() -> DuplicateRequest { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn clear_region_id(&mut self) { self.region_id = 0 }
#[inline] pub fn set_region_id(&mut self, v: u64) { self.region_id = v; }
#[inline] pub fn get_region_id(&self) -> u64 { self.region_id }
}
impl ::protobuf::Clear for DuplicateRequest {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for DuplicateRequest {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static DuplicateRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: DuplicateRequest = DuplicateRequest::default();
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
impl Compatibility {
pub fn new_() -> Compatibility { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn clear_required_version(&mut self) { self.required_version.clear(); }
#[inline] pub fn set_required_version(&mut self, v: :: prost :: alloc :: string :: String) { self.required_version = v; }
#[inline] pub fn get_required_version(&self) -> &str { &self.required_version }
#[inline] pub fn mut_required_version(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.required_version }
#[inline] pub fn take_required_version(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.required_version, Default::default()) }
}
impl ::protobuf::Clear for Compatibility {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for Compatibility {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static Compatibility {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: Compatibility = Compatibility::default();
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
impl Error {
pub fn new_() -> Error { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn has_not_leader(&self) -> bool { self.not_leader.is_some() }
#[inline] pub fn clear_not_leader(&mut self) { self.not_leader = ::std::option::Option::None }
#[inline] pub fn set_not_leader(&mut self, v: super :: errorpb :: NotLeader) { self.not_leader = ::std::option::Option::Some(v); }
#[inline] pub fn get_not_leader(&self) -> &super :: errorpb :: NotLeader { match self.not_leader.as_ref() {
                            Some(v) => v,
                            None => super :: errorpb :: NotLeader::default_ref(),
                        } }
#[inline] pub fn mut_not_leader(&mut self) -> &mut super :: errorpb :: NotLeader { if self.not_leader.is_none() {
                                self.not_leader = ::std::option::Option::Some(super :: errorpb :: NotLeader::default());
                            }
                            self.not_leader.as_mut().unwrap() } 
#[inline] pub fn take_not_leader(&mut self) -> super :: errorpb :: NotLeader { self.not_leader.take().unwrap_or_else(super :: errorpb :: NotLeader::default) }
#[inline] pub fn has_region_not_found(&self) -> bool { self.region_not_found.is_some() }
#[inline] pub fn clear_region_not_found(&mut self) { self.region_not_found = ::std::option::Option::None }
#[inline] pub fn set_region_not_found(&mut self, v: super :: errorpb :: RegionNotFound) { self.region_not_found = ::std::option::Option::Some(v); }
#[inline] pub fn get_region_not_found(&self) -> &super :: errorpb :: RegionNotFound { match self.region_not_found.as_ref() {
                            Some(v) => v,
                            None => super :: errorpb :: RegionNotFound::default_ref(),
                        } }
#[inline] pub fn mut_region_not_found(&mut self) -> &mut super :: errorpb :: RegionNotFound { if self.region_not_found.is_none() {
                                self.region_not_found = ::std::option::Option::Some(super :: errorpb :: RegionNotFound::default());
                            }
                            self.region_not_found.as_mut().unwrap() } 
#[inline] pub fn take_region_not_found(&mut self) -> super :: errorpb :: RegionNotFound { self.region_not_found.take().unwrap_or_else(super :: errorpb :: RegionNotFound::default) }
#[inline] pub fn has_epoch_not_match(&self) -> bool { self.epoch_not_match.is_some() }
#[inline] pub fn clear_epoch_not_match(&mut self) { self.epoch_not_match = ::std::option::Option::None }
#[inline] pub fn set_epoch_not_match(&mut self, v: super :: errorpb :: EpochNotMatch) { self.epoch_not_match = ::std::option::Option::Some(v); }
#[inline] pub fn get_epoch_not_match(&self) -> &super :: errorpb :: EpochNotMatch { match self.epoch_not_match.as_ref() {
                            Some(v) => v,
                            None => super :: errorpb :: EpochNotMatch::default_ref(),
                        } }
#[inline] pub fn mut_epoch_not_match(&mut self) -> &mut super :: errorpb :: EpochNotMatch { if self.epoch_not_match.is_none() {
                                self.epoch_not_match = ::std::option::Option::Some(super :: errorpb :: EpochNotMatch::default());
                            }
                            self.epoch_not_match.as_mut().unwrap() } 
#[inline] pub fn take_epoch_not_match(&mut self) -> super :: errorpb :: EpochNotMatch { self.epoch_not_match.take().unwrap_or_else(super :: errorpb :: EpochNotMatch::default) }
#[inline] pub fn has_duplicate_request(&self) -> bool { self.duplicate_request.is_some() }
#[inline] pub fn clear_duplicate_request(&mut self) { self.duplicate_request = ::std::option::Option::None }
#[inline] pub fn set_duplicate_request(&mut self, v: DuplicateRequest) { self.duplicate_request = ::std::option::Option::Some(v); }
#[inline] pub fn get_duplicate_request(&self) -> &DuplicateRequest { match self.duplicate_request.as_ref() {
                            Some(v) => v,
                            None => DuplicateRequest::default_ref(),
                        } }
#[inline] pub fn mut_duplicate_request(&mut self) -> &mut DuplicateRequest { if self.duplicate_request.is_none() {
                                self.duplicate_request = ::std::option::Option::Some(DuplicateRequest::default());
                            }
                            self.duplicate_request.as_mut().unwrap() } 
#[inline] pub fn take_duplicate_request(&mut self) -> DuplicateRequest { self.duplicate_request.take().unwrap_or_else(DuplicateRequest::default) }
#[inline] pub fn has_compatibility(&self) -> bool { self.compatibility.is_some() }
#[inline] pub fn clear_compatibility(&mut self) { self.compatibility = ::std::option::Option::None }
#[inline] pub fn set_compatibility(&mut self, v: Compatibility) { self.compatibility = ::std::option::Option::Some(v); }
#[inline] pub fn get_compatibility(&self) -> &Compatibility { match self.compatibility.as_ref() {
                            Some(v) => v,
                            None => Compatibility::default_ref(),
                        } }
#[inline] pub fn mut_compatibility(&mut self) -> &mut Compatibility { if self.compatibility.is_none() {
                                self.compatibility = ::std::option::Option::Some(Compatibility::default());
                            }
                            self.compatibility.as_mut().unwrap() } 
#[inline] pub fn take_compatibility(&mut self) -> Compatibility { self.compatibility.take().unwrap_or_else(Compatibility::default) }
}
impl ::protobuf::Clear for Error {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for Error {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static Error {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: Error = Error::default();
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
impl TxnInfo {
pub fn new_() -> TxnInfo { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn clear_start_ts(&mut self) { self.start_ts = 0 }
#[inline] pub fn set_start_ts(&mut self, v: u64) { self.start_ts = v; }
#[inline] pub fn get_start_ts(&self) -> u64 { self.start_ts }
#[inline] pub fn clear_primary(&mut self) { self.primary.clear(); }
#[inline] pub fn set_primary(&mut self, v: :: prost :: alloc :: vec :: Vec < u8 >) { self.primary = v; }
#[inline] pub fn get_primary(&self) -> &[u8] { &self.primary }
#[inline] pub fn mut_primary(&mut self) -> &mut :: prost :: alloc :: vec :: Vec < u8 > { &mut self.primary }
#[inline] pub fn take_primary(&mut self) -> :: prost :: alloc :: vec :: Vec < u8 > { ::std::mem::replace(&mut self.primary, Default::default()) }
}
impl ::protobuf::Clear for TxnInfo {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for TxnInfo {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static TxnInfo {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: TxnInfo = TxnInfo::default();
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
impl TxnStatus {
pub fn new_() -> TxnStatus { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn clear_start_ts(&mut self) { self.start_ts = 0 }
#[inline] pub fn set_start_ts(&mut self, v: u64) { self.start_ts = v; }
#[inline] pub fn get_start_ts(&self) -> u64 { self.start_ts }
#[inline] pub fn clear_min_commit_ts(&mut self) { self.min_commit_ts = 0 }
#[inline] pub fn set_min_commit_ts(&mut self, v: u64) { self.min_commit_ts = v; }
#[inline] pub fn get_min_commit_ts(&self) -> u64 { self.min_commit_ts }
#[inline] pub fn clear_commit_ts(&mut self) { self.commit_ts = 0 }
#[inline] pub fn set_commit_ts(&mut self, v: u64) { self.commit_ts = v; }
#[inline] pub fn get_commit_ts(&self) -> u64 { self.commit_ts }
#[inline] pub fn clear_is_rolled_back(&mut self) { self.is_rolled_back = false }
#[inline] pub fn set_is_rolled_back(&mut self, v: bool) { self.is_rolled_back = v; }
#[inline] pub fn get_is_rolled_back(&self) -> bool { self.is_rolled_back }
}
impl ::protobuf::Clear for TxnStatus {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for TxnStatus {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static TxnStatus {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: TxnStatus = TxnStatus::default();
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
impl Event {
pub fn new_() -> Event { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn clear_region_id(&mut self) { self.region_id = 0 }
#[inline] pub fn set_region_id(&mut self, v: u64) { self.region_id = v; }
#[inline] pub fn get_region_id(&self) -> u64 { self.region_id }
#[inline] pub fn clear_index(&mut self) { self.index = 0 }
#[inline] pub fn set_index(&mut self, v: u64) { self.index = v; }
#[inline] pub fn get_index(&self) -> u64 { self.index }
#[inline] pub fn clear_request_id(&mut self) { self.request_id = 0 }
#[inline] pub fn set_request_id(&mut self, v: u64) { self.request_id = v; }
#[inline] pub fn get_request_id(&self) -> u64 { self.request_id }
}
impl ::protobuf::Clear for Event {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for Event {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static Event {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: Event = Event::default();
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
impl event::Row {
pub fn new_() -> event::Row { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn clear_start_ts(&mut self) { self.start_ts = 0 }
#[inline] pub fn set_start_ts(&mut self, v: u64) { self.start_ts = v; }
#[inline] pub fn get_start_ts(&self) -> u64 { self.start_ts }
#[inline] pub fn clear_commit_ts(&mut self) { self.commit_ts = 0 }
#[inline] pub fn set_commit_ts(&mut self, v: u64) { self.commit_ts = v; }
#[inline] pub fn get_commit_ts(&self) -> u64 { self.commit_ts }
#[inline] pub fn clear_type(&mut self) { self.r#type = 0 }
#[inline] pub fn get_type(&self) -> event::LogType { match event :: LogType::from_i32(self.r#type) {Some(e) => e,
                        None => panic!("Unknown enum variant: {}", self.r#type),
                    } }
#[inline] pub fn clear_op_type(&mut self) { self.op_type = 0 }
#[inline] pub fn get_op_type(&self) -> event::row::OpType { match event :: row :: OpType::from_i32(self.op_type) {Some(e) => e,
                        None => panic!("Unknown enum variant: {}", self.op_type),
                    } }
#[inline] pub fn clear_key(&mut self) { self.key.clear(); }
#[inline] pub fn set_key(&mut self, v: :: prost :: alloc :: vec :: Vec < u8 >) { self.key = v; }
#[inline] pub fn get_key(&self) -> &[u8] { &self.key }
#[inline] pub fn mut_key(&mut self) -> &mut :: prost :: alloc :: vec :: Vec < u8 > { &mut self.key }
#[inline] pub fn take_key(&mut self) -> :: prost :: alloc :: vec :: Vec < u8 > { ::std::mem::replace(&mut self.key, Default::default()) }
#[inline] pub fn clear_value(&mut self) { self.value.clear(); }
#[inline] pub fn set_value(&mut self, v: :: prost :: alloc :: vec :: Vec < u8 >) { self.value = v; }
#[inline] pub fn get_value(&self) -> &[u8] { &self.value }
#[inline] pub fn mut_value(&mut self) -> &mut :: prost :: alloc :: vec :: Vec < u8 > { &mut self.value }
#[inline] pub fn take_value(&mut self) -> :: prost :: alloc :: vec :: Vec < u8 > { ::std::mem::replace(&mut self.value, Default::default()) }
#[inline] pub fn clear_old_value(&mut self) { self.old_value.clear(); }
#[inline] pub fn set_old_value(&mut self, v: :: prost :: alloc :: vec :: Vec < u8 >) { self.old_value = v; }
#[inline] pub fn get_old_value(&self) -> &[u8] { &self.old_value }
#[inline] pub fn mut_old_value(&mut self) -> &mut :: prost :: alloc :: vec :: Vec < u8 > { &mut self.old_value }
#[inline] pub fn take_old_value(&mut self) -> :: prost :: alloc :: vec :: Vec < u8 > { ::std::mem::replace(&mut self.old_value, Default::default()) }
}
impl ::protobuf::Clear for event::Row {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for event::Row {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static event::Row {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: event::Row = event::Row::default();
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
impl event::row::OpType {
pub fn values() -> &'static [Self] {
static VALUES: &'static [event::row::OpType] = &[
event::row::OpType::Unknown,
event::row::OpType::Put,
event::row::OpType::Delete,
];
VALUES
}
}
impl event::Entries {
pub fn new_() -> event::Entries { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn clear_entries(&mut self) { self.entries.clear(); }
#[inline] pub fn set_entries(&mut self, v: ::std::vec::Vec<event::Row>) { self.entries = v; }
#[inline] pub fn get_entries(&self) -> &[event::Row] { &self.entries }
#[inline] pub fn mut_entries(&mut self) -> &mut ::std::vec::Vec<event::Row> { &mut self.entries }
#[inline] pub fn take_entries(&mut self) -> ::std::vec::Vec<event::Row> { ::std::mem::replace(&mut self.entries, ::std::vec::Vec::new()) }
}
impl ::protobuf::Clear for event::Entries {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for event::Entries {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static event::Entries {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: event::Entries = event::Entries::default();
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
impl event::Admin {
pub fn new_() -> event::Admin { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn has_admin_request(&self) -> bool { self.admin_request.is_some() }
#[inline] pub fn clear_admin_request(&mut self) { self.admin_request = ::std::option::Option::None }
#[inline] pub fn set_admin_request(&mut self, v: super :: super :: raft_cmdpb :: AdminRequest) { self.admin_request = ::std::option::Option::Some(v); }
#[inline] pub fn get_admin_request(&self) -> &super :: super :: raft_cmdpb :: AdminRequest { match self.admin_request.as_ref() {
                            Some(v) => v,
                            None => super :: super :: raft_cmdpb :: AdminRequest::default_ref(),
                        } }
#[inline] pub fn mut_admin_request(&mut self) -> &mut super :: super :: raft_cmdpb :: AdminRequest { if self.admin_request.is_none() {
                                self.admin_request = ::std::option::Option::Some(super :: super :: raft_cmdpb :: AdminRequest::default());
                            }
                            self.admin_request.as_mut().unwrap() } 
#[inline] pub fn take_admin_request(&mut self) -> super :: super :: raft_cmdpb :: AdminRequest { self.admin_request.take().unwrap_or_else(super :: super :: raft_cmdpb :: AdminRequest::default) }
#[inline] pub fn has_admin_response(&self) -> bool { self.admin_response.is_some() }
#[inline] pub fn clear_admin_response(&mut self) { self.admin_response = ::std::option::Option::None }
#[inline] pub fn set_admin_response(&mut self, v: super :: super :: raft_cmdpb :: AdminResponse) { self.admin_response = ::std::option::Option::Some(v); }
#[inline] pub fn get_admin_response(&self) -> &super :: super :: raft_cmdpb :: AdminResponse { match self.admin_response.as_ref() {
                            Some(v) => v,
                            None => super :: super :: raft_cmdpb :: AdminResponse::default_ref(),
                        } }
#[inline] pub fn mut_admin_response(&mut self) -> &mut super :: super :: raft_cmdpb :: AdminResponse { if self.admin_response.is_none() {
                                self.admin_response = ::std::option::Option::Some(super :: super :: raft_cmdpb :: AdminResponse::default());
                            }
                            self.admin_response.as_mut().unwrap() } 
#[inline] pub fn take_admin_response(&mut self) -> super :: super :: raft_cmdpb :: AdminResponse { self.admin_response.take().unwrap_or_else(super :: super :: raft_cmdpb :: AdminResponse::default) }
}
impl ::protobuf::Clear for event::Admin {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for event::Admin {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static event::Admin {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: event::Admin = event::Admin::default();
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
impl event::LongTxn {
pub fn new_() -> event::LongTxn { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn clear_txn_info(&mut self) { self.txn_info.clear(); }
#[inline] pub fn set_txn_info(&mut self, v: ::std::vec::Vec< TxnInfo>) { self.txn_info = v; }
#[inline] pub fn get_txn_info(&self) -> &[ TxnInfo] { &self.txn_info }
#[inline] pub fn mut_txn_info(&mut self) -> &mut ::std::vec::Vec< TxnInfo> { &mut self.txn_info }
#[inline] pub fn take_txn_info(&mut self) -> ::std::vec::Vec< TxnInfo> { ::std::mem::replace(&mut self.txn_info, ::std::vec::Vec::new()) }
}
impl ::protobuf::Clear for event::LongTxn {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for event::LongTxn {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static event::LongTxn {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: event::LongTxn = event::LongTxn::default();
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
impl event::LogType {
pub fn values() -> &'static [Self] {
static VALUES: &'static [event::LogType] = &[
event::LogType::Unknown,
event::LogType::Prewrite,
event::LogType::Commit,
event::LogType::Rollback,
event::LogType::Committed,
event::LogType::Initialized,
];
VALUES
}
}
impl ChangeDataEvent {
pub fn new_() -> ChangeDataEvent { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn clear_events(&mut self) { self.events.clear(); }
#[inline] pub fn set_events(&mut self, v: ::std::vec::Vec<Event>) { self.events = v; }
#[inline] pub fn get_events(&self) -> &[Event] { &self.events }
#[inline] pub fn mut_events(&mut self) -> &mut ::std::vec::Vec<Event> { &mut self.events }
#[inline] pub fn take_events(&mut self) -> ::std::vec::Vec<Event> { ::std::mem::replace(&mut self.events, ::std::vec::Vec::new()) }
#[inline] pub fn has_resolved_ts(&self) -> bool { self.resolved_ts.is_some() }
#[inline] pub fn clear_resolved_ts(&mut self) { self.resolved_ts = ::std::option::Option::None }
#[inline] pub fn set_resolved_ts(&mut self, v: ResolvedTs) { self.resolved_ts = ::std::option::Option::Some(v); }
#[inline] pub fn get_resolved_ts(&self) -> &ResolvedTs { match self.resolved_ts.as_ref() {
                            Some(v) => v,
                            None => ResolvedTs::default_ref(),
                        } }
#[inline] pub fn mut_resolved_ts(&mut self) -> &mut ResolvedTs { if self.resolved_ts.is_none() {
                                self.resolved_ts = ::std::option::Option::Some(ResolvedTs::default());
                            }
                            self.resolved_ts.as_mut().unwrap() } 
#[inline] pub fn take_resolved_ts(&mut self) -> ResolvedTs { self.resolved_ts.take().unwrap_or_else(ResolvedTs::default) }
}
impl ::protobuf::Clear for ChangeDataEvent {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for ChangeDataEvent {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static ChangeDataEvent {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: ChangeDataEvent = ChangeDataEvent::default();
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
impl ResolvedTs {
pub fn new_() -> ResolvedTs { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn clear_regions(&mut self) { self.regions.clear(); }
#[inline] pub fn set_regions(&mut self, v: ::std::vec::Vec<u64>) { self.regions = v; }
#[inline] pub fn get_regions(&self) -> &[u64] { &self.regions }
#[inline] pub fn mut_regions(&mut self) -> &mut ::std::vec::Vec<u64> { &mut self.regions }
#[inline] pub fn take_regions(&mut self) -> ::std::vec::Vec<u64> { ::std::mem::replace(&mut self.regions, ::std::vec::Vec::new()) }
#[inline] pub fn clear_ts(&mut self) { self.ts = 0 }
#[inline] pub fn set_ts(&mut self, v: u64) { self.ts = v; }
#[inline] pub fn get_ts(&self) -> u64 { self.ts }
}
impl ::protobuf::Clear for ResolvedTs {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for ResolvedTs {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static ResolvedTs {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: ResolvedTs = ResolvedTs::default();
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
impl ChangeDataRequest {
pub fn new_() -> ChangeDataRequest { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn has_header(&self) -> bool { self.header.is_some() }
#[inline] pub fn clear_header(&mut self) { self.header = ::std::option::Option::None }
#[inline] pub fn set_header(&mut self, v: Header) { self.header = ::std::option::Option::Some(v); }
#[inline] pub fn get_header(&self) -> &Header { match self.header.as_ref() {
                            Some(v) => v,
                            None => Header::default_ref(),
                        } }
#[inline] pub fn mut_header(&mut self) -> &mut Header { if self.header.is_none() {
                                self.header = ::std::option::Option::Some(Header::default());
                            }
                            self.header.as_mut().unwrap() } 
#[inline] pub fn take_header(&mut self) -> Header { self.header.take().unwrap_or_else(Header::default) }
#[inline] pub fn clear_region_id(&mut self) { self.region_id = 0 }
#[inline] pub fn set_region_id(&mut self, v: u64) { self.region_id = v; }
#[inline] pub fn get_region_id(&self) -> u64 { self.region_id }
#[inline] pub fn has_region_epoch(&self) -> bool { self.region_epoch.is_some() }
#[inline] pub fn clear_region_epoch(&mut self) { self.region_epoch = ::std::option::Option::None }
#[inline] pub fn set_region_epoch(&mut self, v: super :: metapb :: RegionEpoch) { self.region_epoch = ::std::option::Option::Some(v); }
#[inline] pub fn get_region_epoch(&self) -> &super :: metapb :: RegionEpoch { match self.region_epoch.as_ref() {
                            Some(v) => v,
                            None => super :: metapb :: RegionEpoch::default_ref(),
                        } }
#[inline] pub fn mut_region_epoch(&mut self) -> &mut super :: metapb :: RegionEpoch { if self.region_epoch.is_none() {
                                self.region_epoch = ::std::option::Option::Some(super :: metapb :: RegionEpoch::default());
                            }
                            self.region_epoch.as_mut().unwrap() } 
#[inline] pub fn take_region_epoch(&mut self) -> super :: metapb :: RegionEpoch { self.region_epoch.take().unwrap_or_else(super :: metapb :: RegionEpoch::default) }
#[inline] pub fn clear_checkpoint_ts(&mut self) { self.checkpoint_ts = 0 }
#[inline] pub fn set_checkpoint_ts(&mut self, v: u64) { self.checkpoint_ts = v; }
#[inline] pub fn get_checkpoint_ts(&self) -> u64 { self.checkpoint_ts }
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
#[inline] pub fn clear_request_id(&mut self) { self.request_id = 0 }
#[inline] pub fn set_request_id(&mut self, v: u64) { self.request_id = v; }
#[inline] pub fn get_request_id(&self) -> u64 { self.request_id }
#[inline] pub fn clear_extra_op(&mut self) { self.extra_op = 0 }
#[inline] pub fn get_extra_op(&self) -> super::kvrpcpb::ExtraOp { match super :: kvrpcpb :: ExtraOp::from_i32(self.extra_op) {Some(e) => e,
                        None => panic!("Unknown enum variant: {}", self.extra_op),
                    } }
}
impl ::protobuf::Clear for ChangeDataRequest {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for ChangeDataRequest {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static ChangeDataRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: ChangeDataRequest = ChangeDataRequest::default();
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
impl change_data_request::Register {
pub fn new_() -> change_data_request::Register { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
}
impl ::protobuf::Clear for change_data_request::Register {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for change_data_request::Register {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static change_data_request::Register {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: change_data_request::Register = change_data_request::Register::default();
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
impl change_data_request::NotifyTxnStatus {
pub fn new_() -> change_data_request::NotifyTxnStatus { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn clear_txn_status(&mut self) { self.txn_status.clear(); }
#[inline] pub fn set_txn_status(&mut self, v: ::std::vec::Vec< TxnStatus>) { self.txn_status = v; }
#[inline] pub fn get_txn_status(&self) -> &[ TxnStatus] { &self.txn_status }
#[inline] pub fn mut_txn_status(&mut self) -> &mut ::std::vec::Vec< TxnStatus> { &mut self.txn_status }
#[inline] pub fn take_txn_status(&mut self) -> ::std::vec::Vec< TxnStatus> { ::std::mem::replace(&mut self.txn_status, ::std::vec::Vec::new()) }
}
impl ::protobuf::Clear for change_data_request::NotifyTxnStatus {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for change_data_request::NotifyTxnStatus {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static change_data_request::NotifyTxnStatus {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: change_data_request::NotifyTxnStatus = change_data_request::NotifyTxnStatus::default();
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
