// Generated file, please don't edit manually.

impl TaskMeta {
pub fn new_() -> TaskMeta { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn clear_start_ts(&mut self) { self.start_ts = 0 }
#[inline] pub fn set_start_ts(&mut self, v: u64) { self.start_ts = v; }
#[inline] pub fn get_start_ts(&self) -> u64 { self.start_ts }
#[inline] pub fn clear_task_id(&mut self) { self.task_id = 0 }
#[inline] pub fn set_task_id(&mut self, v: i64) { self.task_id = v; }
#[inline] pub fn get_task_id(&self) -> i64 { self.task_id }
#[inline] pub fn clear_partition_id(&mut self) { self.partition_id = 0 }
#[inline] pub fn set_partition_id(&mut self, v: i64) { self.partition_id = v; }
#[inline] pub fn get_partition_id(&self) -> i64 { self.partition_id }
#[inline] pub fn clear_address(&mut self) { self.address.clear(); }
#[inline] pub fn set_address(&mut self, v: :: prost :: alloc :: string :: String) { self.address = v; }
#[inline] pub fn get_address(&self) -> &str { &self.address }
#[inline] pub fn mut_address(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.address }
#[inline] pub fn take_address(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.address, Default::default()) }
}
impl ::protobuf::Clear for TaskMeta {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for TaskMeta {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static TaskMeta {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: TaskMeta = TaskMeta::default();
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
impl DispatchTaskRequest {
pub fn new_() -> DispatchTaskRequest { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn has_meta(&self) -> bool { self.meta.is_some() }
#[inline] pub fn clear_meta(&mut self) { self.meta = ::std::option::Option::None }
#[inline] pub fn set_meta(&mut self, v: TaskMeta) { self.meta = ::std::option::Option::Some(v); }
#[inline] pub fn get_meta(&self) -> &TaskMeta { match self.meta.as_ref() {
                            Some(v) => v,
                            None => TaskMeta::default_ref(),
                        } }
#[inline] pub fn mut_meta(&mut self) -> &mut TaskMeta { if self.meta.is_none() {
                                self.meta = ::std::option::Option::Some(TaskMeta::default());
                            }
                            self.meta.as_mut().unwrap() } 
#[inline] pub fn take_meta(&mut self) -> TaskMeta { self.meta.take().unwrap_or_else(TaskMeta::default) }
#[inline] pub fn clear_encoded_plan(&mut self) { self.encoded_plan.clear(); }
#[inline] pub fn set_encoded_plan(&mut self, v: :: prost :: alloc :: vec :: Vec < u8 >) { self.encoded_plan = v; }
#[inline] pub fn get_encoded_plan(&self) -> &[u8] { &self.encoded_plan }
#[inline] pub fn mut_encoded_plan(&mut self) -> &mut :: prost :: alloc :: vec :: Vec < u8 > { &mut self.encoded_plan }
#[inline] pub fn take_encoded_plan(&mut self) -> :: prost :: alloc :: vec :: Vec < u8 > { ::std::mem::replace(&mut self.encoded_plan, Default::default()) }
#[inline] pub fn clear_timeout(&mut self) { self.timeout = 0 }
#[inline] pub fn set_timeout(&mut self, v: i64) { self.timeout = v; }
#[inline] pub fn get_timeout(&self) -> i64 { self.timeout }
#[inline] pub fn clear_regions(&mut self) { self.regions.clear(); }
#[inline] pub fn set_regions(&mut self, v: ::std::vec::Vec<super :: coprocessor :: RegionInfo>) { self.regions = v; }
#[inline] pub fn get_regions(&self) -> &[super :: coprocessor :: RegionInfo] { &self.regions }
#[inline] pub fn mut_regions(&mut self) -> &mut ::std::vec::Vec<super :: coprocessor :: RegionInfo> { &mut self.regions }
#[inline] pub fn take_regions(&mut self) -> ::std::vec::Vec<super :: coprocessor :: RegionInfo> { ::std::mem::replace(&mut self.regions, ::std::vec::Vec::new()) }
#[inline] pub fn clear_schema_ver(&mut self) { self.schema_ver = 0 }
#[inline] pub fn set_schema_ver(&mut self, v: i64) { self.schema_ver = v; }
#[inline] pub fn get_schema_ver(&self) -> i64 { self.schema_ver }
}
impl ::protobuf::Clear for DispatchTaskRequest {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for DispatchTaskRequest {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static DispatchTaskRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: DispatchTaskRequest = DispatchTaskRequest::default();
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
impl DispatchTaskResponse {
pub fn new_() -> DispatchTaskResponse { ::std::default::Default::default() }
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
}
impl ::protobuf::Clear for DispatchTaskResponse {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for DispatchTaskResponse {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static DispatchTaskResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: DispatchTaskResponse = DispatchTaskResponse::default();
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
impl CancelTaskRequest {
pub fn new_() -> CancelTaskRequest { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn has_meta(&self) -> bool { self.meta.is_some() }
#[inline] pub fn clear_meta(&mut self) { self.meta = ::std::option::Option::None }
#[inline] pub fn set_meta(&mut self, v: TaskMeta) { self.meta = ::std::option::Option::Some(v); }
#[inline] pub fn get_meta(&self) -> &TaskMeta { match self.meta.as_ref() {
                            Some(v) => v,
                            None => TaskMeta::default_ref(),
                        } }
#[inline] pub fn mut_meta(&mut self) -> &mut TaskMeta { if self.meta.is_none() {
                                self.meta = ::std::option::Option::Some(TaskMeta::default());
                            }
                            self.meta.as_mut().unwrap() } 
#[inline] pub fn take_meta(&mut self) -> TaskMeta { self.meta.take().unwrap_or_else(TaskMeta::default) }
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
}
impl ::protobuf::Clear for CancelTaskRequest {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for CancelTaskRequest {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static CancelTaskRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: CancelTaskRequest = CancelTaskRequest::default();
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
impl CancelTaskResponse {
pub fn new_() -> CancelTaskResponse { ::std::default::Default::default() }
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
}
impl ::protobuf::Clear for CancelTaskResponse {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for CancelTaskResponse {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static CancelTaskResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: CancelTaskResponse = CancelTaskResponse::default();
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
impl EstablishMppConnectionRequest {
pub fn new_() -> EstablishMppConnectionRequest { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn has_sender_meta(&self) -> bool { self.sender_meta.is_some() }
#[inline] pub fn clear_sender_meta(&mut self) { self.sender_meta = ::std::option::Option::None }
#[inline] pub fn set_sender_meta(&mut self, v: TaskMeta) { self.sender_meta = ::std::option::Option::Some(v); }
#[inline] pub fn get_sender_meta(&self) -> &TaskMeta { match self.sender_meta.as_ref() {
                            Some(v) => v,
                            None => TaskMeta::default_ref(),
                        } }
#[inline] pub fn mut_sender_meta(&mut self) -> &mut TaskMeta { if self.sender_meta.is_none() {
                                self.sender_meta = ::std::option::Option::Some(TaskMeta::default());
                            }
                            self.sender_meta.as_mut().unwrap() } 
#[inline] pub fn take_sender_meta(&mut self) -> TaskMeta { self.sender_meta.take().unwrap_or_else(TaskMeta::default) }
#[inline] pub fn has_receiver_meta(&self) -> bool { self.receiver_meta.is_some() }
#[inline] pub fn clear_receiver_meta(&mut self) { self.receiver_meta = ::std::option::Option::None }
#[inline] pub fn set_receiver_meta(&mut self, v: TaskMeta) { self.receiver_meta = ::std::option::Option::Some(v); }
#[inline] pub fn get_receiver_meta(&self) -> &TaskMeta { match self.receiver_meta.as_ref() {
                            Some(v) => v,
                            None => TaskMeta::default_ref(),
                        } }
#[inline] pub fn mut_receiver_meta(&mut self) -> &mut TaskMeta { if self.receiver_meta.is_none() {
                                self.receiver_meta = ::std::option::Option::Some(TaskMeta::default());
                            }
                            self.receiver_meta.as_mut().unwrap() } 
#[inline] pub fn take_receiver_meta(&mut self) -> TaskMeta { self.receiver_meta.take().unwrap_or_else(TaskMeta::default) }
}
impl ::protobuf::Clear for EstablishMppConnectionRequest {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for EstablishMppConnectionRequest {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static EstablishMppConnectionRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: EstablishMppConnectionRequest = EstablishMppConnectionRequest::default();
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
impl MppDataPacket {
pub fn new_() -> MppDataPacket { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn clear_data(&mut self) { self.data.clear(); }
#[inline] pub fn set_data(&mut self, v: :: prost :: alloc :: vec :: Vec < u8 >) { self.data = v; }
#[inline] pub fn get_data(&self) -> &[u8] { &self.data }
#[inline] pub fn mut_data(&mut self) -> &mut :: prost :: alloc :: vec :: Vec < u8 > { &mut self.data }
#[inline] pub fn take_data(&mut self) -> :: prost :: alloc :: vec :: Vec < u8 > { ::std::mem::replace(&mut self.data, Default::default()) }
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
}
impl ::protobuf::Clear for MppDataPacket {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for MppDataPacket {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static MppDataPacket {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: MppDataPacket = MppDataPacket::default();
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
#[inline] pub fn clear_code(&mut self) { self.code = 0 }
#[inline] pub fn set_code(&mut self, v: i32) { self.code = v; }
#[inline] pub fn get_code(&self) -> i32 { self.code }
#[inline] pub fn clear_msg(&mut self) { self.msg.clear(); }
#[inline] pub fn set_msg(&mut self, v: :: prost :: alloc :: string :: String) { self.msg = v; }
#[inline] pub fn get_msg(&self) -> &str { &self.msg }
#[inline] pub fn mut_msg(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.msg }
#[inline] pub fn take_msg(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.msg, Default::default()) }
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
