// Generated file, please don't edit manually.

impl KeyRange {
pub fn new_() -> KeyRange { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn clear_start(&mut self) { self.start.clear(); }
#[inline] pub fn set_start(&mut self, v: :: prost :: alloc :: vec :: Vec < u8 >) { self.start = v; }
#[inline] pub fn get_start(&self) -> &[u8] { &self.start }
#[inline] pub fn mut_start(&mut self) -> &mut :: prost :: alloc :: vec :: Vec < u8 > { &mut self.start }
#[inline] pub fn take_start(&mut self) -> :: prost :: alloc :: vec :: Vec < u8 > { ::std::mem::replace(&mut self.start, Default::default()) }
#[inline] pub fn clear_end(&mut self) { self.end.clear(); }
#[inline] pub fn set_end(&mut self, v: :: prost :: alloc :: vec :: Vec < u8 >) { self.end = v; }
#[inline] pub fn get_end(&self) -> &[u8] { &self.end }
#[inline] pub fn mut_end(&mut self) -> &mut :: prost :: alloc :: vec :: Vec < u8 > { &mut self.end }
#[inline] pub fn take_end(&mut self) -> :: prost :: alloc :: vec :: Vec < u8 > { ::std::mem::replace(&mut self.end, Default::default()) }
}
impl ::protobuf::Clear for KeyRange {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for KeyRange {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static KeyRange {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: KeyRange = KeyRange::default();
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
impl Request {
pub fn new_() -> Request { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn has_context(&self) -> bool { self.context.is_some() }
#[inline] pub fn clear_context(&mut self) { self.context = ::std::option::Option::None }
#[inline] pub fn set_context(&mut self, v: super :: kvrpcpb :: Context) { self.context = ::std::option::Option::Some(v); }
#[inline] pub fn get_context(&self) -> &super :: kvrpcpb :: Context { match self.context.as_ref() {
                            Some(v) => v,
                            None => super :: kvrpcpb :: Context::default_ref(),
                        } }
#[inline] pub fn mut_context(&mut self) -> &mut super :: kvrpcpb :: Context { if self.context.is_none() {
                                self.context = ::std::option::Option::Some(super :: kvrpcpb :: Context::default());
                            }
                            self.context.as_mut().unwrap() } 
#[inline] pub fn take_context(&mut self) -> super :: kvrpcpb :: Context { self.context.take().unwrap_or_else(super :: kvrpcpb :: Context::default) }
#[inline] pub fn clear_tp(&mut self) { self.tp = 0 }
#[inline] pub fn set_tp(&mut self, v: i64) { self.tp = v; }
#[inline] pub fn get_tp(&self) -> i64 { self.tp }
#[inline] pub fn clear_data(&mut self) { self.data.clear(); }
#[inline] pub fn set_data(&mut self, v: :: prost :: alloc :: vec :: Vec < u8 >) { self.data = v; }
#[inline] pub fn get_data(&self) -> &[u8] { &self.data }
#[inline] pub fn mut_data(&mut self) -> &mut :: prost :: alloc :: vec :: Vec < u8 > { &mut self.data }
#[inline] pub fn take_data(&mut self) -> :: prost :: alloc :: vec :: Vec < u8 > { ::std::mem::replace(&mut self.data, Default::default()) }
#[inline] pub fn clear_start_ts(&mut self) { self.start_ts = 0 }
#[inline] pub fn set_start_ts(&mut self, v: u64) { self.start_ts = v; }
#[inline] pub fn get_start_ts(&self) -> u64 { self.start_ts }
#[inline] pub fn clear_ranges(&mut self) { self.ranges.clear(); }
#[inline] pub fn set_ranges(&mut self, v: ::std::vec::Vec<KeyRange>) { self.ranges = v; }
#[inline] pub fn get_ranges(&self) -> &[KeyRange] { &self.ranges }
#[inline] pub fn mut_ranges(&mut self) -> &mut ::std::vec::Vec<KeyRange> { &mut self.ranges }
#[inline] pub fn take_ranges(&mut self) -> ::std::vec::Vec<KeyRange> { ::std::mem::replace(&mut self.ranges, ::std::vec::Vec::new()) }
#[inline] pub fn clear_is_cache_enabled(&mut self) { self.is_cache_enabled = false }
#[inline] pub fn set_is_cache_enabled(&mut self, v: bool) { self.is_cache_enabled = v; }
#[inline] pub fn get_is_cache_enabled(&self) -> bool { self.is_cache_enabled }
#[inline] pub fn clear_cache_if_match_version(&mut self) { self.cache_if_match_version = 0 }
#[inline] pub fn set_cache_if_match_version(&mut self, v: u64) { self.cache_if_match_version = v; }
#[inline] pub fn get_cache_if_match_version(&self) -> u64 { self.cache_if_match_version }
#[inline] pub fn clear_schema_ver(&mut self) { self.schema_ver = 0 }
#[inline] pub fn set_schema_ver(&mut self, v: i64) { self.schema_ver = v; }
#[inline] pub fn get_schema_ver(&self) -> i64 { self.schema_ver }
#[inline] pub fn clear_is_trace_enabled(&mut self) { self.is_trace_enabled = false }
#[inline] pub fn set_is_trace_enabled(&mut self, v: bool) { self.is_trace_enabled = v; }
#[inline] pub fn get_is_trace_enabled(&self) -> bool { self.is_trace_enabled }
}
impl ::protobuf::Clear for Request {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for Request {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static Request {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: Request = Request::default();
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
impl Response {
pub fn new_() -> Response { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn clear_data(&mut self) { self.data.clear(); }
#[inline] pub fn set_data(&mut self, v: :: prost :: alloc :: vec :: Vec < u8 >) { self.data = v; }
#[inline] pub fn get_data(&self) -> &[u8] { &self.data }
#[inline] pub fn mut_data(&mut self) -> &mut :: prost :: alloc :: vec :: Vec < u8 > { &mut self.data }
#[inline] pub fn take_data(&mut self) -> :: prost :: alloc :: vec :: Vec < u8 > { ::std::mem::replace(&mut self.data, Default::default()) }
#[inline] pub fn has_region_error(&self) -> bool { self.region_error.is_some() }
#[inline] pub fn clear_region_error(&mut self) { self.region_error = ::std::option::Option::None }
#[inline] pub fn set_region_error(&mut self, v: super :: errorpb :: Error) { self.region_error = ::std::option::Option::Some(v); }
#[inline] pub fn get_region_error(&self) -> &super :: errorpb :: Error { match self.region_error.as_ref() {
                            Some(v) => v,
                            None => super :: errorpb :: Error::default_ref(),
                        } }
#[inline] pub fn mut_region_error(&mut self) -> &mut super :: errorpb :: Error { if self.region_error.is_none() {
                                self.region_error = ::std::option::Option::Some(super :: errorpb :: Error::default());
                            }
                            self.region_error.as_mut().unwrap() } 
#[inline] pub fn take_region_error(&mut self) -> super :: errorpb :: Error { self.region_error.take().unwrap_or_else(super :: errorpb :: Error::default) }
#[inline] pub fn has_locked(&self) -> bool { self.locked.is_some() }
#[inline] pub fn clear_locked(&mut self) { self.locked = ::std::option::Option::None }
#[inline] pub fn set_locked(&mut self, v: super :: kvrpcpb :: LockInfo) { self.locked = ::std::option::Option::Some(v); }
#[inline] pub fn get_locked(&self) -> &super :: kvrpcpb :: LockInfo { match self.locked.as_ref() {
                            Some(v) => v,
                            None => super :: kvrpcpb :: LockInfo::default_ref(),
                        } }
#[inline] pub fn mut_locked(&mut self) -> &mut super :: kvrpcpb :: LockInfo { if self.locked.is_none() {
                                self.locked = ::std::option::Option::Some(super :: kvrpcpb :: LockInfo::default());
                            }
                            self.locked.as_mut().unwrap() } 
#[inline] pub fn take_locked(&mut self) -> super :: kvrpcpb :: LockInfo { self.locked.take().unwrap_or_else(super :: kvrpcpb :: LockInfo::default) }
#[inline] pub fn clear_other_error(&mut self) { self.other_error.clear(); }
#[inline] pub fn set_other_error(&mut self, v: :: prost :: alloc :: string :: String) { self.other_error = v; }
#[inline] pub fn get_other_error(&self) -> &str { &self.other_error }
#[inline] pub fn mut_other_error(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.other_error }
#[inline] pub fn take_other_error(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.other_error, Default::default()) }
#[inline] pub fn has_range(&self) -> bool { self.range.is_some() }
#[inline] pub fn clear_range(&mut self) { self.range = ::std::option::Option::None }
#[inline] pub fn set_range(&mut self, v: KeyRange) { self.range = ::std::option::Option::Some(v); }
#[inline] pub fn get_range(&self) -> &KeyRange { match self.range.as_ref() {
                            Some(v) => v,
                            None => KeyRange::default_ref(),
                        } }
#[inline] pub fn mut_range(&mut self) -> &mut KeyRange { if self.range.is_none() {
                                self.range = ::std::option::Option::Some(KeyRange::default());
                            }
                            self.range.as_mut().unwrap() } 
#[inline] pub fn take_range(&mut self) -> KeyRange { self.range.take().unwrap_or_else(KeyRange::default) }
#[inline] pub fn has_exec_details(&self) -> bool { self.exec_details.is_some() }
#[inline] pub fn clear_exec_details(&mut self) { self.exec_details = ::std::option::Option::None }
#[inline] pub fn set_exec_details(&mut self, v: super :: kvrpcpb :: ExecDetails) { self.exec_details = ::std::option::Option::Some(v); }
#[inline] pub fn get_exec_details(&self) -> &super :: kvrpcpb :: ExecDetails { match self.exec_details.as_ref() {
                            Some(v) => v,
                            None => super :: kvrpcpb :: ExecDetails::default_ref(),
                        } }
#[inline] pub fn mut_exec_details(&mut self) -> &mut super :: kvrpcpb :: ExecDetails { if self.exec_details.is_none() {
                                self.exec_details = ::std::option::Option::Some(super :: kvrpcpb :: ExecDetails::default());
                            }
                            self.exec_details.as_mut().unwrap() } 
#[inline] pub fn take_exec_details(&mut self) -> super :: kvrpcpb :: ExecDetails { self.exec_details.take().unwrap_or_else(super :: kvrpcpb :: ExecDetails::default) }
#[inline] pub fn has_exec_details_v2(&self) -> bool { self.exec_details_v2.is_some() }
#[inline] pub fn clear_exec_details_v2(&mut self) { self.exec_details_v2 = ::std::option::Option::None }
#[inline] pub fn set_exec_details_v2(&mut self, v: super :: kvrpcpb :: ExecDetailsV2) { self.exec_details_v2 = ::std::option::Option::Some(v); }
#[inline] pub fn get_exec_details_v2(&self) -> &super :: kvrpcpb :: ExecDetailsV2 { match self.exec_details_v2.as_ref() {
                            Some(v) => v,
                            None => super :: kvrpcpb :: ExecDetailsV2::default_ref(),
                        } }
#[inline] pub fn mut_exec_details_v2(&mut self) -> &mut super :: kvrpcpb :: ExecDetailsV2 { if self.exec_details_v2.is_none() {
                                self.exec_details_v2 = ::std::option::Option::Some(super :: kvrpcpb :: ExecDetailsV2::default());
                            }
                            self.exec_details_v2.as_mut().unwrap() } 
#[inline] pub fn take_exec_details_v2(&mut self) -> super :: kvrpcpb :: ExecDetailsV2 { self.exec_details_v2.take().unwrap_or_else(super :: kvrpcpb :: ExecDetailsV2::default) }
#[inline] pub fn clear_is_cache_hit(&mut self) { self.is_cache_hit = false }
#[inline] pub fn set_is_cache_hit(&mut self, v: bool) { self.is_cache_hit = v; }
#[inline] pub fn get_is_cache_hit(&self) -> bool { self.is_cache_hit }
#[inline] pub fn clear_cache_last_version(&mut self) { self.cache_last_version = 0 }
#[inline] pub fn set_cache_last_version(&mut self, v: u64) { self.cache_last_version = v; }
#[inline] pub fn get_cache_last_version(&self) -> u64 { self.cache_last_version }
#[inline] pub fn clear_can_be_cached(&mut self) { self.can_be_cached = false }
#[inline] pub fn set_can_be_cached(&mut self, v: bool) { self.can_be_cached = v; }
#[inline] pub fn get_can_be_cached(&self) -> bool { self.can_be_cached }
#[inline] pub fn clear_spans(&mut self) { self.spans.clear(); }
#[inline] pub fn set_spans(&mut self, v: ::std::vec::Vec<super :: span :: SpanSet>) { self.spans = v; }
#[inline] pub fn get_spans(&self) -> &[super :: span :: SpanSet] { &self.spans }
#[inline] pub fn mut_spans(&mut self) -> &mut ::std::vec::Vec<super :: span :: SpanSet> { &mut self.spans }
#[inline] pub fn take_spans(&mut self) -> ::std::vec::Vec<super :: span :: SpanSet> { ::std::mem::replace(&mut self.spans, ::std::vec::Vec::new()) }
}
impl ::protobuf::Clear for Response {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for Response {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static Response {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: Response = Response::default();
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
impl RegionInfo {
pub fn new_() -> RegionInfo { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
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
#[inline] pub fn clear_ranges(&mut self) { self.ranges.clear(); }
#[inline] pub fn set_ranges(&mut self, v: ::std::vec::Vec<KeyRange>) { self.ranges = v; }
#[inline] pub fn get_ranges(&self) -> &[KeyRange] { &self.ranges }
#[inline] pub fn mut_ranges(&mut self) -> &mut ::std::vec::Vec<KeyRange> { &mut self.ranges }
#[inline] pub fn take_ranges(&mut self) -> ::std::vec::Vec<KeyRange> { ::std::mem::replace(&mut self.ranges, ::std::vec::Vec::new()) }
}
impl ::protobuf::Clear for RegionInfo {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for RegionInfo {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static RegionInfo {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: RegionInfo = RegionInfo::default();
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
impl BatchRequest {
pub fn new_() -> BatchRequest { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn has_context(&self) -> bool { self.context.is_some() }
#[inline] pub fn clear_context(&mut self) { self.context = ::std::option::Option::None }
#[inline] pub fn set_context(&mut self, v: super :: kvrpcpb :: Context) { self.context = ::std::option::Option::Some(v); }
#[inline] pub fn get_context(&self) -> &super :: kvrpcpb :: Context { match self.context.as_ref() {
                            Some(v) => v,
                            None => super :: kvrpcpb :: Context::default_ref(),
                        } }
#[inline] pub fn mut_context(&mut self) -> &mut super :: kvrpcpb :: Context { if self.context.is_none() {
                                self.context = ::std::option::Option::Some(super :: kvrpcpb :: Context::default());
                            }
                            self.context.as_mut().unwrap() } 
#[inline] pub fn take_context(&mut self) -> super :: kvrpcpb :: Context { self.context.take().unwrap_or_else(super :: kvrpcpb :: Context::default) }
#[inline] pub fn clear_tp(&mut self) { self.tp = 0 }
#[inline] pub fn set_tp(&mut self, v: i64) { self.tp = v; }
#[inline] pub fn get_tp(&self) -> i64 { self.tp }
#[inline] pub fn clear_data(&mut self) { self.data.clear(); }
#[inline] pub fn set_data(&mut self, v: :: prost :: alloc :: vec :: Vec < u8 >) { self.data = v; }
#[inline] pub fn get_data(&self) -> &[u8] { &self.data }
#[inline] pub fn mut_data(&mut self) -> &mut :: prost :: alloc :: vec :: Vec < u8 > { &mut self.data }
#[inline] pub fn take_data(&mut self) -> :: prost :: alloc :: vec :: Vec < u8 > { ::std::mem::replace(&mut self.data, Default::default()) }
#[inline] pub fn clear_regions(&mut self) { self.regions.clear(); }
#[inline] pub fn set_regions(&mut self, v: ::std::vec::Vec<RegionInfo>) { self.regions = v; }
#[inline] pub fn get_regions(&self) -> &[RegionInfo] { &self.regions }
#[inline] pub fn mut_regions(&mut self) -> &mut ::std::vec::Vec<RegionInfo> { &mut self.regions }
#[inline] pub fn take_regions(&mut self) -> ::std::vec::Vec<RegionInfo> { ::std::mem::replace(&mut self.regions, ::std::vec::Vec::new()) }
#[inline] pub fn clear_start_ts(&mut self) { self.start_ts = 0 }
#[inline] pub fn set_start_ts(&mut self, v: u64) { self.start_ts = v; }
#[inline] pub fn get_start_ts(&self) -> u64 { self.start_ts }
#[inline] pub fn clear_schema_ver(&mut self) { self.schema_ver = 0 }
#[inline] pub fn set_schema_ver(&mut self, v: i64) { self.schema_ver = v; }
#[inline] pub fn get_schema_ver(&self) -> i64 { self.schema_ver }
}
impl ::protobuf::Clear for BatchRequest {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for BatchRequest {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static BatchRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: BatchRequest = BatchRequest::default();
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
impl BatchResponse {
pub fn new_() -> BatchResponse { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn clear_data(&mut self) { self.data.clear(); }
#[inline] pub fn set_data(&mut self, v: :: prost :: alloc :: vec :: Vec < u8 >) { self.data = v; }
#[inline] pub fn get_data(&self) -> &[u8] { &self.data }
#[inline] pub fn mut_data(&mut self) -> &mut :: prost :: alloc :: vec :: Vec < u8 > { &mut self.data }
#[inline] pub fn take_data(&mut self) -> :: prost :: alloc :: vec :: Vec < u8 > { ::std::mem::replace(&mut self.data, Default::default()) }
#[inline] pub fn clear_other_error(&mut self) { self.other_error.clear(); }
#[inline] pub fn set_other_error(&mut self, v: :: prost :: alloc :: string :: String) { self.other_error = v; }
#[inline] pub fn get_other_error(&self) -> &str { &self.other_error }
#[inline] pub fn mut_other_error(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.other_error }
#[inline] pub fn take_other_error(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.other_error, Default::default()) }
#[inline] pub fn has_exec_details(&self) -> bool { self.exec_details.is_some() }
#[inline] pub fn clear_exec_details(&mut self) { self.exec_details = ::std::option::Option::None }
#[inline] pub fn set_exec_details(&mut self, v: super :: kvrpcpb :: ExecDetails) { self.exec_details = ::std::option::Option::Some(v); }
#[inline] pub fn get_exec_details(&self) -> &super :: kvrpcpb :: ExecDetails { match self.exec_details.as_ref() {
                            Some(v) => v,
                            None => super :: kvrpcpb :: ExecDetails::default_ref(),
                        } }
#[inline] pub fn mut_exec_details(&mut self) -> &mut super :: kvrpcpb :: ExecDetails { if self.exec_details.is_none() {
                                self.exec_details = ::std::option::Option::Some(super :: kvrpcpb :: ExecDetails::default());
                            }
                            self.exec_details.as_mut().unwrap() } 
#[inline] pub fn take_exec_details(&mut self) -> super :: kvrpcpb :: ExecDetails { self.exec_details.take().unwrap_or_else(super :: kvrpcpb :: ExecDetails::default) }
}
impl ::protobuf::Clear for BatchResponse {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for BatchResponse {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static BatchResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: BatchResponse = BatchResponse::default();
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
