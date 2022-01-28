// Generated file, please don't edit manually.

impl RawCoprocessorRequest {
pub fn new_() -> RawCoprocessorRequest { ::std::default::Default::default() }
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
#[inline] pub fn clear_copr_name(&mut self) { self.copr_name.clear(); }
#[inline] pub fn set_copr_name(&mut self, v: :: prost :: alloc :: string :: String) { self.copr_name = v; }
#[inline] pub fn get_copr_name(&self) -> &str { &self.copr_name }
#[inline] pub fn mut_copr_name(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.copr_name }
#[inline] pub fn take_copr_name(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.copr_name, Default::default()) }
#[inline] pub fn clear_copr_version_constraint(&mut self) { self.copr_version_constraint.clear(); }
#[inline] pub fn set_copr_version_constraint(&mut self, v: :: prost :: alloc :: string :: String) { self.copr_version_constraint = v; }
#[inline] pub fn get_copr_version_constraint(&self) -> &str { &self.copr_version_constraint }
#[inline] pub fn mut_copr_version_constraint(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.copr_version_constraint }
#[inline] pub fn take_copr_version_constraint(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.copr_version_constraint, Default::default()) }
#[inline] pub fn clear_data(&mut self) { self.data.clear(); }
#[inline] pub fn set_data(&mut self, v: :: prost :: alloc :: vec :: Vec < u8 >) { self.data = v; }
#[inline] pub fn get_data(&self) -> &[u8] { &self.data }
#[inline] pub fn mut_data(&mut self) -> &mut :: prost :: alloc :: vec :: Vec < u8 > { &mut self.data }
#[inline] pub fn take_data(&mut self) -> :: prost :: alloc :: vec :: Vec < u8 > { ::std::mem::replace(&mut self.data, Default::default()) }
}
impl ::protobuf::Clear for RawCoprocessorRequest {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for RawCoprocessorRequest {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static RawCoprocessorRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: RawCoprocessorRequest = RawCoprocessorRequest::default();
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
impl RawCoprocessorResponse {
pub fn new_() -> RawCoprocessorResponse { ::std::default::Default::default() }
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
#[inline] pub fn clear_other_error(&mut self) { self.other_error.clear(); }
#[inline] pub fn set_other_error(&mut self, v: :: prost :: alloc :: string :: String) { self.other_error = v; }
#[inline] pub fn get_other_error(&self) -> &str { &self.other_error }
#[inline] pub fn mut_other_error(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.other_error }
#[inline] pub fn take_other_error(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.other_error, Default::default()) }
}
impl ::protobuf::Clear for RawCoprocessorResponse {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for RawCoprocessorResponse {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static RawCoprocessorResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: RawCoprocessorResponse = RawCoprocessorResponse::default();
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
