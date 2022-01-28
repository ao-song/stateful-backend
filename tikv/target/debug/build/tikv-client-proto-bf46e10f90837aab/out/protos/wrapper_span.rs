// Generated file, please don't edit manually.

impl SpanSet {
pub fn new_() -> SpanSet { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn clear_start_time_ns(&mut self) { self.start_time_ns = 0 }
#[inline] pub fn set_start_time_ns(&mut self, v: u64) { self.start_time_ns = v; }
#[inline] pub fn get_start_time_ns(&self) -> u64 { self.start_time_ns }
#[inline] pub fn clear_cycles_per_sec(&mut self) { self.cycles_per_sec = 0 }
#[inline] pub fn set_cycles_per_sec(&mut self, v: u64) { self.cycles_per_sec = v; }
#[inline] pub fn get_cycles_per_sec(&self) -> u64 { self.cycles_per_sec }
#[inline] pub fn clear_spans(&mut self) { self.spans.clear(); }
#[inline] pub fn set_spans(&mut self, v: ::std::vec::Vec<Span>) { self.spans = v; }
#[inline] pub fn get_spans(&self) -> &[Span] { &self.spans }
#[inline] pub fn mut_spans(&mut self) -> &mut ::std::vec::Vec<Span> { &mut self.spans }
#[inline] pub fn take_spans(&mut self) -> ::std::vec::Vec<Span> { ::std::mem::replace(&mut self.spans, ::std::vec::Vec::new()) }
#[inline] pub fn clear_create_time_ns(&mut self) { self.create_time_ns = 0 }
#[inline] pub fn set_create_time_ns(&mut self, v: u64) { self.create_time_ns = v; }
#[inline] pub fn get_create_time_ns(&self) -> u64 { self.create_time_ns }
}
impl ::protobuf::Clear for SpanSet {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for SpanSet {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static SpanSet {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: SpanSet = SpanSet::default();
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
impl Root {
pub fn new_() -> Root { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
}
impl ::protobuf::Clear for Root {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for Root {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static Root {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: Root = Root::default();
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
impl Parent {
pub fn new_() -> Parent { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn clear_id(&mut self) { self.id = 0 }
#[inline] pub fn set_id(&mut self, v: u64) { self.id = v; }
#[inline] pub fn get_id(&self) -> u64 { self.id }
}
impl ::protobuf::Clear for Parent {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for Parent {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static Parent {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: Parent = Parent::default();
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
impl Continue {
pub fn new_() -> Continue { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn clear_id(&mut self) { self.id = 0 }
#[inline] pub fn set_id(&mut self, v: u64) { self.id = v; }
#[inline] pub fn get_id(&self) -> u64 { self.id }
}
impl ::protobuf::Clear for Continue {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for Continue {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static Continue {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: Continue = Continue::default();
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
impl Link {
pub fn new_() -> Link { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
}
impl ::protobuf::Clear for Link {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for Link {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static Link {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: Link = Link::default();
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
impl Span {
pub fn new_() -> Span { ::std::default::Default::default() }
#[inline] pub fn default_ref() -> &'static Self { ::protobuf::Message::default_instance() }
#[inline] pub fn clear_id(&mut self) { self.id = 0 }
#[inline] pub fn set_id(&mut self, v: u64) { self.id = v; }
#[inline] pub fn get_id(&self) -> u64 { self.id }
#[inline] pub fn has_link(&self) -> bool { self.link.is_some() }
#[inline] pub fn clear_link(&mut self) { self.link = ::std::option::Option::None }
#[inline] pub fn set_link(&mut self, v: Link) { self.link = ::std::option::Option::Some(v); }
#[inline] pub fn get_link(&self) -> &Link { match self.link.as_ref() {
                            Some(v) => v,
                            None => Link::default_ref(),
                        } }
#[inline] pub fn mut_link(&mut self) -> &mut Link { if self.link.is_none() {
                                self.link = ::std::option::Option::Some(Link::default());
                            }
                            self.link.as_mut().unwrap() } 
#[inline] pub fn take_link(&mut self) -> Link { self.link.take().unwrap_or_else(Link::default) }
#[inline] pub fn clear_begin_cycles(&mut self) { self.begin_cycles = 0 }
#[inline] pub fn set_begin_cycles(&mut self, v: u64) { self.begin_cycles = v; }
#[inline] pub fn get_begin_cycles(&self) -> u64 { self.begin_cycles }
#[inline] pub fn clear_end_cycles(&mut self) { self.end_cycles = 0 }
#[inline] pub fn set_end_cycles(&mut self, v: u64) { self.end_cycles = v; }
#[inline] pub fn get_end_cycles(&self) -> u64 { self.end_cycles }
#[inline] pub fn clear_event(&mut self) { self.event = 0 }
#[inline] pub fn set_event(&mut self, v: u32) { self.event = v; }
#[inline] pub fn get_event(&self) -> u32 { self.event }
}
impl ::protobuf::Clear for Span {fn clear(&mut self) { ::prost::Message::clear(self); }
}
impl ::protobuf::Message for Span {fn compute_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn get_cached_size(&self) -> u32 { ::prost::Message::encoded_len(self) as u32 }
fn as_any(&self) -> &dyn ::std::any::Any { self as &dyn ::std::any::Any }
fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor { Self::descriptor_static() }
fn new() -> Self { Self::default() }
fn default_instance() -> &'static Span {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: Span = Span::default();
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
