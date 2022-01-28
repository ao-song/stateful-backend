// Generated file, please don't edit manually.

impl SwitchModeRequest {
pub fn new_() -> SwitchModeRequest { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn clear_mode(&mut self) { self.mode = 0 }
#[inline] pub fn get_mode(&self) -> SwitchMode { match SwitchMode::from_i32(self.mode) {Some(e) => e,
                        None => panic!("Unknown enum variant: {}", self.mode),
                    } }
}
impl ::protobuf::Clear for SwitchModeRequest {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for SwitchModeRequest {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static SwitchModeRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: SwitchModeRequest = SwitchModeRequest::default();
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
impl SwitchModeResponse {
pub fn new_() -> SwitchModeResponse { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
}
impl ::protobuf::Clear for SwitchModeResponse {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for SwitchModeResponse {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static SwitchModeResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: SwitchModeResponse = SwitchModeResponse::default();
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
impl Range {
pub fn new_() -> Range { ::std::default::Default::default() }
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
impl ::protobuf::Clear for Range {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for Range {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static Range {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: Range = Range::default();
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
impl SstMeta {
pub fn new_() -> SstMeta { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn clear_uuid(&mut self) { self.uuid.clear(); }
#[inline] pub fn set_uuid(&mut self, v: :: prost :: alloc :: vec :: Vec < u8 >) { self.uuid = v; }
#[inline] pub fn get_uuid(&self) -> &[u8] { &self.uuid }
#[inline] pub fn mut_uuid(&mut self) -> &mut :: prost :: alloc :: vec :: Vec < u8 > { &mut self.uuid }
#[inline] pub fn take_uuid(&mut self) -> :: prost :: alloc :: vec :: Vec < u8 > { ::std::mem::replace(&mut self.uuid, Default::default()) }
#[inline] pub fn has_range(&self) -> bool { self.range.is_some() }
#[inline] pub fn clear_range(&mut self) { self.range = ::std::option::Option::None }
#[inline] pub fn set_range(&mut self, v: Range) { self.range = ::std::option::Option::Some(v); }
#[inline] pub fn get_range(&self) -> &Range { match self.range.as_ref() {
                            Some(v) => v,
                            None => Range::default_ref(),
                        } }
#[inline] pub fn mut_range(&mut self) -> &mut Range { if self.range.is_none() {
                                self.range = ::std::option::Option::Some(Range::default());
                            }
                            self.range.as_mut().unwrap() } 
#[inline] pub fn take_range(&mut self) -> Range { self.range.take().unwrap_or_else(Range::default) }
#[inline] pub fn clear_crc32(&mut self) { self.crc32 = 0 }
#[inline] pub fn set_crc32(&mut self, v: u32) { self.crc32 = v; }
#[inline] pub fn get_crc32(&self) -> u32 { self.crc32 }
#[inline] pub fn clear_length(&mut self) { self.length = 0 }
#[inline] pub fn set_length(&mut self, v: u64) { self.length = v; }
#[inline] pub fn get_length(&self) -> u64 { self.length }
#[inline] pub fn clear_cf_name(&mut self) { self.cf_name.clear(); }
#[inline] pub fn set_cf_name(&mut self, v: :: prost :: alloc :: string :: String) { self.cf_name = v; }
#[inline] pub fn get_cf_name(&self) -> &str { &self.cf_name }
#[inline] pub fn mut_cf_name(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.cf_name }
#[inline] pub fn take_cf_name(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.cf_name, Default::default()) }
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
#[inline] pub fn clear_end_key_exclusive(&mut self) { self.end_key_exclusive = false }
#[inline] pub fn set_end_key_exclusive(&mut self, v: bool) { self.end_key_exclusive = v; }
#[inline] pub fn get_end_key_exclusive(&self) -> bool { self.end_key_exclusive }
#[inline] pub fn clear_total_kvs(&mut self) { self.total_kvs = 0 }
#[inline] pub fn set_total_kvs(&mut self, v: u64) { self.total_kvs = v; }
#[inline] pub fn get_total_kvs(&self) -> u64 { self.total_kvs }
#[inline] pub fn clear_total_bytes(&mut self) { self.total_bytes = 0 }
#[inline] pub fn set_total_bytes(&mut self, v: u64) { self.total_bytes = v; }
#[inline] pub fn get_total_bytes(&self) -> u64 { self.total_bytes }
}
impl ::protobuf::Clear for SstMeta {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for SstMeta {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static SstMeta {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: SstMeta = SstMeta::default();
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
impl RewriteRule {
pub fn new_() -> RewriteRule { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn clear_old_key_prefix(&mut self) { self.old_key_prefix.clear(); }
#[inline] pub fn set_old_key_prefix(&mut self, v: :: prost :: alloc :: vec :: Vec < u8 >) { self.old_key_prefix = v; }
#[inline] pub fn get_old_key_prefix(&self) -> &[u8] { &self.old_key_prefix }
#[inline] pub fn mut_old_key_prefix(&mut self) -> &mut :: prost :: alloc :: vec :: Vec < u8 > { &mut self.old_key_prefix }
#[inline] pub fn take_old_key_prefix(&mut self) -> :: prost :: alloc :: vec :: Vec < u8 > { ::std::mem::replace(&mut self.old_key_prefix, Default::default()) }
#[inline] pub fn clear_new_key_prefix(&mut self) { self.new_key_prefix.clear(); }
#[inline] pub fn set_new_key_prefix(&mut self, v: :: prost :: alloc :: vec :: Vec < u8 >) { self.new_key_prefix = v; }
#[inline] pub fn get_new_key_prefix(&self) -> &[u8] { &self.new_key_prefix }
#[inline] pub fn mut_new_key_prefix(&mut self) -> &mut :: prost :: alloc :: vec :: Vec < u8 > { &mut self.new_key_prefix }
#[inline] pub fn take_new_key_prefix(&mut self) -> :: prost :: alloc :: vec :: Vec < u8 > { ::std::mem::replace(&mut self.new_key_prefix, Default::default()) }
#[inline] pub fn clear_new_timestamp(&mut self) { self.new_timestamp = 0 }
#[inline] pub fn set_new_timestamp(&mut self, v: u64) { self.new_timestamp = v; }
#[inline] pub fn get_new_timestamp(&self) -> u64 { self.new_timestamp }
}
impl ::protobuf::Clear for RewriteRule {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for RewriteRule {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static RewriteRule {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: RewriteRule = RewriteRule::default();
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
impl UploadRequest {
pub fn new_() -> UploadRequest { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
}
impl ::protobuf::Clear for UploadRequest {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for UploadRequest {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static UploadRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: UploadRequest = UploadRequest::default();
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
impl UploadResponse {
pub fn new_() -> UploadResponse { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
}
impl ::protobuf::Clear for UploadResponse {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for UploadResponse {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static UploadResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: UploadResponse = UploadResponse::default();
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
impl IngestRequest {
pub fn new_() -> IngestRequest { ::std::default::Default::default() }
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
#[inline] pub fn has_sst(&self) -> bool { self.sst.is_some() }
#[inline] pub fn clear_sst(&mut self) { self.sst = ::std::option::Option::None }
#[inline] pub fn set_sst(&mut self, v: SstMeta) { self.sst = ::std::option::Option::Some(v); }
#[inline] pub fn get_sst(&self) -> &SstMeta { match self.sst.as_ref() {
                            Some(v) => v,
                            None => SstMeta::default_ref(),
                        } }
#[inline] pub fn mut_sst(&mut self) -> &mut SstMeta { if self.sst.is_none() {
                                self.sst = ::std::option::Option::Some(SstMeta::default());
                            }
                            self.sst.as_mut().unwrap() } 
#[inline] pub fn take_sst(&mut self) -> SstMeta { self.sst.take().unwrap_or_else(SstMeta::default) }
}
impl ::protobuf::Clear for IngestRequest {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for IngestRequest {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static IngestRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: IngestRequest = IngestRequest::default();
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
impl IngestResponse {
pub fn new_() -> IngestResponse { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn has_error(&self) -> bool { self.error.is_some() }
#[inline] pub fn clear_error(&mut self) { self.error = ::std::option::Option::None }
#[inline] pub fn set_error(&mut self, v: super :: errorpb :: Error) { self.error = ::std::option::Option::Some(v); }
#[inline] pub fn get_error(&self) -> &super :: errorpb :: Error { match self.error.as_ref() {
                            Some(v) => v,
                            None => super :: errorpb :: Error::default_ref(),
                        } }
#[inline] pub fn mut_error(&mut self) -> &mut super :: errorpb :: Error { if self.error.is_none() {
                                self.error = ::std::option::Option::Some(super :: errorpb :: Error::default());
                            }
                            self.error.as_mut().unwrap() } 
#[inline] pub fn take_error(&mut self) -> super :: errorpb :: Error { self.error.take().unwrap_or_else(super :: errorpb :: Error::default) }
}
impl ::protobuf::Clear for IngestResponse {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for IngestResponse {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static IngestResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: IngestResponse = IngestResponse::default();
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
impl CompactRequest {
pub fn new_() -> CompactRequest { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn has_range(&self) -> bool { self.range.is_some() }
#[inline] pub fn clear_range(&mut self) { self.range = ::std::option::Option::None }
#[inline] pub fn set_range(&mut self, v: Range) { self.range = ::std::option::Option::Some(v); }
#[inline] pub fn get_range(&self) -> &Range { match self.range.as_ref() {
                            Some(v) => v,
                            None => Range::default_ref(),
                        } }
#[inline] pub fn mut_range(&mut self) -> &mut Range { if self.range.is_none() {
                                self.range = ::std::option::Option::Some(Range::default());
                            }
                            self.range.as_mut().unwrap() } 
#[inline] pub fn take_range(&mut self) -> Range { self.range.take().unwrap_or_else(Range::default) }
#[inline] pub fn clear_output_level(&mut self) { self.output_level = 0 }
#[inline] pub fn set_output_level(&mut self, v: i32) { self.output_level = v; }
#[inline] pub fn get_output_level(&self) -> i32 { self.output_level }
}
impl ::protobuf::Clear for CompactRequest {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for CompactRequest {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static CompactRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: CompactRequest = CompactRequest::default();
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
impl CompactResponse {
pub fn new_() -> CompactResponse { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
}
impl ::protobuf::Clear for CompactResponse {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for CompactResponse {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static CompactResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: CompactResponse = CompactResponse::default();
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
impl DownloadRequest {
pub fn new_() -> DownloadRequest { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn has_sst(&self) -> bool { self.sst.is_some() }
#[inline] pub fn clear_sst(&mut self) { self.sst = ::std::option::Option::None }
#[inline] pub fn set_sst(&mut self, v: SstMeta) { self.sst = ::std::option::Option::Some(v); }
#[inline] pub fn get_sst(&self) -> &SstMeta { match self.sst.as_ref() {
                            Some(v) => v,
                            None => SstMeta::default_ref(),
                        } }
#[inline] pub fn mut_sst(&mut self) -> &mut SstMeta { if self.sst.is_none() {
                                self.sst = ::std::option::Option::Some(SstMeta::default());
                            }
                            self.sst.as_mut().unwrap() } 
#[inline] pub fn take_sst(&mut self) -> SstMeta { self.sst.take().unwrap_or_else(SstMeta::default) }
#[inline] pub fn clear_name(&mut self) { self.name.clear(); }
#[inline] pub fn set_name(&mut self, v: :: prost :: alloc :: string :: String) { self.name = v; }
#[inline] pub fn get_name(&self) -> &str { &self.name }
#[inline] pub fn mut_name(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.name }
#[inline] pub fn take_name(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.name, Default::default()) }
#[inline] pub fn has_rewrite_rule(&self) -> bool { self.rewrite_rule.is_some() }
#[inline] pub fn clear_rewrite_rule(&mut self) { self.rewrite_rule = ::std::option::Option::None }
#[inline] pub fn set_rewrite_rule(&mut self, v: RewriteRule) { self.rewrite_rule = ::std::option::Option::Some(v); }
#[inline] pub fn get_rewrite_rule(&self) -> &RewriteRule { match self.rewrite_rule.as_ref() {
                            Some(v) => v,
                            None => RewriteRule::default_ref(),
                        } }
#[inline] pub fn mut_rewrite_rule(&mut self) -> &mut RewriteRule { if self.rewrite_rule.is_none() {
                                self.rewrite_rule = ::std::option::Option::Some(RewriteRule::default());
                            }
                            self.rewrite_rule.as_mut().unwrap() } 
#[inline] pub fn take_rewrite_rule(&mut self) -> RewriteRule { self.rewrite_rule.take().unwrap_or_else(RewriteRule::default) }
#[inline] pub fn has_storage_backend(&self) -> bool { self.storage_backend.is_some() }
#[inline] pub fn clear_storage_backend(&mut self) { self.storage_backend = ::std::option::Option::None }
#[inline] pub fn set_storage_backend(&mut self, v: super :: backup :: StorageBackend) { self.storage_backend = ::std::option::Option::Some(v); }
#[inline] pub fn get_storage_backend(&self) -> &super :: backup :: StorageBackend { match self.storage_backend.as_ref() {
                            Some(v) => v,
                            None => super :: backup :: StorageBackend::default_ref(),
                        } }
#[inline] pub fn mut_storage_backend(&mut self) -> &mut super :: backup :: StorageBackend { if self.storage_backend.is_none() {
                                self.storage_backend = ::std::option::Option::Some(super :: backup :: StorageBackend::default());
                            }
                            self.storage_backend.as_mut().unwrap() } 
#[inline] pub fn take_storage_backend(&mut self) -> super :: backup :: StorageBackend { self.storage_backend.take().unwrap_or_else(super :: backup :: StorageBackend::default) }
#[inline] pub fn clear_is_raw_kv(&mut self) { self.is_raw_kv = false }
#[inline] pub fn set_is_raw_kv(&mut self, v: bool) { self.is_raw_kv = v; }
#[inline] pub fn get_is_raw_kv(&self) -> bool { self.is_raw_kv }
}
impl ::protobuf::Clear for DownloadRequest {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for DownloadRequest {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static DownloadRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: DownloadRequest = DownloadRequest::default();
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
#[inline] pub fn clear_message(&mut self) { self.message.clear(); }
#[inline] pub fn set_message(&mut self, v: :: prost :: alloc :: string :: String) { self.message = v; }
#[inline] pub fn get_message(&self) -> &str { &self.message }
#[inline] pub fn mut_message(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.message }
#[inline] pub fn take_message(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.message, Default::default()) }
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
impl DownloadResponse {
pub fn new_() -> DownloadResponse { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn has_range(&self) -> bool { self.range.is_some() }
#[inline] pub fn clear_range(&mut self) { self.range = ::std::option::Option::None }
#[inline] pub fn set_range(&mut self, v: Range) { self.range = ::std::option::Option::Some(v); }
#[inline] pub fn get_range(&self) -> &Range { match self.range.as_ref() {
                            Some(v) => v,
                            None => Range::default_ref(),
                        } }
#[inline] pub fn mut_range(&mut self) -> &mut Range { if self.range.is_none() {
                                self.range = ::std::option::Option::Some(Range::default());
                            }
                            self.range.as_mut().unwrap() } 
#[inline] pub fn take_range(&mut self) -> Range { self.range.take().unwrap_or_else(Range::default) }
#[inline] pub fn clear_is_empty(&mut self) { self.is_empty = false }
#[inline] pub fn set_is_empty(&mut self, v: bool) { self.is_empty = v; }
#[inline] pub fn get_is_empty(&self) -> bool { self.is_empty }
#[inline] pub fn has_error(&self) -> bool { self.error.is_some() }
#[inline] pub fn clear_error(&mut self) { self.error = ::std::option::Option::None }
#[inline] pub fn set_error(&mut self, v: Error) { self.error = ::std::option::Option::Some(v); }
#[inline] pub fn get_error(&self) -> &Error { match self.error.as_ref() {
                            Some(v) => v,
                            None => Error::default_ref(),
                        } }
#[inline] pub fn mut_error(&mut self) -> &mut Error { if self.error.is_none() {
                                self.error = ::std::option::Option::Some(Error::default());
                            }
                            self.error.as_mut().unwrap() } 
#[inline] pub fn take_error(&mut self) -> Error { self.error.take().unwrap_or_else(Error::default) }
#[inline] pub fn clear_crc32(&mut self) { self.crc32 = 0 }
#[inline] pub fn set_crc32(&mut self, v: u32) { self.crc32 = v; }
#[inline] pub fn get_crc32(&self) -> u32 { self.crc32 }
#[inline] pub fn clear_length(&mut self) { self.length = 0 }
#[inline] pub fn set_length(&mut self, v: u64) { self.length = v; }
#[inline] pub fn get_length(&self) -> u64 { self.length }
}
impl ::protobuf::Clear for DownloadResponse {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for DownloadResponse {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static DownloadResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: DownloadResponse = DownloadResponse::default();
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
impl SetDownloadSpeedLimitRequest {
pub fn new_() -> SetDownloadSpeedLimitRequest { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn clear_speed_limit(&mut self) { self.speed_limit = 0 }
#[inline] pub fn set_speed_limit(&mut self, v: u64) { self.speed_limit = v; }
#[inline] pub fn get_speed_limit(&self) -> u64 { self.speed_limit }
}
impl ::protobuf::Clear for SetDownloadSpeedLimitRequest {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for SetDownloadSpeedLimitRequest {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static SetDownloadSpeedLimitRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: SetDownloadSpeedLimitRequest = SetDownloadSpeedLimitRequest::default();
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
impl SetDownloadSpeedLimitResponse {
pub fn new_() -> SetDownloadSpeedLimitResponse { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
}
impl ::protobuf::Clear for SetDownloadSpeedLimitResponse {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for SetDownloadSpeedLimitResponse {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static SetDownloadSpeedLimitResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: SetDownloadSpeedLimitResponse = SetDownloadSpeedLimitResponse::default();
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
impl Pair {
pub fn new_() -> Pair { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
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
#[inline] pub fn clear_op(&mut self) { self.op = 0 }
#[inline] pub fn get_op(&self) -> pair::Op { match pair :: Op::from_i32(self.op) {Some(e) => e,
                        None => panic!("Unknown enum variant: {}", self.op),
                    } }
}
impl ::protobuf::Clear for Pair {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for Pair {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static Pair {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: Pair = Pair::default();
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
impl pair::Op {
pub fn values() -> &'static [Self] {
static VALUES: &'static [pair::Op] = &[
pair::Op::Put,
pair::Op::Delete,
];
VALUES
}
}
impl WriteBatch {
pub fn new_() -> WriteBatch { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn clear_commit_ts(&mut self) { self.commit_ts = 0 }
#[inline] pub fn set_commit_ts(&mut self, v: u64) { self.commit_ts = v; }
#[inline] pub fn get_commit_ts(&self) -> u64 { self.commit_ts }
#[inline] pub fn clear_pairs(&mut self) { self.pairs.clear(); }
#[inline] pub fn set_pairs(&mut self, v: ::std::vec::Vec<Pair>) { self.pairs = v; }
#[inline] pub fn get_pairs(&self) -> &[Pair] { &self.pairs }
#[inline] pub fn mut_pairs(&mut self) -> &mut ::std::vec::Vec<Pair> { &mut self.pairs }
#[inline] pub fn take_pairs(&mut self) -> ::std::vec::Vec<Pair> { ::std::mem::replace(&mut self.pairs, ::std::vec::Vec::new()) }
}
impl ::protobuf::Clear for WriteBatch {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for WriteBatch {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static WriteBatch {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: WriteBatch = WriteBatch::default();
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
impl WriteRequest {
pub fn new_() -> WriteRequest { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
}
impl ::protobuf::Clear for WriteRequest {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for WriteRequest {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static WriteRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: WriteRequest = WriteRequest::default();
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
impl WriteResponse {
pub fn new_() -> WriteResponse { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn has_error(&self) -> bool { self.error.is_some() }
#[inline] pub fn clear_error(&mut self) { self.error = ::std::option::Option::None }
#[inline] pub fn set_error(&mut self, v: Error) { self.error = ::std::option::Option::Some(v); }
#[inline] pub fn get_error(&self) -> &Error { match self.error.as_ref() {
                            Some(v) => v,
                            None => Error::default_ref(),
                        } }
#[inline] pub fn mut_error(&mut self) -> &mut Error { if self.error.is_none() {
                                self.error = ::std::option::Option::Some(Error::default());
                            }
                            self.error.as_mut().unwrap() } 
#[inline] pub fn take_error(&mut self) -> Error { self.error.take().unwrap_or_else(Error::default) }
#[inline] pub fn clear_metas(&mut self) { self.metas.clear(); }
#[inline] pub fn set_metas(&mut self, v: ::std::vec::Vec<SstMeta>) { self.metas = v; }
#[inline] pub fn get_metas(&self) -> &[SstMeta] { &self.metas }
#[inline] pub fn mut_metas(&mut self) -> &mut ::std::vec::Vec<SstMeta> { &mut self.metas }
#[inline] pub fn take_metas(&mut self) -> ::std::vec::Vec<SstMeta> { ::std::mem::replace(&mut self.metas, ::std::vec::Vec::new()) }
}
impl ::protobuf::Clear for WriteResponse {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for WriteResponse {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static WriteResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: WriteResponse = WriteResponse::default();
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
impl SwitchMode {
pub fn values() -> &'static [Self] {
static VALUES: &'static [SwitchMode] = &[
SwitchMode::Normal,
SwitchMode::Import,
];
VALUES
}
}
