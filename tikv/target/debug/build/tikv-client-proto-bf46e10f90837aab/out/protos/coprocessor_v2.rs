#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RawCoprocessorRequest {
    #[prost(message, optional, tag="1")]
    pub context: ::core::option::Option<super::kvrpcpb::Context>,
    #[prost(string, tag="2")]
    pub copr_name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub copr_version_constraint: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="4")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RawCoprocessorResponse {
    #[prost(bytes="vec", tag="1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="2")]
    pub region_error: ::core::option::Option<super::errorpb::Error>,
    /// Error message for cases like if no coprocessor with a matching name is found
    /// or on a version mismatch between plugin_api and the coprocessor.
    #[prost(string, tag="4")]
    pub other_error: ::prost::alloc::string::String,
}
