// Generated file, please don't edit manually.

impl EncryptionMeta {
pub fn new_() -> EncryptionMeta { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn clear_key_id(&mut self) { self.key_id = 0 }
#[inline] pub fn set_key_id(&mut self, v: u64) { self.key_id = v; }
#[inline] pub fn get_key_id(&self) -> u64 { self.key_id }
#[inline] pub fn clear_iv(&mut self) { self.iv.clear(); }
#[inline] pub fn set_iv(&mut self, v: :: prost :: alloc :: vec :: Vec < u8 >) { self.iv = v; }
#[inline] pub fn get_iv(&self) -> &[u8] { &self.iv }
#[inline] pub fn mut_iv(&mut self) -> &mut :: prost :: alloc :: vec :: Vec < u8 > { &mut self.iv }
#[inline] pub fn take_iv(&mut self) -> :: prost :: alloc :: vec :: Vec < u8 > { ::std::mem::replace(&mut self.iv, Default::default()) }
}
impl ::protobuf::Clear for EncryptionMeta {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for EncryptionMeta {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static EncryptionMeta {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: EncryptionMeta = EncryptionMeta::default();
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
impl FileInfo {
pub fn new_() -> FileInfo { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn clear_key_id(&mut self) { self.key_id = 0 }
#[inline] pub fn set_key_id(&mut self, v: u64) { self.key_id = v; }
#[inline] pub fn get_key_id(&self) -> u64 { self.key_id }
#[inline] pub fn clear_iv(&mut self) { self.iv.clear(); }
#[inline] pub fn set_iv(&mut self, v: :: prost :: alloc :: vec :: Vec < u8 >) { self.iv = v; }
#[inline] pub fn get_iv(&self) -> &[u8] { &self.iv }
#[inline] pub fn mut_iv(&mut self) -> &mut :: prost :: alloc :: vec :: Vec < u8 > { &mut self.iv }
#[inline] pub fn take_iv(&mut self) -> :: prost :: alloc :: vec :: Vec < u8 > { ::std::mem::replace(&mut self.iv, Default::default()) }
#[inline] pub fn clear_method(&mut self) { self.method = 0 }
#[inline] pub fn get_method(&self) -> EncryptionMethod { match EncryptionMethod::from_i32(self.method) {Some(e) => e,
                        None => panic!("Unknown enum variant: {}", self.method),
                    } }
}
impl ::protobuf::Clear for FileInfo {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for FileInfo {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static FileInfo {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: FileInfo = FileInfo::default();
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
impl FileDictionary {
pub fn new_() -> FileDictionary { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn clear_files(&mut self) { self.files.clear(); }
#[inline] pub fn set_files(&mut self, v: :: std :: collections :: HashMap < :: prost :: alloc :: string :: String , FileInfo >) { self.files = v; }
#[inline] pub fn get_files(&self) -> &:: std :: collections :: HashMap < :: prost :: alloc :: string :: String , FileInfo > { &self.files }
#[inline] pub fn mut_files(&mut self) -> &mut :: std :: collections :: HashMap < :: prost :: alloc :: string :: String , FileInfo > { &mut self.files }
}
impl ::protobuf::Clear for FileDictionary {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for FileDictionary {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static FileDictionary {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: FileDictionary = FileDictionary::default();
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
impl DataKey {
pub fn new_() -> DataKey { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn clear_key(&mut self) { self.key.clear(); }
#[inline] pub fn set_key(&mut self, v: :: prost :: alloc :: vec :: Vec < u8 >) { self.key = v; }
#[inline] pub fn get_key(&self) -> &[u8] { &self.key }
#[inline] pub fn mut_key(&mut self) -> &mut :: prost :: alloc :: vec :: Vec < u8 > { &mut self.key }
#[inline] pub fn take_key(&mut self) -> :: prost :: alloc :: vec :: Vec < u8 > { ::std::mem::replace(&mut self.key, Default::default()) }
#[inline] pub fn clear_method(&mut self) { self.method = 0 }
#[inline] pub fn get_method(&self) -> EncryptionMethod { match EncryptionMethod::from_i32(self.method) {Some(e) => e,
                        None => panic!("Unknown enum variant: {}", self.method),
                    } }
#[inline] pub fn clear_creation_time(&mut self) { self.creation_time = 0 }
#[inline] pub fn set_creation_time(&mut self, v: u64) { self.creation_time = v; }
#[inline] pub fn get_creation_time(&self) -> u64 { self.creation_time }
#[inline] pub fn clear_was_exposed(&mut self) { self.was_exposed = false }
#[inline] pub fn set_was_exposed(&mut self, v: bool) { self.was_exposed = v; }
#[inline] pub fn get_was_exposed(&self) -> bool { self.was_exposed }
}
impl ::protobuf::Clear for DataKey {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for DataKey {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static DataKey {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: DataKey = DataKey::default();
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
impl KeyDictionary {
pub fn new_() -> KeyDictionary { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn clear_keys(&mut self) { self.keys.clear(); }
#[inline] pub fn set_keys(&mut self, v: :: std :: collections :: HashMap < u64 , DataKey >) { self.keys = v; }
#[inline] pub fn get_keys(&self) -> &:: std :: collections :: HashMap < u64 , DataKey > { &self.keys }
#[inline] pub fn mut_keys(&mut self) -> &mut :: std :: collections :: HashMap < u64 , DataKey > { &mut self.keys }
#[inline] pub fn clear_current_key_id(&mut self) { self.current_key_id = 0 }
#[inline] pub fn set_current_key_id(&mut self, v: u64) { self.current_key_id = v; }
#[inline] pub fn get_current_key_id(&self) -> u64 { self.current_key_id }
}
impl ::protobuf::Clear for KeyDictionary {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for KeyDictionary {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static KeyDictionary {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: KeyDictionary = KeyDictionary::default();
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
impl MasterKey {
pub fn new_() -> MasterKey { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
}
impl ::protobuf::Clear for MasterKey {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for MasterKey {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static MasterKey {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: MasterKey = MasterKey::default();
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
impl MasterKeyPlaintext {
pub fn new_() -> MasterKeyPlaintext { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
}
impl ::protobuf::Clear for MasterKeyPlaintext {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for MasterKeyPlaintext {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static MasterKeyPlaintext {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: MasterKeyPlaintext = MasterKeyPlaintext::default();
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
impl MasterKeyFile {
pub fn new_() -> MasterKeyFile { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn clear_path(&mut self) { self.path.clear(); }
#[inline] pub fn set_path(&mut self, v: :: prost :: alloc :: string :: String) { self.path = v; }
#[inline] pub fn get_path(&self) -> &str { &self.path }
#[inline] pub fn mut_path(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.path }
#[inline] pub fn take_path(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.path, Default::default()) }
}
impl ::protobuf::Clear for MasterKeyFile {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for MasterKeyFile {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static MasterKeyFile {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: MasterKeyFile = MasterKeyFile::default();
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
impl MasterKeyKms {
pub fn new_() -> MasterKeyKms { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn clear_vendor(&mut self) { self.vendor.clear(); }
#[inline] pub fn set_vendor(&mut self, v: :: prost :: alloc :: string :: String) { self.vendor = v; }
#[inline] pub fn get_vendor(&self) -> &str { &self.vendor }
#[inline] pub fn mut_vendor(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.vendor }
#[inline] pub fn take_vendor(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.vendor, Default::default()) }
#[inline] pub fn clear_key_id(&mut self) { self.key_id.clear(); }
#[inline] pub fn set_key_id(&mut self, v: :: prost :: alloc :: string :: String) { self.key_id = v; }
#[inline] pub fn get_key_id(&self) -> &str { &self.key_id }
#[inline] pub fn mut_key_id(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.key_id }
#[inline] pub fn take_key_id(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.key_id, Default::default()) }
#[inline] pub fn clear_region(&mut self) { self.region.clear(); }
#[inline] pub fn set_region(&mut self, v: :: prost :: alloc :: string :: String) { self.region = v; }
#[inline] pub fn get_region(&self) -> &str { &self.region }
#[inline] pub fn mut_region(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.region }
#[inline] pub fn take_region(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.region, Default::default()) }
#[inline] pub fn clear_endpoint(&mut self) { self.endpoint.clear(); }
#[inline] pub fn set_endpoint(&mut self, v: :: prost :: alloc :: string :: String) { self.endpoint = v; }
#[inline] pub fn get_endpoint(&self) -> &str { &self.endpoint }
#[inline] pub fn mut_endpoint(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.endpoint }
#[inline] pub fn take_endpoint(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.endpoint, Default::default()) }
}
impl ::protobuf::Clear for MasterKeyKms {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for MasterKeyKms {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static MasterKeyKms {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: MasterKeyKms = MasterKeyKms::default();
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
impl EncryptedContent {
pub fn new_() -> EncryptedContent { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn clear_metadata(&mut self) { self.metadata.clear(); }
#[inline] pub fn set_metadata(&mut self, v: :: std :: collections :: HashMap < :: prost :: alloc :: string :: String , :: prost :: alloc :: vec :: Vec < u8 > >) { self.metadata = v; }
#[inline] pub fn get_metadata(&self) -> &:: std :: collections :: HashMap < :: prost :: alloc :: string :: String , :: prost :: alloc :: vec :: Vec < u8 > > { &self.metadata }
#[inline] pub fn mut_metadata(&mut self) -> &mut :: std :: collections :: HashMap < :: prost :: alloc :: string :: String , :: prost :: alloc :: vec :: Vec < u8 > > { &mut self.metadata }
#[inline] pub fn clear_content(&mut self) { self.content.clear(); }
#[inline] pub fn set_content(&mut self, v: :: prost :: alloc :: vec :: Vec < u8 >) { self.content = v; }
#[inline] pub fn get_content(&self) -> &[u8] { &self.content }
#[inline] pub fn mut_content(&mut self) -> &mut :: prost :: alloc :: vec :: Vec < u8 > { &mut self.content }
#[inline] pub fn take_content(&mut self) -> :: prost :: alloc :: vec :: Vec < u8 > { ::std::mem::replace(&mut self.content, Default::default()) }
#[inline] pub fn has_master_key(&self) -> bool { self.master_key.is_some() }
#[inline] pub fn clear_master_key(&mut self) { self.master_key = ::std::option::Option::None }
#[inline] pub fn set_master_key(&mut self, v: MasterKey) { self.master_key = ::std::option::Option::Some(v); }
#[inline] pub fn get_master_key(&self) -> &MasterKey { match self.master_key.as_ref() {
                            Some(v) => v,
                            None => MasterKey::default_ref(),
                        } }
#[inline] pub fn mut_master_key(&mut self) -> &mut MasterKey { if self.master_key.is_none() {
                                self.master_key = ::std::option::Option::Some(MasterKey::default());
                            }
                            self.master_key.as_mut().unwrap() } 
#[inline] pub fn take_master_key(&mut self) -> MasterKey { self.master_key.take().unwrap_or_else(MasterKey::default) }
#[inline] pub fn clear_iv(&mut self) { self.iv.clear(); }
#[inline] pub fn set_iv(&mut self, v: :: prost :: alloc :: vec :: Vec < u8 >) { self.iv = v; }
#[inline] pub fn get_iv(&self) -> &[u8] { &self.iv }
#[inline] pub fn mut_iv(&mut self) -> &mut :: prost :: alloc :: vec :: Vec < u8 > { &mut self.iv }
#[inline] pub fn take_iv(&mut self) -> :: prost :: alloc :: vec :: Vec < u8 > { ::std::mem::replace(&mut self.iv, Default::default()) }
#[inline] pub fn clear_ciphertext_key(&mut self) { self.ciphertext_key.clear(); }
#[inline] pub fn set_ciphertext_key(&mut self, v: :: prost :: alloc :: vec :: Vec < u8 >) { self.ciphertext_key = v; }
#[inline] pub fn get_ciphertext_key(&self) -> &[u8] { &self.ciphertext_key }
#[inline] pub fn mut_ciphertext_key(&mut self) -> &mut :: prost :: alloc :: vec :: Vec < u8 > { &mut self.ciphertext_key }
#[inline] pub fn take_ciphertext_key(&mut self) -> :: prost :: alloc :: vec :: Vec < u8 > { ::std::mem::replace(&mut self.ciphertext_key, Default::default()) }
}
impl ::protobuf::Clear for EncryptedContent {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for EncryptedContent {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static EncryptedContent {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: EncryptedContent = EncryptedContent::default();
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
impl EncryptionMethod {
pub fn values() -> &'static [Self] {
static VALUES: &'static [EncryptionMethod] = &[
EncryptionMethod::Unknown,
EncryptionMethod::Plaintext,
EncryptionMethod::Aes128Ctr,
EncryptionMethod::Aes192Ctr,
EncryptionMethod::Aes256Ctr,
];
VALUES
}
}
