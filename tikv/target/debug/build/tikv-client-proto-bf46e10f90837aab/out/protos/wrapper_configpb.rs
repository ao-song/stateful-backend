// Generated file, please don't edit manually.

impl Status {
pub fn new_() -> Status { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn clear_code(&mut self) { self.code = 0 }
#[inline] pub fn get_code(&self) -> StatusCode { match StatusCode::from_i32(self.code) {Some(e) => e,
                        None => panic!("Unknown enum variant: {}", self.code),
                    } }
#[inline] pub fn clear_message(&mut self) { self.message.clear(); }
#[inline] pub fn set_message(&mut self, v: :: prost :: alloc :: string :: String) { self.message = v; }
#[inline] pub fn get_message(&self) -> &str { &self.message }
#[inline] pub fn mut_message(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.message }
#[inline] pub fn take_message(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.message, Default::default()) }
}
impl ::protobuf::Clear for Status {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for Status {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static Status {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: Status = Status::default();
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
impl Version {
pub fn new_() -> Version { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn clear_local(&mut self) { self.local = 0 }
#[inline] pub fn set_local(&mut self, v: u64) { self.local = v; }
#[inline] pub fn get_local(&self) -> u64 { self.local }
#[inline] pub fn clear_global(&mut self) { self.global = 0 }
#[inline] pub fn set_global(&mut self, v: u64) { self.global = v; }
#[inline] pub fn get_global(&self) -> u64 { self.global }
}
impl ::protobuf::Clear for Version {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for Version {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static Version {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: Version = Version::default();
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
impl Local {
pub fn new_() -> Local { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn clear_component_id(&mut self) { self.component_id.clear(); }
#[inline] pub fn set_component_id(&mut self, v: :: prost :: alloc :: string :: String) { self.component_id = v; }
#[inline] pub fn get_component_id(&self) -> &str { &self.component_id }
#[inline] pub fn mut_component_id(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.component_id }
#[inline] pub fn take_component_id(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.component_id, Default::default()) }
}
impl ::protobuf::Clear for Local {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for Local {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static Local {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: Local = Local::default();
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
impl Global {
pub fn new_() -> Global { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn clear_component(&mut self) { self.component.clear(); }
#[inline] pub fn set_component(&mut self, v: :: prost :: alloc :: string :: String) { self.component = v; }
#[inline] pub fn get_component(&self) -> &str { &self.component }
#[inline] pub fn mut_component(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.component }
#[inline] pub fn take_component(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.component, Default::default()) }
}
impl ::protobuf::Clear for Global {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for Global {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static Global {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: Global = Global::default();
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
impl ConfigKind {
pub fn new_() -> ConfigKind { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
}
impl ::protobuf::Clear for ConfigKind {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for ConfigKind {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static ConfigKind {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: ConfigKind = ConfigKind::default();
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
impl ConfigEntry {
pub fn new_() -> ConfigEntry { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn clear_name(&mut self) { self.name.clear(); }
#[inline] pub fn set_name(&mut self, v: :: prost :: alloc :: string :: String) { self.name = v; }
#[inline] pub fn get_name(&self) -> &str { &self.name }
#[inline] pub fn mut_name(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.name }
#[inline] pub fn take_name(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.name, Default::default()) }
#[inline] pub fn clear_value(&mut self) { self.value.clear(); }
#[inline] pub fn set_value(&mut self, v: :: prost :: alloc :: string :: String) { self.value = v; }
#[inline] pub fn get_value(&self) -> &str { &self.value }
#[inline] pub fn mut_value(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.value }
#[inline] pub fn take_value(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.value, Default::default()) }
}
impl ::protobuf::Clear for ConfigEntry {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for ConfigEntry {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static ConfigEntry {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: ConfigEntry = ConfigEntry::default();
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
impl LocalConfig {
pub fn new_() -> LocalConfig { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn has_version(&self) -> bool { self.version.is_some() }
#[inline] pub fn clear_version(&mut self) { self.version = ::std::option::Option::None }
#[inline] pub fn set_version(&mut self, v: Version) { self.version = ::std::option::Option::Some(v); }
#[inline] pub fn get_version(&self) -> &Version { match self.version.as_ref() {
                            Some(v) => v,
                            None => Version::default_ref(),
                        } }
#[inline] pub fn mut_version(&mut self) -> &mut Version { if self.version.is_none() {
                                self.version = ::std::option::Option::Some(Version::default());
                            }
                            self.version.as_mut().unwrap() } 
#[inline] pub fn take_version(&mut self) -> Version { self.version.take().unwrap_or_else(Version::default) }
#[inline] pub fn clear_component(&mut self) { self.component.clear(); }
#[inline] pub fn set_component(&mut self, v: :: prost :: alloc :: string :: String) { self.component = v; }
#[inline] pub fn get_component(&self) -> &str { &self.component }
#[inline] pub fn mut_component(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.component }
#[inline] pub fn take_component(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.component, Default::default()) }
#[inline] pub fn clear_component_id(&mut self) { self.component_id.clear(); }
#[inline] pub fn set_component_id(&mut self, v: :: prost :: alloc :: string :: String) { self.component_id = v; }
#[inline] pub fn get_component_id(&self) -> &str { &self.component_id }
#[inline] pub fn mut_component_id(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.component_id }
#[inline] pub fn take_component_id(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.component_id, Default::default()) }
#[inline] pub fn clear_config(&mut self) { self.config.clear(); }
#[inline] pub fn set_config(&mut self, v: :: prost :: alloc :: string :: String) { self.config = v; }
#[inline] pub fn get_config(&self) -> &str { &self.config }
#[inline] pub fn mut_config(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.config }
#[inline] pub fn take_config(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.config, Default::default()) }
}
impl ::protobuf::Clear for LocalConfig {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for LocalConfig {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static LocalConfig {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: LocalConfig = LocalConfig::default();
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
impl Header {
pub fn new_() -> Header { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn clear_cluster_id(&mut self) { self.cluster_id = 0 }
#[inline] pub fn set_cluster_id(&mut self, v: u64) { self.cluster_id = v; }
#[inline] pub fn get_cluster_id(&self) -> u64 { self.cluster_id }
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
impl CreateRequest {
pub fn new_() -> CreateRequest { ::std::default::Default::default() }
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
#[inline] pub fn has_version(&self) -> bool { self.version.is_some() }
#[inline] pub fn clear_version(&mut self) { self.version = ::std::option::Option::None }
#[inline] pub fn set_version(&mut self, v: Version) { self.version = ::std::option::Option::Some(v); }
#[inline] pub fn get_version(&self) -> &Version { match self.version.as_ref() {
                            Some(v) => v,
                            None => Version::default_ref(),
                        } }
#[inline] pub fn mut_version(&mut self) -> &mut Version { if self.version.is_none() {
                                self.version = ::std::option::Option::Some(Version::default());
                            }
                            self.version.as_mut().unwrap() } 
#[inline] pub fn take_version(&mut self) -> Version { self.version.take().unwrap_or_else(Version::default) }
#[inline] pub fn clear_component(&mut self) { self.component.clear(); }
#[inline] pub fn set_component(&mut self, v: :: prost :: alloc :: string :: String) { self.component = v; }
#[inline] pub fn get_component(&self) -> &str { &self.component }
#[inline] pub fn mut_component(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.component }
#[inline] pub fn take_component(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.component, Default::default()) }
#[inline] pub fn clear_component_id(&mut self) { self.component_id.clear(); }
#[inline] pub fn set_component_id(&mut self, v: :: prost :: alloc :: string :: String) { self.component_id = v; }
#[inline] pub fn get_component_id(&self) -> &str { &self.component_id }
#[inline] pub fn mut_component_id(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.component_id }
#[inline] pub fn take_component_id(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.component_id, Default::default()) }
#[inline] pub fn clear_config(&mut self) { self.config.clear(); }
#[inline] pub fn set_config(&mut self, v: :: prost :: alloc :: string :: String) { self.config = v; }
#[inline] pub fn get_config(&self) -> &str { &self.config }
#[inline] pub fn mut_config(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.config }
#[inline] pub fn take_config(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.config, Default::default()) }
}
impl ::protobuf::Clear for CreateRequest {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for CreateRequest {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static CreateRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: CreateRequest = CreateRequest::default();
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
impl CreateResponse {
pub fn new_() -> CreateResponse { ::std::default::Default::default() }
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
#[inline] pub fn has_status(&self) -> bool { self.status.is_some() }
#[inline] pub fn clear_status(&mut self) { self.status = ::std::option::Option::None }
#[inline] pub fn set_status(&mut self, v: Status) { self.status = ::std::option::Option::Some(v); }
#[inline] pub fn get_status(&self) -> &Status { match self.status.as_ref() {
                            Some(v) => v,
                            None => Status::default_ref(),
                        } }
#[inline] pub fn mut_status(&mut self) -> &mut Status { if self.status.is_none() {
                                self.status = ::std::option::Option::Some(Status::default());
                            }
                            self.status.as_mut().unwrap() } 
#[inline] pub fn take_status(&mut self) -> Status { self.status.take().unwrap_or_else(Status::default) }
#[inline] pub fn has_version(&self) -> bool { self.version.is_some() }
#[inline] pub fn clear_version(&mut self) { self.version = ::std::option::Option::None }
#[inline] pub fn set_version(&mut self, v: Version) { self.version = ::std::option::Option::Some(v); }
#[inline] pub fn get_version(&self) -> &Version { match self.version.as_ref() {
                            Some(v) => v,
                            None => Version::default_ref(),
                        } }
#[inline] pub fn mut_version(&mut self) -> &mut Version { if self.version.is_none() {
                                self.version = ::std::option::Option::Some(Version::default());
                            }
                            self.version.as_mut().unwrap() } 
#[inline] pub fn take_version(&mut self) -> Version { self.version.take().unwrap_or_else(Version::default) }
#[inline] pub fn clear_config(&mut self) { self.config.clear(); }
#[inline] pub fn set_config(&mut self, v: :: prost :: alloc :: string :: String) { self.config = v; }
#[inline] pub fn get_config(&self) -> &str { &self.config }
#[inline] pub fn mut_config(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.config }
#[inline] pub fn take_config(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.config, Default::default()) }
}
impl ::protobuf::Clear for CreateResponse {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for CreateResponse {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static CreateResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: CreateResponse = CreateResponse::default();
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
impl GetAllRequest {
pub fn new_() -> GetAllRequest { ::std::default::Default::default() }
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
}
impl ::protobuf::Clear for GetAllRequest {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for GetAllRequest {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static GetAllRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: GetAllRequest = GetAllRequest::default();
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
impl GetAllResponse {
pub fn new_() -> GetAllResponse { ::std::default::Default::default() }
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
#[inline] pub fn has_status(&self) -> bool { self.status.is_some() }
#[inline] pub fn clear_status(&mut self) { self.status = ::std::option::Option::None }
#[inline] pub fn set_status(&mut self, v: Status) { self.status = ::std::option::Option::Some(v); }
#[inline] pub fn get_status(&self) -> &Status { match self.status.as_ref() {
                            Some(v) => v,
                            None => Status::default_ref(),
                        } }
#[inline] pub fn mut_status(&mut self) -> &mut Status { if self.status.is_none() {
                                self.status = ::std::option::Option::Some(Status::default());
                            }
                            self.status.as_mut().unwrap() } 
#[inline] pub fn take_status(&mut self) -> Status { self.status.take().unwrap_or_else(Status::default) }
#[inline] pub fn clear_local_configs(&mut self) { self.local_configs.clear(); }
#[inline] pub fn set_local_configs(&mut self, v: ::std::vec::Vec<LocalConfig>) { self.local_configs = v; }
#[inline] pub fn get_local_configs(&self) -> &[LocalConfig] { &self.local_configs }
#[inline] pub fn mut_local_configs(&mut self) -> &mut ::std::vec::Vec<LocalConfig> { &mut self.local_configs }
#[inline] pub fn take_local_configs(&mut self) -> ::std::vec::Vec<LocalConfig> { ::std::mem::replace(&mut self.local_configs, ::std::vec::Vec::new()) }
}
impl ::protobuf::Clear for GetAllResponse {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for GetAllResponse {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static GetAllResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: GetAllResponse = GetAllResponse::default();
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
impl GetRequest {
pub fn new_() -> GetRequest { ::std::default::Default::default() }
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
#[inline] pub fn has_version(&self) -> bool { self.version.is_some() }
#[inline] pub fn clear_version(&mut self) { self.version = ::std::option::Option::None }
#[inline] pub fn set_version(&mut self, v: Version) { self.version = ::std::option::Option::Some(v); }
#[inline] pub fn get_version(&self) -> &Version { match self.version.as_ref() {
                            Some(v) => v,
                            None => Version::default_ref(),
                        } }
#[inline] pub fn mut_version(&mut self) -> &mut Version { if self.version.is_none() {
                                self.version = ::std::option::Option::Some(Version::default());
                            }
                            self.version.as_mut().unwrap() } 
#[inline] pub fn take_version(&mut self) -> Version { self.version.take().unwrap_or_else(Version::default) }
#[inline] pub fn clear_component(&mut self) { self.component.clear(); }
#[inline] pub fn set_component(&mut self, v: :: prost :: alloc :: string :: String) { self.component = v; }
#[inline] pub fn get_component(&self) -> &str { &self.component }
#[inline] pub fn mut_component(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.component }
#[inline] pub fn take_component(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.component, Default::default()) }
#[inline] pub fn clear_component_id(&mut self) { self.component_id.clear(); }
#[inline] pub fn set_component_id(&mut self, v: :: prost :: alloc :: string :: String) { self.component_id = v; }
#[inline] pub fn get_component_id(&self) -> &str { &self.component_id }
#[inline] pub fn mut_component_id(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.component_id }
#[inline] pub fn take_component_id(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.component_id, Default::default()) }
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
#[inline] pub fn has_status(&self) -> bool { self.status.is_some() }
#[inline] pub fn clear_status(&mut self) { self.status = ::std::option::Option::None }
#[inline] pub fn set_status(&mut self, v: Status) { self.status = ::std::option::Option::Some(v); }
#[inline] pub fn get_status(&self) -> &Status { match self.status.as_ref() {
                            Some(v) => v,
                            None => Status::default_ref(),
                        } }
#[inline] pub fn mut_status(&mut self) -> &mut Status { if self.status.is_none() {
                                self.status = ::std::option::Option::Some(Status::default());
                            }
                            self.status.as_mut().unwrap() } 
#[inline] pub fn take_status(&mut self) -> Status { self.status.take().unwrap_or_else(Status::default) }
#[inline] pub fn has_version(&self) -> bool { self.version.is_some() }
#[inline] pub fn clear_version(&mut self) { self.version = ::std::option::Option::None }
#[inline] pub fn set_version(&mut self, v: Version) { self.version = ::std::option::Option::Some(v); }
#[inline] pub fn get_version(&self) -> &Version { match self.version.as_ref() {
                            Some(v) => v,
                            None => Version::default_ref(),
                        } }
#[inline] pub fn mut_version(&mut self) -> &mut Version { if self.version.is_none() {
                                self.version = ::std::option::Option::Some(Version::default());
                            }
                            self.version.as_mut().unwrap() } 
#[inline] pub fn take_version(&mut self) -> Version { self.version.take().unwrap_or_else(Version::default) }
#[inline] pub fn clear_config(&mut self) { self.config.clear(); }
#[inline] pub fn set_config(&mut self, v: :: prost :: alloc :: string :: String) { self.config = v; }
#[inline] pub fn get_config(&self) -> &str { &self.config }
#[inline] pub fn mut_config(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.config }
#[inline] pub fn take_config(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.config, Default::default()) }
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
impl UpdateRequest {
pub fn new_() -> UpdateRequest { ::std::default::Default::default() }
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
#[inline] pub fn has_version(&self) -> bool { self.version.is_some() }
#[inline] pub fn clear_version(&mut self) { self.version = ::std::option::Option::None }
#[inline] pub fn set_version(&mut self, v: Version) { self.version = ::std::option::Option::Some(v); }
#[inline] pub fn get_version(&self) -> &Version { match self.version.as_ref() {
                            Some(v) => v,
                            None => Version::default_ref(),
                        } }
#[inline] pub fn mut_version(&mut self) -> &mut Version { if self.version.is_none() {
                                self.version = ::std::option::Option::Some(Version::default());
                            }
                            self.version.as_mut().unwrap() } 
#[inline] pub fn take_version(&mut self) -> Version { self.version.take().unwrap_or_else(Version::default) }
#[inline] pub fn has_kind(&self) -> bool { self.kind.is_some() }
#[inline] pub fn clear_kind(&mut self) { self.kind = ::std::option::Option::None }
#[inline] pub fn set_kind(&mut self, v: ConfigKind) { self.kind = ::std::option::Option::Some(v); }
#[inline] pub fn get_kind(&self) -> &ConfigKind { match self.kind.as_ref() {
                            Some(v) => v,
                            None => ConfigKind::default_ref(),
                        } }
#[inline] pub fn mut_kind(&mut self) -> &mut ConfigKind { if self.kind.is_none() {
                                self.kind = ::std::option::Option::Some(ConfigKind::default());
                            }
                            self.kind.as_mut().unwrap() } 
#[inline] pub fn take_kind(&mut self) -> ConfigKind { self.kind.take().unwrap_or_else(ConfigKind::default) }
#[inline] pub fn clear_entries(&mut self) { self.entries.clear(); }
#[inline] pub fn set_entries(&mut self, v: ::std::vec::Vec<ConfigEntry>) { self.entries = v; }
#[inline] pub fn get_entries(&self) -> &[ConfigEntry] { &self.entries }
#[inline] pub fn mut_entries(&mut self) -> &mut ::std::vec::Vec<ConfigEntry> { &mut self.entries }
#[inline] pub fn take_entries(&mut self) -> ::std::vec::Vec<ConfigEntry> { ::std::mem::replace(&mut self.entries, ::std::vec::Vec::new()) }
}
impl ::protobuf::Clear for UpdateRequest {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for UpdateRequest {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static UpdateRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: UpdateRequest = UpdateRequest::default();
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
impl UpdateResponse {
pub fn new_() -> UpdateResponse { ::std::default::Default::default() }
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
#[inline] pub fn has_status(&self) -> bool { self.status.is_some() }
#[inline] pub fn clear_status(&mut self) { self.status = ::std::option::Option::None }
#[inline] pub fn set_status(&mut self, v: Status) { self.status = ::std::option::Option::Some(v); }
#[inline] pub fn get_status(&self) -> &Status { match self.status.as_ref() {
                            Some(v) => v,
                            None => Status::default_ref(),
                        } }
#[inline] pub fn mut_status(&mut self) -> &mut Status { if self.status.is_none() {
                                self.status = ::std::option::Option::Some(Status::default());
                            }
                            self.status.as_mut().unwrap() } 
#[inline] pub fn take_status(&mut self) -> Status { self.status.take().unwrap_or_else(Status::default) }
#[inline] pub fn has_version(&self) -> bool { self.version.is_some() }
#[inline] pub fn clear_version(&mut self) { self.version = ::std::option::Option::None }
#[inline] pub fn set_version(&mut self, v: Version) { self.version = ::std::option::Option::Some(v); }
#[inline] pub fn get_version(&self) -> &Version { match self.version.as_ref() {
                            Some(v) => v,
                            None => Version::default_ref(),
                        } }
#[inline] pub fn mut_version(&mut self) -> &mut Version { if self.version.is_none() {
                                self.version = ::std::option::Option::Some(Version::default());
                            }
                            self.version.as_mut().unwrap() } 
#[inline] pub fn take_version(&mut self) -> Version { self.version.take().unwrap_or_else(Version::default) }
#[inline] pub fn clear_config(&mut self) { self.config.clear(); }
#[inline] pub fn set_config(&mut self, v: :: prost :: alloc :: string :: String) { self.config = v; }
#[inline] pub fn get_config(&self) -> &str { &self.config }
#[inline] pub fn mut_config(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.config }
#[inline] pub fn take_config(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.config, Default::default()) }
}
impl ::protobuf::Clear for UpdateResponse {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for UpdateResponse {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static UpdateResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: UpdateResponse = UpdateResponse::default();
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
impl DeleteRequest {
pub fn new_() -> DeleteRequest { ::std::default::Default::default() }
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
#[inline] pub fn has_version(&self) -> bool { self.version.is_some() }
#[inline] pub fn clear_version(&mut self) { self.version = ::std::option::Option::None }
#[inline] pub fn set_version(&mut self, v: Version) { self.version = ::std::option::Option::Some(v); }
#[inline] pub fn get_version(&self) -> &Version { match self.version.as_ref() {
                            Some(v) => v,
                            None => Version::default_ref(),
                        } }
#[inline] pub fn mut_version(&mut self) -> &mut Version { if self.version.is_none() {
                                self.version = ::std::option::Option::Some(Version::default());
                            }
                            self.version.as_mut().unwrap() } 
#[inline] pub fn take_version(&mut self) -> Version { self.version.take().unwrap_or_else(Version::default) }
#[inline] pub fn has_kind(&self) -> bool { self.kind.is_some() }
#[inline] pub fn clear_kind(&mut self) { self.kind = ::std::option::Option::None }
#[inline] pub fn set_kind(&mut self, v: ConfigKind) { self.kind = ::std::option::Option::Some(v); }
#[inline] pub fn get_kind(&self) -> &ConfigKind { match self.kind.as_ref() {
                            Some(v) => v,
                            None => ConfigKind::default_ref(),
                        } }
#[inline] pub fn mut_kind(&mut self) -> &mut ConfigKind { if self.kind.is_none() {
                                self.kind = ::std::option::Option::Some(ConfigKind::default());
                            }
                            self.kind.as_mut().unwrap() } 
#[inline] pub fn take_kind(&mut self) -> ConfigKind { self.kind.take().unwrap_or_else(ConfigKind::default) }
}
impl ::protobuf::Clear for DeleteRequest {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for DeleteRequest {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static DeleteRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: DeleteRequest = DeleteRequest::default();
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
impl DeleteResponse {
pub fn new_() -> DeleteResponse { ::std::default::Default::default() }
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
#[inline] pub fn has_status(&self) -> bool { self.status.is_some() }
#[inline] pub fn clear_status(&mut self) { self.status = ::std::option::Option::None }
#[inline] pub fn set_status(&mut self, v: Status) { self.status = ::std::option::Option::Some(v); }
#[inline] pub fn get_status(&self) -> &Status { match self.status.as_ref() {
                            Some(v) => v,
                            None => Status::default_ref(),
                        } }
#[inline] pub fn mut_status(&mut self) -> &mut Status { if self.status.is_none() {
                                self.status = ::std::option::Option::Some(Status::default());
                            }
                            self.status.as_mut().unwrap() } 
#[inline] pub fn take_status(&mut self) -> Status { self.status.take().unwrap_or_else(Status::default) }
#[inline] pub fn has_version(&self) -> bool { self.version.is_some() }
#[inline] pub fn clear_version(&mut self) { self.version = ::std::option::Option::None }
#[inline] pub fn set_version(&mut self, v: Version) { self.version = ::std::option::Option::Some(v); }
#[inline] pub fn get_version(&self) -> &Version { match self.version.as_ref() {
                            Some(v) => v,
                            None => Version::default_ref(),
                        } }
#[inline] pub fn mut_version(&mut self) -> &mut Version { if self.version.is_none() {
                                self.version = ::std::option::Option::Some(Version::default());
                            }
                            self.version.as_mut().unwrap() } 
#[inline] pub fn take_version(&mut self) -> Version { self.version.take().unwrap_or_else(Version::default) }
}
impl ::protobuf::Clear for DeleteResponse {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for DeleteResponse {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static DeleteResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: DeleteResponse = DeleteResponse::default();
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
impl StatusCode {
pub fn values() -> &'static [Self] {
static VALUES: &'static [StatusCode] = &[
StatusCode::Unknown,
StatusCode::Ok,
StatusCode::WrongVersion,
StatusCode::NotChange,
StatusCode::ComponentNotFound,
StatusCode::ComponentIdNotFound,
];
VALUES
}
}
