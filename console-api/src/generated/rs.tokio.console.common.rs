/// Unique identifier for each task.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Id {
    /// The unique identifier's concrete value.
    #[prost(uint64, tag="1")]
    pub id: u64,
}
/// A Rust source code location.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Location {
    /// The file path
    #[prost(string, optional, tag="1")]
    pub file: ::core::option::Option<::prost::alloc::string::String>,
    /// The Rust module path
    #[prost(string, optional, tag="2")]
    pub module_path: ::core::option::Option<::prost::alloc::string::String>,
    /// The line number in the source code file.
    #[prost(uint32, optional, tag="3")]
    pub line: ::core::option::Option<u32>,
    /// The character in `line`.
    #[prost(uint32, optional, tag="4")]
    pub column: ::core::option::Option<u32>,
}
/// Unique identifier for metadata.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetaId {
    /// The unique identifier's concrete value.
    #[prost(uint64, tag="1")]
    pub id: u64,
}
/// Unique identifier for spans.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpanId {
    /// The unique identifier's concrete value.
    #[prost(uint64, tag="1")]
    pub id: u64,
}
/// A message representing a key-value pair of data associated with a `Span`
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Field {
    /// Metadata for the task span that the field came from.
    #[prost(message, optional, tag="8")]
    pub metadata_id: ::core::option::Option<MetaId>,
    /// The key of the key-value pair.
    ///
    /// This is either represented as a string, or as an index into a `Metadata`'s 
    /// array of field name strings.
    #[prost(oneof="field::Name", tags="1, 2")]
    pub name: ::core::option::Option<field::Name>,
    /// The value of the key-value pair.
    #[prost(oneof="field::Value", tags="3, 4, 5, 6, 7")]
    pub value: ::core::option::Option<field::Value>,
}
/// Nested message and enum types in `Field`.
pub mod field {
    /// The key of the key-value pair.
    ///
    /// This is either represented as a string, or as an index into a `Metadata`'s 
    /// array of field name strings.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Name {
        /// The string representation of the name.
        #[prost(string, tag="1")]
        StrName(::prost::alloc::string::String),
        /// An index position into the `Metadata.field_names` of the metadata
        /// for the task span that the field came from.
        #[prost(uint64, tag="2")]
        NameIdx(u64),
    }
    /// The value of the key-value pair.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        /// A value serialized to a string using `fmt::Debug`.
        #[prost(string, tag="3")]
        DebugVal(::prost::alloc::string::String),
        /// A string value.
        #[prost(string, tag="4")]
        StrVal(::prost::alloc::string::String),
        /// An unsigned integer value.
        #[prost(uint64, tag="5")]
        U64Val(u64),
        /// A signed integer value.
        #[prost(sint64, tag="6")]
        I64Val(i64),
        /// A boolean value.
        #[prost(bool, tag="7")]
        BoolVal(bool),
    }
}
/// Represents a period of time in which a program was executing in a particular context.
///
/// Corresponds to `Span` in the `tracing` crate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Span {
    /// An Id that uniquely identifies it in relation to other spans.
    #[prost(message, optional, tag="1")]
    pub id: ::core::option::Option<SpanId>,
    /// Identifier for metadata describing static characteristics of all spans originating
    /// from that callsite, such as its name, source code location, verbosity level, and
    /// the names of its fields.
    #[prost(message, optional, tag="2")]
    pub metadata_id: ::core::option::Option<MetaId>,
    /// User-defined key-value pairs of arbitrary data that describe the context the span represents,
    #[prost(message, repeated, tag="3")]
    pub fields: ::prost::alloc::vec::Vec<Field>,
    /// Timestamp for the span.
    #[prost(message, optional, tag="4")]
    pub at: ::core::option::Option<::prost_types::Timestamp>,
}
/// Any new metadata that was registered since the last update.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterMetadata {
    /// The new metadata that was registered since the last update.
    #[prost(message, repeated, tag="1")]
    pub metadata: ::prost::alloc::vec::Vec<register_metadata::NewMetadata>,
}
/// Nested message and enum types in `RegisterMetadata`.
pub mod register_metadata {
    /// One metadata element registered since the last update.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct NewMetadata {
        /// Unique identifier for `metadata`.
        #[prost(message, optional, tag="1")]
        pub id: ::core::option::Option<super::MetaId>,
        /// The metadata payload.
        #[prost(message, optional, tag="2")]
        pub metadata: ::core::option::Option<super::Metadata>,
    }
}
/// Metadata associated with a span or event.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Metadata {
    /// The name of the span or event.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Describes the part of the system where the span or event that this
    /// metadata describes occurred.
    #[prost(string, tag="2")]
    pub target: ::prost::alloc::string::String,
    /// The path to the Rust module where the span occurred.
    #[prost(string, tag="3")]
    pub module_path: ::prost::alloc::string::String,
    /// The Rust source location associated with the span or event.
    #[prost(message, optional, tag="4")]
    pub location: ::core::option::Option<Location>,
    /// Indicates whether metadata is associated with a span or with an event.
    #[prost(enumeration="metadata::Kind", tag="5")]
    pub kind: i32,
    /// Describes the level of verbosity of a span or event.
    #[prost(enumeration="metadata::Level", tag="6")]
    pub level: i32,
    /// The names of the key-value fields attached to the
    /// span or event this metadata is associated with.
    #[prost(string, repeated, tag="7")]
    pub field_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Nested message and enum types in `Metadata`.
pub mod metadata {
    /// Indicates whether metadata is associated with a span or with an event.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Kind {
        /// Indicates metadata is associated with a span.
        Span = 0,
        /// Indicates metadata is associated with an event.
        Event = 1,
    }
    impl Kind {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Kind::Span => "SPAN",
                Kind::Event => "EVENT",
            }
        }
    }
    /// Describes the level of verbosity of a span or event.
    ///
    /// Corresponds to `Level` in the `tracing` crate.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Level {
        /// The "error" level.
        ///
        /// Designates very serious errors.
        Error = 0,
        /// The "warn" level.
        ///
        /// Designates hazardous situations.
        Warn = 1,
        /// The "info" level.
        /// Designates useful information.
        Info = 2,
        /// The "debug" level.
        ///
        /// Designates lower priority information.
        Debug = 3,
        /// The "trace" level.
        ///
        /// Designates very low priority, often extremely verbose, information.
        Trace = 4,
    }
    impl Level {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Level::Error => "ERROR",
                Level::Warn => "WARN",
                Level::Info => "INFO",
                Level::Debug => "DEBUG",
                Level::Trace => "TRACE",
            }
        }
    }
}
/// Contains stats about objects that can be polled. Currently these can be:
/// - tasks that have been spawned
/// - async operations on resources that are performed within the context of a task
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PollStats {
    /// The total number of times this object has been polled.
    #[prost(uint64, tag="1")]
    pub polls: u64,
    /// The timestamp of the first time this object was polled.
    ///
    /// If this is `None`, the object has not yet been polled.
    ///
    /// Subtracting this timestamp from `created_at` can be used to calculate the
    /// time to first poll for this object, a measurement of executor latency.
    #[prost(message, optional, tag="3")]
    pub first_poll: ::core::option::Option<::prost_types::Timestamp>,
    /// The timestamp of the most recent time this objects's poll method was invoked.
    ///
    /// If this is `None`, the object has not yet been polled.
    ///
    /// If the object has only been polled a single time, then this value may be
    /// equal to the `first_poll` timestamp.
    ///
    #[prost(message, optional, tag="4")]
    pub last_poll_started: ::core::option::Option<::prost_types::Timestamp>,
    /// The timestamp of the most recent time this objects's poll method finished execution.
    ///
    /// If this is `None`, the object has not yet been polled or is currently being polled.
    ///
    /// If the object does not exist anymore, then this is the time the final invocation of
    /// its poll method has completed.
    #[prost(message, optional, tag="5")]
    pub last_poll_ended: ::core::option::Option<::prost_types::Timestamp>,
    /// The total duration this object was being *actively polled*, summed across
    /// all polls. Note that this includes only polls that have completed and is
    /// not reflecting any inprogress polls. Subtracting `busy_time` from the
    /// total lifetime of the polled object results in the amount of time it
    /// has spent *waiting* to be polled.
    #[prost(message, optional, tag="6")]
    pub busy_time: ::core::option::Option<::prost_types::Duration>,
}
/// State attributes of an entity. These are dependent on the type of the entity.
///
/// For example, a timer resource will have a duration, while a semaphore resource may
/// have a permit count. Likewise, the async ops of a semaphore may have attributes
/// indicating how many permits they are trying to acquire vs how many are acquired.
/// These values may change over time. Therefore, they live in the runtime stats rather
/// than the static data describing the entity.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Attribute {
    /// The key-value pair for the attribute
    #[prost(message, optional, tag="1")]
    pub field: ::core::option::Option<Field>,
    /// Some values carry a unit of measurement. For example, a duration
    /// carries an associated unit of time, such as "ms" for milliseconds.
    #[prost(string, optional, tag="2")]
    pub unit: ::core::option::Option<::prost::alloc::string::String>,
}
