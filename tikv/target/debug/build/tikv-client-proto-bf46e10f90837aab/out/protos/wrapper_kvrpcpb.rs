// Generated file, please don't edit manually.

impl GetRequest {
pub fn new_() -> GetRequest { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn has_context(&self) -> bool { self.context.is_some() }
#[inline] pub fn clear_context(&mut self) { self.context = ::std::option::Option::None }
#[inline] pub fn set_context(&mut self, v: Context) { self.context = ::std::option::Option::Some(v); }
#[inline] pub fn get_context(&self) -> &Context { match self.context.as_ref() {
                            Some(v) => v,
                            None => Context::default_ref(),
                        } }
#[inline] pub fn mut_context(&mut self) -> &mut Context { if self.context.is_none() {
                                self.context = ::std::option::Option::Some(Context::default());
                            }
                            self.context.as_mut().unwrap() } 
#[inline] pub fn take_context(&mut self) -> Context { self.context.take().unwrap_or_else(Context::default) }
#[inline] pub fn clear_key(&mut self) { self.key.clear(); }
#[inline] pub fn set_key(&mut self, v: :: prost :: alloc :: vec :: Vec < u8 >) { self.key = v; }
#[inline] pub fn get_key(&self) -> &[u8] { &self.key }
#[inline] pub fn mut_key(&mut self) -> &mut :: prost :: alloc :: vec :: Vec < u8 > { &mut self.key }
#[inline] pub fn take_key(&mut self) -> :: prost :: alloc :: vec :: Vec < u8 > { ::std::mem::replace(&mut self.key, Default::default()) }
#[inline] pub fn clear_version(&mut self) { self.version = 0 }
#[inline] pub fn set_version(&mut self, v: u64) { self.version = v; }
#[inline] pub fn get_version(&self) -> u64 { self.version }
}
impl ::protobuf::Clear for GetRequest {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for GetRequest {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static GetRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: GetRequest = GetRequest::default();
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
impl GetResponse {
pub fn new_() -> GetResponse { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
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
#[inline] pub fn has_error(&self) -> bool { self.error.is_some() }
#[inline] pub fn clear_error(&mut self) { self.error = ::std::option::Option::None }
#[inline] pub fn set_error(&mut self, v: KeyError) { self.error = ::std::option::Option::Some(v); }
#[inline] pub fn get_error(&self) -> &KeyError { match self.error.as_ref() {
                            Some(v) => v,
                            None => KeyError::default_ref(),
                        } }
#[inline] pub fn mut_error(&mut self) -> &mut KeyError { if self.error.is_none() {
                                self.error = ::std::option::Option::Some(KeyError::default());
                            }
                            self.error.as_mut().unwrap() } 
#[inline] pub fn take_error(&mut self) -> KeyError { self.error.take().unwrap_or_else(KeyError::default) }
#[inline] pub fn clear_value(&mut self) { self.value.clear(); }
#[inline] pub fn set_value(&mut self, v: :: prost :: alloc :: vec :: Vec < u8 >) { self.value = v; }
#[inline] pub fn get_value(&self) -> &[u8] { &self.value }
#[inline] pub fn mut_value(&mut self) -> &mut :: prost :: alloc :: vec :: Vec < u8 > { &mut self.value }
#[inline] pub fn take_value(&mut self) -> :: prost :: alloc :: vec :: Vec < u8 > { ::std::mem::replace(&mut self.value, Default::default()) }
#[inline] pub fn clear_not_found(&mut self) { self.not_found = false }
#[inline] pub fn set_not_found(&mut self, v: bool) { self.not_found = v; }
#[inline] pub fn get_not_found(&self) -> bool { self.not_found }
#[inline] pub fn has_exec_details_v2(&self) -> bool { self.exec_details_v2.is_some() }
#[inline] pub fn clear_exec_details_v2(&mut self) { self.exec_details_v2 = ::std::option::Option::None }
#[inline] pub fn set_exec_details_v2(&mut self, v: ExecDetailsV2) { self.exec_details_v2 = ::std::option::Option::Some(v); }
#[inline] pub fn get_exec_details_v2(&self) -> &ExecDetailsV2 { match self.exec_details_v2.as_ref() {
                            Some(v) => v,
                            None => ExecDetailsV2::default_ref(),
                        } }
#[inline] pub fn mut_exec_details_v2(&mut self) -> &mut ExecDetailsV2 { if self.exec_details_v2.is_none() {
                                self.exec_details_v2 = ::std::option::Option::Some(ExecDetailsV2::default());
                            }
                            self.exec_details_v2.as_mut().unwrap() } 
#[inline] pub fn take_exec_details_v2(&mut self) -> ExecDetailsV2 { self.exec_details_v2.take().unwrap_or_else(ExecDetailsV2::default) }
}
impl ::protobuf::Clear for GetResponse {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for GetResponse {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static GetResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: GetResponse = GetResponse::default();
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
impl ScanRequest {
pub fn new_() -> ScanRequest { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn has_context(&self) -> bool { self.context.is_some() }
#[inline] pub fn clear_context(&mut self) { self.context = ::std::option::Option::None }
#[inline] pub fn set_context(&mut self, v: Context) { self.context = ::std::option::Option::Some(v); }
#[inline] pub fn get_context(&self) -> &Context { match self.context.as_ref() {
                            Some(v) => v,
                            None => Context::default_ref(),
                        } }
#[inline] pub fn mut_context(&mut self) -> &mut Context { if self.context.is_none() {
                                self.context = ::std::option::Option::Some(Context::default());
                            }
                            self.context.as_mut().unwrap() } 
#[inline] pub fn take_context(&mut self) -> Context { self.context.take().unwrap_or_else(Context::default) }
#[inline] pub fn clear_start_key(&mut self) { self.start_key.clear(); }
#[inline] pub fn set_start_key(&mut self, v: :: prost :: alloc :: vec :: Vec < u8 >) { self.start_key = v; }
#[inline] pub fn get_start_key(&self) -> &[u8] { &self.start_key }
#[inline] pub fn mut_start_key(&mut self) -> &mut :: prost :: alloc :: vec :: Vec < u8 > { &mut self.start_key }
#[inline] pub fn take_start_key(&mut self) -> :: prost :: alloc :: vec :: Vec < u8 > { ::std::mem::replace(&mut self.start_key, Default::default()) }
#[inline] pub fn clear_limit(&mut self) { self.limit = 0 }
#[inline] pub fn set_limit(&mut self, v: u32) { self.limit = v; }
#[inline] pub fn get_limit(&self) -> u32 { self.limit }
#[inline] pub fn clear_version(&mut self) { self.version = 0 }
#[inline] pub fn set_version(&mut self, v: u64) { self.version = v; }
#[inline] pub fn get_version(&self) -> u64 { self.version }
#[inline] pub fn clear_key_only(&mut self) { self.key_only = false }
#[inline] pub fn set_key_only(&mut self, v: bool) { self.key_only = v; }
#[inline] pub fn get_key_only(&self) -> bool { self.key_only }
#[inline] pub fn clear_reverse(&mut self) { self.reverse = false }
#[inline] pub fn set_reverse(&mut self, v: bool) { self.reverse = v; }
#[inline] pub fn get_reverse(&self) -> bool { self.reverse }
#[inline] pub fn clear_end_key(&mut self) { self.end_key.clear(); }
#[inline] pub fn set_end_key(&mut self, v: :: prost :: alloc :: vec :: Vec < u8 >) { self.end_key = v; }
#[inline] pub fn get_end_key(&self) -> &[u8] { &self.end_key }
#[inline] pub fn mut_end_key(&mut self) -> &mut :: prost :: alloc :: vec :: Vec < u8 > { &mut self.end_key }
#[inline] pub fn take_end_key(&mut self) -> :: prost :: alloc :: vec :: Vec < u8 > { ::std::mem::replace(&mut self.end_key, Default::default()) }
#[inline] pub fn clear_sample_step(&mut self) { self.sample_step = 0 }
#[inline] pub fn set_sample_step(&mut self, v: u32) { self.sample_step = v; }
#[inline] pub fn get_sample_step(&self) -> u32 { self.sample_step }
}
impl ::protobuf::Clear for ScanRequest {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for ScanRequest {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static ScanRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: ScanRequest = ScanRequest::default();
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
impl ScanResponse {
pub fn new_() -> ScanResponse { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
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
#[inline] pub fn clear_pairs(&mut self) { self.pairs.clear(); }
#[inline] pub fn set_pairs(&mut self, v: ::std::vec::Vec<KvPair>) { self.pairs = v; }
#[inline] pub fn get_pairs(&self) -> &[KvPair] { &self.pairs }
#[inline] pub fn mut_pairs(&mut self) -> &mut ::std::vec::Vec<KvPair> { &mut self.pairs }
#[inline] pub fn take_pairs(&mut self) -> ::std::vec::Vec<KvPair> { ::std::mem::replace(&mut self.pairs, ::std::vec::Vec::new()) }
#[inline] pub fn has_error(&self) -> bool { self.error.is_some() }
#[inline] pub fn clear_error(&mut self) { self.error = ::std::option::Option::None }
#[inline] pub fn set_error(&mut self, v: KeyError) { self.error = ::std::option::Option::Some(v); }
#[inline] pub fn get_error(&self) -> &KeyError { match self.error.as_ref() {
                            Some(v) => v,
                            None => KeyError::default_ref(),
                        } }
#[inline] pub fn mut_error(&mut self) -> &mut KeyError { if self.error.is_none() {
                                self.error = ::std::option::Option::Some(KeyError::default());
                            }
                            self.error.as_mut().unwrap() } 
#[inline] pub fn take_error(&mut self) -> KeyError { self.error.take().unwrap_or_else(KeyError::default) }
}
impl ::protobuf::Clear for ScanResponse {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for ScanResponse {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static ScanResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: ScanResponse = ScanResponse::default();
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
impl PrewriteRequest {
pub fn new_() -> PrewriteRequest { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn has_context(&self) -> bool { self.context.is_some() }
#[inline] pub fn clear_context(&mut self) { self.context = ::std::option::Option::None }
#[inline] pub fn set_context(&mut self, v: Context) { self.context = ::std::option::Option::Some(v); }
#[inline] pub fn get_context(&self) -> &Context { match self.context.as_ref() {
                            Some(v) => v,
                            None => Context::default_ref(),
                        } }
#[inline] pub fn mut_context(&mut self) -> &mut Context { if self.context.is_none() {
                                self.context = ::std::option::Option::Some(Context::default());
                            }
                            self.context.as_mut().unwrap() } 
#[inline] pub fn take_context(&mut self) -> Context { self.context.take().unwrap_or_else(Context::default) }
#[inline] pub fn clear_mutations(&mut self) { self.mutations.clear(); }
#[inline] pub fn set_mutations(&mut self, v: ::std::vec::Vec<Mutation>) { self.mutations = v; }
#[inline] pub fn get_mutations(&self) -> &[Mutation] { &self.mutations }
#[inline] pub fn mut_mutations(&mut self) -> &mut ::std::vec::Vec<Mutation> { &mut self.mutations }
#[inline] pub fn take_mutations(&mut self) -> ::std::vec::Vec<Mutation> { ::std::mem::replace(&mut self.mutations, ::std::vec::Vec::new()) }
#[inline] pub fn clear_primary_lock(&mut self) { self.primary_lock.clear(); }
#[inline] pub fn set_primary_lock(&mut self, v: :: prost :: alloc :: vec :: Vec < u8 >) { self.primary_lock = v; }
#[inline] pub fn get_primary_lock(&self) -> &[u8] { &self.primary_lock }
#[inline] pub fn mut_primary_lock(&mut self) -> &mut :: prost :: alloc :: vec :: Vec < u8 > { &mut self.primary_lock }
#[inline] pub fn take_primary_lock(&mut self) -> :: prost :: alloc :: vec :: Vec < u8 > { ::std::mem::replace(&mut self.primary_lock, Default::default()) }
#[inline] pub fn clear_start_version(&mut self) { self.start_version = 0 }
#[inline] pub fn set_start_version(&mut self, v: u64) { self.start_version = v; }
#[inline] pub fn get_start_version(&self) -> u64 { self.start_version }
#[inline] pub fn clear_lock_ttl(&mut self) { self.lock_ttl = 0 }
#[inline] pub fn set_lock_ttl(&mut self, v: u64) { self.lock_ttl = v; }
#[inline] pub fn get_lock_ttl(&self) -> u64 { self.lock_ttl }
#[inline] pub fn clear_skip_constraint_check(&mut self) { self.skip_constraint_check = false }
#[inline] pub fn set_skip_constraint_check(&mut self, v: bool) { self.skip_constraint_check = v; }
#[inline] pub fn get_skip_constraint_check(&self) -> bool { self.skip_constraint_check }
#[inline] pub fn clear_is_pessimistic_lock(&mut self) { self.is_pessimistic_lock.clear(); }
#[inline] pub fn set_is_pessimistic_lock(&mut self, v: ::std::vec::Vec<bool>) { self.is_pessimistic_lock = v; }
#[inline] pub fn get_is_pessimistic_lock(&self) -> &[bool] { &self.is_pessimistic_lock }
#[inline] pub fn mut_is_pessimistic_lock(&mut self) -> &mut ::std::vec::Vec<bool> { &mut self.is_pessimistic_lock }
#[inline] pub fn take_is_pessimistic_lock(&mut self) -> ::std::vec::Vec<bool> { ::std::mem::replace(&mut self.is_pessimistic_lock, ::std::vec::Vec::new()) }
#[inline] pub fn clear_txn_size(&mut self) { self.txn_size = 0 }
#[inline] pub fn set_txn_size(&mut self, v: u64) { self.txn_size = v; }
#[inline] pub fn get_txn_size(&self) -> u64 { self.txn_size }
#[inline] pub fn clear_for_update_ts(&mut self) { self.for_update_ts = 0 }
#[inline] pub fn set_for_update_ts(&mut self, v: u64) { self.for_update_ts = v; }
#[inline] pub fn get_for_update_ts(&self) -> u64 { self.for_update_ts }
#[inline] pub fn clear_min_commit_ts(&mut self) { self.min_commit_ts = 0 }
#[inline] pub fn set_min_commit_ts(&mut self, v: u64) { self.min_commit_ts = v; }
#[inline] pub fn get_min_commit_ts(&self) -> u64 { self.min_commit_ts }
#[inline] pub fn clear_use_async_commit(&mut self) { self.use_async_commit = false }
#[inline] pub fn set_use_async_commit(&mut self, v: bool) { self.use_async_commit = v; }
#[inline] pub fn get_use_async_commit(&self) -> bool { self.use_async_commit }
#[inline] pub fn clear_secondaries(&mut self) { self.secondaries.clear(); }
#[inline] pub fn set_secondaries(&mut self, v: ::std::vec::Vec<:: prost :: alloc :: vec :: Vec < u8 >>) { self.secondaries = v; }
#[inline] pub fn get_secondaries(&self) -> &[:: prost :: alloc :: vec :: Vec < u8 >] { &self.secondaries }
#[inline] pub fn mut_secondaries(&mut self) -> &mut ::std::vec::Vec<:: prost :: alloc :: vec :: Vec < u8 >> { &mut self.secondaries }
#[inline] pub fn take_secondaries(&mut self) -> ::std::vec::Vec<:: prost :: alloc :: vec :: Vec < u8 >> { ::std::mem::replace(&mut self.secondaries, ::std::vec::Vec::new()) }
#[inline] pub fn clear_try_one_pc(&mut self) { self.try_one_pc = false }
#[inline] pub fn set_try_one_pc(&mut self, v: bool) { self.try_one_pc = v; }
#[inline] pub fn get_try_one_pc(&self) -> bool { self.try_one_pc }
#[inline] pub fn clear_max_commit_ts(&mut self) { self.max_commit_ts = 0 }
#[inline] pub fn set_max_commit_ts(&mut self, v: u64) { self.max_commit_ts = v; }
#[inline] pub fn get_max_commit_ts(&self) -> u64 { self.max_commit_ts }
}
impl ::protobuf::Clear for PrewriteRequest {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for PrewriteRequest {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static PrewriteRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: PrewriteRequest = PrewriteRequest::default();
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
impl PrewriteResponse {
pub fn new_() -> PrewriteResponse { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
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
#[inline] pub fn clear_errors(&mut self) { self.errors.clear(); }
#[inline] pub fn set_errors(&mut self, v: ::std::vec::Vec<KeyError>) { self.errors = v; }
#[inline] pub fn get_errors(&self) -> &[KeyError] { &self.errors }
#[inline] pub fn mut_errors(&mut self) -> &mut ::std::vec::Vec<KeyError> { &mut self.errors }
#[inline] pub fn take_errors(&mut self) -> ::std::vec::Vec<KeyError> { ::std::mem::replace(&mut self.errors, ::std::vec::Vec::new()) }
#[inline] pub fn clear_min_commit_ts(&mut self) { self.min_commit_ts = 0 }
#[inline] pub fn set_min_commit_ts(&mut self, v: u64) { self.min_commit_ts = v; }
#[inline] pub fn get_min_commit_ts(&self) -> u64 { self.min_commit_ts }
#[inline] pub fn clear_one_pc_commit_ts(&mut self) { self.one_pc_commit_ts = 0 }
#[inline] pub fn set_one_pc_commit_ts(&mut self, v: u64) { self.one_pc_commit_ts = v; }
#[inline] pub fn get_one_pc_commit_ts(&self) -> u64 { self.one_pc_commit_ts }
}
impl ::protobuf::Clear for PrewriteResponse {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for PrewriteResponse {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static PrewriteResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: PrewriteResponse = PrewriteResponse::default();
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
impl PessimisticLockRequest {
pub fn new_() -> PessimisticLockRequest { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn has_context(&self) -> bool { self.context.is_some() }
#[inline] pub fn clear_context(&mut self) { self.context = ::std::option::Option::None }
#[inline] pub fn set_context(&mut self, v: Context) { self.context = ::std::option::Option::Some(v); }
#[inline] pub fn get_context(&self) -> &Context { match self.context.as_ref() {
                            Some(v) => v,
                            None => Context::default_ref(),
                        } }
#[inline] pub fn mut_context(&mut self) -> &mut Context { if self.context.is_none() {
                                self.context = ::std::option::Option::Some(Context::default());
                            }
                            self.context.as_mut().unwrap() } 
#[inline] pub fn take_context(&mut self) -> Context { self.context.take().unwrap_or_else(Context::default) }
#[inline] pub fn clear_mutations(&mut self) { self.mutations.clear(); }
#[inline] pub fn set_mutations(&mut self, v: ::std::vec::Vec<Mutation>) { self.mutations = v; }
#[inline] pub fn get_mutations(&self) -> &[Mutation] { &self.mutations }
#[inline] pub fn mut_mutations(&mut self) -> &mut ::std::vec::Vec<Mutation> { &mut self.mutations }
#[inline] pub fn take_mutations(&mut self) -> ::std::vec::Vec<Mutation> { ::std::mem::replace(&mut self.mutations, ::std::vec::Vec::new()) }
#[inline] pub fn clear_primary_lock(&mut self) { self.primary_lock.clear(); }
#[inline] pub fn set_primary_lock(&mut self, v: :: prost :: alloc :: vec :: Vec < u8 >) { self.primary_lock = v; }
#[inline] pub fn get_primary_lock(&self) -> &[u8] { &self.primary_lock }
#[inline] pub fn mut_primary_lock(&mut self) -> &mut :: prost :: alloc :: vec :: Vec < u8 > { &mut self.primary_lock }
#[inline] pub fn take_primary_lock(&mut self) -> :: prost :: alloc :: vec :: Vec < u8 > { ::std::mem::replace(&mut self.primary_lock, Default::default()) }
#[inline] pub fn clear_start_version(&mut self) { self.start_version = 0 }
#[inline] pub fn set_start_version(&mut self, v: u64) { self.start_version = v; }
#[inline] pub fn get_start_version(&self) -> u64 { self.start_version }
#[inline] pub fn clear_lock_ttl(&mut self) { self.lock_ttl = 0 }
#[inline] pub fn set_lock_ttl(&mut self, v: u64) { self.lock_ttl = v; }
#[inline] pub fn get_lock_ttl(&self) -> u64 { self.lock_ttl }
#[inline] pub fn clear_for_update_ts(&mut self) { self.for_update_ts = 0 }
#[inline] pub fn set_for_update_ts(&mut self, v: u64) { self.for_update_ts = v; }
#[inline] pub fn get_for_update_ts(&self) -> u64 { self.for_update_ts }
#[inline] pub fn clear_is_first_lock(&mut self) { self.is_first_lock = false }
#[inline] pub fn set_is_first_lock(&mut self, v: bool) { self.is_first_lock = v; }
#[inline] pub fn get_is_first_lock(&self) -> bool { self.is_first_lock }
#[inline] pub fn clear_wait_timeout(&mut self) { self.wait_timeout = 0 }
#[inline] pub fn set_wait_timeout(&mut self, v: i64) { self.wait_timeout = v; }
#[inline] pub fn get_wait_timeout(&self) -> i64 { self.wait_timeout }
#[inline] pub fn clear_force(&mut self) { self.force = false }
#[inline] pub fn set_force(&mut self, v: bool) { self.force = v; }
#[inline] pub fn get_force(&self) -> bool { self.force }
#[inline] pub fn clear_return_values(&mut self) { self.return_values = false }
#[inline] pub fn set_return_values(&mut self, v: bool) { self.return_values = v; }
#[inline] pub fn get_return_values(&self) -> bool { self.return_values }
#[inline] pub fn clear_min_commit_ts(&mut self) { self.min_commit_ts = 0 }
#[inline] pub fn set_min_commit_ts(&mut self, v: u64) { self.min_commit_ts = v; }
#[inline] pub fn get_min_commit_ts(&self) -> u64 { self.min_commit_ts }
}
impl ::protobuf::Clear for PessimisticLockRequest {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for PessimisticLockRequest {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static PessimisticLockRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: PessimisticLockRequest = PessimisticLockRequest::default();
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
impl PessimisticLockResponse {
pub fn new_() -> PessimisticLockResponse { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
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
#[inline] pub fn clear_errors(&mut self) { self.errors.clear(); }
#[inline] pub fn set_errors(&mut self, v: ::std::vec::Vec<KeyError>) { self.errors = v; }
#[inline] pub fn get_errors(&self) -> &[KeyError] { &self.errors }
#[inline] pub fn mut_errors(&mut self) -> &mut ::std::vec::Vec<KeyError> { &mut self.errors }
#[inline] pub fn take_errors(&mut self) -> ::std::vec::Vec<KeyError> { ::std::mem::replace(&mut self.errors, ::std::vec::Vec::new()) }
#[inline] pub fn clear_commit_ts(&mut self) { self.commit_ts = 0 }
#[inline] pub fn set_commit_ts(&mut self, v: u64) { self.commit_ts = v; }
#[inline] pub fn get_commit_ts(&self) -> u64 { self.commit_ts }
#[inline] pub fn clear_value(&mut self) { self.value.clear(); }
#[inline] pub fn set_value(&mut self, v: :: prost :: alloc :: vec :: Vec < u8 >) { self.value = v; }
#[inline] pub fn get_value(&self) -> &[u8] { &self.value }
#[inline] pub fn mut_value(&mut self) -> &mut :: prost :: alloc :: vec :: Vec < u8 > { &mut self.value }
#[inline] pub fn take_value(&mut self) -> :: prost :: alloc :: vec :: Vec < u8 > { ::std::mem::replace(&mut self.value, Default::default()) }
#[inline] pub fn clear_values(&mut self) { self.values.clear(); }
#[inline] pub fn set_values(&mut self, v: ::std::vec::Vec<:: prost :: alloc :: vec :: Vec < u8 >>) { self.values = v; }
#[inline] pub fn get_values(&self) -> &[:: prost :: alloc :: vec :: Vec < u8 >] { &self.values }
#[inline] pub fn mut_values(&mut self) -> &mut ::std::vec::Vec<:: prost :: alloc :: vec :: Vec < u8 >> { &mut self.values }
#[inline] pub fn take_values(&mut self) -> ::std::vec::Vec<:: prost :: alloc :: vec :: Vec < u8 >> { ::std::mem::replace(&mut self.values, ::std::vec::Vec::new()) }
#[inline] pub fn clear_not_founds(&mut self) { self.not_founds.clear(); }
#[inline] pub fn set_not_founds(&mut self, v: ::std::vec::Vec<bool>) { self.not_founds = v; }
#[inline] pub fn get_not_founds(&self) -> &[bool] { &self.not_founds }
#[inline] pub fn mut_not_founds(&mut self) -> &mut ::std::vec::Vec<bool> { &mut self.not_founds }
#[inline] pub fn take_not_founds(&mut self) -> ::std::vec::Vec<bool> { ::std::mem::replace(&mut self.not_founds, ::std::vec::Vec::new()) }
}
impl ::protobuf::Clear for PessimisticLockResponse {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for PessimisticLockResponse {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static PessimisticLockResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: PessimisticLockResponse = PessimisticLockResponse::default();
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
impl PessimisticRollbackRequest {
pub fn new_() -> PessimisticRollbackRequest { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn has_context(&self) -> bool { self.context.is_some() }
#[inline] pub fn clear_context(&mut self) { self.context = ::std::option::Option::None }
#[inline] pub fn set_context(&mut self, v: Context) { self.context = ::std::option::Option::Some(v); }
#[inline] pub fn get_context(&self) -> &Context { match self.context.as_ref() {
                            Some(v) => v,
                            None => Context::default_ref(),
                        } }
#[inline] pub fn mut_context(&mut self) -> &mut Context { if self.context.is_none() {
                                self.context = ::std::option::Option::Some(Context::default());
                            }
                            self.context.as_mut().unwrap() } 
#[inline] pub fn take_context(&mut self) -> Context { self.context.take().unwrap_or_else(Context::default) }
#[inline] pub fn clear_start_version(&mut self) { self.start_version = 0 }
#[inline] pub fn set_start_version(&mut self, v: u64) { self.start_version = v; }
#[inline] pub fn get_start_version(&self) -> u64 { self.start_version }
#[inline] pub fn clear_for_update_ts(&mut self) { self.for_update_ts = 0 }
#[inline] pub fn set_for_update_ts(&mut self, v: u64) { self.for_update_ts = v; }
#[inline] pub fn get_for_update_ts(&self) -> u64 { self.for_update_ts }
#[inline] pub fn clear_keys(&mut self) { self.keys.clear(); }
#[inline] pub fn set_keys(&mut self, v: ::std::vec::Vec<:: prost :: alloc :: vec :: Vec < u8 >>) { self.keys = v; }
#[inline] pub fn get_keys(&self) -> &[:: prost :: alloc :: vec :: Vec < u8 >] { &self.keys }
#[inline] pub fn mut_keys(&mut self) -> &mut ::std::vec::Vec<:: prost :: alloc :: vec :: Vec < u8 >> { &mut self.keys }
#[inline] pub fn take_keys(&mut self) -> ::std::vec::Vec<:: prost :: alloc :: vec :: Vec < u8 >> { ::std::mem::replace(&mut self.keys, ::std::vec::Vec::new()) }
}
impl ::protobuf::Clear for PessimisticRollbackRequest {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for PessimisticRollbackRequest {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static PessimisticRollbackRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: PessimisticRollbackRequest = PessimisticRollbackRequest::default();
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
impl PessimisticRollbackResponse {
pub fn new_() -> PessimisticRollbackResponse { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
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
#[inline] pub fn clear_errors(&mut self) { self.errors.clear(); }
#[inline] pub fn set_errors(&mut self, v: ::std::vec::Vec<KeyError>) { self.errors = v; }
#[inline] pub fn get_errors(&self) -> &[KeyError] { &self.errors }
#[inline] pub fn mut_errors(&mut self) -> &mut ::std::vec::Vec<KeyError> { &mut self.errors }
#[inline] pub fn take_errors(&mut self) -> ::std::vec::Vec<KeyError> { ::std::mem::replace(&mut self.errors, ::std::vec::Vec::new()) }
}
impl ::protobuf::Clear for PessimisticRollbackResponse {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for PessimisticRollbackResponse {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static PessimisticRollbackResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: PessimisticRollbackResponse = PessimisticRollbackResponse::default();
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
impl TxnHeartBeatRequest {
pub fn new_() -> TxnHeartBeatRequest { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn has_context(&self) -> bool { self.context.is_some() }
#[inline] pub fn clear_context(&mut self) { self.context = ::std::option::Option::None }
#[inline] pub fn set_context(&mut self, v: Context) { self.context = ::std::option::Option::Some(v); }
#[inline] pub fn get_context(&self) -> &Context { match self.context.as_ref() {
                            Some(v) => v,
                            None => Context::default_ref(),
                        } }
#[inline] pub fn mut_context(&mut self) -> &mut Context { if self.context.is_none() {
                                self.context = ::std::option::Option::Some(Context::default());
                            }
                            self.context.as_mut().unwrap() } 
#[inline] pub fn take_context(&mut self) -> Context { self.context.take().unwrap_or_else(Context::default) }
#[inline] pub fn clear_primary_lock(&mut self) { self.primary_lock.clear(); }
#[inline] pub fn set_primary_lock(&mut self, v: :: prost :: alloc :: vec :: Vec < u8 >) { self.primary_lock = v; }
#[inline] pub fn get_primary_lock(&self) -> &[u8] { &self.primary_lock }
#[inline] pub fn mut_primary_lock(&mut self) -> &mut :: prost :: alloc :: vec :: Vec < u8 > { &mut self.primary_lock }
#[inline] pub fn take_primary_lock(&mut self) -> :: prost :: alloc :: vec :: Vec < u8 > { ::std::mem::replace(&mut self.primary_lock, Default::default()) }
#[inline] pub fn clear_start_version(&mut self) { self.start_version = 0 }
#[inline] pub fn set_start_version(&mut self, v: u64) { self.start_version = v; }
#[inline] pub fn get_start_version(&self) -> u64 { self.start_version }
#[inline] pub fn clear_advise_lock_ttl(&mut self) { self.advise_lock_ttl = 0 }
#[inline] pub fn set_advise_lock_ttl(&mut self, v: u64) { self.advise_lock_ttl = v; }
#[inline] pub fn get_advise_lock_ttl(&self) -> u64 { self.advise_lock_ttl }
}
impl ::protobuf::Clear for TxnHeartBeatRequest {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for TxnHeartBeatRequest {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static TxnHeartBeatRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: TxnHeartBeatRequest = TxnHeartBeatRequest::default();
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
impl TxnHeartBeatResponse {
pub fn new_() -> TxnHeartBeatResponse { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
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
#[inline] pub fn has_error(&self) -> bool { self.error.is_some() }
#[inline] pub fn clear_error(&mut self) { self.error = ::std::option::Option::None }
#[inline] pub fn set_error(&mut self, v: KeyError) { self.error = ::std::option::Option::Some(v); }
#[inline] pub fn get_error(&self) -> &KeyError { match self.error.as_ref() {
                            Some(v) => v,
                            None => KeyError::default_ref(),
                        } }
#[inline] pub fn mut_error(&mut self) -> &mut KeyError { if self.error.is_none() {
                                self.error = ::std::option::Option::Some(KeyError::default());
                            }
                            self.error.as_mut().unwrap() } 
#[inline] pub fn take_error(&mut self) -> KeyError { self.error.take().unwrap_or_else(KeyError::default) }
#[inline] pub fn clear_lock_ttl(&mut self) { self.lock_ttl = 0 }
#[inline] pub fn set_lock_ttl(&mut self, v: u64) { self.lock_ttl = v; }
#[inline] pub fn get_lock_ttl(&self) -> u64 { self.lock_ttl }
}
impl ::protobuf::Clear for TxnHeartBeatResponse {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for TxnHeartBeatResponse {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static TxnHeartBeatResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: TxnHeartBeatResponse = TxnHeartBeatResponse::default();
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
impl CheckTxnStatusRequest {
pub fn new_() -> CheckTxnStatusRequest { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn has_context(&self) -> bool { self.context.is_some() }
#[inline] pub fn clear_context(&mut self) { self.context = ::std::option::Option::None }
#[inline] pub fn set_context(&mut self, v: Context) { self.context = ::std::option::Option::Some(v); }
#[inline] pub fn get_context(&self) -> &Context { match self.context.as_ref() {
                            Some(v) => v,
                            None => Context::default_ref(),
                        } }
#[inline] pub fn mut_context(&mut self) -> &mut Context { if self.context.is_none() {
                                self.context = ::std::option::Option::Some(Context::default());
                            }
                            self.context.as_mut().unwrap() } 
#[inline] pub fn take_context(&mut self) -> Context { self.context.take().unwrap_or_else(Context::default) }
#[inline] pub fn clear_primary_key(&mut self) { self.primary_key.clear(); }
#[inline] pub fn set_primary_key(&mut self, v: :: prost :: alloc :: vec :: Vec < u8 >) { self.primary_key = v; }
#[inline] pub fn get_primary_key(&self) -> &[u8] { &self.primary_key }
#[inline] pub fn mut_primary_key(&mut self) -> &mut :: prost :: alloc :: vec :: Vec < u8 > { &mut self.primary_key }
#[inline] pub fn take_primary_key(&mut self) -> :: prost :: alloc :: vec :: Vec < u8 > { ::std::mem::replace(&mut self.primary_key, Default::default()) }
#[inline] pub fn clear_lock_ts(&mut self) { self.lock_ts = 0 }
#[inline] pub fn set_lock_ts(&mut self, v: u64) { self.lock_ts = v; }
#[inline] pub fn get_lock_ts(&self) -> u64 { self.lock_ts }
#[inline] pub fn clear_caller_start_ts(&mut self) { self.caller_start_ts = 0 }
#[inline] pub fn set_caller_start_ts(&mut self, v: u64) { self.caller_start_ts = v; }
#[inline] pub fn get_caller_start_ts(&self) -> u64 { self.caller_start_ts }
#[inline] pub fn clear_current_ts(&mut self) { self.current_ts = 0 }
#[inline] pub fn set_current_ts(&mut self, v: u64) { self.current_ts = v; }
#[inline] pub fn get_current_ts(&self) -> u64 { self.current_ts }
#[inline] pub fn clear_rollback_if_not_exist(&mut self) { self.rollback_if_not_exist = false }
#[inline] pub fn set_rollback_if_not_exist(&mut self, v: bool) { self.rollback_if_not_exist = v; }
#[inline] pub fn get_rollback_if_not_exist(&self) -> bool { self.rollback_if_not_exist }
#[inline] pub fn clear_force_sync_commit(&mut self) { self.force_sync_commit = false }
#[inline] pub fn set_force_sync_commit(&mut self, v: bool) { self.force_sync_commit = v; }
#[inline] pub fn get_force_sync_commit(&self) -> bool { self.force_sync_commit }
#[inline] pub fn clear_resolving_pessimistic_lock(&mut self) { self.resolving_pessimistic_lock = false }
#[inline] pub fn set_resolving_pessimistic_lock(&mut self, v: bool) { self.resolving_pessimistic_lock = v; }
#[inline] pub fn get_resolving_pessimistic_lock(&self) -> bool { self.resolving_pessimistic_lock }
}
impl ::protobuf::Clear for CheckTxnStatusRequest {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for CheckTxnStatusRequest {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static CheckTxnStatusRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: CheckTxnStatusRequest = CheckTxnStatusRequest::default();
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
impl CheckTxnStatusResponse {
pub fn new_() -> CheckTxnStatusResponse { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
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
#[inline] pub fn has_error(&self) -> bool { self.error.is_some() }
#[inline] pub fn clear_error(&mut self) { self.error = ::std::option::Option::None }
#[inline] pub fn set_error(&mut self, v: KeyError) { self.error = ::std::option::Option::Some(v); }
#[inline] pub fn get_error(&self) -> &KeyError { match self.error.as_ref() {
                            Some(v) => v,
                            None => KeyError::default_ref(),
                        } }
#[inline] pub fn mut_error(&mut self) -> &mut KeyError { if self.error.is_none() {
                                self.error = ::std::option::Option::Some(KeyError::default());
                            }
                            self.error.as_mut().unwrap() } 
#[inline] pub fn take_error(&mut self) -> KeyError { self.error.take().unwrap_or_else(KeyError::default) }
#[inline] pub fn clear_lock_ttl(&mut self) { self.lock_ttl = 0 }
#[inline] pub fn set_lock_ttl(&mut self, v: u64) { self.lock_ttl = v; }
#[inline] pub fn get_lock_ttl(&self) -> u64 { self.lock_ttl }
#[inline] pub fn clear_commit_version(&mut self) { self.commit_version = 0 }
#[inline] pub fn set_commit_version(&mut self, v: u64) { self.commit_version = v; }
#[inline] pub fn get_commit_version(&self) -> u64 { self.commit_version }
#[inline] pub fn clear_action(&mut self) { self.action = 0 }
#[inline] pub fn get_action(&self) -> Action { match Action::from_i32(self.action) {Some(e) => e,
                        None => panic!("Unknown enum variant: {}", self.action),
                    } }
#[inline] pub fn has_lock_info(&self) -> bool { self.lock_info.is_some() }
#[inline] pub fn clear_lock_info(&mut self) { self.lock_info = ::std::option::Option::None }
#[inline] pub fn set_lock_info(&mut self, v: LockInfo) { self.lock_info = ::std::option::Option::Some(v); }
#[inline] pub fn get_lock_info(&self) -> &LockInfo { match self.lock_info.as_ref() {
                            Some(v) => v,
                            None => LockInfo::default_ref(),
                        } }
#[inline] pub fn mut_lock_info(&mut self) -> &mut LockInfo { if self.lock_info.is_none() {
                                self.lock_info = ::std::option::Option::Some(LockInfo::default());
                            }
                            self.lock_info.as_mut().unwrap() } 
#[inline] pub fn take_lock_info(&mut self) -> LockInfo { self.lock_info.take().unwrap_or_else(LockInfo::default) }
}
impl ::protobuf::Clear for CheckTxnStatusResponse {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for CheckTxnStatusResponse {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static CheckTxnStatusResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: CheckTxnStatusResponse = CheckTxnStatusResponse::default();
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
impl CheckSecondaryLocksRequest {
pub fn new_() -> CheckSecondaryLocksRequest { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn has_context(&self) -> bool { self.context.is_some() }
#[inline] pub fn clear_context(&mut self) { self.context = ::std::option::Option::None }
#[inline] pub fn set_context(&mut self, v: Context) { self.context = ::std::option::Option::Some(v); }
#[inline] pub fn get_context(&self) -> &Context { match self.context.as_ref() {
                            Some(v) => v,
                            None => Context::default_ref(),
                        } }
#[inline] pub fn mut_context(&mut self) -> &mut Context { if self.context.is_none() {
                                self.context = ::std::option::Option::Some(Context::default());
                            }
                            self.context.as_mut().unwrap() } 
#[inline] pub fn take_context(&mut self) -> Context { self.context.take().unwrap_or_else(Context::default) }
#[inline] pub fn clear_keys(&mut self) { self.keys.clear(); }
#[inline] pub fn set_keys(&mut self, v: ::std::vec::Vec<:: prost :: alloc :: vec :: Vec < u8 >>) { self.keys = v; }
#[inline] pub fn get_keys(&self) -> &[:: prost :: alloc :: vec :: Vec < u8 >] { &self.keys }
#[inline] pub fn mut_keys(&mut self) -> &mut ::std::vec::Vec<:: prost :: alloc :: vec :: Vec < u8 >> { &mut self.keys }
#[inline] pub fn take_keys(&mut self) -> ::std::vec::Vec<:: prost :: alloc :: vec :: Vec < u8 >> { ::std::mem::replace(&mut self.keys, ::std::vec::Vec::new()) }
#[inline] pub fn clear_start_version(&mut self) { self.start_version = 0 }
#[inline] pub fn set_start_version(&mut self, v: u64) { self.start_version = v; }
#[inline] pub fn get_start_version(&self) -> u64 { self.start_version }
}
impl ::protobuf::Clear for CheckSecondaryLocksRequest {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for CheckSecondaryLocksRequest {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static CheckSecondaryLocksRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: CheckSecondaryLocksRequest = CheckSecondaryLocksRequest::default();
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
impl CheckSecondaryLocksResponse {
pub fn new_() -> CheckSecondaryLocksResponse { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
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
#[inline] pub fn has_error(&self) -> bool { self.error.is_some() }
#[inline] pub fn clear_error(&mut self) { self.error = ::std::option::Option::None }
#[inline] pub fn set_error(&mut self, v: KeyError) { self.error = ::std::option::Option::Some(v); }
#[inline] pub fn get_error(&self) -> &KeyError { match self.error.as_ref() {
                            Some(v) => v,
                            None => KeyError::default_ref(),
                        } }
#[inline] pub fn mut_error(&mut self) -> &mut KeyError { if self.error.is_none() {
                                self.error = ::std::option::Option::Some(KeyError::default());
                            }
                            self.error.as_mut().unwrap() } 
#[inline] pub fn take_error(&mut self) -> KeyError { self.error.take().unwrap_or_else(KeyError::default) }
#[inline] pub fn clear_locks(&mut self) { self.locks.clear(); }
#[inline] pub fn set_locks(&mut self, v: ::std::vec::Vec<LockInfo>) { self.locks = v; }
#[inline] pub fn get_locks(&self) -> &[LockInfo] { &self.locks }
#[inline] pub fn mut_locks(&mut self) -> &mut ::std::vec::Vec<LockInfo> { &mut self.locks }
#[inline] pub fn take_locks(&mut self) -> ::std::vec::Vec<LockInfo> { ::std::mem::replace(&mut self.locks, ::std::vec::Vec::new()) }
#[inline] pub fn clear_commit_ts(&mut self) { self.commit_ts = 0 }
#[inline] pub fn set_commit_ts(&mut self, v: u64) { self.commit_ts = v; }
#[inline] pub fn get_commit_ts(&self) -> u64 { self.commit_ts }
}
impl ::protobuf::Clear for CheckSecondaryLocksResponse {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for CheckSecondaryLocksResponse {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static CheckSecondaryLocksResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: CheckSecondaryLocksResponse = CheckSecondaryLocksResponse::default();
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
impl CommitRequest {
pub fn new_() -> CommitRequest { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn has_context(&self) -> bool { self.context.is_some() }
#[inline] pub fn clear_context(&mut self) { self.context = ::std::option::Option::None }
#[inline] pub fn set_context(&mut self, v: Context) { self.context = ::std::option::Option::Some(v); }
#[inline] pub fn get_context(&self) -> &Context { match self.context.as_ref() {
                            Some(v) => v,
                            None => Context::default_ref(),
                        } }
#[inline] pub fn mut_context(&mut self) -> &mut Context { if self.context.is_none() {
                                self.context = ::std::option::Option::Some(Context::default());
                            }
                            self.context.as_mut().unwrap() } 
#[inline] pub fn take_context(&mut self) -> Context { self.context.take().unwrap_or_else(Context::default) }
#[inline] pub fn clear_start_version(&mut self) { self.start_version = 0 }
#[inline] pub fn set_start_version(&mut self, v: u64) { self.start_version = v; }
#[inline] pub fn get_start_version(&self) -> u64 { self.start_version }
#[inline] pub fn clear_keys(&mut self) { self.keys.clear(); }
#[inline] pub fn set_keys(&mut self, v: ::std::vec::Vec<:: prost :: alloc :: vec :: Vec < u8 >>) { self.keys = v; }
#[inline] pub fn get_keys(&self) -> &[:: prost :: alloc :: vec :: Vec < u8 >] { &self.keys }
#[inline] pub fn mut_keys(&mut self) -> &mut ::std::vec::Vec<:: prost :: alloc :: vec :: Vec < u8 >> { &mut self.keys }
#[inline] pub fn take_keys(&mut self) -> ::std::vec::Vec<:: prost :: alloc :: vec :: Vec < u8 >> { ::std::mem::replace(&mut self.keys, ::std::vec::Vec::new()) }
#[inline] pub fn clear_commit_version(&mut self) { self.commit_version = 0 }
#[inline] pub fn set_commit_version(&mut self, v: u64) { self.commit_version = v; }
#[inline] pub fn get_commit_version(&self) -> u64 { self.commit_version }
}
impl ::protobuf::Clear for CommitRequest {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for CommitRequest {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static CommitRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: CommitRequest = CommitRequest::default();
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
impl CommitResponse {
pub fn new_() -> CommitResponse { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
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
#[inline] pub fn has_error(&self) -> bool { self.error.is_some() }
#[inline] pub fn clear_error(&mut self) { self.error = ::std::option::Option::None }
#[inline] pub fn set_error(&mut self, v: KeyError) { self.error = ::std::option::Option::Some(v); }
#[inline] pub fn get_error(&self) -> &KeyError { match self.error.as_ref() {
                            Some(v) => v,
                            None => KeyError::default_ref(),
                        } }
#[inline] pub fn mut_error(&mut self) -> &mut KeyError { if self.error.is_none() {
                                self.error = ::std::option::Option::Some(KeyError::default());
                            }
                            self.error.as_mut().unwrap() } 
#[inline] pub fn take_error(&mut self) -> KeyError { self.error.take().unwrap_or_else(KeyError::default) }
#[inline] pub fn clear_commit_version(&mut self) { self.commit_version = 0 }
#[inline] pub fn set_commit_version(&mut self, v: u64) { self.commit_version = v; }
#[inline] pub fn get_commit_version(&self) -> u64 { self.commit_version }
}
impl ::protobuf::Clear for CommitResponse {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for CommitResponse {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static CommitResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: CommitResponse = CommitResponse::default();
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
impl ImportRequest {
pub fn new_() -> ImportRequest { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn clear_mutations(&mut self) { self.mutations.clear(); }
#[inline] pub fn set_mutations(&mut self, v: ::std::vec::Vec<Mutation>) { self.mutations = v; }
#[inline] pub fn get_mutations(&self) -> &[Mutation] { &self.mutations }
#[inline] pub fn mut_mutations(&mut self) -> &mut ::std::vec::Vec<Mutation> { &mut self.mutations }
#[inline] pub fn take_mutations(&mut self) -> ::std::vec::Vec<Mutation> { ::std::mem::replace(&mut self.mutations, ::std::vec::Vec::new()) }
#[inline] pub fn clear_commit_version(&mut self) { self.commit_version = 0 }
#[inline] pub fn set_commit_version(&mut self, v: u64) { self.commit_version = v; }
#[inline] pub fn get_commit_version(&self) -> u64 { self.commit_version }
}
impl ::protobuf::Clear for ImportRequest {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for ImportRequest {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static ImportRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: ImportRequest = ImportRequest::default();
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
impl ImportResponse {
pub fn new_() -> ImportResponse { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
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
#[inline] pub fn clear_error(&mut self) { self.error.clear(); }
#[inline] pub fn set_error(&mut self, v: :: prost :: alloc :: string :: String) { self.error = v; }
#[inline] pub fn get_error(&self) -> &str { &self.error }
#[inline] pub fn mut_error(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.error }
#[inline] pub fn take_error(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.error, Default::default()) }
}
impl ::protobuf::Clear for ImportResponse {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for ImportResponse {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static ImportResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: ImportResponse = ImportResponse::default();
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
impl CleanupRequest {
pub fn new_() -> CleanupRequest { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn has_context(&self) -> bool { self.context.is_some() }
#[inline] pub fn clear_context(&mut self) { self.context = ::std::option::Option::None }
#[inline] pub fn set_context(&mut self, v: Context) { self.context = ::std::option::Option::Some(v); }
#[inline] pub fn get_context(&self) -> &Context { match self.context.as_ref() {
                            Some(v) => v,
                            None => Context::default_ref(),
                        } }
#[inline] pub fn mut_context(&mut self) -> &mut Context { if self.context.is_none() {
                                self.context = ::std::option::Option::Some(Context::default());
                            }
                            self.context.as_mut().unwrap() } 
#[inline] pub fn take_context(&mut self) -> Context { self.context.take().unwrap_or_else(Context::default) }
#[inline] pub fn clear_key(&mut self) { self.key.clear(); }
#[inline] pub fn set_key(&mut self, v: :: prost :: alloc :: vec :: Vec < u8 >) { self.key = v; }
#[inline] pub fn get_key(&self) -> &[u8] { &self.key }
#[inline] pub fn mut_key(&mut self) -> &mut :: prost :: alloc :: vec :: Vec < u8 > { &mut self.key }
#[inline] pub fn take_key(&mut self) -> :: prost :: alloc :: vec :: Vec < u8 > { ::std::mem::replace(&mut self.key, Default::default()) }
#[inline] pub fn clear_start_version(&mut self) { self.start_version = 0 }
#[inline] pub fn set_start_version(&mut self, v: u64) { self.start_version = v; }
#[inline] pub fn get_start_version(&self) -> u64 { self.start_version }
#[inline] pub fn clear_current_ts(&mut self) { self.current_ts = 0 }
#[inline] pub fn set_current_ts(&mut self, v: u64) { self.current_ts = v; }
#[inline] pub fn get_current_ts(&self) -> u64 { self.current_ts }
}
impl ::protobuf::Clear for CleanupRequest {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for CleanupRequest {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static CleanupRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: CleanupRequest = CleanupRequest::default();
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
impl CleanupResponse {
pub fn new_() -> CleanupResponse { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
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
#[inline] pub fn has_error(&self) -> bool { self.error.is_some() }
#[inline] pub fn clear_error(&mut self) { self.error = ::std::option::Option::None }
#[inline] pub fn set_error(&mut self, v: KeyError) { self.error = ::std::option::Option::Some(v); }
#[inline] pub fn get_error(&self) -> &KeyError { match self.error.as_ref() {
                            Some(v) => v,
                            None => KeyError::default_ref(),
                        } }
#[inline] pub fn mut_error(&mut self) -> &mut KeyError { if self.error.is_none() {
                                self.error = ::std::option::Option::Some(KeyError::default());
                            }
                            self.error.as_mut().unwrap() } 
#[inline] pub fn take_error(&mut self) -> KeyError { self.error.take().unwrap_or_else(KeyError::default) }
#[inline] pub fn clear_commit_version(&mut self) { self.commit_version = 0 }
#[inline] pub fn set_commit_version(&mut self, v: u64) { self.commit_version = v; }
#[inline] pub fn get_commit_version(&self) -> u64 { self.commit_version }
}
impl ::protobuf::Clear for CleanupResponse {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for CleanupResponse {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static CleanupResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: CleanupResponse = CleanupResponse::default();
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
impl BatchGetRequest {
pub fn new_() -> BatchGetRequest { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn has_context(&self) -> bool { self.context.is_some() }
#[inline] pub fn clear_context(&mut self) { self.context = ::std::option::Option::None }
#[inline] pub fn set_context(&mut self, v: Context) { self.context = ::std::option::Option::Some(v); }
#[inline] pub fn get_context(&self) -> &Context { match self.context.as_ref() {
                            Some(v) => v,
                            None => Context::default_ref(),
                        } }
#[inline] pub fn mut_context(&mut self) -> &mut Context { if self.context.is_none() {
                                self.context = ::std::option::Option::Some(Context::default());
                            }
                            self.context.as_mut().unwrap() } 
#[inline] pub fn take_context(&mut self) -> Context { self.context.take().unwrap_or_else(Context::default) }
#[inline] pub fn clear_keys(&mut self) { self.keys.clear(); }
#[inline] pub fn set_keys(&mut self, v: ::std::vec::Vec<:: prost :: alloc :: vec :: Vec < u8 >>) { self.keys = v; }
#[inline] pub fn get_keys(&self) -> &[:: prost :: alloc :: vec :: Vec < u8 >] { &self.keys }
#[inline] pub fn mut_keys(&mut self) -> &mut ::std::vec::Vec<:: prost :: alloc :: vec :: Vec < u8 >> { &mut self.keys }
#[inline] pub fn take_keys(&mut self) -> ::std::vec::Vec<:: prost :: alloc :: vec :: Vec < u8 >> { ::std::mem::replace(&mut self.keys, ::std::vec::Vec::new()) }
#[inline] pub fn clear_version(&mut self) { self.version = 0 }
#[inline] pub fn set_version(&mut self, v: u64) { self.version = v; }
#[inline] pub fn get_version(&self) -> u64 { self.version }
}
impl ::protobuf::Clear for BatchGetRequest {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for BatchGetRequest {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static BatchGetRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: BatchGetRequest = BatchGetRequest::default();
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
impl BatchGetResponse {
pub fn new_() -> BatchGetResponse { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
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
#[inline] pub fn clear_pairs(&mut self) { self.pairs.clear(); }
#[inline] pub fn set_pairs(&mut self, v: ::std::vec::Vec<KvPair>) { self.pairs = v; }
#[inline] pub fn get_pairs(&self) -> &[KvPair] { &self.pairs }
#[inline] pub fn mut_pairs(&mut self) -> &mut ::std::vec::Vec<KvPair> { &mut self.pairs }
#[inline] pub fn take_pairs(&mut self) -> ::std::vec::Vec<KvPair> { ::std::mem::replace(&mut self.pairs, ::std::vec::Vec::new()) }
#[inline] pub fn has_exec_details_v2(&self) -> bool { self.exec_details_v2.is_some() }
#[inline] pub fn clear_exec_details_v2(&mut self) { self.exec_details_v2 = ::std::option::Option::None }
#[inline] pub fn set_exec_details_v2(&mut self, v: ExecDetailsV2) { self.exec_details_v2 = ::std::option::Option::Some(v); }
#[inline] pub fn get_exec_details_v2(&self) -> &ExecDetailsV2 { match self.exec_details_v2.as_ref() {
                            Some(v) => v,
                            None => ExecDetailsV2::default_ref(),
                        } }
#[inline] pub fn mut_exec_details_v2(&mut self) -> &mut ExecDetailsV2 { if self.exec_details_v2.is_none() {
                                self.exec_details_v2 = ::std::option::Option::Some(ExecDetailsV2::default());
                            }
                            self.exec_details_v2.as_mut().unwrap() } 
#[inline] pub fn take_exec_details_v2(&mut self) -> ExecDetailsV2 { self.exec_details_v2.take().unwrap_or_else(ExecDetailsV2::default) }
#[inline] pub fn has_error(&self) -> bool { self.error.is_some() }
#[inline] pub fn clear_error(&mut self) { self.error = ::std::option::Option::None }
#[inline] pub fn set_error(&mut self, v: KeyError) { self.error = ::std::option::Option::Some(v); }
#[inline] pub fn get_error(&self) -> &KeyError { match self.error.as_ref() {
                            Some(v) => v,
                            None => KeyError::default_ref(),
                        } }
#[inline] pub fn mut_error(&mut self) -> &mut KeyError { if self.error.is_none() {
                                self.error = ::std::option::Option::Some(KeyError::default());
                            }
                            self.error.as_mut().unwrap() } 
#[inline] pub fn take_error(&mut self) -> KeyError { self.error.take().unwrap_or_else(KeyError::default) }
}
impl ::protobuf::Clear for BatchGetResponse {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for BatchGetResponse {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static BatchGetResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: BatchGetResponse = BatchGetResponse::default();
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
impl BatchRollbackRequest {
pub fn new_() -> BatchRollbackRequest { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn has_context(&self) -> bool { self.context.is_some() }
#[inline] pub fn clear_context(&mut self) { self.context = ::std::option::Option::None }
#[inline] pub fn set_context(&mut self, v: Context) { self.context = ::std::option::Option::Some(v); }
#[inline] pub fn get_context(&self) -> &Context { match self.context.as_ref() {
                            Some(v) => v,
                            None => Context::default_ref(),
                        } }
#[inline] pub fn mut_context(&mut self) -> &mut Context { if self.context.is_none() {
                                self.context = ::std::option::Option::Some(Context::default());
                            }
                            self.context.as_mut().unwrap() } 
#[inline] pub fn take_context(&mut self) -> Context { self.context.take().unwrap_or_else(Context::default) }
#[inline] pub fn clear_start_version(&mut self) { self.start_version = 0 }
#[inline] pub fn set_start_version(&mut self, v: u64) { self.start_version = v; }
#[inline] pub fn get_start_version(&self) -> u64 { self.start_version }
#[inline] pub fn clear_keys(&mut self) { self.keys.clear(); }
#[inline] pub fn set_keys(&mut self, v: ::std::vec::Vec<:: prost :: alloc :: vec :: Vec < u8 >>) { self.keys = v; }
#[inline] pub fn get_keys(&self) -> &[:: prost :: alloc :: vec :: Vec < u8 >] { &self.keys }
#[inline] pub fn mut_keys(&mut self) -> &mut ::std::vec::Vec<:: prost :: alloc :: vec :: Vec < u8 >> { &mut self.keys }
#[inline] pub fn take_keys(&mut self) -> ::std::vec::Vec<:: prost :: alloc :: vec :: Vec < u8 >> { ::std::mem::replace(&mut self.keys, ::std::vec::Vec::new()) }
}
impl ::protobuf::Clear for BatchRollbackRequest {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for BatchRollbackRequest {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static BatchRollbackRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: BatchRollbackRequest = BatchRollbackRequest::default();
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
impl BatchRollbackResponse {
pub fn new_() -> BatchRollbackResponse { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
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
#[inline] pub fn has_error(&self) -> bool { self.error.is_some() }
#[inline] pub fn clear_error(&mut self) { self.error = ::std::option::Option::None }
#[inline] pub fn set_error(&mut self, v: KeyError) { self.error = ::std::option::Option::Some(v); }
#[inline] pub fn get_error(&self) -> &KeyError { match self.error.as_ref() {
                            Some(v) => v,
                            None => KeyError::default_ref(),
                        } }
#[inline] pub fn mut_error(&mut self) -> &mut KeyError { if self.error.is_none() {
                                self.error = ::std::option::Option::Some(KeyError::default());
                            }
                            self.error.as_mut().unwrap() } 
#[inline] pub fn take_error(&mut self) -> KeyError { self.error.take().unwrap_or_else(KeyError::default) }
}
impl ::protobuf::Clear for BatchRollbackResponse {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for BatchRollbackResponse {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static BatchRollbackResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: BatchRollbackResponse = BatchRollbackResponse::default();
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
impl ScanLockRequest {
pub fn new_() -> ScanLockRequest { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn has_context(&self) -> bool { self.context.is_some() }
#[inline] pub fn clear_context(&mut self) { self.context = ::std::option::Option::None }
#[inline] pub fn set_context(&mut self, v: Context) { self.context = ::std::option::Option::Some(v); }
#[inline] pub fn get_context(&self) -> &Context { match self.context.as_ref() {
                            Some(v) => v,
                            None => Context::default_ref(),
                        } }
#[inline] pub fn mut_context(&mut self) -> &mut Context { if self.context.is_none() {
                                self.context = ::std::option::Option::Some(Context::default());
                            }
                            self.context.as_mut().unwrap() } 
#[inline] pub fn take_context(&mut self) -> Context { self.context.take().unwrap_or_else(Context::default) }
#[inline] pub fn clear_max_version(&mut self) { self.max_version = 0 }
#[inline] pub fn set_max_version(&mut self, v: u64) { self.max_version = v; }
#[inline] pub fn get_max_version(&self) -> u64 { self.max_version }
#[inline] pub fn clear_start_key(&mut self) { self.start_key.clear(); }
#[inline] pub fn set_start_key(&mut self, v: :: prost :: alloc :: vec :: Vec < u8 >) { self.start_key = v; }
#[inline] pub fn get_start_key(&self) -> &[u8] { &self.start_key }
#[inline] pub fn mut_start_key(&mut self) -> &mut :: prost :: alloc :: vec :: Vec < u8 > { &mut self.start_key }
#[inline] pub fn take_start_key(&mut self) -> :: prost :: alloc :: vec :: Vec < u8 > { ::std::mem::replace(&mut self.start_key, Default::default()) }
#[inline] pub fn clear_limit(&mut self) { self.limit = 0 }
#[inline] pub fn set_limit(&mut self, v: u32) { self.limit = v; }
#[inline] pub fn get_limit(&self) -> u32 { self.limit }
#[inline] pub fn clear_end_key(&mut self) { self.end_key.clear(); }
#[inline] pub fn set_end_key(&mut self, v: :: prost :: alloc :: vec :: Vec < u8 >) { self.end_key = v; }
#[inline] pub fn get_end_key(&self) -> &[u8] { &self.end_key }
#[inline] pub fn mut_end_key(&mut self) -> &mut :: prost :: alloc :: vec :: Vec < u8 > { &mut self.end_key }
#[inline] pub fn take_end_key(&mut self) -> :: prost :: alloc :: vec :: Vec < u8 > { ::std::mem::replace(&mut self.end_key, Default::default()) }
}
impl ::protobuf::Clear for ScanLockRequest {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for ScanLockRequest {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static ScanLockRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: ScanLockRequest = ScanLockRequest::default();
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
impl ScanLockResponse {
pub fn new_() -> ScanLockResponse { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
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
#[inline] pub fn has_error(&self) -> bool { self.error.is_some() }
#[inline] pub fn clear_error(&mut self) { self.error = ::std::option::Option::None }
#[inline] pub fn set_error(&mut self, v: KeyError) { self.error = ::std::option::Option::Some(v); }
#[inline] pub fn get_error(&self) -> &KeyError { match self.error.as_ref() {
                            Some(v) => v,
                            None => KeyError::default_ref(),
                        } }
#[inline] pub fn mut_error(&mut self) -> &mut KeyError { if self.error.is_none() {
                                self.error = ::std::option::Option::Some(KeyError::default());
                            }
                            self.error.as_mut().unwrap() } 
#[inline] pub fn take_error(&mut self) -> KeyError { self.error.take().unwrap_or_else(KeyError::default) }
#[inline] pub fn clear_locks(&mut self) { self.locks.clear(); }
#[inline] pub fn set_locks(&mut self, v: ::std::vec::Vec<LockInfo>) { self.locks = v; }
#[inline] pub fn get_locks(&self) -> &[LockInfo] { &self.locks }
#[inline] pub fn mut_locks(&mut self) -> &mut ::std::vec::Vec<LockInfo> { &mut self.locks }
#[inline] pub fn take_locks(&mut self) -> ::std::vec::Vec<LockInfo> { ::std::mem::replace(&mut self.locks, ::std::vec::Vec::new()) }
}
impl ::protobuf::Clear for ScanLockResponse {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for ScanLockResponse {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static ScanLockResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: ScanLockResponse = ScanLockResponse::default();
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
impl ResolveLockRequest {
pub fn new_() -> ResolveLockRequest { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn has_context(&self) -> bool { self.context.is_some() }
#[inline] pub fn clear_context(&mut self) { self.context = ::std::option::Option::None }
#[inline] pub fn set_context(&mut self, v: Context) { self.context = ::std::option::Option::Some(v); }
#[inline] pub fn get_context(&self) -> &Context { match self.context.as_ref() {
                            Some(v) => v,
                            None => Context::default_ref(),
                        } }
#[inline] pub fn mut_context(&mut self) -> &mut Context { if self.context.is_none() {
                                self.context = ::std::option::Option::Some(Context::default());
                            }
                            self.context.as_mut().unwrap() } 
#[inline] pub fn take_context(&mut self) -> Context { self.context.take().unwrap_or_else(Context::default) }
#[inline] pub fn clear_start_version(&mut self) { self.start_version = 0 }
#[inline] pub fn set_start_version(&mut self, v: u64) { self.start_version = v; }
#[inline] pub fn get_start_version(&self) -> u64 { self.start_version }
#[inline] pub fn clear_commit_version(&mut self) { self.commit_version = 0 }
#[inline] pub fn set_commit_version(&mut self, v: u64) { self.commit_version = v; }
#[inline] pub fn get_commit_version(&self) -> u64 { self.commit_version }
#[inline] pub fn clear_txn_infos(&mut self) { self.txn_infos.clear(); }
#[inline] pub fn set_txn_infos(&mut self, v: ::std::vec::Vec<TxnInfo>) { self.txn_infos = v; }
#[inline] pub fn get_txn_infos(&self) -> &[TxnInfo] { &self.txn_infos }
#[inline] pub fn mut_txn_infos(&mut self) -> &mut ::std::vec::Vec<TxnInfo> { &mut self.txn_infos }
#[inline] pub fn take_txn_infos(&mut self) -> ::std::vec::Vec<TxnInfo> { ::std::mem::replace(&mut self.txn_infos, ::std::vec::Vec::new()) }
#[inline] pub fn clear_keys(&mut self) { self.keys.clear(); }
#[inline] pub fn set_keys(&mut self, v: ::std::vec::Vec<:: prost :: alloc :: vec :: Vec < u8 >>) { self.keys = v; }
#[inline] pub fn get_keys(&self) -> &[:: prost :: alloc :: vec :: Vec < u8 >] { &self.keys }
#[inline] pub fn mut_keys(&mut self) -> &mut ::std::vec::Vec<:: prost :: alloc :: vec :: Vec < u8 >> { &mut self.keys }
#[inline] pub fn take_keys(&mut self) -> ::std::vec::Vec<:: prost :: alloc :: vec :: Vec < u8 >> { ::std::mem::replace(&mut self.keys, ::std::vec::Vec::new()) }
}
impl ::protobuf::Clear for ResolveLockRequest {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for ResolveLockRequest {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static ResolveLockRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: ResolveLockRequest = ResolveLockRequest::default();
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
impl ResolveLockResponse {
pub fn new_() -> ResolveLockResponse { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
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
#[inline] pub fn has_error(&self) -> bool { self.error.is_some() }
#[inline] pub fn clear_error(&mut self) { self.error = ::std::option::Option::None }
#[inline] pub fn set_error(&mut self, v: KeyError) { self.error = ::std::option::Option::Some(v); }
#[inline] pub fn get_error(&self) -> &KeyError { match self.error.as_ref() {
                            Some(v) => v,
                            None => KeyError::default_ref(),
                        } }
#[inline] pub fn mut_error(&mut self) -> &mut KeyError { if self.error.is_none() {
                                self.error = ::std::option::Option::Some(KeyError::default());
                            }
                            self.error.as_mut().unwrap() } 
#[inline] pub fn take_error(&mut self) -> KeyError { self.error.take().unwrap_or_else(KeyError::default) }
}
impl ::protobuf::Clear for ResolveLockResponse {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for ResolveLockResponse {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static ResolveLockResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: ResolveLockResponse = ResolveLockResponse::default();
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
impl GcRequest {
pub fn new_() -> GcRequest { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn has_context(&self) -> bool { self.context.is_some() }
#[inline] pub fn clear_context(&mut self) { self.context = ::std::option::Option::None }
#[inline] pub fn set_context(&mut self, v: Context) { self.context = ::std::option::Option::Some(v); }
#[inline] pub fn get_context(&self) -> &Context { match self.context.as_ref() {
                            Some(v) => v,
                            None => Context::default_ref(),
                        } }
#[inline] pub fn mut_context(&mut self) -> &mut Context { if self.context.is_none() {
                                self.context = ::std::option::Option::Some(Context::default());
                            }
                            self.context.as_mut().unwrap() } 
#[inline] pub fn take_context(&mut self) -> Context { self.context.take().unwrap_or_else(Context::default) }
#[inline] pub fn clear_safe_point(&mut self) { self.safe_point = 0 }
#[inline] pub fn set_safe_point(&mut self, v: u64) { self.safe_point = v; }
#[inline] pub fn get_safe_point(&self) -> u64 { self.safe_point }
}
impl ::protobuf::Clear for GcRequest {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for GcRequest {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static GcRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: GcRequest = GcRequest::default();
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
impl GcResponse {
pub fn new_() -> GcResponse { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
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
#[inline] pub fn has_error(&self) -> bool { self.error.is_some() }
#[inline] pub fn clear_error(&mut self) { self.error = ::std::option::Option::None }
#[inline] pub fn set_error(&mut self, v: KeyError) { self.error = ::std::option::Option::Some(v); }
#[inline] pub fn get_error(&self) -> &KeyError { match self.error.as_ref() {
                            Some(v) => v,
                            None => KeyError::default_ref(),
                        } }
#[inline] pub fn mut_error(&mut self) -> &mut KeyError { if self.error.is_none() {
                                self.error = ::std::option::Option::Some(KeyError::default());
                            }
                            self.error.as_mut().unwrap() } 
#[inline] pub fn take_error(&mut self) -> KeyError { self.error.take().unwrap_or_else(KeyError::default) }
}
impl ::protobuf::Clear for GcResponse {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for GcResponse {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static GcResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: GcResponse = GcResponse::default();
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
impl DeleteRangeRequest {
pub fn new_() -> DeleteRangeRequest { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn has_context(&self) -> bool { self.context.is_some() }
#[inline] pub fn clear_context(&mut self) { self.context = ::std::option::Option::None }
#[inline] pub fn set_context(&mut self, v: Context) { self.context = ::std::option::Option::Some(v); }
#[inline] pub fn get_context(&self) -> &Context { match self.context.as_ref() {
                            Some(v) => v,
                            None => Context::default_ref(),
                        } }
#[inline] pub fn mut_context(&mut self) -> &mut Context { if self.context.is_none() {
                                self.context = ::std::option::Option::Some(Context::default());
                            }
                            self.context.as_mut().unwrap() } 
#[inline] pub fn take_context(&mut self) -> Context { self.context.take().unwrap_or_else(Context::default) }
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
#[inline] pub fn clear_notify_only(&mut self) { self.notify_only = false }
#[inline] pub fn set_notify_only(&mut self, v: bool) { self.notify_only = v; }
#[inline] pub fn get_notify_only(&self) -> bool { self.notify_only }
}
impl ::protobuf::Clear for DeleteRangeRequest {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for DeleteRangeRequest {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static DeleteRangeRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: DeleteRangeRequest = DeleteRangeRequest::default();
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
impl DeleteRangeResponse {
pub fn new_() -> DeleteRangeResponse { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
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
#[inline] pub fn clear_error(&mut self) { self.error.clear(); }
#[inline] pub fn set_error(&mut self, v: :: prost :: alloc :: string :: String) { self.error = v; }
#[inline] pub fn get_error(&self) -> &str { &self.error }
#[inline] pub fn mut_error(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.error }
#[inline] pub fn take_error(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.error, Default::default()) }
}
impl ::protobuf::Clear for DeleteRangeResponse {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for DeleteRangeResponse {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static DeleteRangeResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: DeleteRangeResponse = DeleteRangeResponse::default();
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
impl RawGetRequest {
pub fn new_() -> RawGetRequest { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn has_context(&self) -> bool { self.context.is_some() }
#[inline] pub fn clear_context(&mut self) { self.context = ::std::option::Option::None }
#[inline] pub fn set_context(&mut self, v: Context) { self.context = ::std::option::Option::Some(v); }
#[inline] pub fn get_context(&self) -> &Context { match self.context.as_ref() {
                            Some(v) => v,
                            None => Context::default_ref(),
                        } }
#[inline] pub fn mut_context(&mut self) -> &mut Context { if self.context.is_none() {
                                self.context = ::std::option::Option::Some(Context::default());
                            }
                            self.context.as_mut().unwrap() } 
#[inline] pub fn take_context(&mut self) -> Context { self.context.take().unwrap_or_else(Context::default) }
#[inline] pub fn clear_key(&mut self) { self.key.clear(); }
#[inline] pub fn set_key(&mut self, v: :: prost :: alloc :: vec :: Vec < u8 >) { self.key = v; }
#[inline] pub fn get_key(&self) -> &[u8] { &self.key }
#[inline] pub fn mut_key(&mut self) -> &mut :: prost :: alloc :: vec :: Vec < u8 > { &mut self.key }
#[inline] pub fn take_key(&mut self) -> :: prost :: alloc :: vec :: Vec < u8 > { ::std::mem::replace(&mut self.key, Default::default()) }
#[inline] pub fn clear_cf(&mut self) { self.cf.clear(); }
#[inline] pub fn set_cf(&mut self, v: :: prost :: alloc :: string :: String) { self.cf = v; }
#[inline] pub fn get_cf(&self) -> &str { &self.cf }
#[inline] pub fn mut_cf(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.cf }
#[inline] pub fn take_cf(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.cf, Default::default()) }
}
impl ::protobuf::Clear for RawGetRequest {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for RawGetRequest {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static RawGetRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: RawGetRequest = RawGetRequest::default();
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
impl RawGetResponse {
pub fn new_() -> RawGetResponse { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
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
#[inline] pub fn clear_error(&mut self) { self.error.clear(); }
#[inline] pub fn set_error(&mut self, v: :: prost :: alloc :: string :: String) { self.error = v; }
#[inline] pub fn get_error(&self) -> &str { &self.error }
#[inline] pub fn mut_error(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.error }
#[inline] pub fn take_error(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.error, Default::default()) }
#[inline] pub fn clear_value(&mut self) { self.value.clear(); }
#[inline] pub fn set_value(&mut self, v: :: prost :: alloc :: vec :: Vec < u8 >) { self.value = v; }
#[inline] pub fn get_value(&self) -> &[u8] { &self.value }
#[inline] pub fn mut_value(&mut self) -> &mut :: prost :: alloc :: vec :: Vec < u8 > { &mut self.value }
#[inline] pub fn take_value(&mut self) -> :: prost :: alloc :: vec :: Vec < u8 > { ::std::mem::replace(&mut self.value, Default::default()) }
#[inline] pub fn clear_not_found(&mut self) { self.not_found = false }
#[inline] pub fn set_not_found(&mut self, v: bool) { self.not_found = v; }
#[inline] pub fn get_not_found(&self) -> bool { self.not_found }
}
impl ::protobuf::Clear for RawGetResponse {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for RawGetResponse {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static RawGetResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: RawGetResponse = RawGetResponse::default();
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
impl RawBatchGetRequest {
pub fn new_() -> RawBatchGetRequest { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn has_context(&self) -> bool { self.context.is_some() }
#[inline] pub fn clear_context(&mut self) { self.context = ::std::option::Option::None }
#[inline] pub fn set_context(&mut self, v: Context) { self.context = ::std::option::Option::Some(v); }
#[inline] pub fn get_context(&self) -> &Context { match self.context.as_ref() {
                            Some(v) => v,
                            None => Context::default_ref(),
                        } }
#[inline] pub fn mut_context(&mut self) -> &mut Context { if self.context.is_none() {
                                self.context = ::std::option::Option::Some(Context::default());
                            }
                            self.context.as_mut().unwrap() } 
#[inline] pub fn take_context(&mut self) -> Context { self.context.take().unwrap_or_else(Context::default) }
#[inline] pub fn clear_keys(&mut self) { self.keys.clear(); }
#[inline] pub fn set_keys(&mut self, v: ::std::vec::Vec<:: prost :: alloc :: vec :: Vec < u8 >>) { self.keys = v; }
#[inline] pub fn get_keys(&self) -> &[:: prost :: alloc :: vec :: Vec < u8 >] { &self.keys }
#[inline] pub fn mut_keys(&mut self) -> &mut ::std::vec::Vec<:: prost :: alloc :: vec :: Vec < u8 >> { &mut self.keys }
#[inline] pub fn take_keys(&mut self) -> ::std::vec::Vec<:: prost :: alloc :: vec :: Vec < u8 >> { ::std::mem::replace(&mut self.keys, ::std::vec::Vec::new()) }
#[inline] pub fn clear_cf(&mut self) { self.cf.clear(); }
#[inline] pub fn set_cf(&mut self, v: :: prost :: alloc :: string :: String) { self.cf = v; }
#[inline] pub fn get_cf(&self) -> &str { &self.cf }
#[inline] pub fn mut_cf(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.cf }
#[inline] pub fn take_cf(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.cf, Default::default()) }
}
impl ::protobuf::Clear for RawBatchGetRequest {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for RawBatchGetRequest {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static RawBatchGetRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: RawBatchGetRequest = RawBatchGetRequest::default();
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
impl RawBatchGetResponse {
pub fn new_() -> RawBatchGetResponse { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
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
#[inline] pub fn clear_pairs(&mut self) { self.pairs.clear(); }
#[inline] pub fn set_pairs(&mut self, v: ::std::vec::Vec<KvPair>) { self.pairs = v; }
#[inline] pub fn get_pairs(&self) -> &[KvPair] { &self.pairs }
#[inline] pub fn mut_pairs(&mut self) -> &mut ::std::vec::Vec<KvPair> { &mut self.pairs }
#[inline] pub fn take_pairs(&mut self) -> ::std::vec::Vec<KvPair> { ::std::mem::replace(&mut self.pairs, ::std::vec::Vec::new()) }
}
impl ::protobuf::Clear for RawBatchGetResponse {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for RawBatchGetResponse {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static RawBatchGetResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: RawBatchGetResponse = RawBatchGetResponse::default();
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
impl RawPutRequest {
pub fn new_() -> RawPutRequest { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn has_context(&self) -> bool { self.context.is_some() }
#[inline] pub fn clear_context(&mut self) { self.context = ::std::option::Option::None }
#[inline] pub fn set_context(&mut self, v: Context) { self.context = ::std::option::Option::Some(v); }
#[inline] pub fn get_context(&self) -> &Context { match self.context.as_ref() {
                            Some(v) => v,
                            None => Context::default_ref(),
                        } }
#[inline] pub fn mut_context(&mut self) -> &mut Context { if self.context.is_none() {
                                self.context = ::std::option::Option::Some(Context::default());
                            }
                            self.context.as_mut().unwrap() } 
#[inline] pub fn take_context(&mut self) -> Context { self.context.take().unwrap_or_else(Context::default) }
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
#[inline] pub fn clear_cf(&mut self) { self.cf.clear(); }
#[inline] pub fn set_cf(&mut self, v: :: prost :: alloc :: string :: String) { self.cf = v; }
#[inline] pub fn get_cf(&self) -> &str { &self.cf }
#[inline] pub fn mut_cf(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.cf }
#[inline] pub fn take_cf(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.cf, Default::default()) }
#[inline] pub fn clear_ttl(&mut self) { self.ttl = 0 }
#[inline] pub fn set_ttl(&mut self, v: u64) { self.ttl = v; }
#[inline] pub fn get_ttl(&self) -> u64 { self.ttl }
#[inline] pub fn clear_for_cas(&mut self) { self.for_cas = false }
#[inline] pub fn set_for_cas(&mut self, v: bool) { self.for_cas = v; }
#[inline] pub fn get_for_cas(&self) -> bool { self.for_cas }
}
impl ::protobuf::Clear for RawPutRequest {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for RawPutRequest {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static RawPutRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: RawPutRequest = RawPutRequest::default();
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
impl RawPutResponse {
pub fn new_() -> RawPutResponse { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
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
#[inline] pub fn clear_error(&mut self) { self.error.clear(); }
#[inline] pub fn set_error(&mut self, v: :: prost :: alloc :: string :: String) { self.error = v; }
#[inline] pub fn get_error(&self) -> &str { &self.error }
#[inline] pub fn mut_error(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.error }
#[inline] pub fn take_error(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.error, Default::default()) }
}
impl ::protobuf::Clear for RawPutResponse {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for RawPutResponse {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static RawPutResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: RawPutResponse = RawPutResponse::default();
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
impl RawBatchPutRequest {
pub fn new_() -> RawBatchPutRequest { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn has_context(&self) -> bool { self.context.is_some() }
#[inline] pub fn clear_context(&mut self) { self.context = ::std::option::Option::None }
#[inline] pub fn set_context(&mut self, v: Context) { self.context = ::std::option::Option::Some(v); }
#[inline] pub fn get_context(&self) -> &Context { match self.context.as_ref() {
                            Some(v) => v,
                            None => Context::default_ref(),
                        } }
#[inline] pub fn mut_context(&mut self) -> &mut Context { if self.context.is_none() {
                                self.context = ::std::option::Option::Some(Context::default());
                            }
                            self.context.as_mut().unwrap() } 
#[inline] pub fn take_context(&mut self) -> Context { self.context.take().unwrap_or_else(Context::default) }
#[inline] pub fn clear_pairs(&mut self) { self.pairs.clear(); }
#[inline] pub fn set_pairs(&mut self, v: ::std::vec::Vec<KvPair>) { self.pairs = v; }
#[inline] pub fn get_pairs(&self) -> &[KvPair] { &self.pairs }
#[inline] pub fn mut_pairs(&mut self) -> &mut ::std::vec::Vec<KvPair> { &mut self.pairs }
#[inline] pub fn take_pairs(&mut self) -> ::std::vec::Vec<KvPair> { ::std::mem::replace(&mut self.pairs, ::std::vec::Vec::new()) }
#[inline] pub fn clear_cf(&mut self) { self.cf.clear(); }
#[inline] pub fn set_cf(&mut self, v: :: prost :: alloc :: string :: String) { self.cf = v; }
#[inline] pub fn get_cf(&self) -> &str { &self.cf }
#[inline] pub fn mut_cf(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.cf }
#[inline] pub fn take_cf(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.cf, Default::default()) }
#[inline] pub fn clear_ttl(&mut self) { self.ttl = 0 }
#[inline] pub fn set_ttl(&mut self, v: u64) { self.ttl = v; }
#[inline] pub fn get_ttl(&self) -> u64 { self.ttl }
#[inline] pub fn clear_for_cas(&mut self) { self.for_cas = false }
#[inline] pub fn set_for_cas(&mut self, v: bool) { self.for_cas = v; }
#[inline] pub fn get_for_cas(&self) -> bool { self.for_cas }
}
impl ::protobuf::Clear for RawBatchPutRequest {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for RawBatchPutRequest {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static RawBatchPutRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: RawBatchPutRequest = RawBatchPutRequest::default();
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
impl RawBatchPutResponse {
pub fn new_() -> RawBatchPutResponse { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
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
#[inline] pub fn clear_error(&mut self) { self.error.clear(); }
#[inline] pub fn set_error(&mut self, v: :: prost :: alloc :: string :: String) { self.error = v; }
#[inline] pub fn get_error(&self) -> &str { &self.error }
#[inline] pub fn mut_error(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.error }
#[inline] pub fn take_error(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.error, Default::default()) }
}
impl ::protobuf::Clear for RawBatchPutResponse {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for RawBatchPutResponse {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static RawBatchPutResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: RawBatchPutResponse = RawBatchPutResponse::default();
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
impl RawDeleteRequest {
pub fn new_() -> RawDeleteRequest { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn has_context(&self) -> bool { self.context.is_some() }
#[inline] pub fn clear_context(&mut self) { self.context = ::std::option::Option::None }
#[inline] pub fn set_context(&mut self, v: Context) { self.context = ::std::option::Option::Some(v); }
#[inline] pub fn get_context(&self) -> &Context { match self.context.as_ref() {
                            Some(v) => v,
                            None => Context::default_ref(),
                        } }
#[inline] pub fn mut_context(&mut self) -> &mut Context { if self.context.is_none() {
                                self.context = ::std::option::Option::Some(Context::default());
                            }
                            self.context.as_mut().unwrap() } 
#[inline] pub fn take_context(&mut self) -> Context { self.context.take().unwrap_or_else(Context::default) }
#[inline] pub fn clear_key(&mut self) { self.key.clear(); }
#[inline] pub fn set_key(&mut self, v: :: prost :: alloc :: vec :: Vec < u8 >) { self.key = v; }
#[inline] pub fn get_key(&self) -> &[u8] { &self.key }
#[inline] pub fn mut_key(&mut self) -> &mut :: prost :: alloc :: vec :: Vec < u8 > { &mut self.key }
#[inline] pub fn take_key(&mut self) -> :: prost :: alloc :: vec :: Vec < u8 > { ::std::mem::replace(&mut self.key, Default::default()) }
#[inline] pub fn clear_cf(&mut self) { self.cf.clear(); }
#[inline] pub fn set_cf(&mut self, v: :: prost :: alloc :: string :: String) { self.cf = v; }
#[inline] pub fn get_cf(&self) -> &str { &self.cf }
#[inline] pub fn mut_cf(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.cf }
#[inline] pub fn take_cf(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.cf, Default::default()) }
#[inline] pub fn clear_for_cas(&mut self) { self.for_cas = false }
#[inline] pub fn set_for_cas(&mut self, v: bool) { self.for_cas = v; }
#[inline] pub fn get_for_cas(&self) -> bool { self.for_cas }
}
impl ::protobuf::Clear for RawDeleteRequest {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for RawDeleteRequest {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static RawDeleteRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: RawDeleteRequest = RawDeleteRequest::default();
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
impl RawDeleteResponse {
pub fn new_() -> RawDeleteResponse { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
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
#[inline] pub fn clear_error(&mut self) { self.error.clear(); }
#[inline] pub fn set_error(&mut self, v: :: prost :: alloc :: string :: String) { self.error = v; }
#[inline] pub fn get_error(&self) -> &str { &self.error }
#[inline] pub fn mut_error(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.error }
#[inline] pub fn take_error(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.error, Default::default()) }
}
impl ::protobuf::Clear for RawDeleteResponse {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for RawDeleteResponse {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static RawDeleteResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: RawDeleteResponse = RawDeleteResponse::default();
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
impl RawBatchDeleteRequest {
pub fn new_() -> RawBatchDeleteRequest { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn has_context(&self) -> bool { self.context.is_some() }
#[inline] pub fn clear_context(&mut self) { self.context = ::std::option::Option::None }
#[inline] pub fn set_context(&mut self, v: Context) { self.context = ::std::option::Option::Some(v); }
#[inline] pub fn get_context(&self) -> &Context { match self.context.as_ref() {
                            Some(v) => v,
                            None => Context::default_ref(),
                        } }
#[inline] pub fn mut_context(&mut self) -> &mut Context { if self.context.is_none() {
                                self.context = ::std::option::Option::Some(Context::default());
                            }
                            self.context.as_mut().unwrap() } 
#[inline] pub fn take_context(&mut self) -> Context { self.context.take().unwrap_or_else(Context::default) }
#[inline] pub fn clear_keys(&mut self) { self.keys.clear(); }
#[inline] pub fn set_keys(&mut self, v: ::std::vec::Vec<:: prost :: alloc :: vec :: Vec < u8 >>) { self.keys = v; }
#[inline] pub fn get_keys(&self) -> &[:: prost :: alloc :: vec :: Vec < u8 >] { &self.keys }
#[inline] pub fn mut_keys(&mut self) -> &mut ::std::vec::Vec<:: prost :: alloc :: vec :: Vec < u8 >> { &mut self.keys }
#[inline] pub fn take_keys(&mut self) -> ::std::vec::Vec<:: prost :: alloc :: vec :: Vec < u8 >> { ::std::mem::replace(&mut self.keys, ::std::vec::Vec::new()) }
#[inline] pub fn clear_cf(&mut self) { self.cf.clear(); }
#[inline] pub fn set_cf(&mut self, v: :: prost :: alloc :: string :: String) { self.cf = v; }
#[inline] pub fn get_cf(&self) -> &str { &self.cf }
#[inline] pub fn mut_cf(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.cf }
#[inline] pub fn take_cf(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.cf, Default::default()) }
#[inline] pub fn clear_for_cas(&mut self) { self.for_cas = false }
#[inline] pub fn set_for_cas(&mut self, v: bool) { self.for_cas = v; }
#[inline] pub fn get_for_cas(&self) -> bool { self.for_cas }
}
impl ::protobuf::Clear for RawBatchDeleteRequest {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for RawBatchDeleteRequest {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static RawBatchDeleteRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: RawBatchDeleteRequest = RawBatchDeleteRequest::default();
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
impl RawBatchDeleteResponse {
pub fn new_() -> RawBatchDeleteResponse { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
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
#[inline] pub fn clear_error(&mut self) { self.error.clear(); }
#[inline] pub fn set_error(&mut self, v: :: prost :: alloc :: string :: String) { self.error = v; }
#[inline] pub fn get_error(&self) -> &str { &self.error }
#[inline] pub fn mut_error(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.error }
#[inline] pub fn take_error(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.error, Default::default()) }
}
impl ::protobuf::Clear for RawBatchDeleteResponse {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for RawBatchDeleteResponse {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static RawBatchDeleteResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: RawBatchDeleteResponse = RawBatchDeleteResponse::default();
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
impl RawScanRequest {
pub fn new_() -> RawScanRequest { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn has_context(&self) -> bool { self.context.is_some() }
#[inline] pub fn clear_context(&mut self) { self.context = ::std::option::Option::None }
#[inline] pub fn set_context(&mut self, v: Context) { self.context = ::std::option::Option::Some(v); }
#[inline] pub fn get_context(&self) -> &Context { match self.context.as_ref() {
                            Some(v) => v,
                            None => Context::default_ref(),
                        } }
#[inline] pub fn mut_context(&mut self) -> &mut Context { if self.context.is_none() {
                                self.context = ::std::option::Option::Some(Context::default());
                            }
                            self.context.as_mut().unwrap() } 
#[inline] pub fn take_context(&mut self) -> Context { self.context.take().unwrap_or_else(Context::default) }
#[inline] pub fn clear_start_key(&mut self) { self.start_key.clear(); }
#[inline] pub fn set_start_key(&mut self, v: :: prost :: alloc :: vec :: Vec < u8 >) { self.start_key = v; }
#[inline] pub fn get_start_key(&self) -> &[u8] { &self.start_key }
#[inline] pub fn mut_start_key(&mut self) -> &mut :: prost :: alloc :: vec :: Vec < u8 > { &mut self.start_key }
#[inline] pub fn take_start_key(&mut self) -> :: prost :: alloc :: vec :: Vec < u8 > { ::std::mem::replace(&mut self.start_key, Default::default()) }
#[inline] pub fn clear_limit(&mut self) { self.limit = 0 }
#[inline] pub fn set_limit(&mut self, v: u32) { self.limit = v; }
#[inline] pub fn get_limit(&self) -> u32 { self.limit }
#[inline] pub fn clear_key_only(&mut self) { self.key_only = false }
#[inline] pub fn set_key_only(&mut self, v: bool) { self.key_only = v; }
#[inline] pub fn get_key_only(&self) -> bool { self.key_only }
#[inline] pub fn clear_cf(&mut self) { self.cf.clear(); }
#[inline] pub fn set_cf(&mut self, v: :: prost :: alloc :: string :: String) { self.cf = v; }
#[inline] pub fn get_cf(&self) -> &str { &self.cf }
#[inline] pub fn mut_cf(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.cf }
#[inline] pub fn take_cf(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.cf, Default::default()) }
#[inline] pub fn clear_reverse(&mut self) { self.reverse = false }
#[inline] pub fn set_reverse(&mut self, v: bool) { self.reverse = v; }
#[inline] pub fn get_reverse(&self) -> bool { self.reverse }
#[inline] pub fn clear_end_key(&mut self) { self.end_key.clear(); }
#[inline] pub fn set_end_key(&mut self, v: :: prost :: alloc :: vec :: Vec < u8 >) { self.end_key = v; }
#[inline] pub fn get_end_key(&self) -> &[u8] { &self.end_key }
#[inline] pub fn mut_end_key(&mut self) -> &mut :: prost :: alloc :: vec :: Vec < u8 > { &mut self.end_key }
#[inline] pub fn take_end_key(&mut self) -> :: prost :: alloc :: vec :: Vec < u8 > { ::std::mem::replace(&mut self.end_key, Default::default()) }
}
impl ::protobuf::Clear for RawScanRequest {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for RawScanRequest {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static RawScanRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: RawScanRequest = RawScanRequest::default();
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
impl RawScanResponse {
pub fn new_() -> RawScanResponse { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
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
#[inline] pub fn clear_kvs(&mut self) { self.kvs.clear(); }
#[inline] pub fn set_kvs(&mut self, v: ::std::vec::Vec<KvPair>) { self.kvs = v; }
#[inline] pub fn get_kvs(&self) -> &[KvPair] { &self.kvs }
#[inline] pub fn mut_kvs(&mut self) -> &mut ::std::vec::Vec<KvPair> { &mut self.kvs }
#[inline] pub fn take_kvs(&mut self) -> ::std::vec::Vec<KvPair> { ::std::mem::replace(&mut self.kvs, ::std::vec::Vec::new()) }
}
impl ::protobuf::Clear for RawScanResponse {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for RawScanResponse {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static RawScanResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: RawScanResponse = RawScanResponse::default();
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
impl RawDeleteRangeRequest {
pub fn new_() -> RawDeleteRangeRequest { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn has_context(&self) -> bool { self.context.is_some() }
#[inline] pub fn clear_context(&mut self) { self.context = ::std::option::Option::None }
#[inline] pub fn set_context(&mut self, v: Context) { self.context = ::std::option::Option::Some(v); }
#[inline] pub fn get_context(&self) -> &Context { match self.context.as_ref() {
                            Some(v) => v,
                            None => Context::default_ref(),
                        } }
#[inline] pub fn mut_context(&mut self) -> &mut Context { if self.context.is_none() {
                                self.context = ::std::option::Option::Some(Context::default());
                            }
                            self.context.as_mut().unwrap() } 
#[inline] pub fn take_context(&mut self) -> Context { self.context.take().unwrap_or_else(Context::default) }
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
#[inline] pub fn clear_cf(&mut self) { self.cf.clear(); }
#[inline] pub fn set_cf(&mut self, v: :: prost :: alloc :: string :: String) { self.cf = v; }
#[inline] pub fn get_cf(&self) -> &str { &self.cf }
#[inline] pub fn mut_cf(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.cf }
#[inline] pub fn take_cf(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.cf, Default::default()) }
}
impl ::protobuf::Clear for RawDeleteRangeRequest {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for RawDeleteRangeRequest {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static RawDeleteRangeRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: RawDeleteRangeRequest = RawDeleteRangeRequest::default();
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
impl RawDeleteRangeResponse {
pub fn new_() -> RawDeleteRangeResponse { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
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
#[inline] pub fn clear_error(&mut self) { self.error.clear(); }
#[inline] pub fn set_error(&mut self, v: :: prost :: alloc :: string :: String) { self.error = v; }
#[inline] pub fn get_error(&self) -> &str { &self.error }
#[inline] pub fn mut_error(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.error }
#[inline] pub fn take_error(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.error, Default::default()) }
}
impl ::protobuf::Clear for RawDeleteRangeResponse {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for RawDeleteRangeResponse {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static RawDeleteRangeResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: RawDeleteRangeResponse = RawDeleteRangeResponse::default();
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
impl RawBatchScanRequest {
pub fn new_() -> RawBatchScanRequest { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn has_context(&self) -> bool { self.context.is_some() }
#[inline] pub fn clear_context(&mut self) { self.context = ::std::option::Option::None }
#[inline] pub fn set_context(&mut self, v: Context) { self.context = ::std::option::Option::Some(v); }
#[inline] pub fn get_context(&self) -> &Context { match self.context.as_ref() {
                            Some(v) => v,
                            None => Context::default_ref(),
                        } }
#[inline] pub fn mut_context(&mut self) -> &mut Context { if self.context.is_none() {
                                self.context = ::std::option::Option::Some(Context::default());
                            }
                            self.context.as_mut().unwrap() } 
#[inline] pub fn take_context(&mut self) -> Context { self.context.take().unwrap_or_else(Context::default) }
#[inline] pub fn clear_ranges(&mut self) { self.ranges.clear(); }
#[inline] pub fn set_ranges(&mut self, v: ::std::vec::Vec<KeyRange>) { self.ranges = v; }
#[inline] pub fn get_ranges(&self) -> &[KeyRange] { &self.ranges }
#[inline] pub fn mut_ranges(&mut self) -> &mut ::std::vec::Vec<KeyRange> { &mut self.ranges }
#[inline] pub fn take_ranges(&mut self) -> ::std::vec::Vec<KeyRange> { ::std::mem::replace(&mut self.ranges, ::std::vec::Vec::new()) }
#[inline] pub fn clear_each_limit(&mut self) { self.each_limit = 0 }
#[inline] pub fn set_each_limit(&mut self, v: u32) { self.each_limit = v; }
#[inline] pub fn get_each_limit(&self) -> u32 { self.each_limit }
#[inline] pub fn clear_key_only(&mut self) { self.key_only = false }
#[inline] pub fn set_key_only(&mut self, v: bool) { self.key_only = v; }
#[inline] pub fn get_key_only(&self) -> bool { self.key_only }
#[inline] pub fn clear_cf(&mut self) { self.cf.clear(); }
#[inline] pub fn set_cf(&mut self, v: :: prost :: alloc :: string :: String) { self.cf = v; }
#[inline] pub fn get_cf(&self) -> &str { &self.cf }
#[inline] pub fn mut_cf(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.cf }
#[inline] pub fn take_cf(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.cf, Default::default()) }
#[inline] pub fn clear_reverse(&mut self) { self.reverse = false }
#[inline] pub fn set_reverse(&mut self, v: bool) { self.reverse = v; }
#[inline] pub fn get_reverse(&self) -> bool { self.reverse }
}
impl ::protobuf::Clear for RawBatchScanRequest {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for RawBatchScanRequest {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static RawBatchScanRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: RawBatchScanRequest = RawBatchScanRequest::default();
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
impl RawBatchScanResponse {
pub fn new_() -> RawBatchScanResponse { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
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
#[inline] pub fn clear_kvs(&mut self) { self.kvs.clear(); }
#[inline] pub fn set_kvs(&mut self, v: ::std::vec::Vec<KvPair>) { self.kvs = v; }
#[inline] pub fn get_kvs(&self) -> &[KvPair] { &self.kvs }
#[inline] pub fn mut_kvs(&mut self) -> &mut ::std::vec::Vec<KvPair> { &mut self.kvs }
#[inline] pub fn take_kvs(&mut self) -> ::std::vec::Vec<KvPair> { ::std::mem::replace(&mut self.kvs, ::std::vec::Vec::new()) }
}
impl ::protobuf::Clear for RawBatchScanResponse {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for RawBatchScanResponse {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static RawBatchScanResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: RawBatchScanResponse = RawBatchScanResponse::default();
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
impl UnsafeDestroyRangeRequest {
pub fn new_() -> UnsafeDestroyRangeRequest { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn has_context(&self) -> bool { self.context.is_some() }
#[inline] pub fn clear_context(&mut self) { self.context = ::std::option::Option::None }
#[inline] pub fn set_context(&mut self, v: Context) { self.context = ::std::option::Option::Some(v); }
#[inline] pub fn get_context(&self) -> &Context { match self.context.as_ref() {
                            Some(v) => v,
                            None => Context::default_ref(),
                        } }
#[inline] pub fn mut_context(&mut self) -> &mut Context { if self.context.is_none() {
                                self.context = ::std::option::Option::Some(Context::default());
                            }
                            self.context.as_mut().unwrap() } 
#[inline] pub fn take_context(&mut self) -> Context { self.context.take().unwrap_or_else(Context::default) }
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
}
impl ::protobuf::Clear for UnsafeDestroyRangeRequest {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for UnsafeDestroyRangeRequest {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static UnsafeDestroyRangeRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: UnsafeDestroyRangeRequest = UnsafeDestroyRangeRequest::default();
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
impl UnsafeDestroyRangeResponse {
pub fn new_() -> UnsafeDestroyRangeResponse { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
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
#[inline] pub fn clear_error(&mut self) { self.error.clear(); }
#[inline] pub fn set_error(&mut self, v: :: prost :: alloc :: string :: String) { self.error = v; }
#[inline] pub fn get_error(&self) -> &str { &self.error }
#[inline] pub fn mut_error(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.error }
#[inline] pub fn take_error(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.error, Default::default()) }
}
impl ::protobuf::Clear for UnsafeDestroyRangeResponse {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for UnsafeDestroyRangeResponse {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static UnsafeDestroyRangeResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: UnsafeDestroyRangeResponse = UnsafeDestroyRangeResponse::default();
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
impl RegisterLockObserverRequest {
pub fn new_() -> RegisterLockObserverRequest { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn has_context(&self) -> bool { self.context.is_some() }
#[inline] pub fn clear_context(&mut self) { self.context = ::std::option::Option::None }
#[inline] pub fn set_context(&mut self, v: Context) { self.context = ::std::option::Option::Some(v); }
#[inline] pub fn get_context(&self) -> &Context { match self.context.as_ref() {
                            Some(v) => v,
                            None => Context::default_ref(),
                        } }
#[inline] pub fn mut_context(&mut self) -> &mut Context { if self.context.is_none() {
                                self.context = ::std::option::Option::Some(Context::default());
                            }
                            self.context.as_mut().unwrap() } 
#[inline] pub fn take_context(&mut self) -> Context { self.context.take().unwrap_or_else(Context::default) }
#[inline] pub fn clear_max_ts(&mut self) { self.max_ts = 0 }
#[inline] pub fn set_max_ts(&mut self, v: u64) { self.max_ts = v; }
#[inline] pub fn get_max_ts(&self) -> u64 { self.max_ts }
}
impl ::protobuf::Clear for RegisterLockObserverRequest {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for RegisterLockObserverRequest {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static RegisterLockObserverRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: RegisterLockObserverRequest = RegisterLockObserverRequest::default();
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
impl RegisterLockObserverResponse {
pub fn new_() -> RegisterLockObserverResponse { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn clear_error(&mut self) { self.error.clear(); }
#[inline] pub fn set_error(&mut self, v: :: prost :: alloc :: string :: String) { self.error = v; }
#[inline] pub fn get_error(&self) -> &str { &self.error }
#[inline] pub fn mut_error(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.error }
#[inline] pub fn take_error(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.error, Default::default()) }
}
impl ::protobuf::Clear for RegisterLockObserverResponse {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for RegisterLockObserverResponse {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static RegisterLockObserverResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: RegisterLockObserverResponse = RegisterLockObserverResponse::default();
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
impl CheckLockObserverRequest {
pub fn new_() -> CheckLockObserverRequest { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn has_context(&self) -> bool { self.context.is_some() }
#[inline] pub fn clear_context(&mut self) { self.context = ::std::option::Option::None }
#[inline] pub fn set_context(&mut self, v: Context) { self.context = ::std::option::Option::Some(v); }
#[inline] pub fn get_context(&self) -> &Context { match self.context.as_ref() {
                            Some(v) => v,
                            None => Context::default_ref(),
                        } }
#[inline] pub fn mut_context(&mut self) -> &mut Context { if self.context.is_none() {
                                self.context = ::std::option::Option::Some(Context::default());
                            }
                            self.context.as_mut().unwrap() } 
#[inline] pub fn take_context(&mut self) -> Context { self.context.take().unwrap_or_else(Context::default) }
#[inline] pub fn clear_max_ts(&mut self) { self.max_ts = 0 }
#[inline] pub fn set_max_ts(&mut self, v: u64) { self.max_ts = v; }
#[inline] pub fn get_max_ts(&self) -> u64 { self.max_ts }
}
impl ::protobuf::Clear for CheckLockObserverRequest {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for CheckLockObserverRequest {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static CheckLockObserverRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: CheckLockObserverRequest = CheckLockObserverRequest::default();
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
impl CheckLockObserverResponse {
pub fn new_() -> CheckLockObserverResponse { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn clear_error(&mut self) { self.error.clear(); }
#[inline] pub fn set_error(&mut self, v: :: prost :: alloc :: string :: String) { self.error = v; }
#[inline] pub fn get_error(&self) -> &str { &self.error }
#[inline] pub fn mut_error(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.error }
#[inline] pub fn take_error(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.error, Default::default()) }
#[inline] pub fn clear_is_clean(&mut self) { self.is_clean = false }
#[inline] pub fn set_is_clean(&mut self, v: bool) { self.is_clean = v; }
#[inline] pub fn get_is_clean(&self) -> bool { self.is_clean }
#[inline] pub fn clear_locks(&mut self) { self.locks.clear(); }
#[inline] pub fn set_locks(&mut self, v: ::std::vec::Vec<LockInfo>) { self.locks = v; }
#[inline] pub fn get_locks(&self) -> &[LockInfo] { &self.locks }
#[inline] pub fn mut_locks(&mut self) -> &mut ::std::vec::Vec<LockInfo> { &mut self.locks }
#[inline] pub fn take_locks(&mut self) -> ::std::vec::Vec<LockInfo> { ::std::mem::replace(&mut self.locks, ::std::vec::Vec::new()) }
}
impl ::protobuf::Clear for CheckLockObserverResponse {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for CheckLockObserverResponse {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static CheckLockObserverResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: CheckLockObserverResponse = CheckLockObserverResponse::default();
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
impl RemoveLockObserverRequest {
pub fn new_() -> RemoveLockObserverRequest { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn has_context(&self) -> bool { self.context.is_some() }
#[inline] pub fn clear_context(&mut self) { self.context = ::std::option::Option::None }
#[inline] pub fn set_context(&mut self, v: Context) { self.context = ::std::option::Option::Some(v); }
#[inline] pub fn get_context(&self) -> &Context { match self.context.as_ref() {
                            Some(v) => v,
                            None => Context::default_ref(),
                        } }
#[inline] pub fn mut_context(&mut self) -> &mut Context { if self.context.is_none() {
                                self.context = ::std::option::Option::Some(Context::default());
                            }
                            self.context.as_mut().unwrap() } 
#[inline] pub fn take_context(&mut self) -> Context { self.context.take().unwrap_or_else(Context::default) }
#[inline] pub fn clear_max_ts(&mut self) { self.max_ts = 0 }
#[inline] pub fn set_max_ts(&mut self, v: u64) { self.max_ts = v; }
#[inline] pub fn get_max_ts(&self) -> u64 { self.max_ts }
}
impl ::protobuf::Clear for RemoveLockObserverRequest {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for RemoveLockObserverRequest {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static RemoveLockObserverRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: RemoveLockObserverRequest = RemoveLockObserverRequest::default();
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
impl RemoveLockObserverResponse {
pub fn new_() -> RemoveLockObserverResponse { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn clear_error(&mut self) { self.error.clear(); }
#[inline] pub fn set_error(&mut self, v: :: prost :: alloc :: string :: String) { self.error = v; }
#[inline] pub fn get_error(&self) -> &str { &self.error }
#[inline] pub fn mut_error(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.error }
#[inline] pub fn take_error(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.error, Default::default()) }
}
impl ::protobuf::Clear for RemoveLockObserverResponse {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for RemoveLockObserverResponse {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static RemoveLockObserverResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: RemoveLockObserverResponse = RemoveLockObserverResponse::default();
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
impl PhysicalScanLockRequest {
pub fn new_() -> PhysicalScanLockRequest { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn has_context(&self) -> bool { self.context.is_some() }
#[inline] pub fn clear_context(&mut self) { self.context = ::std::option::Option::None }
#[inline] pub fn set_context(&mut self, v: Context) { self.context = ::std::option::Option::Some(v); }
#[inline] pub fn get_context(&self) -> &Context { match self.context.as_ref() {
                            Some(v) => v,
                            None => Context::default_ref(),
                        } }
#[inline] pub fn mut_context(&mut self) -> &mut Context { if self.context.is_none() {
                                self.context = ::std::option::Option::Some(Context::default());
                            }
                            self.context.as_mut().unwrap() } 
#[inline] pub fn take_context(&mut self) -> Context { self.context.take().unwrap_or_else(Context::default) }
#[inline] pub fn clear_max_ts(&mut self) { self.max_ts = 0 }
#[inline] pub fn set_max_ts(&mut self, v: u64) { self.max_ts = v; }
#[inline] pub fn get_max_ts(&self) -> u64 { self.max_ts }
#[inline] pub fn clear_start_key(&mut self) { self.start_key.clear(); }
#[inline] pub fn set_start_key(&mut self, v: :: prost :: alloc :: vec :: Vec < u8 >) { self.start_key = v; }
#[inline] pub fn get_start_key(&self) -> &[u8] { &self.start_key }
#[inline] pub fn mut_start_key(&mut self) -> &mut :: prost :: alloc :: vec :: Vec < u8 > { &mut self.start_key }
#[inline] pub fn take_start_key(&mut self) -> :: prost :: alloc :: vec :: Vec < u8 > { ::std::mem::replace(&mut self.start_key, Default::default()) }
#[inline] pub fn clear_limit(&mut self) { self.limit = 0 }
#[inline] pub fn set_limit(&mut self, v: u32) { self.limit = v; }
#[inline] pub fn get_limit(&self) -> u32 { self.limit }
}
impl ::protobuf::Clear for PhysicalScanLockRequest {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for PhysicalScanLockRequest {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static PhysicalScanLockRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: PhysicalScanLockRequest = PhysicalScanLockRequest::default();
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
impl PhysicalScanLockResponse {
pub fn new_() -> PhysicalScanLockResponse { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn clear_error(&mut self) { self.error.clear(); }
#[inline] pub fn set_error(&mut self, v: :: prost :: alloc :: string :: String) { self.error = v; }
#[inline] pub fn get_error(&self) -> &str { &self.error }
#[inline] pub fn mut_error(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.error }
#[inline] pub fn take_error(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.error, Default::default()) }
#[inline] pub fn clear_locks(&mut self) { self.locks.clear(); }
#[inline] pub fn set_locks(&mut self, v: ::std::vec::Vec<LockInfo>) { self.locks = v; }
#[inline] pub fn get_locks(&self) -> &[LockInfo] { &self.locks }
#[inline] pub fn mut_locks(&mut self) -> &mut ::std::vec::Vec<LockInfo> { &mut self.locks }
#[inline] pub fn take_locks(&mut self) -> ::std::vec::Vec<LockInfo> { ::std::mem::replace(&mut self.locks, ::std::vec::Vec::new()) }
}
impl ::protobuf::Clear for PhysicalScanLockResponse {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for PhysicalScanLockResponse {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static PhysicalScanLockResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: PhysicalScanLockResponse = PhysicalScanLockResponse::default();
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
impl SplitRegionRequest {
pub fn new_() -> SplitRegionRequest { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn has_context(&self) -> bool { self.context.is_some() }
#[inline] pub fn clear_context(&mut self) { self.context = ::std::option::Option::None }
#[inline] pub fn set_context(&mut self, v: Context) { self.context = ::std::option::Option::Some(v); }
#[inline] pub fn get_context(&self) -> &Context { match self.context.as_ref() {
                            Some(v) => v,
                            None => Context::default_ref(),
                        } }
#[inline] pub fn mut_context(&mut self) -> &mut Context { if self.context.is_none() {
                                self.context = ::std::option::Option::Some(Context::default());
                            }
                            self.context.as_mut().unwrap() } 
#[inline] pub fn take_context(&mut self) -> Context { self.context.take().unwrap_or_else(Context::default) }
#[allow(deprecated)] #[inline] pub fn clear_split_key(&mut self) { self.split_key.clear(); }
#[allow(deprecated)] #[inline] pub fn set_split_key(&mut self, v: :: prost :: alloc :: vec :: Vec < u8 >) { self.split_key = v; }
#[allow(deprecated)] #[inline] pub fn get_split_key(&self) -> &[u8] { &self.split_key }
#[allow(deprecated)] #[inline] pub fn mut_split_key(&mut self) -> &mut :: prost :: alloc :: vec :: Vec < u8 > { &mut self.split_key }
#[allow(deprecated)] #[inline] pub fn take_split_key(&mut self) -> :: prost :: alloc :: vec :: Vec < u8 > { ::std::mem::replace(&mut self.split_key, Default::default()) }
#[inline] pub fn clear_split_keys(&mut self) { self.split_keys.clear(); }
#[inline] pub fn set_split_keys(&mut self, v: ::std::vec::Vec<:: prost :: alloc :: vec :: Vec < u8 >>) { self.split_keys = v; }
#[inline] pub fn get_split_keys(&self) -> &[:: prost :: alloc :: vec :: Vec < u8 >] { &self.split_keys }
#[inline] pub fn mut_split_keys(&mut self) -> &mut ::std::vec::Vec<:: prost :: alloc :: vec :: Vec < u8 >> { &mut self.split_keys }
#[inline] pub fn take_split_keys(&mut self) -> ::std::vec::Vec<:: prost :: alloc :: vec :: Vec < u8 >> { ::std::mem::replace(&mut self.split_keys, ::std::vec::Vec::new()) }
}
impl ::protobuf::Clear for SplitRegionRequest {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for SplitRegionRequest {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static SplitRegionRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: SplitRegionRequest = SplitRegionRequest::default();
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
impl SplitRegionResponse {
pub fn new_() -> SplitRegionResponse { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
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
#[allow(deprecated)] #[inline] pub fn has_left(&self) -> bool { self.left.is_some() }
#[allow(deprecated)] #[inline] pub fn clear_left(&mut self) { self.left = ::std::option::Option::None }
#[allow(deprecated)] #[inline] pub fn set_left(&mut self, v: super :: metapb :: Region) { self.left = ::std::option::Option::Some(v); }
#[allow(deprecated)] #[inline] pub fn get_left(&self) -> &super :: metapb :: Region { match self.left.as_ref() {
                            Some(v) => v,
                            None => super :: metapb :: Region::default_ref(),
                        } }
#[allow(deprecated)] #[inline] pub fn mut_left(&mut self) -> &mut super :: metapb :: Region { if self.left.is_none() {
                                self.left = ::std::option::Option::Some(super :: metapb :: Region::default());
                            }
                            self.left.as_mut().unwrap() } 
#[allow(deprecated)] #[inline] pub fn take_left(&mut self) -> super :: metapb :: Region { self.left.take().unwrap_or_else(super :: metapb :: Region::default) }
#[allow(deprecated)] #[inline] pub fn has_right(&self) -> bool { self.right.is_some() }
#[allow(deprecated)] #[inline] pub fn clear_right(&mut self) { self.right = ::std::option::Option::None }
#[allow(deprecated)] #[inline] pub fn set_right(&mut self, v: super :: metapb :: Region) { self.right = ::std::option::Option::Some(v); }
#[allow(deprecated)] #[inline] pub fn get_right(&self) -> &super :: metapb :: Region { match self.right.as_ref() {
                            Some(v) => v,
                            None => super :: metapb :: Region::default_ref(),
                        } }
#[allow(deprecated)] #[inline] pub fn mut_right(&mut self) -> &mut super :: metapb :: Region { if self.right.is_none() {
                                self.right = ::std::option::Option::Some(super :: metapb :: Region::default());
                            }
                            self.right.as_mut().unwrap() } 
#[allow(deprecated)] #[inline] pub fn take_right(&mut self) -> super :: metapb :: Region { self.right.take().unwrap_or_else(super :: metapb :: Region::default) }
#[inline] pub fn clear_regions(&mut self) { self.regions.clear(); }
#[inline] pub fn set_regions(&mut self, v: ::std::vec::Vec<super :: metapb :: Region>) { self.regions = v; }
#[inline] pub fn get_regions(&self) -> &[super :: metapb :: Region] { &self.regions }
#[inline] pub fn mut_regions(&mut self) -> &mut ::std::vec::Vec<super :: metapb :: Region> { &mut self.regions }
#[inline] pub fn take_regions(&mut self) -> ::std::vec::Vec<super :: metapb :: Region> { ::std::mem::replace(&mut self.regions, ::std::vec::Vec::new()) }
}
impl ::protobuf::Clear for SplitRegionResponse {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for SplitRegionResponse {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static SplitRegionResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: SplitRegionResponse = SplitRegionResponse::default();
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
impl ReadIndexRequest {
pub fn new_() -> ReadIndexRequest { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn has_context(&self) -> bool { self.context.is_some() }
#[inline] pub fn clear_context(&mut self) { self.context = ::std::option::Option::None }
#[inline] pub fn set_context(&mut self, v: Context) { self.context = ::std::option::Option::Some(v); }
#[inline] pub fn get_context(&self) -> &Context { match self.context.as_ref() {
                            Some(v) => v,
                            None => Context::default_ref(),
                        } }
#[inline] pub fn mut_context(&mut self) -> &mut Context { if self.context.is_none() {
                                self.context = ::std::option::Option::Some(Context::default());
                            }
                            self.context.as_mut().unwrap() } 
#[inline] pub fn take_context(&mut self) -> Context { self.context.take().unwrap_or_else(Context::default) }
#[inline] pub fn clear_start_ts(&mut self) { self.start_ts = 0 }
#[inline] pub fn set_start_ts(&mut self, v: u64) { self.start_ts = v; }
#[inline] pub fn get_start_ts(&self) -> u64 { self.start_ts }
#[inline] pub fn clear_ranges(&mut self) { self.ranges.clear(); }
#[inline] pub fn set_ranges(&mut self, v: ::std::vec::Vec<KeyRange>) { self.ranges = v; }
#[inline] pub fn get_ranges(&self) -> &[KeyRange] { &self.ranges }
#[inline] pub fn mut_ranges(&mut self) -> &mut ::std::vec::Vec<KeyRange> { &mut self.ranges }
#[inline] pub fn take_ranges(&mut self) -> ::std::vec::Vec<KeyRange> { ::std::mem::replace(&mut self.ranges, ::std::vec::Vec::new()) }
}
impl ::protobuf::Clear for ReadIndexRequest {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for ReadIndexRequest {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static ReadIndexRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: ReadIndexRequest = ReadIndexRequest::default();
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
impl ReadIndexResponse {
pub fn new_() -> ReadIndexResponse { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
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
#[inline] pub fn clear_read_index(&mut self) { self.read_index = 0 }
#[inline] pub fn set_read_index(&mut self, v: u64) { self.read_index = v; }
#[inline] pub fn get_read_index(&self) -> u64 { self.read_index }
#[inline] pub fn has_locked(&self) -> bool { self.locked.is_some() }
#[inline] pub fn clear_locked(&mut self) { self.locked = ::std::option::Option::None }
#[inline] pub fn set_locked(&mut self, v: LockInfo) { self.locked = ::std::option::Option::Some(v); }
#[inline] pub fn get_locked(&self) -> &LockInfo { match self.locked.as_ref() {
                            Some(v) => v,
                            None => LockInfo::default_ref(),
                        } }
#[inline] pub fn mut_locked(&mut self) -> &mut LockInfo { if self.locked.is_none() {
                                self.locked = ::std::option::Option::Some(LockInfo::default());
                            }
                            self.locked.as_mut().unwrap() } 
#[inline] pub fn take_locked(&mut self) -> LockInfo { self.locked.take().unwrap_or_else(LockInfo::default) }
}
impl ::protobuf::Clear for ReadIndexResponse {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for ReadIndexResponse {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static ReadIndexResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: ReadIndexResponse = ReadIndexResponse::default();
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
impl VerMutation {
pub fn new_() -> VerMutation { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn clear_op(&mut self) { self.op = 0 }
#[inline] pub fn get_op(&self) -> VerOp { match VerOp::from_i32(self.op) {Some(e) => e,
                        None => panic!("Unknown enum variant: {}", self.op),
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
}
impl ::protobuf::Clear for VerMutation {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for VerMutation {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static VerMutation {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: VerMutation = VerMutation::default();
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
impl VerValue {
pub fn new_() -> VerValue { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn clear_value(&mut self) { self.value.clear(); }
#[inline] pub fn set_value(&mut self, v: :: prost :: alloc :: vec :: Vec < u8 >) { self.value = v; }
#[inline] pub fn get_value(&self) -> &[u8] { &self.value }
#[inline] pub fn mut_value(&mut self) -> &mut :: prost :: alloc :: vec :: Vec < u8 > { &mut self.value }
#[inline] pub fn take_value(&mut self) -> :: prost :: alloc :: vec :: Vec < u8 > { ::std::mem::replace(&mut self.value, Default::default()) }
#[inline] pub fn clear_version(&mut self) { self.version = 0 }
#[inline] pub fn set_version(&mut self, v: u64) { self.version = v; }
#[inline] pub fn get_version(&self) -> u64 { self.version }
}
impl ::protobuf::Clear for VerValue {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for VerValue {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static VerValue {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: VerValue = VerValue::default();
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
impl VerError {
pub fn new_() -> VerError { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn clear_error(&mut self) { self.error.clear(); }
#[inline] pub fn set_error(&mut self, v: :: prost :: alloc :: string :: String) { self.error = v; }
#[inline] pub fn get_error(&self) -> &str { &self.error }
#[inline] pub fn mut_error(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.error }
#[inline] pub fn take_error(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.error, Default::default()) }
}
impl ::protobuf::Clear for VerError {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for VerError {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static VerError {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: VerError = VerError::default();
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
impl VerKvPair {
pub fn new_() -> VerKvPair { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn has_error(&self) -> bool { self.error.is_some() }
#[inline] pub fn clear_error(&mut self) { self.error = ::std::option::Option::None }
#[inline] pub fn set_error(&mut self, v: VerError) { self.error = ::std::option::Option::Some(v); }
#[inline] pub fn get_error(&self) -> &VerError { match self.error.as_ref() {
                            Some(v) => v,
                            None => VerError::default_ref(),
                        } }
#[inline] pub fn mut_error(&mut self) -> &mut VerError { if self.error.is_none() {
                                self.error = ::std::option::Option::Some(VerError::default());
                            }
                            self.error.as_mut().unwrap() } 
#[inline] pub fn take_error(&mut self) -> VerError { self.error.take().unwrap_or_else(VerError::default) }
#[inline] pub fn clear_key(&mut self) { self.key.clear(); }
#[inline] pub fn set_key(&mut self, v: :: prost :: alloc :: vec :: Vec < u8 >) { self.key = v; }
#[inline] pub fn get_key(&self) -> &[u8] { &self.key }
#[inline] pub fn mut_key(&mut self) -> &mut :: prost :: alloc :: vec :: Vec < u8 > { &mut self.key }
#[inline] pub fn take_key(&mut self) -> :: prost :: alloc :: vec :: Vec < u8 > { ::std::mem::replace(&mut self.key, Default::default()) }
#[inline] pub fn has_value(&self) -> bool { self.value.is_some() }
#[inline] pub fn clear_value(&mut self) { self.value = ::std::option::Option::None }
#[inline] pub fn set_value(&mut self, v: VerValue) { self.value = ::std::option::Option::Some(v); }
#[inline] pub fn get_value(&self) -> &VerValue { match self.value.as_ref() {
                            Some(v) => v,
                            None => VerValue::default_ref(),
                        } }
#[inline] pub fn mut_value(&mut self) -> &mut VerValue { if self.value.is_none() {
                                self.value = ::std::option::Option::Some(VerValue::default());
                            }
                            self.value.as_mut().unwrap() } 
#[inline] pub fn take_value(&mut self) -> VerValue { self.value.take().unwrap_or_else(VerValue::default) }
}
impl ::protobuf::Clear for VerKvPair {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for VerKvPair {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static VerKvPair {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: VerKvPair = VerKvPair::default();
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
impl VerGetRequest {
pub fn new_() -> VerGetRequest { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn has_context(&self) -> bool { self.context.is_some() }
#[inline] pub fn clear_context(&mut self) { self.context = ::std::option::Option::None }
#[inline] pub fn set_context(&mut self, v: Context) { self.context = ::std::option::Option::Some(v); }
#[inline] pub fn get_context(&self) -> &Context { match self.context.as_ref() {
                            Some(v) => v,
                            None => Context::default_ref(),
                        } }
#[inline] pub fn mut_context(&mut self) -> &mut Context { if self.context.is_none() {
                                self.context = ::std::option::Option::Some(Context::default());
                            }
                            self.context.as_mut().unwrap() } 
#[inline] pub fn take_context(&mut self) -> Context { self.context.take().unwrap_or_else(Context::default) }
#[inline] pub fn clear_key(&mut self) { self.key.clear(); }
#[inline] pub fn set_key(&mut self, v: :: prost :: alloc :: vec :: Vec < u8 >) { self.key = v; }
#[inline] pub fn get_key(&self) -> &[u8] { &self.key }
#[inline] pub fn mut_key(&mut self) -> &mut :: prost :: alloc :: vec :: Vec < u8 > { &mut self.key }
#[inline] pub fn take_key(&mut self) -> :: prost :: alloc :: vec :: Vec < u8 > { ::std::mem::replace(&mut self.key, Default::default()) }
#[inline] pub fn clear_start_version(&mut self) { self.start_version = 0 }
#[inline] pub fn set_start_version(&mut self, v: u64) { self.start_version = v; }
#[inline] pub fn get_start_version(&self) -> u64 { self.start_version }
}
impl ::protobuf::Clear for VerGetRequest {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for VerGetRequest {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static VerGetRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: VerGetRequest = VerGetRequest::default();
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
impl VerGetResponse {
pub fn new_() -> VerGetResponse { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
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
#[inline] pub fn has_error(&self) -> bool { self.error.is_some() }
#[inline] pub fn clear_error(&mut self) { self.error = ::std::option::Option::None }
#[inline] pub fn set_error(&mut self, v: VerError) { self.error = ::std::option::Option::Some(v); }
#[inline] pub fn get_error(&self) -> &VerError { match self.error.as_ref() {
                            Some(v) => v,
                            None => VerError::default_ref(),
                        } }
#[inline] pub fn mut_error(&mut self) -> &mut VerError { if self.error.is_none() {
                                self.error = ::std::option::Option::Some(VerError::default());
                            }
                            self.error.as_mut().unwrap() } 
#[inline] pub fn take_error(&mut self) -> VerError { self.error.take().unwrap_or_else(VerError::default) }
#[inline] pub fn has_value(&self) -> bool { self.value.is_some() }
#[inline] pub fn clear_value(&mut self) { self.value = ::std::option::Option::None }
#[inline] pub fn set_value(&mut self, v: VerValue) { self.value = ::std::option::Option::Some(v); }
#[inline] pub fn get_value(&self) -> &VerValue { match self.value.as_ref() {
                            Some(v) => v,
                            None => VerValue::default_ref(),
                        } }
#[inline] pub fn mut_value(&mut self) -> &mut VerValue { if self.value.is_none() {
                                self.value = ::std::option::Option::Some(VerValue::default());
                            }
                            self.value.as_mut().unwrap() } 
#[inline] pub fn take_value(&mut self) -> VerValue { self.value.take().unwrap_or_else(VerValue::default) }
#[inline] pub fn clear_not_found(&mut self) { self.not_found = false }
#[inline] pub fn set_not_found(&mut self, v: bool) { self.not_found = v; }
#[inline] pub fn get_not_found(&self) -> bool { self.not_found }
}
impl ::protobuf::Clear for VerGetResponse {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for VerGetResponse {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static VerGetResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: VerGetResponse = VerGetResponse::default();
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
impl VerBatchGetRequest {
pub fn new_() -> VerBatchGetRequest { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn has_context(&self) -> bool { self.context.is_some() }
#[inline] pub fn clear_context(&mut self) { self.context = ::std::option::Option::None }
#[inline] pub fn set_context(&mut self, v: Context) { self.context = ::std::option::Option::Some(v); }
#[inline] pub fn get_context(&self) -> &Context { match self.context.as_ref() {
                            Some(v) => v,
                            None => Context::default_ref(),
                        } }
#[inline] pub fn mut_context(&mut self) -> &mut Context { if self.context.is_none() {
                                self.context = ::std::option::Option::Some(Context::default());
                            }
                            self.context.as_mut().unwrap() } 
#[inline] pub fn take_context(&mut self) -> Context { self.context.take().unwrap_or_else(Context::default) }
#[inline] pub fn clear_key(&mut self) { self.key.clear(); }
#[inline] pub fn set_key(&mut self, v: ::std::vec::Vec<:: prost :: alloc :: vec :: Vec < u8 >>) { self.key = v; }
#[inline] pub fn get_key(&self) -> &[:: prost :: alloc :: vec :: Vec < u8 >] { &self.key }
#[inline] pub fn mut_key(&mut self) -> &mut ::std::vec::Vec<:: prost :: alloc :: vec :: Vec < u8 >> { &mut self.key }
#[inline] pub fn take_key(&mut self) -> ::std::vec::Vec<:: prost :: alloc :: vec :: Vec < u8 >> { ::std::mem::replace(&mut self.key, ::std::vec::Vec::new()) }
#[inline] pub fn clear_start_version(&mut self) { self.start_version = 0 }
#[inline] pub fn set_start_version(&mut self, v: u64) { self.start_version = v; }
#[inline] pub fn get_start_version(&self) -> u64 { self.start_version }
}
impl ::protobuf::Clear for VerBatchGetRequest {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for VerBatchGetRequest {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static VerBatchGetRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: VerBatchGetRequest = VerBatchGetRequest::default();
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
impl VerBatchGetResponse {
pub fn new_() -> VerBatchGetResponse { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
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
#[inline] pub fn clear_pairs(&mut self) { self.pairs.clear(); }
#[inline] pub fn set_pairs(&mut self, v: ::std::vec::Vec<VerKvPair>) { self.pairs = v; }
#[inline] pub fn get_pairs(&self) -> &[VerKvPair] { &self.pairs }
#[inline] pub fn mut_pairs(&mut self) -> &mut ::std::vec::Vec<VerKvPair> { &mut self.pairs }
#[inline] pub fn take_pairs(&mut self) -> ::std::vec::Vec<VerKvPair> { ::std::mem::replace(&mut self.pairs, ::std::vec::Vec::new()) }
}
impl ::protobuf::Clear for VerBatchGetResponse {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for VerBatchGetResponse {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static VerBatchGetResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: VerBatchGetResponse = VerBatchGetResponse::default();
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
impl VerMutRequest {
pub fn new_() -> VerMutRequest { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn has_context(&self) -> bool { self.context.is_some() }
#[inline] pub fn clear_context(&mut self) { self.context = ::std::option::Option::None }
#[inline] pub fn set_context(&mut self, v: Context) { self.context = ::std::option::Option::Some(v); }
#[inline] pub fn get_context(&self) -> &Context { match self.context.as_ref() {
                            Some(v) => v,
                            None => Context::default_ref(),
                        } }
#[inline] pub fn mut_context(&mut self) -> &mut Context { if self.context.is_none() {
                                self.context = ::std::option::Option::Some(Context::default());
                            }
                            self.context.as_mut().unwrap() } 
#[inline] pub fn take_context(&mut self) -> Context { self.context.take().unwrap_or_else(Context::default) }
#[inline] pub fn has_mut(&self) -> bool { self.r#mut.is_some() }
#[inline] pub fn clear_mut(&mut self) { self.r#mut = ::std::option::Option::None }
#[inline] pub fn set_mut(&mut self, v: VerMutation) { self.r#mut = ::std::option::Option::Some(v); }
#[inline] pub fn get_mut(&self) -> &VerMutation { match self.r#mut.as_ref() {
                            Some(v) => v,
                            None => VerMutation::default_ref(),
                        } }
#[inline] pub fn mut_mut(&mut self) -> &mut VerMutation { if self.r#mut.is_none() {
                                self.r#mut = ::std::option::Option::Some(VerMutation::default());
                            }
                            self.r#mut.as_mut().unwrap() } 
#[inline] pub fn take_mut(&mut self) -> VerMutation { self.r#mut.take().unwrap_or_else(VerMutation::default) }
#[inline] pub fn clear_version(&mut self) { self.version = 0 }
#[inline] pub fn set_version(&mut self, v: u64) { self.version = v; }
#[inline] pub fn get_version(&self) -> u64 { self.version }
}
impl ::protobuf::Clear for VerMutRequest {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for VerMutRequest {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static VerMutRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: VerMutRequest = VerMutRequest::default();
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
impl VerMutResponse {
pub fn new_() -> VerMutResponse { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
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
#[inline] pub fn has_error(&self) -> bool { self.error.is_some() }
#[inline] pub fn clear_error(&mut self) { self.error = ::std::option::Option::None }
#[inline] pub fn set_error(&mut self, v: VerError) { self.error = ::std::option::Option::Some(v); }
#[inline] pub fn get_error(&self) -> &VerError { match self.error.as_ref() {
                            Some(v) => v,
                            None => VerError::default_ref(),
                        } }
#[inline] pub fn mut_error(&mut self) -> &mut VerError { if self.error.is_none() {
                                self.error = ::std::option::Option::Some(VerError::default());
                            }
                            self.error.as_mut().unwrap() } 
#[inline] pub fn take_error(&mut self) -> VerError { self.error.take().unwrap_or_else(VerError::default) }
}
impl ::protobuf::Clear for VerMutResponse {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for VerMutResponse {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static VerMutResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: VerMutResponse = VerMutResponse::default();
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
impl VerBatchMutRequest {
pub fn new_() -> VerBatchMutRequest { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn has_context(&self) -> bool { self.context.is_some() }
#[inline] pub fn clear_context(&mut self) { self.context = ::std::option::Option::None }
#[inline] pub fn set_context(&mut self, v: Context) { self.context = ::std::option::Option::Some(v); }
#[inline] pub fn get_context(&self) -> &Context { match self.context.as_ref() {
                            Some(v) => v,
                            None => Context::default_ref(),
                        } }
#[inline] pub fn mut_context(&mut self) -> &mut Context { if self.context.is_none() {
                                self.context = ::std::option::Option::Some(Context::default());
                            }
                            self.context.as_mut().unwrap() } 
#[inline] pub fn take_context(&mut self) -> Context { self.context.take().unwrap_or_else(Context::default) }
#[inline] pub fn clear_muts(&mut self) { self.muts.clear(); }
#[inline] pub fn set_muts(&mut self, v: ::std::vec::Vec<VerMutation>) { self.muts = v; }
#[inline] pub fn get_muts(&self) -> &[VerMutation] { &self.muts }
#[inline] pub fn mut_muts(&mut self) -> &mut ::std::vec::Vec<VerMutation> { &mut self.muts }
#[inline] pub fn take_muts(&mut self) -> ::std::vec::Vec<VerMutation> { ::std::mem::replace(&mut self.muts, ::std::vec::Vec::new()) }
#[inline] pub fn clear_version(&mut self) { self.version = 0 }
#[inline] pub fn set_version(&mut self, v: u64) { self.version = v; }
#[inline] pub fn get_version(&self) -> u64 { self.version }
}
impl ::protobuf::Clear for VerBatchMutRequest {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for VerBatchMutRequest {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static VerBatchMutRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: VerBatchMutRequest = VerBatchMutRequest::default();
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
impl VerBatchMutResponse {
pub fn new_() -> VerBatchMutResponse { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
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
#[inline] pub fn has_error(&self) -> bool { self.error.is_some() }
#[inline] pub fn clear_error(&mut self) { self.error = ::std::option::Option::None }
#[inline] pub fn set_error(&mut self, v: VerError) { self.error = ::std::option::Option::Some(v); }
#[inline] pub fn get_error(&self) -> &VerError { match self.error.as_ref() {
                            Some(v) => v,
                            None => VerError::default_ref(),
                        } }
#[inline] pub fn mut_error(&mut self) -> &mut VerError { if self.error.is_none() {
                                self.error = ::std::option::Option::Some(VerError::default());
                            }
                            self.error.as_mut().unwrap() } 
#[inline] pub fn take_error(&mut self) -> VerError { self.error.take().unwrap_or_else(VerError::default) }
}
impl ::protobuf::Clear for VerBatchMutResponse {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for VerBatchMutResponse {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static VerBatchMutResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: VerBatchMutResponse = VerBatchMutResponse::default();
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
impl VerScanRequest {
pub fn new_() -> VerScanRequest { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn has_context(&self) -> bool { self.context.is_some() }
#[inline] pub fn clear_context(&mut self) { self.context = ::std::option::Option::None }
#[inline] pub fn set_context(&mut self, v: Context) { self.context = ::std::option::Option::Some(v); }
#[inline] pub fn get_context(&self) -> &Context { match self.context.as_ref() {
                            Some(v) => v,
                            None => Context::default_ref(),
                        } }
#[inline] pub fn mut_context(&mut self) -> &mut Context { if self.context.is_none() {
                                self.context = ::std::option::Option::Some(Context::default());
                            }
                            self.context.as_mut().unwrap() } 
#[inline] pub fn take_context(&mut self) -> Context { self.context.take().unwrap_or_else(Context::default) }
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
#[inline] pub fn clear_limit(&mut self) { self.limit = 0 }
#[inline] pub fn set_limit(&mut self, v: u32) { self.limit = v; }
#[inline] pub fn get_limit(&self) -> u32 { self.limit }
#[inline] pub fn clear_key_only(&mut self) { self.key_only = false }
#[inline] pub fn set_key_only(&mut self, v: bool) { self.key_only = v; }
#[inline] pub fn get_key_only(&self) -> bool { self.key_only }
#[inline] pub fn clear_reverse(&mut self) { self.reverse = false }
#[inline] pub fn set_reverse(&mut self, v: bool) { self.reverse = v; }
#[inline] pub fn get_reverse(&self) -> bool { self.reverse }
#[inline] pub fn clear_start_version(&mut self) { self.start_version = 0 }
#[inline] pub fn set_start_version(&mut self, v: u64) { self.start_version = v; }
#[inline] pub fn get_start_version(&self) -> u64 { self.start_version }
}
impl ::protobuf::Clear for VerScanRequest {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for VerScanRequest {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static VerScanRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: VerScanRequest = VerScanRequest::default();
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
impl VerScanResponse {
pub fn new_() -> VerScanResponse { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
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
#[inline] pub fn clear_pairs(&mut self) { self.pairs.clear(); }
#[inline] pub fn set_pairs(&mut self, v: ::std::vec::Vec<VerKvPair>) { self.pairs = v; }
#[inline] pub fn get_pairs(&self) -> &[VerKvPair] { &self.pairs }
#[inline] pub fn mut_pairs(&mut self) -> &mut ::std::vec::Vec<VerKvPair> { &mut self.pairs }
#[inline] pub fn take_pairs(&mut self) -> ::std::vec::Vec<VerKvPair> { ::std::mem::replace(&mut self.pairs, ::std::vec::Vec::new()) }
}
impl ::protobuf::Clear for VerScanResponse {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for VerScanResponse {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static VerScanResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: VerScanResponse = VerScanResponse::default();
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
impl VerDeleteRangeRequest {
pub fn new_() -> VerDeleteRangeRequest { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn has_context(&self) -> bool { self.context.is_some() }
#[inline] pub fn clear_context(&mut self) { self.context = ::std::option::Option::None }
#[inline] pub fn set_context(&mut self, v: Context) { self.context = ::std::option::Option::Some(v); }
#[inline] pub fn get_context(&self) -> &Context { match self.context.as_ref() {
                            Some(v) => v,
                            None => Context::default_ref(),
                        } }
#[inline] pub fn mut_context(&mut self) -> &mut Context { if self.context.is_none() {
                                self.context = ::std::option::Option::Some(Context::default());
                            }
                            self.context.as_mut().unwrap() } 
#[inline] pub fn take_context(&mut self) -> Context { self.context.take().unwrap_or_else(Context::default) }
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
}
impl ::protobuf::Clear for VerDeleteRangeRequest {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for VerDeleteRangeRequest {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static VerDeleteRangeRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: VerDeleteRangeRequest = VerDeleteRangeRequest::default();
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
impl VerDeleteRangeResponse {
pub fn new_() -> VerDeleteRangeResponse { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
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
#[inline] pub fn has_error(&self) -> bool { self.error.is_some() }
#[inline] pub fn clear_error(&mut self) { self.error = ::std::option::Option::None }
#[inline] pub fn set_error(&mut self, v: VerError) { self.error = ::std::option::Option::Some(v); }
#[inline] pub fn get_error(&self) -> &VerError { match self.error.as_ref() {
                            Some(v) => v,
                            None => VerError::default_ref(),
                        } }
#[inline] pub fn mut_error(&mut self) -> &mut VerError { if self.error.is_none() {
                                self.error = ::std::option::Option::Some(VerError::default());
                            }
                            self.error.as_mut().unwrap() } 
#[inline] pub fn take_error(&mut self) -> VerError { self.error.take().unwrap_or_else(VerError::default) }
}
impl ::protobuf::Clear for VerDeleteRangeResponse {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for VerDeleteRangeResponse {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static VerDeleteRangeResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: VerDeleteRangeResponse = VerDeleteRangeResponse::default();
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
impl MvccGetByKeyRequest {
pub fn new_() -> MvccGetByKeyRequest { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn has_context(&self) -> bool { self.context.is_some() }
#[inline] pub fn clear_context(&mut self) { self.context = ::std::option::Option::None }
#[inline] pub fn set_context(&mut self, v: Context) { self.context = ::std::option::Option::Some(v); }
#[inline] pub fn get_context(&self) -> &Context { match self.context.as_ref() {
                            Some(v) => v,
                            None => Context::default_ref(),
                        } }
#[inline] pub fn mut_context(&mut self) -> &mut Context { if self.context.is_none() {
                                self.context = ::std::option::Option::Some(Context::default());
                            }
                            self.context.as_mut().unwrap() } 
#[inline] pub fn take_context(&mut self) -> Context { self.context.take().unwrap_or_else(Context::default) }
#[inline] pub fn clear_key(&mut self) { self.key.clear(); }
#[inline] pub fn set_key(&mut self, v: :: prost :: alloc :: vec :: Vec < u8 >) { self.key = v; }
#[inline] pub fn get_key(&self) -> &[u8] { &self.key }
#[inline] pub fn mut_key(&mut self) -> &mut :: prost :: alloc :: vec :: Vec < u8 > { &mut self.key }
#[inline] pub fn take_key(&mut self) -> :: prost :: alloc :: vec :: Vec < u8 > { ::std::mem::replace(&mut self.key, Default::default()) }
}
impl ::protobuf::Clear for MvccGetByKeyRequest {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for MvccGetByKeyRequest {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static MvccGetByKeyRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: MvccGetByKeyRequest = MvccGetByKeyRequest::default();
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
impl MvccGetByKeyResponse {
pub fn new_() -> MvccGetByKeyResponse { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
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
#[inline] pub fn clear_error(&mut self) { self.error.clear(); }
#[inline] pub fn set_error(&mut self, v: :: prost :: alloc :: string :: String) { self.error = v; }
#[inline] pub fn get_error(&self) -> &str { &self.error }
#[inline] pub fn mut_error(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.error }
#[inline] pub fn take_error(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.error, Default::default()) }
#[inline] pub fn has_info(&self) -> bool { self.info.is_some() }
#[inline] pub fn clear_info(&mut self) { self.info = ::std::option::Option::None }
#[inline] pub fn set_info(&mut self, v: MvccInfo) { self.info = ::std::option::Option::Some(v); }
#[inline] pub fn get_info(&self) -> &MvccInfo { match self.info.as_ref() {
                            Some(v) => v,
                            None => MvccInfo::default_ref(),
                        } }
#[inline] pub fn mut_info(&mut self) -> &mut MvccInfo { if self.info.is_none() {
                                self.info = ::std::option::Option::Some(MvccInfo::default());
                            }
                            self.info.as_mut().unwrap() } 
#[inline] pub fn take_info(&mut self) -> MvccInfo { self.info.take().unwrap_or_else(MvccInfo::default) }
}
impl ::protobuf::Clear for MvccGetByKeyResponse {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for MvccGetByKeyResponse {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static MvccGetByKeyResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: MvccGetByKeyResponse = MvccGetByKeyResponse::default();
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
impl MvccGetByStartTsRequest {
pub fn new_() -> MvccGetByStartTsRequest { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn has_context(&self) -> bool { self.context.is_some() }
#[inline] pub fn clear_context(&mut self) { self.context = ::std::option::Option::None }
#[inline] pub fn set_context(&mut self, v: Context) { self.context = ::std::option::Option::Some(v); }
#[inline] pub fn get_context(&self) -> &Context { match self.context.as_ref() {
                            Some(v) => v,
                            None => Context::default_ref(),
                        } }
#[inline] pub fn mut_context(&mut self) -> &mut Context { if self.context.is_none() {
                                self.context = ::std::option::Option::Some(Context::default());
                            }
                            self.context.as_mut().unwrap() } 
#[inline] pub fn take_context(&mut self) -> Context { self.context.take().unwrap_or_else(Context::default) }
#[inline] pub fn clear_start_ts(&mut self) { self.start_ts = 0 }
#[inline] pub fn set_start_ts(&mut self, v: u64) { self.start_ts = v; }
#[inline] pub fn get_start_ts(&self) -> u64 { self.start_ts }
}
impl ::protobuf::Clear for MvccGetByStartTsRequest {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for MvccGetByStartTsRequest {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static MvccGetByStartTsRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: MvccGetByStartTsRequest = MvccGetByStartTsRequest::default();
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
impl MvccGetByStartTsResponse {
pub fn new_() -> MvccGetByStartTsResponse { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
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
#[inline] pub fn clear_error(&mut self) { self.error.clear(); }
#[inline] pub fn set_error(&mut self, v: :: prost :: alloc :: string :: String) { self.error = v; }
#[inline] pub fn get_error(&self) -> &str { &self.error }
#[inline] pub fn mut_error(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.error }
#[inline] pub fn take_error(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.error, Default::default()) }
#[inline] pub fn clear_key(&mut self) { self.key.clear(); }
#[inline] pub fn set_key(&mut self, v: :: prost :: alloc :: vec :: Vec < u8 >) { self.key = v; }
#[inline] pub fn get_key(&self) -> &[u8] { &self.key }
#[inline] pub fn mut_key(&mut self) -> &mut :: prost :: alloc :: vec :: Vec < u8 > { &mut self.key }
#[inline] pub fn take_key(&mut self) -> :: prost :: alloc :: vec :: Vec < u8 > { ::std::mem::replace(&mut self.key, Default::default()) }
#[inline] pub fn has_info(&self) -> bool { self.info.is_some() }
#[inline] pub fn clear_info(&mut self) { self.info = ::std::option::Option::None }
#[inline] pub fn set_info(&mut self, v: MvccInfo) { self.info = ::std::option::Option::Some(v); }
#[inline] pub fn get_info(&self) -> &MvccInfo { match self.info.as_ref() {
                            Some(v) => v,
                            None => MvccInfo::default_ref(),
                        } }
#[inline] pub fn mut_info(&mut self) -> &mut MvccInfo { if self.info.is_none() {
                                self.info = ::std::option::Option::Some(MvccInfo::default());
                            }
                            self.info.as_mut().unwrap() } 
#[inline] pub fn take_info(&mut self) -> MvccInfo { self.info.take().unwrap_or_else(MvccInfo::default) }
}
impl ::protobuf::Clear for MvccGetByStartTsResponse {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for MvccGetByStartTsResponse {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static MvccGetByStartTsResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: MvccGetByStartTsResponse = MvccGetByStartTsResponse::default();
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
impl Context {
pub fn new_() -> Context { ::std::default::Default::default() }
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
#[inline] pub fn has_peer(&self) -> bool { self.peer.is_some() }
#[inline] pub fn clear_peer(&mut self) { self.peer = ::std::option::Option::None }
#[inline] pub fn set_peer(&mut self, v: super :: metapb :: Peer) { self.peer = ::std::option::Option::Some(v); }
#[inline] pub fn get_peer(&self) -> &super :: metapb :: Peer { match self.peer.as_ref() {
                            Some(v) => v,
                            None => super :: metapb :: Peer::default_ref(),
                        } }
#[inline] pub fn mut_peer(&mut self) -> &mut super :: metapb :: Peer { if self.peer.is_none() {
                                self.peer = ::std::option::Option::Some(super :: metapb :: Peer::default());
                            }
                            self.peer.as_mut().unwrap() } 
#[inline] pub fn take_peer(&mut self) -> super :: metapb :: Peer { self.peer.take().unwrap_or_else(super :: metapb :: Peer::default) }
#[inline] pub fn clear_term(&mut self) { self.term = 0 }
#[inline] pub fn set_term(&mut self, v: u64) { self.term = v; }
#[inline] pub fn get_term(&self) -> u64 { self.term }
#[inline] pub fn clear_priority(&mut self) { self.priority = 0 }
#[inline] pub fn get_priority(&self) -> CommandPri { match CommandPri::from_i32(self.priority) {Some(e) => e,
                        None => panic!("Unknown enum variant: {}", self.priority),
                    } }
#[inline] pub fn clear_isolation_level(&mut self) { self.isolation_level = 0 }
#[inline] pub fn get_isolation_level(&self) -> IsolationLevel { match IsolationLevel::from_i32(self.isolation_level) {Some(e) => e,
                        None => panic!("Unknown enum variant: {}", self.isolation_level),
                    } }
#[inline] pub fn clear_not_fill_cache(&mut self) { self.not_fill_cache = false }
#[inline] pub fn set_not_fill_cache(&mut self, v: bool) { self.not_fill_cache = v; }
#[inline] pub fn get_not_fill_cache(&self) -> bool { self.not_fill_cache }
#[inline] pub fn clear_sync_log(&mut self) { self.sync_log = false }
#[inline] pub fn set_sync_log(&mut self, v: bool) { self.sync_log = v; }
#[inline] pub fn get_sync_log(&self) -> bool { self.sync_log }
#[inline] pub fn clear_record_time_stat(&mut self) { self.record_time_stat = false }
#[inline] pub fn set_record_time_stat(&mut self, v: bool) { self.record_time_stat = v; }
#[inline] pub fn get_record_time_stat(&self) -> bool { self.record_time_stat }
#[inline] pub fn clear_record_scan_stat(&mut self) { self.record_scan_stat = false }
#[inline] pub fn set_record_scan_stat(&mut self, v: bool) { self.record_scan_stat = v; }
#[inline] pub fn get_record_scan_stat(&self) -> bool { self.record_scan_stat }
#[inline] pub fn clear_replica_read(&mut self) { self.replica_read = false }
#[inline] pub fn set_replica_read(&mut self, v: bool) { self.replica_read = v; }
#[inline] pub fn get_replica_read(&self) -> bool { self.replica_read }
#[inline] pub fn clear_resolved_locks(&mut self) { self.resolved_locks.clear(); }
#[inline] pub fn set_resolved_locks(&mut self, v: ::std::vec::Vec<u64>) { self.resolved_locks = v; }
#[inline] pub fn get_resolved_locks(&self) -> &[u64] { &self.resolved_locks }
#[inline] pub fn mut_resolved_locks(&mut self) -> &mut ::std::vec::Vec<u64> { &mut self.resolved_locks }
#[inline] pub fn take_resolved_locks(&mut self) -> ::std::vec::Vec<u64> { ::std::mem::replace(&mut self.resolved_locks, ::std::vec::Vec::new()) }
#[inline] pub fn clear_max_execution_duration_ms(&mut self) { self.max_execution_duration_ms = 0 }
#[inline] pub fn set_max_execution_duration_ms(&mut self, v: u64) { self.max_execution_duration_ms = v; }
#[inline] pub fn get_max_execution_duration_ms(&self) -> u64 { self.max_execution_duration_ms }
#[inline] pub fn clear_applied_index(&mut self) { self.applied_index = 0 }
#[inline] pub fn set_applied_index(&mut self, v: u64) { self.applied_index = v; }
#[inline] pub fn get_applied_index(&self) -> u64 { self.applied_index }
#[inline] pub fn clear_task_id(&mut self) { self.task_id = 0 }
#[inline] pub fn set_task_id(&mut self, v: u64) { self.task_id = v; }
#[inline] pub fn get_task_id(&self) -> u64 { self.task_id }
#[inline] pub fn clear_stale_read(&mut self) { self.stale_read = false }
#[inline] pub fn set_stale_read(&mut self, v: bool) { self.stale_read = v; }
#[inline] pub fn get_stale_read(&self) -> bool { self.stale_read }
}
impl ::protobuf::Clear for Context {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for Context {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static Context {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: Context = Context::default();
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
impl LockInfo {
pub fn new_() -> LockInfo { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn clear_primary_lock(&mut self) { self.primary_lock.clear(); }
#[inline] pub fn set_primary_lock(&mut self, v: :: prost :: alloc :: vec :: Vec < u8 >) { self.primary_lock = v; }
#[inline] pub fn get_primary_lock(&self) -> &[u8] { &self.primary_lock }
#[inline] pub fn mut_primary_lock(&mut self) -> &mut :: prost :: alloc :: vec :: Vec < u8 > { &mut self.primary_lock }
#[inline] pub fn take_primary_lock(&mut self) -> :: prost :: alloc :: vec :: Vec < u8 > { ::std::mem::replace(&mut self.primary_lock, Default::default()) }
#[inline] pub fn clear_lock_version(&mut self) { self.lock_version = 0 }
#[inline] pub fn set_lock_version(&mut self, v: u64) { self.lock_version = v; }
#[inline] pub fn get_lock_version(&self) -> u64 { self.lock_version }
#[inline] pub fn clear_key(&mut self) { self.key.clear(); }
#[inline] pub fn set_key(&mut self, v: :: prost :: alloc :: vec :: Vec < u8 >) { self.key = v; }
#[inline] pub fn get_key(&self) -> &[u8] { &self.key }
#[inline] pub fn mut_key(&mut self) -> &mut :: prost :: alloc :: vec :: Vec < u8 > { &mut self.key }
#[inline] pub fn take_key(&mut self) -> :: prost :: alloc :: vec :: Vec < u8 > { ::std::mem::replace(&mut self.key, Default::default()) }
#[inline] pub fn clear_lock_ttl(&mut self) { self.lock_ttl = 0 }
#[inline] pub fn set_lock_ttl(&mut self, v: u64) { self.lock_ttl = v; }
#[inline] pub fn get_lock_ttl(&self) -> u64 { self.lock_ttl }
#[inline] pub fn clear_txn_size(&mut self) { self.txn_size = 0 }
#[inline] pub fn set_txn_size(&mut self, v: u64) { self.txn_size = v; }
#[inline] pub fn get_txn_size(&self) -> u64 { self.txn_size }
#[inline] pub fn clear_lock_type(&mut self) { self.lock_type = 0 }
#[inline] pub fn get_lock_type(&self) -> Op { match Op::from_i32(self.lock_type) {Some(e) => e,
                        None => panic!("Unknown enum variant: {}", self.lock_type),
                    } }
#[inline] pub fn clear_lock_for_update_ts(&mut self) { self.lock_for_update_ts = 0 }
#[inline] pub fn set_lock_for_update_ts(&mut self, v: u64) { self.lock_for_update_ts = v; }
#[inline] pub fn get_lock_for_update_ts(&self) -> u64 { self.lock_for_update_ts }
#[inline] pub fn clear_use_async_commit(&mut self) { self.use_async_commit = false }
#[inline] pub fn set_use_async_commit(&mut self, v: bool) { self.use_async_commit = v; }
#[inline] pub fn get_use_async_commit(&self) -> bool { self.use_async_commit }
#[inline] pub fn clear_min_commit_ts(&mut self) { self.min_commit_ts = 0 }
#[inline] pub fn set_min_commit_ts(&mut self, v: u64) { self.min_commit_ts = v; }
#[inline] pub fn get_min_commit_ts(&self) -> u64 { self.min_commit_ts }
#[inline] pub fn clear_secondaries(&mut self) { self.secondaries.clear(); }
#[inline] pub fn set_secondaries(&mut self, v: ::std::vec::Vec<:: prost :: alloc :: vec :: Vec < u8 >>) { self.secondaries = v; }
#[inline] pub fn get_secondaries(&self) -> &[:: prost :: alloc :: vec :: Vec < u8 >] { &self.secondaries }
#[inline] pub fn mut_secondaries(&mut self) -> &mut ::std::vec::Vec<:: prost :: alloc :: vec :: Vec < u8 >> { &mut self.secondaries }
#[inline] pub fn take_secondaries(&mut self) -> ::std::vec::Vec<:: prost :: alloc :: vec :: Vec < u8 >> { ::std::mem::replace(&mut self.secondaries, ::std::vec::Vec::new()) }
}
impl ::protobuf::Clear for LockInfo {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for LockInfo {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static LockInfo {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: LockInfo = LockInfo::default();
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
impl KeyError {
pub fn new_() -> KeyError { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn has_locked(&self) -> bool { self.locked.is_some() }
#[inline] pub fn clear_locked(&mut self) { self.locked = ::std::option::Option::None }
#[inline] pub fn set_locked(&mut self, v: LockInfo) { self.locked = ::std::option::Option::Some(v); }
#[inline] pub fn get_locked(&self) -> &LockInfo { match self.locked.as_ref() {
                            Some(v) => v,
                            None => LockInfo::default_ref(),
                        } }
#[inline] pub fn mut_locked(&mut self) -> &mut LockInfo { if self.locked.is_none() {
                                self.locked = ::std::option::Option::Some(LockInfo::default());
                            }
                            self.locked.as_mut().unwrap() } 
#[inline] pub fn take_locked(&mut self) -> LockInfo { self.locked.take().unwrap_or_else(LockInfo::default) }
#[inline] pub fn clear_retryable(&mut self) { self.retryable.clear(); }
#[inline] pub fn set_retryable(&mut self, v: :: prost :: alloc :: string :: String) { self.retryable = v; }
#[inline] pub fn get_retryable(&self) -> &str { &self.retryable }
#[inline] pub fn mut_retryable(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.retryable }
#[inline] pub fn take_retryable(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.retryable, Default::default()) }
#[inline] pub fn clear_abort(&mut self) { self.abort.clear(); }
#[inline] pub fn set_abort(&mut self, v: :: prost :: alloc :: string :: String) { self.abort = v; }
#[inline] pub fn get_abort(&self) -> &str { &self.abort }
#[inline] pub fn mut_abort(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.abort }
#[inline] pub fn take_abort(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.abort, Default::default()) }
#[inline] pub fn has_conflict(&self) -> bool { self.conflict.is_some() }
#[inline] pub fn clear_conflict(&mut self) { self.conflict = ::std::option::Option::None }
#[inline] pub fn set_conflict(&mut self, v: WriteConflict) { self.conflict = ::std::option::Option::Some(v); }
#[inline] pub fn get_conflict(&self) -> &WriteConflict { match self.conflict.as_ref() {
                            Some(v) => v,
                            None => WriteConflict::default_ref(),
                        } }
#[inline] pub fn mut_conflict(&mut self) -> &mut WriteConflict { if self.conflict.is_none() {
                                self.conflict = ::std::option::Option::Some(WriteConflict::default());
                            }
                            self.conflict.as_mut().unwrap() } 
#[inline] pub fn take_conflict(&mut self) -> WriteConflict { self.conflict.take().unwrap_or_else(WriteConflict::default) }
#[inline] pub fn has_already_exist(&self) -> bool { self.already_exist.is_some() }
#[inline] pub fn clear_already_exist(&mut self) { self.already_exist = ::std::option::Option::None }
#[inline] pub fn set_already_exist(&mut self, v: AlreadyExist) { self.already_exist = ::std::option::Option::Some(v); }
#[inline] pub fn get_already_exist(&self) -> &AlreadyExist { match self.already_exist.as_ref() {
                            Some(v) => v,
                            None => AlreadyExist::default_ref(),
                        } }
#[inline] pub fn mut_already_exist(&mut self) -> &mut AlreadyExist { if self.already_exist.is_none() {
                                self.already_exist = ::std::option::Option::Some(AlreadyExist::default());
                            }
                            self.already_exist.as_mut().unwrap() } 
#[inline] pub fn take_already_exist(&mut self) -> AlreadyExist { self.already_exist.take().unwrap_or_else(AlreadyExist::default) }
#[inline] pub fn has_deadlock(&self) -> bool { self.deadlock.is_some() }
#[inline] pub fn clear_deadlock(&mut self) { self.deadlock = ::std::option::Option::None }
#[inline] pub fn set_deadlock(&mut self, v: Deadlock) { self.deadlock = ::std::option::Option::Some(v); }
#[inline] pub fn get_deadlock(&self) -> &Deadlock { match self.deadlock.as_ref() {
                            Some(v) => v,
                            None => Deadlock::default_ref(),
                        } }
#[inline] pub fn mut_deadlock(&mut self) -> &mut Deadlock { if self.deadlock.is_none() {
                                self.deadlock = ::std::option::Option::Some(Deadlock::default());
                            }
                            self.deadlock.as_mut().unwrap() } 
#[inline] pub fn take_deadlock(&mut self) -> Deadlock { self.deadlock.take().unwrap_or_else(Deadlock::default) }
#[inline] pub fn has_commit_ts_expired(&self) -> bool { self.commit_ts_expired.is_some() }
#[inline] pub fn clear_commit_ts_expired(&mut self) { self.commit_ts_expired = ::std::option::Option::None }
#[inline] pub fn set_commit_ts_expired(&mut self, v: CommitTsExpired) { self.commit_ts_expired = ::std::option::Option::Some(v); }
#[inline] pub fn get_commit_ts_expired(&self) -> &CommitTsExpired { match self.commit_ts_expired.as_ref() {
                            Some(v) => v,
                            None => CommitTsExpired::default_ref(),
                        } }
#[inline] pub fn mut_commit_ts_expired(&mut self) -> &mut CommitTsExpired { if self.commit_ts_expired.is_none() {
                                self.commit_ts_expired = ::std::option::Option::Some(CommitTsExpired::default());
                            }
                            self.commit_ts_expired.as_mut().unwrap() } 
#[inline] pub fn take_commit_ts_expired(&mut self) -> CommitTsExpired { self.commit_ts_expired.take().unwrap_or_else(CommitTsExpired::default) }
#[inline] pub fn has_txn_not_found(&self) -> bool { self.txn_not_found.is_some() }
#[inline] pub fn clear_txn_not_found(&mut self) { self.txn_not_found = ::std::option::Option::None }
#[inline] pub fn set_txn_not_found(&mut self, v: TxnNotFound) { self.txn_not_found = ::std::option::Option::Some(v); }
#[inline] pub fn get_txn_not_found(&self) -> &TxnNotFound { match self.txn_not_found.as_ref() {
                            Some(v) => v,
                            None => TxnNotFound::default_ref(),
                        } }
#[inline] pub fn mut_txn_not_found(&mut self) -> &mut TxnNotFound { if self.txn_not_found.is_none() {
                                self.txn_not_found = ::std::option::Option::Some(TxnNotFound::default());
                            }
                            self.txn_not_found.as_mut().unwrap() } 
#[inline] pub fn take_txn_not_found(&mut self) -> TxnNotFound { self.txn_not_found.take().unwrap_or_else(TxnNotFound::default) }
#[inline] pub fn has_commit_ts_too_large(&self) -> bool { self.commit_ts_too_large.is_some() }
#[inline] pub fn clear_commit_ts_too_large(&mut self) { self.commit_ts_too_large = ::std::option::Option::None }
#[inline] pub fn set_commit_ts_too_large(&mut self, v: CommitTsTooLarge) { self.commit_ts_too_large = ::std::option::Option::Some(v); }
#[inline] pub fn get_commit_ts_too_large(&self) -> &CommitTsTooLarge { match self.commit_ts_too_large.as_ref() {
                            Some(v) => v,
                            None => CommitTsTooLarge::default_ref(),
                        } }
#[inline] pub fn mut_commit_ts_too_large(&mut self) -> &mut CommitTsTooLarge { if self.commit_ts_too_large.is_none() {
                                self.commit_ts_too_large = ::std::option::Option::Some(CommitTsTooLarge::default());
                            }
                            self.commit_ts_too_large.as_mut().unwrap() } 
#[inline] pub fn take_commit_ts_too_large(&mut self) -> CommitTsTooLarge { self.commit_ts_too_large.take().unwrap_or_else(CommitTsTooLarge::default) }
}
impl ::protobuf::Clear for KeyError {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for KeyError {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static KeyError {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: KeyError = KeyError::default();
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
impl WriteConflict {
pub fn new_() -> WriteConflict { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn clear_start_ts(&mut self) { self.start_ts = 0 }
#[inline] pub fn set_start_ts(&mut self, v: u64) { self.start_ts = v; }
#[inline] pub fn get_start_ts(&self) -> u64 { self.start_ts }
#[inline] pub fn clear_conflict_ts(&mut self) { self.conflict_ts = 0 }
#[inline] pub fn set_conflict_ts(&mut self, v: u64) { self.conflict_ts = v; }
#[inline] pub fn get_conflict_ts(&self) -> u64 { self.conflict_ts }
#[inline] pub fn clear_key(&mut self) { self.key.clear(); }
#[inline] pub fn set_key(&mut self, v: :: prost :: alloc :: vec :: Vec < u8 >) { self.key = v; }
#[inline] pub fn get_key(&self) -> &[u8] { &self.key }
#[inline] pub fn mut_key(&mut self) -> &mut :: prost :: alloc :: vec :: Vec < u8 > { &mut self.key }
#[inline] pub fn take_key(&mut self) -> :: prost :: alloc :: vec :: Vec < u8 > { ::std::mem::replace(&mut self.key, Default::default()) }
#[inline] pub fn clear_primary(&mut self) { self.primary.clear(); }
#[inline] pub fn set_primary(&mut self, v: :: prost :: alloc :: vec :: Vec < u8 >) { self.primary = v; }
#[inline] pub fn get_primary(&self) -> &[u8] { &self.primary }
#[inline] pub fn mut_primary(&mut self) -> &mut :: prost :: alloc :: vec :: Vec < u8 > { &mut self.primary }
#[inline] pub fn take_primary(&mut self) -> :: prost :: alloc :: vec :: Vec < u8 > { ::std::mem::replace(&mut self.primary, Default::default()) }
#[inline] pub fn clear_conflict_commit_ts(&mut self) { self.conflict_commit_ts = 0 }
#[inline] pub fn set_conflict_commit_ts(&mut self, v: u64) { self.conflict_commit_ts = v; }
#[inline] pub fn get_conflict_commit_ts(&self) -> u64 { self.conflict_commit_ts }
}
impl ::protobuf::Clear for WriteConflict {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for WriteConflict {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static WriteConflict {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: WriteConflict = WriteConflict::default();
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
impl AlreadyExist {
pub fn new_() -> AlreadyExist { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn clear_key(&mut self) { self.key.clear(); }
#[inline] pub fn set_key(&mut self, v: :: prost :: alloc :: vec :: Vec < u8 >) { self.key = v; }
#[inline] pub fn get_key(&self) -> &[u8] { &self.key }
#[inline] pub fn mut_key(&mut self) -> &mut :: prost :: alloc :: vec :: Vec < u8 > { &mut self.key }
#[inline] pub fn take_key(&mut self) -> :: prost :: alloc :: vec :: Vec < u8 > { ::std::mem::replace(&mut self.key, Default::default()) }
}
impl ::protobuf::Clear for AlreadyExist {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for AlreadyExist {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static AlreadyExist {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: AlreadyExist = AlreadyExist::default();
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
impl Deadlock {
pub fn new_() -> Deadlock { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn clear_lock_ts(&mut self) { self.lock_ts = 0 }
#[inline] pub fn set_lock_ts(&mut self, v: u64) { self.lock_ts = v; }
#[inline] pub fn get_lock_ts(&self) -> u64 { self.lock_ts }
#[inline] pub fn clear_lock_key(&mut self) { self.lock_key.clear(); }
#[inline] pub fn set_lock_key(&mut self, v: :: prost :: alloc :: vec :: Vec < u8 >) { self.lock_key = v; }
#[inline] pub fn get_lock_key(&self) -> &[u8] { &self.lock_key }
#[inline] pub fn mut_lock_key(&mut self) -> &mut :: prost :: alloc :: vec :: Vec < u8 > { &mut self.lock_key }
#[inline] pub fn take_lock_key(&mut self) -> :: prost :: alloc :: vec :: Vec < u8 > { ::std::mem::replace(&mut self.lock_key, Default::default()) }
#[inline] pub fn clear_deadlock_key_hash(&mut self) { self.deadlock_key_hash = 0 }
#[inline] pub fn set_deadlock_key_hash(&mut self, v: u64) { self.deadlock_key_hash = v; }
#[inline] pub fn get_deadlock_key_hash(&self) -> u64 { self.deadlock_key_hash }
}
impl ::protobuf::Clear for Deadlock {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for Deadlock {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static Deadlock {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: Deadlock = Deadlock::default();
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
impl CommitTsExpired {
pub fn new_() -> CommitTsExpired { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn clear_start_ts(&mut self) { self.start_ts = 0 }
#[inline] pub fn set_start_ts(&mut self, v: u64) { self.start_ts = v; }
#[inline] pub fn get_start_ts(&self) -> u64 { self.start_ts }
#[inline] pub fn clear_attempted_commit_ts(&mut self) { self.attempted_commit_ts = 0 }
#[inline] pub fn set_attempted_commit_ts(&mut self, v: u64) { self.attempted_commit_ts = v; }
#[inline] pub fn get_attempted_commit_ts(&self) -> u64 { self.attempted_commit_ts }
#[inline] pub fn clear_key(&mut self) { self.key.clear(); }
#[inline] pub fn set_key(&mut self, v: :: prost :: alloc :: vec :: Vec < u8 >) { self.key = v; }
#[inline] pub fn get_key(&self) -> &[u8] { &self.key }
#[inline] pub fn mut_key(&mut self) -> &mut :: prost :: alloc :: vec :: Vec < u8 > { &mut self.key }
#[inline] pub fn take_key(&mut self) -> :: prost :: alloc :: vec :: Vec < u8 > { ::std::mem::replace(&mut self.key, Default::default()) }
#[inline] pub fn clear_min_commit_ts(&mut self) { self.min_commit_ts = 0 }
#[inline] pub fn set_min_commit_ts(&mut self, v: u64) { self.min_commit_ts = v; }
#[inline] pub fn get_min_commit_ts(&self) -> u64 { self.min_commit_ts }
}
impl ::protobuf::Clear for CommitTsExpired {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for CommitTsExpired {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static CommitTsExpired {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: CommitTsExpired = CommitTsExpired::default();
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
impl TxnNotFound {
pub fn new_() -> TxnNotFound { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn clear_start_ts(&mut self) { self.start_ts = 0 }
#[inline] pub fn set_start_ts(&mut self, v: u64) { self.start_ts = v; }
#[inline] pub fn get_start_ts(&self) -> u64 { self.start_ts }
#[inline] pub fn clear_primary_key(&mut self) { self.primary_key.clear(); }
#[inline] pub fn set_primary_key(&mut self, v: :: prost :: alloc :: vec :: Vec < u8 >) { self.primary_key = v; }
#[inline] pub fn get_primary_key(&self) -> &[u8] { &self.primary_key }
#[inline] pub fn mut_primary_key(&mut self) -> &mut :: prost :: alloc :: vec :: Vec < u8 > { &mut self.primary_key }
#[inline] pub fn take_primary_key(&mut self) -> :: prost :: alloc :: vec :: Vec < u8 > { ::std::mem::replace(&mut self.primary_key, Default::default()) }
}
impl ::protobuf::Clear for TxnNotFound {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for TxnNotFound {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static TxnNotFound {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: TxnNotFound = TxnNotFound::default();
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
impl CommitTsTooLarge {
pub fn new_() -> CommitTsTooLarge { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn clear_commit_ts(&mut self) { self.commit_ts = 0 }
#[inline] pub fn set_commit_ts(&mut self, v: u64) { self.commit_ts = v; }
#[inline] pub fn get_commit_ts(&self) -> u64 { self.commit_ts }
}
impl ::protobuf::Clear for CommitTsTooLarge {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for CommitTsTooLarge {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static CommitTsTooLarge {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: CommitTsTooLarge = CommitTsTooLarge::default();
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
impl TimeDetail {
pub fn new_() -> TimeDetail { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn clear_wait_wall_time_ms(&mut self) { self.wait_wall_time_ms = 0 }
#[inline] pub fn set_wait_wall_time_ms(&mut self, v: i64) { self.wait_wall_time_ms = v; }
#[inline] pub fn get_wait_wall_time_ms(&self) -> i64 { self.wait_wall_time_ms }
#[inline] pub fn clear_process_wall_time_ms(&mut self) { self.process_wall_time_ms = 0 }
#[inline] pub fn set_process_wall_time_ms(&mut self, v: i64) { self.process_wall_time_ms = v; }
#[inline] pub fn get_process_wall_time_ms(&self) -> i64 { self.process_wall_time_ms }
#[inline] pub fn clear_kv_read_wall_time_ms(&mut self) { self.kv_read_wall_time_ms = 0 }
#[inline] pub fn set_kv_read_wall_time_ms(&mut self, v: i64) { self.kv_read_wall_time_ms = v; }
#[inline] pub fn get_kv_read_wall_time_ms(&self) -> i64 { self.kv_read_wall_time_ms }
}
impl ::protobuf::Clear for TimeDetail {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for TimeDetail {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static TimeDetail {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: TimeDetail = TimeDetail::default();
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
impl ScanInfo {
pub fn new_() -> ScanInfo { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn clear_total(&mut self) { self.total = 0 }
#[inline] pub fn set_total(&mut self, v: i64) { self.total = v; }
#[inline] pub fn get_total(&self) -> i64 { self.total }
#[inline] pub fn clear_processed(&mut self) { self.processed = 0 }
#[inline] pub fn set_processed(&mut self, v: i64) { self.processed = v; }
#[inline] pub fn get_processed(&self) -> i64 { self.processed }
#[inline] pub fn clear_read_bytes(&mut self) { self.read_bytes = 0 }
#[inline] pub fn set_read_bytes(&mut self, v: i64) { self.read_bytes = v; }
#[inline] pub fn get_read_bytes(&self) -> i64 { self.read_bytes }
}
impl ::protobuf::Clear for ScanInfo {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for ScanInfo {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static ScanInfo {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: ScanInfo = ScanInfo::default();
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
impl ScanDetail {
pub fn new_() -> ScanDetail { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn has_write(&self) -> bool { self.write.is_some() }
#[inline] pub fn clear_write(&mut self) { self.write = ::std::option::Option::None }
#[inline] pub fn set_write(&mut self, v: ScanInfo) { self.write = ::std::option::Option::Some(v); }
#[inline] pub fn get_write(&self) -> &ScanInfo { match self.write.as_ref() {
                            Some(v) => v,
                            None => ScanInfo::default_ref(),
                        } }
#[inline] pub fn mut_write(&mut self) -> &mut ScanInfo { if self.write.is_none() {
                                self.write = ::std::option::Option::Some(ScanInfo::default());
                            }
                            self.write.as_mut().unwrap() } 
#[inline] pub fn take_write(&mut self) -> ScanInfo { self.write.take().unwrap_or_else(ScanInfo::default) }
#[inline] pub fn has_lock(&self) -> bool { self.lock.is_some() }
#[inline] pub fn clear_lock(&mut self) { self.lock = ::std::option::Option::None }
#[inline] pub fn set_lock(&mut self, v: ScanInfo) { self.lock = ::std::option::Option::Some(v); }
#[inline] pub fn get_lock(&self) -> &ScanInfo { match self.lock.as_ref() {
                            Some(v) => v,
                            None => ScanInfo::default_ref(),
                        } }
#[inline] pub fn mut_lock(&mut self) -> &mut ScanInfo { if self.lock.is_none() {
                                self.lock = ::std::option::Option::Some(ScanInfo::default());
                            }
                            self.lock.as_mut().unwrap() } 
#[inline] pub fn take_lock(&mut self) -> ScanInfo { self.lock.take().unwrap_or_else(ScanInfo::default) }
#[inline] pub fn has_data(&self) -> bool { self.data.is_some() }
#[inline] pub fn clear_data(&mut self) { self.data = ::std::option::Option::None }
#[inline] pub fn set_data(&mut self, v: ScanInfo) { self.data = ::std::option::Option::Some(v); }
#[inline] pub fn get_data(&self) -> &ScanInfo { match self.data.as_ref() {
                            Some(v) => v,
                            None => ScanInfo::default_ref(),
                        } }
#[inline] pub fn mut_data(&mut self) -> &mut ScanInfo { if self.data.is_none() {
                                self.data = ::std::option::Option::Some(ScanInfo::default());
                            }
                            self.data.as_mut().unwrap() } 
#[inline] pub fn take_data(&mut self) -> ScanInfo { self.data.take().unwrap_or_else(ScanInfo::default) }
}
impl ::protobuf::Clear for ScanDetail {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for ScanDetail {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static ScanDetail {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: ScanDetail = ScanDetail::default();
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
impl ScanDetailV2 {
pub fn new_() -> ScanDetailV2 { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn clear_processed_versions(&mut self) { self.processed_versions = 0 }
#[inline] pub fn set_processed_versions(&mut self, v: u64) { self.processed_versions = v; }
#[inline] pub fn get_processed_versions(&self) -> u64 { self.processed_versions }
#[inline] pub fn clear_total_versions(&mut self) { self.total_versions = 0 }
#[inline] pub fn set_total_versions(&mut self, v: u64) { self.total_versions = v; }
#[inline] pub fn get_total_versions(&self) -> u64 { self.total_versions }
#[inline] pub fn clear_rocksdb_delete_skipped_count(&mut self) { self.rocksdb_delete_skipped_count = 0 }
#[inline] pub fn set_rocksdb_delete_skipped_count(&mut self, v: u64) { self.rocksdb_delete_skipped_count = v; }
#[inline] pub fn get_rocksdb_delete_skipped_count(&self) -> u64 { self.rocksdb_delete_skipped_count }
#[inline] pub fn clear_rocksdb_key_skipped_count(&mut self) { self.rocksdb_key_skipped_count = 0 }
#[inline] pub fn set_rocksdb_key_skipped_count(&mut self, v: u64) { self.rocksdb_key_skipped_count = v; }
#[inline] pub fn get_rocksdb_key_skipped_count(&self) -> u64 { self.rocksdb_key_skipped_count }
#[inline] pub fn clear_rocksdb_block_cache_hit_count(&mut self) { self.rocksdb_block_cache_hit_count = 0 }
#[inline] pub fn set_rocksdb_block_cache_hit_count(&mut self, v: u64) { self.rocksdb_block_cache_hit_count = v; }
#[inline] pub fn get_rocksdb_block_cache_hit_count(&self) -> u64 { self.rocksdb_block_cache_hit_count }
#[inline] pub fn clear_rocksdb_block_read_count(&mut self) { self.rocksdb_block_read_count = 0 }
#[inline] pub fn set_rocksdb_block_read_count(&mut self, v: u64) { self.rocksdb_block_read_count = v; }
#[inline] pub fn get_rocksdb_block_read_count(&self) -> u64 { self.rocksdb_block_read_count }
#[inline] pub fn clear_rocksdb_block_read_byte(&mut self) { self.rocksdb_block_read_byte = 0 }
#[inline] pub fn set_rocksdb_block_read_byte(&mut self, v: u64) { self.rocksdb_block_read_byte = v; }
#[inline] pub fn get_rocksdb_block_read_byte(&self) -> u64 { self.rocksdb_block_read_byte }
}
impl ::protobuf::Clear for ScanDetailV2 {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for ScanDetailV2 {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static ScanDetailV2 {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: ScanDetailV2 = ScanDetailV2::default();
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
impl ExecDetails {
pub fn new_() -> ExecDetails { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn has_time_detail(&self) -> bool { self.time_detail.is_some() }
#[inline] pub fn clear_time_detail(&mut self) { self.time_detail = ::std::option::Option::None }
#[inline] pub fn set_time_detail(&mut self, v: TimeDetail) { self.time_detail = ::std::option::Option::Some(v); }
#[inline] pub fn get_time_detail(&self) -> &TimeDetail { match self.time_detail.as_ref() {
                            Some(v) => v,
                            None => TimeDetail::default_ref(),
                        } }
#[inline] pub fn mut_time_detail(&mut self) -> &mut TimeDetail { if self.time_detail.is_none() {
                                self.time_detail = ::std::option::Option::Some(TimeDetail::default());
                            }
                            self.time_detail.as_mut().unwrap() } 
#[inline] pub fn take_time_detail(&mut self) -> TimeDetail { self.time_detail.take().unwrap_or_else(TimeDetail::default) }
#[inline] pub fn has_scan_detail(&self) -> bool { self.scan_detail.is_some() }
#[inline] pub fn clear_scan_detail(&mut self) { self.scan_detail = ::std::option::Option::None }
#[inline] pub fn set_scan_detail(&mut self, v: ScanDetail) { self.scan_detail = ::std::option::Option::Some(v); }
#[inline] pub fn get_scan_detail(&self) -> &ScanDetail { match self.scan_detail.as_ref() {
                            Some(v) => v,
                            None => ScanDetail::default_ref(),
                        } }
#[inline] pub fn mut_scan_detail(&mut self) -> &mut ScanDetail { if self.scan_detail.is_none() {
                                self.scan_detail = ::std::option::Option::Some(ScanDetail::default());
                            }
                            self.scan_detail.as_mut().unwrap() } 
#[inline] pub fn take_scan_detail(&mut self) -> ScanDetail { self.scan_detail.take().unwrap_or_else(ScanDetail::default) }
}
impl ::protobuf::Clear for ExecDetails {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for ExecDetails {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static ExecDetails {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: ExecDetails = ExecDetails::default();
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
impl ExecDetailsV2 {
pub fn new_() -> ExecDetailsV2 { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn has_time_detail(&self) -> bool { self.time_detail.is_some() }
#[inline] pub fn clear_time_detail(&mut self) { self.time_detail = ::std::option::Option::None }
#[inline] pub fn set_time_detail(&mut self, v: TimeDetail) { self.time_detail = ::std::option::Option::Some(v); }
#[inline] pub fn get_time_detail(&self) -> &TimeDetail { match self.time_detail.as_ref() {
                            Some(v) => v,
                            None => TimeDetail::default_ref(),
                        } }
#[inline] pub fn mut_time_detail(&mut self) -> &mut TimeDetail { if self.time_detail.is_none() {
                                self.time_detail = ::std::option::Option::Some(TimeDetail::default());
                            }
                            self.time_detail.as_mut().unwrap() } 
#[inline] pub fn take_time_detail(&mut self) -> TimeDetail { self.time_detail.take().unwrap_or_else(TimeDetail::default) }
#[inline] pub fn has_scan_detail_v2(&self) -> bool { self.scan_detail_v2.is_some() }
#[inline] pub fn clear_scan_detail_v2(&mut self) { self.scan_detail_v2 = ::std::option::Option::None }
#[inline] pub fn set_scan_detail_v2(&mut self, v: ScanDetailV2) { self.scan_detail_v2 = ::std::option::Option::Some(v); }
#[inline] pub fn get_scan_detail_v2(&self) -> &ScanDetailV2 { match self.scan_detail_v2.as_ref() {
                            Some(v) => v,
                            None => ScanDetailV2::default_ref(),
                        } }
#[inline] pub fn mut_scan_detail_v2(&mut self) -> &mut ScanDetailV2 { if self.scan_detail_v2.is_none() {
                                self.scan_detail_v2 = ::std::option::Option::Some(ScanDetailV2::default());
                            }
                            self.scan_detail_v2.as_mut().unwrap() } 
#[inline] pub fn take_scan_detail_v2(&mut self) -> ScanDetailV2 { self.scan_detail_v2.take().unwrap_or_else(ScanDetailV2::default) }
}
impl ::protobuf::Clear for ExecDetailsV2 {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for ExecDetailsV2 {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static ExecDetailsV2 {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: ExecDetailsV2 = ExecDetailsV2::default();
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
impl KvPair {
pub fn new_() -> KvPair { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn has_error(&self) -> bool { self.error.is_some() }
#[inline] pub fn clear_error(&mut self) { self.error = ::std::option::Option::None }
#[inline] pub fn set_error(&mut self, v: KeyError) { self.error = ::std::option::Option::Some(v); }
#[inline] pub fn get_error(&self) -> &KeyError { match self.error.as_ref() {
                            Some(v) => v,
                            None => KeyError::default_ref(),
                        } }
#[inline] pub fn mut_error(&mut self) -> &mut KeyError { if self.error.is_none() {
                                self.error = ::std::option::Option::Some(KeyError::default());
                            }
                            self.error.as_mut().unwrap() } 
#[inline] pub fn take_error(&mut self) -> KeyError { self.error.take().unwrap_or_else(KeyError::default) }
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
}
impl ::protobuf::Clear for KvPair {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for KvPair {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static KvPair {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: KvPair = KvPair::default();
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
impl Mutation {
pub fn new_() -> Mutation { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn clear_op(&mut self) { self.op = 0 }
#[inline] pub fn get_op(&self) -> Op { match Op::from_i32(self.op) {Some(e) => e,
                        None => panic!("Unknown enum variant: {}", self.op),
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
#[inline] pub fn clear_assertion(&mut self) { self.assertion = 0 }
#[inline] pub fn get_assertion(&self) -> Assertion { match Assertion::from_i32(self.assertion) {Some(e) => e,
                        None => panic!("Unknown enum variant: {}", self.assertion),
                    } }
}
impl ::protobuf::Clear for Mutation {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for Mutation {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static Mutation {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: Mutation = Mutation::default();
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
impl MvccWrite {
pub fn new_() -> MvccWrite { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn clear_type(&mut self) { self.r#type = 0 }
#[inline] pub fn get_type(&self) -> Op { match Op::from_i32(self.r#type) {Some(e) => e,
                        None => panic!("Unknown enum variant: {}", self.r#type),
                    } }
#[inline] pub fn clear_start_ts(&mut self) { self.start_ts = 0 }
#[inline] pub fn set_start_ts(&mut self, v: u64) { self.start_ts = v; }
#[inline] pub fn get_start_ts(&self) -> u64 { self.start_ts }
#[inline] pub fn clear_commit_ts(&mut self) { self.commit_ts = 0 }
#[inline] pub fn set_commit_ts(&mut self, v: u64) { self.commit_ts = v; }
#[inline] pub fn get_commit_ts(&self) -> u64 { self.commit_ts }
#[inline] pub fn clear_short_value(&mut self) { self.short_value.clear(); }
#[inline] pub fn set_short_value(&mut self, v: :: prost :: alloc :: vec :: Vec < u8 >) { self.short_value = v; }
#[inline] pub fn get_short_value(&self) -> &[u8] { &self.short_value }
#[inline] pub fn mut_short_value(&mut self) -> &mut :: prost :: alloc :: vec :: Vec < u8 > { &mut self.short_value }
#[inline] pub fn take_short_value(&mut self) -> :: prost :: alloc :: vec :: Vec < u8 > { ::std::mem::replace(&mut self.short_value, Default::default()) }
}
impl ::protobuf::Clear for MvccWrite {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for MvccWrite {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static MvccWrite {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: MvccWrite = MvccWrite::default();
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
impl MvccValue {
pub fn new_() -> MvccValue { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn clear_start_ts(&mut self) { self.start_ts = 0 }
#[inline] pub fn set_start_ts(&mut self, v: u64) { self.start_ts = v; }
#[inline] pub fn get_start_ts(&self) -> u64 { self.start_ts }
#[inline] pub fn clear_value(&mut self) { self.value.clear(); }
#[inline] pub fn set_value(&mut self, v: :: prost :: alloc :: vec :: Vec < u8 >) { self.value = v; }
#[inline] pub fn get_value(&self) -> &[u8] { &self.value }
#[inline] pub fn mut_value(&mut self) -> &mut :: prost :: alloc :: vec :: Vec < u8 > { &mut self.value }
#[inline] pub fn take_value(&mut self) -> :: prost :: alloc :: vec :: Vec < u8 > { ::std::mem::replace(&mut self.value, Default::default()) }
}
impl ::protobuf::Clear for MvccValue {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for MvccValue {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static MvccValue {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: MvccValue = MvccValue::default();
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
impl MvccLock {
pub fn new_() -> MvccLock { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn clear_type(&mut self) { self.r#type = 0 }
#[inline] pub fn get_type(&self) -> Op { match Op::from_i32(self.r#type) {Some(e) => e,
                        None => panic!("Unknown enum variant: {}", self.r#type),
                    } }
#[inline] pub fn clear_start_ts(&mut self) { self.start_ts = 0 }
#[inline] pub fn set_start_ts(&mut self, v: u64) { self.start_ts = v; }
#[inline] pub fn get_start_ts(&self) -> u64 { self.start_ts }
#[inline] pub fn clear_primary(&mut self) { self.primary.clear(); }
#[inline] pub fn set_primary(&mut self, v: :: prost :: alloc :: vec :: Vec < u8 >) { self.primary = v; }
#[inline] pub fn get_primary(&self) -> &[u8] { &self.primary }
#[inline] pub fn mut_primary(&mut self) -> &mut :: prost :: alloc :: vec :: Vec < u8 > { &mut self.primary }
#[inline] pub fn take_primary(&mut self) -> :: prost :: alloc :: vec :: Vec < u8 > { ::std::mem::replace(&mut self.primary, Default::default()) }
#[inline] pub fn clear_short_value(&mut self) { self.short_value.clear(); }
#[inline] pub fn set_short_value(&mut self, v: :: prost :: alloc :: vec :: Vec < u8 >) { self.short_value = v; }
#[inline] pub fn get_short_value(&self) -> &[u8] { &self.short_value }
#[inline] pub fn mut_short_value(&mut self) -> &mut :: prost :: alloc :: vec :: Vec < u8 > { &mut self.short_value }
#[inline] pub fn take_short_value(&mut self) -> :: prost :: alloc :: vec :: Vec < u8 > { ::std::mem::replace(&mut self.short_value, Default::default()) }
}
impl ::protobuf::Clear for MvccLock {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for MvccLock {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static MvccLock {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: MvccLock = MvccLock::default();
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
impl MvccInfo {
pub fn new_() -> MvccInfo { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn has_lock(&self) -> bool { self.lock.is_some() }
#[inline] pub fn clear_lock(&mut self) { self.lock = ::std::option::Option::None }
#[inline] pub fn set_lock(&mut self, v: MvccLock) { self.lock = ::std::option::Option::Some(v); }
#[inline] pub fn get_lock(&self) -> &MvccLock { match self.lock.as_ref() {
                            Some(v) => v,
                            None => MvccLock::default_ref(),
                        } }
#[inline] pub fn mut_lock(&mut self) -> &mut MvccLock { if self.lock.is_none() {
                                self.lock = ::std::option::Option::Some(MvccLock::default());
                            }
                            self.lock.as_mut().unwrap() } 
#[inline] pub fn take_lock(&mut self) -> MvccLock { self.lock.take().unwrap_or_else(MvccLock::default) }
#[inline] pub fn clear_writes(&mut self) { self.writes.clear(); }
#[inline] pub fn set_writes(&mut self, v: ::std::vec::Vec<MvccWrite>) { self.writes = v; }
#[inline] pub fn get_writes(&self) -> &[MvccWrite] { &self.writes }
#[inline] pub fn mut_writes(&mut self) -> &mut ::std::vec::Vec<MvccWrite> { &mut self.writes }
#[inline] pub fn take_writes(&mut self) -> ::std::vec::Vec<MvccWrite> { ::std::mem::replace(&mut self.writes, ::std::vec::Vec::new()) }
#[inline] pub fn clear_values(&mut self) { self.values.clear(); }
#[inline] pub fn set_values(&mut self, v: ::std::vec::Vec<MvccValue>) { self.values = v; }
#[inline] pub fn get_values(&self) -> &[MvccValue] { &self.values }
#[inline] pub fn mut_values(&mut self) -> &mut ::std::vec::Vec<MvccValue> { &mut self.values }
#[inline] pub fn take_values(&mut self) -> ::std::vec::Vec<MvccValue> { ::std::mem::replace(&mut self.values, ::std::vec::Vec::new()) }
}
impl ::protobuf::Clear for MvccInfo {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for MvccInfo {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static MvccInfo {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: MvccInfo = MvccInfo::default();
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
#[inline] pub fn clear_txn(&mut self) { self.txn = 0 }
#[inline] pub fn set_txn(&mut self, v: u64) { self.txn = v; }
#[inline] pub fn get_txn(&self) -> u64 { self.txn }
#[inline] pub fn clear_status(&mut self) { self.status = 0 }
#[inline] pub fn set_status(&mut self, v: u64) { self.status = v; }
#[inline] pub fn get_status(&self) -> u64 { self.status }
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
impl KeyRange {
pub fn new_() -> KeyRange { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
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
impl LeaderInfo {
pub fn new_() -> LeaderInfo { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn clear_region_id(&mut self) { self.region_id = 0 }
#[inline] pub fn set_region_id(&mut self, v: u64) { self.region_id = v; }
#[inline] pub fn get_region_id(&self) -> u64 { self.region_id }
#[inline] pub fn clear_peer_id(&mut self) { self.peer_id = 0 }
#[inline] pub fn set_peer_id(&mut self, v: u64) { self.peer_id = v; }
#[inline] pub fn get_peer_id(&self) -> u64 { self.peer_id }
#[inline] pub fn clear_term(&mut self) { self.term = 0 }
#[inline] pub fn set_term(&mut self, v: u64) { self.term = v; }
#[inline] pub fn get_term(&self) -> u64 { self.term }
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
}
impl ::protobuf::Clear for LeaderInfo {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for LeaderInfo {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static LeaderInfo {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: LeaderInfo = LeaderInfo::default();
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
impl CheckLeaderRequest {
pub fn new_() -> CheckLeaderRequest { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn clear_regions(&mut self) { self.regions.clear(); }
#[inline] pub fn set_regions(&mut self, v: ::std::vec::Vec<LeaderInfo>) { self.regions = v; }
#[inline] pub fn get_regions(&self) -> &[LeaderInfo] { &self.regions }
#[inline] pub fn mut_regions(&mut self) -> &mut ::std::vec::Vec<LeaderInfo> { &mut self.regions }
#[inline] pub fn take_regions(&mut self) -> ::std::vec::Vec<LeaderInfo> { ::std::mem::replace(&mut self.regions, ::std::vec::Vec::new()) }
#[inline] pub fn clear_ts(&mut self) { self.ts = 0 }
#[inline] pub fn set_ts(&mut self, v: u64) { self.ts = v; }
#[inline] pub fn get_ts(&self) -> u64 { self.ts }
}
impl ::protobuf::Clear for CheckLeaderRequest {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for CheckLeaderRequest {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static CheckLeaderRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: CheckLeaderRequest = CheckLeaderRequest::default();
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
impl CheckLeaderResponse {
pub fn new_() -> CheckLeaderResponse { ::std::default::Default::default() }
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
impl ::protobuf::Clear for CheckLeaderResponse {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for CheckLeaderResponse {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static CheckLeaderResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: CheckLeaderResponse = CheckLeaderResponse::default();
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
impl StoreSafeTsRequest {
pub fn new_() -> StoreSafeTsRequest { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn has_key_range(&self) -> bool { self.key_range.is_some() }
#[inline] pub fn clear_key_range(&mut self) { self.key_range = ::std::option::Option::None }
#[inline] pub fn set_key_range(&mut self, v: KeyRange) { self.key_range = ::std::option::Option::Some(v); }
#[inline] pub fn get_key_range(&self) -> &KeyRange { match self.key_range.as_ref() {
                            Some(v) => v,
                            None => KeyRange::default_ref(),
                        } }
#[inline] pub fn mut_key_range(&mut self) -> &mut KeyRange { if self.key_range.is_none() {
                                self.key_range = ::std::option::Option::Some(KeyRange::default());
                            }
                            self.key_range.as_mut().unwrap() } 
#[inline] pub fn take_key_range(&mut self) -> KeyRange { self.key_range.take().unwrap_or_else(KeyRange::default) }
}
impl ::protobuf::Clear for StoreSafeTsRequest {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for StoreSafeTsRequest {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static StoreSafeTsRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: StoreSafeTsRequest = StoreSafeTsRequest::default();
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
impl StoreSafeTsResponse {
pub fn new_() -> StoreSafeTsResponse { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn clear_safe_ts(&mut self) { self.safe_ts = 0 }
#[inline] pub fn set_safe_ts(&mut self, v: u64) { self.safe_ts = v; }
#[inline] pub fn get_safe_ts(&self) -> u64 { self.safe_ts }
}
impl ::protobuf::Clear for StoreSafeTsResponse {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for StoreSafeTsResponse {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static StoreSafeTsResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: StoreSafeTsResponse = StoreSafeTsResponse::default();
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
impl RawGetKeyTtlRequest {
pub fn new_() -> RawGetKeyTtlRequest { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn has_context(&self) -> bool { self.context.is_some() }
#[inline] pub fn clear_context(&mut self) { self.context = ::std::option::Option::None }
#[inline] pub fn set_context(&mut self, v: Context) { self.context = ::std::option::Option::Some(v); }
#[inline] pub fn get_context(&self) -> &Context { match self.context.as_ref() {
                            Some(v) => v,
                            None => Context::default_ref(),
                        } }
#[inline] pub fn mut_context(&mut self) -> &mut Context { if self.context.is_none() {
                                self.context = ::std::option::Option::Some(Context::default());
                            }
                            self.context.as_mut().unwrap() } 
#[inline] pub fn take_context(&mut self) -> Context { self.context.take().unwrap_or_else(Context::default) }
#[inline] pub fn clear_key(&mut self) { self.key.clear(); }
#[inline] pub fn set_key(&mut self, v: :: prost :: alloc :: vec :: Vec < u8 >) { self.key = v; }
#[inline] pub fn get_key(&self) -> &[u8] { &self.key }
#[inline] pub fn mut_key(&mut self) -> &mut :: prost :: alloc :: vec :: Vec < u8 > { &mut self.key }
#[inline] pub fn take_key(&mut self) -> :: prost :: alloc :: vec :: Vec < u8 > { ::std::mem::replace(&mut self.key, Default::default()) }
#[inline] pub fn clear_cf(&mut self) { self.cf.clear(); }
#[inline] pub fn set_cf(&mut self, v: :: prost :: alloc :: string :: String) { self.cf = v; }
#[inline] pub fn get_cf(&self) -> &str { &self.cf }
#[inline] pub fn mut_cf(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.cf }
#[inline] pub fn take_cf(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.cf, Default::default()) }
}
impl ::protobuf::Clear for RawGetKeyTtlRequest {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for RawGetKeyTtlRequest {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static RawGetKeyTtlRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: RawGetKeyTtlRequest = RawGetKeyTtlRequest::default();
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
impl RawGetKeyTtlResponse {
pub fn new_() -> RawGetKeyTtlResponse { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
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
#[inline] pub fn clear_error(&mut self) { self.error.clear(); }
#[inline] pub fn set_error(&mut self, v: :: prost :: alloc :: string :: String) { self.error = v; }
#[inline] pub fn get_error(&self) -> &str { &self.error }
#[inline] pub fn mut_error(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.error }
#[inline] pub fn take_error(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.error, Default::default()) }
#[inline] pub fn clear_ttl(&mut self) { self.ttl = 0 }
#[inline] pub fn set_ttl(&mut self, v: u64) { self.ttl = v; }
#[inline] pub fn get_ttl(&self) -> u64 { self.ttl }
#[inline] pub fn clear_not_found(&mut self) { self.not_found = false }
#[inline] pub fn set_not_found(&mut self, v: bool) { self.not_found = v; }
#[inline] pub fn get_not_found(&self) -> bool { self.not_found }
}
impl ::protobuf::Clear for RawGetKeyTtlResponse {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for RawGetKeyTtlResponse {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static RawGetKeyTtlResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: RawGetKeyTtlResponse = RawGetKeyTtlResponse::default();
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
impl RawCasRequest {
pub fn new_() -> RawCasRequest { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn has_context(&self) -> bool { self.context.is_some() }
#[inline] pub fn clear_context(&mut self) { self.context = ::std::option::Option::None }
#[inline] pub fn set_context(&mut self, v: Context) { self.context = ::std::option::Option::Some(v); }
#[inline] pub fn get_context(&self) -> &Context { match self.context.as_ref() {
                            Some(v) => v,
                            None => Context::default_ref(),
                        } }
#[inline] pub fn mut_context(&mut self) -> &mut Context { if self.context.is_none() {
                                self.context = ::std::option::Option::Some(Context::default());
                            }
                            self.context.as_mut().unwrap() } 
#[inline] pub fn take_context(&mut self) -> Context { self.context.take().unwrap_or_else(Context::default) }
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
#[inline] pub fn clear_previous_not_exist(&mut self) { self.previous_not_exist = false }
#[inline] pub fn set_previous_not_exist(&mut self, v: bool) { self.previous_not_exist = v; }
#[inline] pub fn get_previous_not_exist(&self) -> bool { self.previous_not_exist }
#[inline] pub fn clear_previous_value(&mut self) { self.previous_value.clear(); }
#[inline] pub fn set_previous_value(&mut self, v: :: prost :: alloc :: vec :: Vec < u8 >) { self.previous_value = v; }
#[inline] pub fn get_previous_value(&self) -> &[u8] { &self.previous_value }
#[inline] pub fn mut_previous_value(&mut self) -> &mut :: prost :: alloc :: vec :: Vec < u8 > { &mut self.previous_value }
#[inline] pub fn take_previous_value(&mut self) -> :: prost :: alloc :: vec :: Vec < u8 > { ::std::mem::replace(&mut self.previous_value, Default::default()) }
#[inline] pub fn clear_cf(&mut self) { self.cf.clear(); }
#[inline] pub fn set_cf(&mut self, v: :: prost :: alloc :: string :: String) { self.cf = v; }
#[inline] pub fn get_cf(&self) -> &str { &self.cf }
#[inline] pub fn mut_cf(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.cf }
#[inline] pub fn take_cf(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.cf, Default::default()) }
#[inline] pub fn clear_ttl(&mut self) { self.ttl = 0 }
#[inline] pub fn set_ttl(&mut self, v: u64) { self.ttl = v; }
#[inline] pub fn get_ttl(&self) -> u64 { self.ttl }
}
impl ::protobuf::Clear for RawCasRequest {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for RawCasRequest {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static RawCasRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: RawCasRequest = RawCasRequest::default();
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
impl RawCasResponse {
pub fn new_() -> RawCasResponse { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
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
#[inline] pub fn clear_error(&mut self) { self.error.clear(); }
#[inline] pub fn set_error(&mut self, v: :: prost :: alloc :: string :: String) { self.error = v; }
#[inline] pub fn get_error(&self) -> &str { &self.error }
#[inline] pub fn mut_error(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.error }
#[inline] pub fn take_error(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.error, Default::default()) }
#[inline] pub fn clear_succeed(&mut self) { self.succeed = false }
#[inline] pub fn set_succeed(&mut self, v: bool) { self.succeed = v; }
#[inline] pub fn get_succeed(&self) -> bool { self.succeed }
#[inline] pub fn clear_previous_not_exist(&mut self) { self.previous_not_exist = false }
#[inline] pub fn set_previous_not_exist(&mut self, v: bool) { self.previous_not_exist = v; }
#[inline] pub fn get_previous_not_exist(&self) -> bool { self.previous_not_exist }
#[inline] pub fn clear_previous_value(&mut self) { self.previous_value.clear(); }
#[inline] pub fn set_previous_value(&mut self, v: :: prost :: alloc :: vec :: Vec < u8 >) { self.previous_value = v; }
#[inline] pub fn get_previous_value(&self) -> &[u8] { &self.previous_value }
#[inline] pub fn mut_previous_value(&mut self) -> &mut :: prost :: alloc :: vec :: Vec < u8 > { &mut self.previous_value }
#[inline] pub fn take_previous_value(&mut self) -> :: prost :: alloc :: vec :: Vec < u8 > { ::std::mem::replace(&mut self.previous_value, Default::default()) }
}
impl ::protobuf::Clear for RawCasResponse {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for RawCasResponse {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static RawCasResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: RawCasResponse = RawCasResponse::default();
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
impl VerOp {
pub fn values() -> &'static [Self] {
static VALUES: &'static [VerOp] = &[
VerOp::VerPut,
VerOp::VerDel,
];
VALUES
}
}
impl CommandPri {
pub fn values() -> &'static [Self] {
static VALUES: &'static [CommandPri] = &[
CommandPri::Normal,
CommandPri::Low,
CommandPri::High,
];
VALUES
}
}
impl IsolationLevel {
pub fn values() -> &'static [Self] {
static VALUES: &'static [IsolationLevel] = &[
IsolationLevel::Si,
IsolationLevel::Rc,
];
VALUES
}
}
impl Op {
pub fn values() -> &'static [Self] {
static VALUES: &'static [Op] = &[
Op::Put,
Op::Del,
Op::Lock,
Op::Rollback,
Op::Insert,
Op::PessimisticLock,
Op::CheckNotExists,
];
VALUES
}
}
impl Assertion {
pub fn values() -> &'static [Self] {
static VALUES: &'static [Assertion] = &[
Assertion::None,
Assertion::Exist,
Assertion::NotExist,
];
VALUES
}
}
impl Action {
pub fn values() -> &'static [Self] {
static VALUES: &'static [Action] = &[
Action::NoAction,
Action::TtlExpireRollback,
Action::LockNotExistRollback,
Action::MinCommitTsPushed,
Action::TtlExpirePessimisticRollback,
Action::LockNotExistDoNothing,
];
VALUES
}
}
impl ExtraOp {
pub fn values() -> &'static [Self] {
static VALUES: &'static [ExtraOp] = &[
ExtraOp::Noop,
ExtraOp::ReadOldValue,
];
VALUES
}
}
