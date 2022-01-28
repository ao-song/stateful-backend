// Generated file, please don't edit manually.

impl Http {
pub fn new_() -> Http { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn clear_rules(&mut self) { self.rules.clear(); }
#[inline] pub fn set_rules(&mut self, v: ::std::vec::Vec<HttpRule>) { self.rules = v; }
#[inline] pub fn get_rules(&self) -> &[HttpRule] { &self.rules }
#[inline] pub fn mut_rules(&mut self) -> &mut ::std::vec::Vec<HttpRule> { &mut self.rules }
#[inline] pub fn take_rules(&mut self) -> ::std::vec::Vec<HttpRule> { ::std::mem::replace(&mut self.rules, ::std::vec::Vec::new()) }
#[inline] pub fn clear_fully_decode_reserved_expansion(&mut self) { self.fully_decode_reserved_expansion = false }
#[inline] pub fn set_fully_decode_reserved_expansion(&mut self, v: bool) { self.fully_decode_reserved_expansion = v; }
#[inline] pub fn get_fully_decode_reserved_expansion(&self) -> bool { self.fully_decode_reserved_expansion }
}
impl ::protobuf::Clear for Http {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for Http {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static Http {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: Http = Http::default();
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
impl HttpRule {
pub fn new_() -> HttpRule { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn clear_selector(&mut self) { self.selector.clear(); }
#[inline] pub fn set_selector(&mut self, v: :: prost :: alloc :: string :: String) { self.selector = v; }
#[inline] pub fn get_selector(&self) -> &str { &self.selector }
#[inline] pub fn mut_selector(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.selector }
#[inline] pub fn take_selector(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.selector, Default::default()) }
#[inline] pub fn clear_body(&mut self) { self.body.clear(); }
#[inline] pub fn set_body(&mut self, v: :: prost :: alloc :: string :: String) { self.body = v; }
#[inline] pub fn get_body(&self) -> &str { &self.body }
#[inline] pub fn mut_body(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.body }
#[inline] pub fn take_body(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.body, Default::default()) }
#[inline] pub fn clear_response_body(&mut self) { self.response_body.clear(); }
#[inline] pub fn set_response_body(&mut self, v: :: prost :: alloc :: string :: String) { self.response_body = v; }
#[inline] pub fn get_response_body(&self) -> &str { &self.response_body }
#[inline] pub fn mut_response_body(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.response_body }
#[inline] pub fn take_response_body(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.response_body, Default::default()) }
#[inline] pub fn clear_additional_bindings(&mut self) { self.additional_bindings.clear(); }
#[inline] pub fn set_additional_bindings(&mut self, v: ::std::vec::Vec<HttpRule>) { self.additional_bindings = v; }
#[inline] pub fn get_additional_bindings(&self) -> &[HttpRule] { &self.additional_bindings }
#[inline] pub fn mut_additional_bindings(&mut self) -> &mut ::std::vec::Vec<HttpRule> { &mut self.additional_bindings }
#[inline] pub fn take_additional_bindings(&mut self) -> ::std::vec::Vec<HttpRule> { ::std::mem::replace(&mut self.additional_bindings, ::std::vec::Vec::new()) }
}
impl ::protobuf::Clear for HttpRule {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for HttpRule {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static HttpRule {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: HttpRule = HttpRule::default();
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
impl CustomHttpPattern {
pub fn new_() -> CustomHttpPattern { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn clear_kind(&mut self) { self.kind.clear(); }
#[inline] pub fn set_kind(&mut self, v: :: prost :: alloc :: string :: String) { self.kind = v; }
#[inline] pub fn get_kind(&self) -> &str { &self.kind }
#[inline] pub fn mut_kind(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.kind }
#[inline] pub fn take_kind(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.kind, Default::default()) }
#[inline] pub fn clear_path(&mut self) { self.path.clear(); }
#[inline] pub fn set_path(&mut self, v: :: prost :: alloc :: string :: String) { self.path = v; }
#[inline] pub fn get_path(&self) -> &str { &self.path }
#[inline] pub fn mut_path(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.path }
#[inline] pub fn take_path(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.path, Default::default()) }
}
impl ::protobuf::Clear for CustomHttpPattern {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for CustomHttpPattern {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static CustomHttpPattern {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: CustomHttpPattern = CustomHttpPattern::default();
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
