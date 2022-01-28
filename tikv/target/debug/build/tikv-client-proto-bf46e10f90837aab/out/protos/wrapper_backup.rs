// Generated file, please don't edit manually.

impl BackupMeta {
pub fn new_() -> BackupMeta { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn clear_cluster_id(&mut self) { self.cluster_id = 0 }
#[inline] pub fn set_cluster_id(&mut self, v: u64) { self.cluster_id = v; }
#[inline] pub fn get_cluster_id(&self) -> u64 { self.cluster_id }
#[inline] pub fn clear_cluster_version(&mut self) { self.cluster_version.clear(); }
#[inline] pub fn set_cluster_version(&mut self, v: :: prost :: alloc :: string :: String) { self.cluster_version = v; }
#[inline] pub fn get_cluster_version(&self) -> &str { &self.cluster_version }
#[inline] pub fn mut_cluster_version(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.cluster_version }
#[inline] pub fn take_cluster_version(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.cluster_version, Default::default()) }
#[inline] pub fn clear_files(&mut self) { self.files.clear(); }
#[inline] pub fn set_files(&mut self, v: ::std::vec::Vec<File>) { self.files = v; }
#[inline] pub fn get_files(&self) -> &[File] { &self.files }
#[inline] pub fn mut_files(&mut self) -> &mut ::std::vec::Vec<File> { &mut self.files }
#[inline] pub fn take_files(&mut self) -> ::std::vec::Vec<File> { ::std::mem::replace(&mut self.files, ::std::vec::Vec::new()) }
#[inline] pub fn clear_start_version(&mut self) { self.start_version = 0 }
#[inline] pub fn set_start_version(&mut self, v: u64) { self.start_version = v; }
#[inline] pub fn get_start_version(&self) -> u64 { self.start_version }
#[inline] pub fn clear_end_version(&mut self) { self.end_version = 0 }
#[inline] pub fn set_end_version(&mut self, v: u64) { self.end_version = v; }
#[inline] pub fn get_end_version(&self) -> u64 { self.end_version }
#[inline] pub fn clear_schemas(&mut self) { self.schemas.clear(); }
#[inline] pub fn set_schemas(&mut self, v: ::std::vec::Vec<Schema>) { self.schemas = v; }
#[inline] pub fn get_schemas(&self) -> &[Schema] { &self.schemas }
#[inline] pub fn mut_schemas(&mut self) -> &mut ::std::vec::Vec<Schema> { &mut self.schemas }
#[inline] pub fn take_schemas(&mut self) -> ::std::vec::Vec<Schema> { ::std::mem::replace(&mut self.schemas, ::std::vec::Vec::new()) }
#[inline] pub fn clear_is_raw_kv(&mut self) { self.is_raw_kv = false }
#[inline] pub fn set_is_raw_kv(&mut self, v: bool) { self.is_raw_kv = v; }
#[inline] pub fn get_is_raw_kv(&self) -> bool { self.is_raw_kv }
#[inline] pub fn clear_raw_ranges(&mut self) { self.raw_ranges.clear(); }
#[inline] pub fn set_raw_ranges(&mut self, v: ::std::vec::Vec<RawRange>) { self.raw_ranges = v; }
#[inline] pub fn get_raw_ranges(&self) -> &[RawRange] { &self.raw_ranges }
#[inline] pub fn mut_raw_ranges(&mut self) -> &mut ::std::vec::Vec<RawRange> { &mut self.raw_ranges }
#[inline] pub fn take_raw_ranges(&mut self) -> ::std::vec::Vec<RawRange> { ::std::mem::replace(&mut self.raw_ranges, ::std::vec::Vec::new()) }
#[inline] pub fn clear_ddls(&mut self) { self.ddls.clear(); }
#[inline] pub fn set_ddls(&mut self, v: :: prost :: alloc :: vec :: Vec < u8 >) { self.ddls = v; }
#[inline] pub fn get_ddls(&self) -> &[u8] { &self.ddls }
#[inline] pub fn mut_ddls(&mut self) -> &mut :: prost :: alloc :: vec :: Vec < u8 > { &mut self.ddls }
#[inline] pub fn take_ddls(&mut self) -> :: prost :: alloc :: vec :: Vec < u8 > { ::std::mem::replace(&mut self.ddls, Default::default()) }
#[inline] pub fn clear_br_version(&mut self) { self.br_version.clear(); }
#[inline] pub fn set_br_version(&mut self, v: :: prost :: alloc :: string :: String) { self.br_version = v; }
#[inline] pub fn get_br_version(&self) -> &str { &self.br_version }
#[inline] pub fn mut_br_version(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.br_version }
#[inline] pub fn take_br_version(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.br_version, Default::default()) }
}
impl ::protobuf::Clear for BackupMeta {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for BackupMeta {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static BackupMeta {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: BackupMeta = BackupMeta::default();
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
impl File {
pub fn new_() -> File { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn clear_name(&mut self) { self.name.clear(); }
#[inline] pub fn set_name(&mut self, v: :: prost :: alloc :: string :: String) { self.name = v; }
#[inline] pub fn get_name(&self) -> &str { &self.name }
#[inline] pub fn mut_name(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.name }
#[inline] pub fn take_name(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.name, Default::default()) }
#[inline] pub fn clear_sha256(&mut self) { self.sha256.clear(); }
#[inline] pub fn set_sha256(&mut self, v: :: prost :: alloc :: vec :: Vec < u8 >) { self.sha256 = v; }
#[inline] pub fn get_sha256(&self) -> &[u8] { &self.sha256 }
#[inline] pub fn mut_sha256(&mut self) -> &mut :: prost :: alloc :: vec :: Vec < u8 > { &mut self.sha256 }
#[inline] pub fn take_sha256(&mut self) -> :: prost :: alloc :: vec :: Vec < u8 > { ::std::mem::replace(&mut self.sha256, Default::default()) }
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
#[inline] pub fn clear_start_version(&mut self) { self.start_version = 0 }
#[inline] pub fn set_start_version(&mut self, v: u64) { self.start_version = v; }
#[inline] pub fn get_start_version(&self) -> u64 { self.start_version }
#[inline] pub fn clear_end_version(&mut self) { self.end_version = 0 }
#[inline] pub fn set_end_version(&mut self, v: u64) { self.end_version = v; }
#[inline] pub fn get_end_version(&self) -> u64 { self.end_version }
#[inline] pub fn clear_crc64xor(&mut self) { self.crc64xor = 0 }
#[inline] pub fn set_crc64xor(&mut self, v: u64) { self.crc64xor = v; }
#[inline] pub fn get_crc64xor(&self) -> u64 { self.crc64xor }
#[inline] pub fn clear_total_kvs(&mut self) { self.total_kvs = 0 }
#[inline] pub fn set_total_kvs(&mut self, v: u64) { self.total_kvs = v; }
#[inline] pub fn get_total_kvs(&self) -> u64 { self.total_kvs }
#[inline] pub fn clear_total_bytes(&mut self) { self.total_bytes = 0 }
#[inline] pub fn set_total_bytes(&mut self, v: u64) { self.total_bytes = v; }
#[inline] pub fn get_total_bytes(&self) -> u64 { self.total_bytes }
#[inline] pub fn clear_cf(&mut self) { self.cf.clear(); }
#[inline] pub fn set_cf(&mut self, v: :: prost :: alloc :: string :: String) { self.cf = v; }
#[inline] pub fn get_cf(&self) -> &str { &self.cf }
#[inline] pub fn mut_cf(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.cf }
#[inline] pub fn take_cf(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.cf, Default::default()) }
#[inline] pub fn clear_size(&mut self) { self.size = 0 }
#[inline] pub fn set_size(&mut self, v: u64) { self.size = v; }
#[inline] pub fn get_size(&self) -> u64 { self.size }
}
impl ::protobuf::Clear for File {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for File {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static File {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: File = File::default();
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
impl Schema {
pub fn new_() -> Schema { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn clear_db(&mut self) { self.db.clear(); }
#[inline] pub fn set_db(&mut self, v: :: prost :: alloc :: vec :: Vec < u8 >) { self.db = v; }
#[inline] pub fn get_db(&self) -> &[u8] { &self.db }
#[inline] pub fn mut_db(&mut self) -> &mut :: prost :: alloc :: vec :: Vec < u8 > { &mut self.db }
#[inline] pub fn take_db(&mut self) -> :: prost :: alloc :: vec :: Vec < u8 > { ::std::mem::replace(&mut self.db, Default::default()) }
#[inline] pub fn clear_table(&mut self) { self.table.clear(); }
#[inline] pub fn set_table(&mut self, v: :: prost :: alloc :: vec :: Vec < u8 >) { self.table = v; }
#[inline] pub fn get_table(&self) -> &[u8] { &self.table }
#[inline] pub fn mut_table(&mut self) -> &mut :: prost :: alloc :: vec :: Vec < u8 > { &mut self.table }
#[inline] pub fn take_table(&mut self) -> :: prost :: alloc :: vec :: Vec < u8 > { ::std::mem::replace(&mut self.table, Default::default()) }
#[inline] pub fn clear_crc64xor(&mut self) { self.crc64xor = 0 }
#[inline] pub fn set_crc64xor(&mut self, v: u64) { self.crc64xor = v; }
#[inline] pub fn get_crc64xor(&self) -> u64 { self.crc64xor }
#[inline] pub fn clear_total_kvs(&mut self) { self.total_kvs = 0 }
#[inline] pub fn set_total_kvs(&mut self, v: u64) { self.total_kvs = v; }
#[inline] pub fn get_total_kvs(&self) -> u64 { self.total_kvs }
#[inline] pub fn clear_total_bytes(&mut self) { self.total_bytes = 0 }
#[inline] pub fn set_total_bytes(&mut self, v: u64) { self.total_bytes = v; }
#[inline] pub fn get_total_bytes(&self) -> u64 { self.total_bytes }
#[inline] pub fn clear_tiflash_replicas(&mut self) { self.tiflash_replicas = 0 }
#[inline] pub fn set_tiflash_replicas(&mut self, v: u32) { self.tiflash_replicas = v; }
#[inline] pub fn get_tiflash_replicas(&self) -> u32 { self.tiflash_replicas }
#[inline] pub fn clear_stats(&mut self) { self.stats.clear(); }
#[inline] pub fn set_stats(&mut self, v: :: prost :: alloc :: vec :: Vec < u8 >) { self.stats = v; }
#[inline] pub fn get_stats(&self) -> &[u8] { &self.stats }
#[inline] pub fn mut_stats(&mut self) -> &mut :: prost :: alloc :: vec :: Vec < u8 > { &mut self.stats }
#[inline] pub fn take_stats(&mut self) -> :: prost :: alloc :: vec :: Vec < u8 > { ::std::mem::replace(&mut self.stats, Default::default()) }
}
impl ::protobuf::Clear for Schema {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for Schema {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static Schema {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: Schema = Schema::default();
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
impl RawRange {
pub fn new_() -> RawRange { ::std::default::Default::default() }
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
#[inline] pub fn clear_cf(&mut self) { self.cf.clear(); }
#[inline] pub fn set_cf(&mut self, v: :: prost :: alloc :: string :: String) { self.cf = v; }
#[inline] pub fn get_cf(&self) -> &str { &self.cf }
#[inline] pub fn mut_cf(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.cf }
#[inline] pub fn take_cf(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.cf, Default::default()) }
}
impl ::protobuf::Clear for RawRange {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for RawRange {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static RawRange {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: RawRange = RawRange::default();
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
impl ClusterIdError {
pub fn new_() -> ClusterIdError { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn clear_current(&mut self) { self.current = 0 }
#[inline] pub fn set_current(&mut self, v: u64) { self.current = v; }
#[inline] pub fn get_current(&self) -> u64 { self.current }
#[inline] pub fn clear_request(&mut self) { self.request = 0 }
#[inline] pub fn set_request(&mut self, v: u64) { self.request = v; }
#[inline] pub fn get_request(&self) -> u64 { self.request }
}
impl ::protobuf::Clear for ClusterIdError {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for ClusterIdError {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static ClusterIdError {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: ClusterIdError = ClusterIdError::default();
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
impl BackupRequest {
pub fn new_() -> BackupRequest { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn clear_cluster_id(&mut self) { self.cluster_id = 0 }
#[inline] pub fn set_cluster_id(&mut self, v: u64) { self.cluster_id = v; }
#[inline] pub fn get_cluster_id(&self) -> u64 { self.cluster_id }
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
#[inline] pub fn clear_start_version(&mut self) { self.start_version = 0 }
#[inline] pub fn set_start_version(&mut self, v: u64) { self.start_version = v; }
#[inline] pub fn get_start_version(&self) -> u64 { self.start_version }
#[inline] pub fn clear_end_version(&mut self) { self.end_version = 0 }
#[inline] pub fn set_end_version(&mut self, v: u64) { self.end_version = v; }
#[inline] pub fn get_end_version(&self) -> u64 { self.end_version }
#[inline] pub fn clear_rate_limit(&mut self) { self.rate_limit = 0 }
#[inline] pub fn set_rate_limit(&mut self, v: u64) { self.rate_limit = v; }
#[inline] pub fn get_rate_limit(&self) -> u64 { self.rate_limit }
#[inline] pub fn clear_concurrency(&mut self) { self.concurrency = 0 }
#[inline] pub fn set_concurrency(&mut self, v: u32) { self.concurrency = v; }
#[inline] pub fn get_concurrency(&self) -> u32 { self.concurrency }
#[inline] pub fn has_storage_backend(&self) -> bool { self.storage_backend.is_some() }
#[inline] pub fn clear_storage_backend(&mut self) { self.storage_backend = ::std::option::Option::None }
#[inline] pub fn set_storage_backend(&mut self, v: StorageBackend) { self.storage_backend = ::std::option::Option::Some(v); }
#[inline] pub fn get_storage_backend(&self) -> &StorageBackend { match self.storage_backend.as_ref() {
                            Some(v) => v,
                            None => StorageBackend::default_ref(),
                        } }
#[inline] pub fn mut_storage_backend(&mut self) -> &mut StorageBackend { if self.storage_backend.is_none() {
                                self.storage_backend = ::std::option::Option::Some(StorageBackend::default());
                            }
                            self.storage_backend.as_mut().unwrap() } 
#[inline] pub fn take_storage_backend(&mut self) -> StorageBackend { self.storage_backend.take().unwrap_or_else(StorageBackend::default) }
#[inline] pub fn clear_is_raw_kv(&mut self) { self.is_raw_kv = false }
#[inline] pub fn set_is_raw_kv(&mut self, v: bool) { self.is_raw_kv = v; }
#[inline] pub fn get_is_raw_kv(&self) -> bool { self.is_raw_kv }
#[inline] pub fn clear_cf(&mut self) { self.cf.clear(); }
#[inline] pub fn set_cf(&mut self, v: :: prost :: alloc :: string :: String) { self.cf = v; }
#[inline] pub fn get_cf(&self) -> &str { &self.cf }
#[inline] pub fn mut_cf(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.cf }
#[inline] pub fn take_cf(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.cf, Default::default()) }
#[inline] pub fn clear_compression_type(&mut self) { self.compression_type = 0 }
#[inline] pub fn get_compression_type(&self) -> CompressionType { match CompressionType::from_i32(self.compression_type) {Some(e) => e,
                        None => panic!("Unknown enum variant: {}", self.compression_type),
                    } }
#[inline] pub fn clear_compression_level(&mut self) { self.compression_level = 0 }
#[inline] pub fn set_compression_level(&mut self, v: i32) { self.compression_level = v; }
#[inline] pub fn get_compression_level(&self) -> i32 { self.compression_level }
}
impl ::protobuf::Clear for BackupRequest {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for BackupRequest {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static BackupRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: BackupRequest = BackupRequest::default();
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
impl StorageBackend {
pub fn new_() -> StorageBackend { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
}
impl ::protobuf::Clear for StorageBackend {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for StorageBackend {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static StorageBackend {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: StorageBackend = StorageBackend::default();
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
impl Noop {
pub fn new_() -> Noop { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
}
impl ::protobuf::Clear for Noop {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for Noop {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static Noop {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: Noop = Noop::default();
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
#[inline] pub fn clear_path(&mut self) { self.path.clear(); }
#[inline] pub fn set_path(&mut self, v: :: prost :: alloc :: string :: String) { self.path = v; }
#[inline] pub fn get_path(&self) -> &str { &self.path }
#[inline] pub fn mut_path(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.path }
#[inline] pub fn take_path(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.path, Default::default()) }
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
impl S3 {
pub fn new_() -> S3 { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn clear_endpoint(&mut self) { self.endpoint.clear(); }
#[inline] pub fn set_endpoint(&mut self, v: :: prost :: alloc :: string :: String) { self.endpoint = v; }
#[inline] pub fn get_endpoint(&self) -> &str { &self.endpoint }
#[inline] pub fn mut_endpoint(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.endpoint }
#[inline] pub fn take_endpoint(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.endpoint, Default::default()) }
#[inline] pub fn clear_region(&mut self) { self.region.clear(); }
#[inline] pub fn set_region(&mut self, v: :: prost :: alloc :: string :: String) { self.region = v; }
#[inline] pub fn get_region(&self) -> &str { &self.region }
#[inline] pub fn mut_region(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.region }
#[inline] pub fn take_region(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.region, Default::default()) }
#[inline] pub fn clear_bucket(&mut self) { self.bucket.clear(); }
#[inline] pub fn set_bucket(&mut self, v: :: prost :: alloc :: string :: String) { self.bucket = v; }
#[inline] pub fn get_bucket(&self) -> &str { &self.bucket }
#[inline] pub fn mut_bucket(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.bucket }
#[inline] pub fn take_bucket(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.bucket, Default::default()) }
#[inline] pub fn clear_prefix(&mut self) { self.prefix.clear(); }
#[inline] pub fn set_prefix(&mut self, v: :: prost :: alloc :: string :: String) { self.prefix = v; }
#[inline] pub fn get_prefix(&self) -> &str { &self.prefix }
#[inline] pub fn mut_prefix(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.prefix }
#[inline] pub fn take_prefix(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.prefix, Default::default()) }
#[inline] pub fn clear_storage_class(&mut self) { self.storage_class.clear(); }
#[inline] pub fn set_storage_class(&mut self, v: :: prost :: alloc :: string :: String) { self.storage_class = v; }
#[inline] pub fn get_storage_class(&self) -> &str { &self.storage_class }
#[inline] pub fn mut_storage_class(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.storage_class }
#[inline] pub fn take_storage_class(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.storage_class, Default::default()) }
#[inline] pub fn clear_sse(&mut self) { self.sse.clear(); }
#[inline] pub fn set_sse(&mut self, v: :: prost :: alloc :: string :: String) { self.sse = v; }
#[inline] pub fn get_sse(&self) -> &str { &self.sse }
#[inline] pub fn mut_sse(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.sse }
#[inline] pub fn take_sse(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.sse, Default::default()) }
#[inline] pub fn clear_acl(&mut self) { self.acl.clear(); }
#[inline] pub fn set_acl(&mut self, v: :: prost :: alloc :: string :: String) { self.acl = v; }
#[inline] pub fn get_acl(&self) -> &str { &self.acl }
#[inline] pub fn mut_acl(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.acl }
#[inline] pub fn take_acl(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.acl, Default::default()) }
#[inline] pub fn clear_access_key(&mut self) { self.access_key.clear(); }
#[inline] pub fn set_access_key(&mut self, v: :: prost :: alloc :: string :: String) { self.access_key = v; }
#[inline] pub fn get_access_key(&self) -> &str { &self.access_key }
#[inline] pub fn mut_access_key(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.access_key }
#[inline] pub fn take_access_key(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.access_key, Default::default()) }
#[inline] pub fn clear_secret_access_key(&mut self) { self.secret_access_key.clear(); }
#[inline] pub fn set_secret_access_key(&mut self, v: :: prost :: alloc :: string :: String) { self.secret_access_key = v; }
#[inline] pub fn get_secret_access_key(&self) -> &str { &self.secret_access_key }
#[inline] pub fn mut_secret_access_key(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.secret_access_key }
#[inline] pub fn take_secret_access_key(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.secret_access_key, Default::default()) }
#[inline] pub fn clear_force_path_style(&mut self) { self.force_path_style = false }
#[inline] pub fn set_force_path_style(&mut self, v: bool) { self.force_path_style = v; }
#[inline] pub fn get_force_path_style(&self) -> bool { self.force_path_style }
#[inline] pub fn clear_sse_kms_key_id(&mut self) { self.sse_kms_key_id.clear(); }
#[inline] pub fn set_sse_kms_key_id(&mut self, v: :: prost :: alloc :: string :: String) { self.sse_kms_key_id = v; }
#[inline] pub fn get_sse_kms_key_id(&self) -> &str { &self.sse_kms_key_id }
#[inline] pub fn mut_sse_kms_key_id(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.sse_kms_key_id }
#[inline] pub fn take_sse_kms_key_id(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.sse_kms_key_id, Default::default()) }
}
impl ::protobuf::Clear for S3 {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for S3 {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static S3 {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: S3 = S3::default();
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
impl Gcs {
pub fn new_() -> Gcs { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn clear_endpoint(&mut self) { self.endpoint.clear(); }
#[inline] pub fn set_endpoint(&mut self, v: :: prost :: alloc :: string :: String) { self.endpoint = v; }
#[inline] pub fn get_endpoint(&self) -> &str { &self.endpoint }
#[inline] pub fn mut_endpoint(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.endpoint }
#[inline] pub fn take_endpoint(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.endpoint, Default::default()) }
#[inline] pub fn clear_bucket(&mut self) { self.bucket.clear(); }
#[inline] pub fn set_bucket(&mut self, v: :: prost :: alloc :: string :: String) { self.bucket = v; }
#[inline] pub fn get_bucket(&self) -> &str { &self.bucket }
#[inline] pub fn mut_bucket(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.bucket }
#[inline] pub fn take_bucket(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.bucket, Default::default()) }
#[inline] pub fn clear_prefix(&mut self) { self.prefix.clear(); }
#[inline] pub fn set_prefix(&mut self, v: :: prost :: alloc :: string :: String) { self.prefix = v; }
#[inline] pub fn get_prefix(&self) -> &str { &self.prefix }
#[inline] pub fn mut_prefix(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.prefix }
#[inline] pub fn take_prefix(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.prefix, Default::default()) }
#[inline] pub fn clear_storage_class(&mut self) { self.storage_class.clear(); }
#[inline] pub fn set_storage_class(&mut self, v: :: prost :: alloc :: string :: String) { self.storage_class = v; }
#[inline] pub fn get_storage_class(&self) -> &str { &self.storage_class }
#[inline] pub fn mut_storage_class(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.storage_class }
#[inline] pub fn take_storage_class(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.storage_class, Default::default()) }
#[inline] pub fn clear_predefined_acl(&mut self) { self.predefined_acl.clear(); }
#[inline] pub fn set_predefined_acl(&mut self, v: :: prost :: alloc :: string :: String) { self.predefined_acl = v; }
#[inline] pub fn get_predefined_acl(&self) -> &str { &self.predefined_acl }
#[inline] pub fn mut_predefined_acl(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.predefined_acl }
#[inline] pub fn take_predefined_acl(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.predefined_acl, Default::default()) }
#[inline] pub fn clear_credentials_blob(&mut self) { self.credentials_blob.clear(); }
#[inline] pub fn set_credentials_blob(&mut self, v: :: prost :: alloc :: string :: String) { self.credentials_blob = v; }
#[inline] pub fn get_credentials_blob(&self) -> &str { &self.credentials_blob }
#[inline] pub fn mut_credentials_blob(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.credentials_blob }
#[inline] pub fn take_credentials_blob(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.credentials_blob, Default::default()) }
}
impl ::protobuf::Clear for Gcs {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for Gcs {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static Gcs {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: Gcs = Gcs::default();
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
impl Bucket {
pub fn new_() -> Bucket { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn clear_endpoint(&mut self) { self.endpoint.clear(); }
#[inline] pub fn set_endpoint(&mut self, v: :: prost :: alloc :: string :: String) { self.endpoint = v; }
#[inline] pub fn get_endpoint(&self) -> &str { &self.endpoint }
#[inline] pub fn mut_endpoint(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.endpoint }
#[inline] pub fn take_endpoint(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.endpoint, Default::default()) }
#[inline] pub fn clear_region(&mut self) { self.region.clear(); }
#[inline] pub fn set_region(&mut self, v: :: prost :: alloc :: string :: String) { self.region = v; }
#[inline] pub fn get_region(&self) -> &str { &self.region }
#[inline] pub fn mut_region(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.region }
#[inline] pub fn take_region(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.region, Default::default()) }
#[inline] pub fn clear_bucket(&mut self) { self.bucket.clear(); }
#[inline] pub fn set_bucket(&mut self, v: :: prost :: alloc :: string :: String) { self.bucket = v; }
#[inline] pub fn get_bucket(&self) -> &str { &self.bucket }
#[inline] pub fn mut_bucket(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.bucket }
#[inline] pub fn take_bucket(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.bucket, Default::default()) }
#[inline] pub fn clear_prefix(&mut self) { self.prefix.clear(); }
#[inline] pub fn set_prefix(&mut self, v: :: prost :: alloc :: string :: String) { self.prefix = v; }
#[inline] pub fn get_prefix(&self) -> &str { &self.prefix }
#[inline] pub fn mut_prefix(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.prefix }
#[inline] pub fn take_prefix(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.prefix, Default::default()) }
#[inline] pub fn clear_storage_class(&mut self) { self.storage_class.clear(); }
#[inline] pub fn set_storage_class(&mut self, v: :: prost :: alloc :: string :: String) { self.storage_class = v; }
#[inline] pub fn get_storage_class(&self) -> &str { &self.storage_class }
#[inline] pub fn mut_storage_class(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.storage_class }
#[inline] pub fn take_storage_class(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.storage_class, Default::default()) }
}
impl ::protobuf::Clear for Bucket {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for Bucket {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static Bucket {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: Bucket = Bucket::default();
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
impl CloudDynamic {
pub fn new_() -> CloudDynamic { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn has_bucket(&self) -> bool { self.bucket.is_some() }
#[inline] pub fn clear_bucket(&mut self) { self.bucket = ::std::option::Option::None }
#[inline] pub fn set_bucket(&mut self, v: Bucket) { self.bucket = ::std::option::Option::Some(v); }
#[inline] pub fn get_bucket(&self) -> &Bucket { match self.bucket.as_ref() {
                            Some(v) => v,
                            None => Bucket::default_ref(),
                        } }
#[inline] pub fn mut_bucket(&mut self) -> &mut Bucket { if self.bucket.is_none() {
                                self.bucket = ::std::option::Option::Some(Bucket::default());
                            }
                            self.bucket.as_mut().unwrap() } 
#[inline] pub fn take_bucket(&mut self) -> Bucket { self.bucket.take().unwrap_or_else(Bucket::default) }
#[inline] pub fn clear_provider_name(&mut self) { self.provider_name.clear(); }
#[inline] pub fn set_provider_name(&mut self, v: :: prost :: alloc :: string :: String) { self.provider_name = v; }
#[inline] pub fn get_provider_name(&self) -> &str { &self.provider_name }
#[inline] pub fn mut_provider_name(&mut self) -> &mut :: prost :: alloc :: string :: String { &mut self.provider_name }
#[inline] pub fn take_provider_name(&mut self) -> :: prost :: alloc :: string :: String { ::std::mem::replace(&mut self.provider_name, Default::default()) }
#[inline] pub fn clear_attrs(&mut self) { self.attrs.clear(); }
#[inline] pub fn set_attrs(&mut self, v: :: std :: collections :: HashMap < :: prost :: alloc :: string :: String , :: prost :: alloc :: string :: String >) { self.attrs = v; }
#[inline] pub fn get_attrs(&self) -> &:: std :: collections :: HashMap < :: prost :: alloc :: string :: String , :: prost :: alloc :: string :: String > { &self.attrs }
#[inline] pub fn mut_attrs(&mut self) -> &mut :: std :: collections :: HashMap < :: prost :: alloc :: string :: String , :: prost :: alloc :: string :: String > { &mut self.attrs }
}
impl ::protobuf::Clear for CloudDynamic {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for CloudDynamic {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static CloudDynamic {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: CloudDynamic = CloudDynamic::default();
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
impl BackupResponse {
pub fn new_() -> BackupResponse { ::std::default::Default::default() }
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
#[inline] pub fn clear_files(&mut self) { self.files.clear(); }
#[inline] pub fn set_files(&mut self, v: ::std::vec::Vec<File>) { self.files = v; }
#[inline] pub fn get_files(&self) -> &[File] { &self.files }
#[inline] pub fn mut_files(&mut self) -> &mut ::std::vec::Vec<File> { &mut self.files }
#[inline] pub fn take_files(&mut self) -> ::std::vec::Vec<File> { ::std::mem::replace(&mut self.files, ::std::vec::Vec::new()) }
}
impl ::protobuf::Clear for BackupResponse {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for BackupResponse {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static BackupResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: BackupResponse = BackupResponse::default();
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
impl CompressionType {
pub fn values() -> &'static [Self] {
static VALUES: &'static [CompressionType] = &[
CompressionType::Unknown,
CompressionType::Lz4,
CompressionType::Snappy,
CompressionType::Zstd,
];
VALUES
}
}
