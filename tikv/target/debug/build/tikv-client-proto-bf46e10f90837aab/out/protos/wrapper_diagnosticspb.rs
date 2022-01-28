// Generated file, please don't edit manually.

impl SearchLogRequest {
pub fn new_() -> SearchLogRequest { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn clear_start_time(&mut self) { self.start_time = 0 }
#[inline] pub fn set_start_time(&mut self, v: i64) { self.start_time = v; }
#[inline] pub fn get_start_time(&self) -> i64 { self.start_time }
#[inline] pub fn clear_end_time(&mut self) { self.end_time = 0 }
#[inline] pub fn set_end_time(&mut self, v: i64) { self.end_time = v; }
#[inline] pub fn get_end_time(&self) -> i64 { self.end_time }
#[inline] pub fn clear_levels(&mut self) { self.levels.clear(); }
#[inline] pub fn set_levels(&mut self, v: ::std::vec::Vec<i32>) { self.levels = v; }
#[inline] pub fn get_levels(&self) -> &[i32] { &self.levels }
#[inline] pub fn mut_levels(&mut self) -> &mut ::std::vec::Vec<i32> { &mut self.levels }
#[inline] pub fn take_levels(&mut self) -> ::std::vec::Vec<i32> { ::std::mem::replace(&mut self.levels, ::std::vec::Vec::new()) }
#[inline] pub fn clear_patterns(&mut self) { self.patterns.clear(); }
#[inline] pub fn set_patterns(&mut self, v: ::std::vec::Vec<:: prost :: alloc :: string :: String>) { self.patterns = v; }
#[inline] pub fn get_patterns(&self) -> &[:: prost :: alloc :: string :: String] { &self.patterns }
#[inline] pub fn mut_patterns(&mut self) -> &mut ::std::vec::Vec<:: prost :: alloc :: string :: String> { &mut self.patterns }
#[inline] pub fn take_patterns(&mut self) -> ::std::vec::Vec<:: prost :: alloc :: string :: String> { ::std::mem::replace(&mut self.patterns, ::std::vec::Vec::new()) }
#[inline] pub fn clear_target(&mut self) { self.target = 0 }
#[inline] pub fn get_target(&self) -> search_log_request::Target { match search_log_request :: Target::from_i32(self.target) {Some(e) => e,
                        None => panic!("Unknown enum variant: {}", self.target),
                    } }
}
impl ::protobuf::Clear for SearchLogRequest {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for SearchLogRequest {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static SearchLogRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: SearchLogRequest = SearchLogRequest::default();
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
impl search_log_request::Target {
pub fn values() -> &'static [Self] {
static VALUES: &'static [search_log_request::Target] = &[
search_log_request::Target::Normal,
search_log_request::Target::Slow,
];
VALUES
}
}
impl SearchLogResponse {
pub fn new_() -> SearchLogResponse { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn clear_messages(&mut self) { self.messages.clear(); }
#[inline] pub fn set_messages(&mut self, v: ::std::vec::Vec<LogMessage>) { self.messages = v; }
#[inline] pub fn get_messages(&self) -> &[LogMessage] { &self.messages }
#[inline] pub fn mut_messages(&mut self) -> &mut ::std::vec::Vec<LogMessage> { &mut self.messages }
#[inline] pub fn take_messages(&mut self) -> ::std::vec::Vec<LogMessage> { ::std::mem::replace(&mut self.messages, ::std::vec::Vec::new()) }
}
impl ::protobuf::Clear for SearchLogResponse {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for SearchLogResponse {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static SearchLogResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: SearchLogResponse = SearchLogResponse::default();
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
impl LogMessage {
pub fn new_() -> LogMessage { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn clear_time(&mut self) { self.time = 0 }
#[inline] pub fn set_time(&mut self, v: i64) { self.time = v; }
#[inline] pub fn get_time(&self) -> i64 { self.time }
#[inline] pub fn clear_level(&mut self) { self.level = 0 }
#[inline] pub fn get_level(&self) -> LogLevel { match LogLevel::from_i32(self.level) {Some(e) => e,
                        None => panic!("Unknown enum variant: {}", self.level),
                    } }
#[inline] pub fn clear_message(&mut self) { self.message.clear(); }
#[inline] pub fn set_message(&mut self, v: :: prost :: alloc :: string :: String) { self.message = v; }
#[inline] pub fn get_message(&self) -> &str { &self.message }
#[inline] pub fn mut_message(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.message }
#[inline] pub fn take_message(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.message, Default::default()) }
}
impl ::protobuf::Clear for LogMessage {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for LogMessage {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static LogMessage {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: LogMessage = LogMessage::default();
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
impl ServerInfoRequest {
pub fn new_() -> ServerInfoRequest { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn clear_tp(&mut self) { self.tp = 0 }
#[inline] pub fn get_tp(&self) -> ServerInfoType { match ServerInfoType::from_i32(self.tp) {Some(e) => e,
                        None => panic!("Unknown enum variant: {}", self.tp),
                    } }
}
impl ::protobuf::Clear for ServerInfoRequest {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for ServerInfoRequest {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static ServerInfoRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: ServerInfoRequest = ServerInfoRequest::default();
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
impl ServerInfoPair {
pub fn new_() -> ServerInfoPair { ::std::default::Default::default() }
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
impl ::protobuf::Clear for ServerInfoPair {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for ServerInfoPair {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static ServerInfoPair {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: ServerInfoPair = ServerInfoPair::default();
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
impl ServerInfoItem {
pub fn new_() -> ServerInfoItem { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn clear_tp(&mut self) { self.tp.clear(); }
#[inline] pub fn set_tp(&mut self, v: :: prost :: alloc :: string :: String) { self.tp = v; }
#[inline] pub fn get_tp(&self) -> &str { &self.tp }
#[inline] pub fn mut_tp(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.tp }
#[inline] pub fn take_tp(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.tp, Default::default()) }
#[inline] pub fn clear_name(&mut self) { self.name.clear(); }
#[inline] pub fn set_name(&mut self, v: :: prost :: alloc :: string :: String) { self.name = v; }
#[inline] pub fn get_name(&self) -> &str { &self.name }
#[inline] pub fn mut_name(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.name }
#[inline] pub fn take_name(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.name, Default::default()) }
#[inline] pub fn clear_pairs(&mut self) { self.pairs.clear(); }
#[inline] pub fn set_pairs(&mut self, v: ::std::vec::Vec<ServerInfoPair>) { self.pairs = v; }
#[inline] pub fn get_pairs(&self) -> &[ServerInfoPair] { &self.pairs }
#[inline] pub fn mut_pairs(&mut self) -> &mut ::std::vec::Vec<ServerInfoPair> { &mut self.pairs }
#[inline] pub fn take_pairs(&mut self) -> ::std::vec::Vec<ServerInfoPair> { ::std::mem::replace(&mut self.pairs, ::std::vec::Vec::new()) }
}
impl ::protobuf::Clear for ServerInfoItem {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for ServerInfoItem {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static ServerInfoItem {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: ServerInfoItem = ServerInfoItem::default();
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
impl ServerInfoResponse {
pub fn new_() -> ServerInfoResponse { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn clear_items(&mut self) { self.items.clear(); }
#[inline] pub fn set_items(&mut self, v: ::std::vec::Vec<ServerInfoItem>) { self.items = v; }
#[inline] pub fn get_items(&self) -> &[ServerInfoItem] { &self.items }
#[inline] pub fn mut_items(&mut self) -> &mut ::std::vec::Vec<ServerInfoItem> { &mut self.items }
#[inline] pub fn take_items(&mut self) -> ::std::vec::Vec<ServerInfoItem> { ::std::mem::replace(&mut self.items, ::std::vec::Vec::new()) }
}
impl ::protobuf::Clear for ServerInfoResponse {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for ServerInfoResponse {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static ServerInfoResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: ServerInfoResponse = ServerInfoResponse::default();
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
impl LogLevel {
pub fn values() -> &'static [Self] {
static VALUES: &'static [LogLevel] = &[
LogLevel::Unknown,
LogLevel::Debug,
LogLevel::Info,
LogLevel::Warn,
LogLevel::Trace,
LogLevel::Critical,
LogLevel::Error,
];
VALUES
}
}
impl ServerInfoType {
pub fn values() -> &'static [Self] {
static VALUES: &'static [ServerInfoType] = &[
ServerInfoType::All,
ServerInfoType::HardwareInfo,
ServerInfoType::SystemInfo,
ServerInfoType::LoadInfo,
];
VALUES
}
}
