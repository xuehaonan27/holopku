// This file is @generated by prost-build.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Post {
    #[prost(int32, tag = "1")]
    pub id: i32,
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    #[prost(int32, tag = "3")]
    pub user_id: i32,
    #[prost(string, tag = "4")]
    pub content: ::prost::alloc::string::String,
    #[prost(int32, tag = "6")]
    pub likes: i32,
    #[prost(int32, tag = "7")]
    pub favorates: i32,
    #[prost(int32, tag = "9")]
    pub created_at: i32,
    #[prost(int32, optional, tag = "10")]
    pub updated_at: ::core::option::Option<i32>,
    /// MANAGED BY FOREIGN KEYS
    #[prost(message, repeated, tag = "8")]
    pub comments: ::prost::alloc::vec::Vec<Comment>,
    /// MANAGED BY FOREIGN KEYS
    #[prost(string, repeated, tag = "5")]
    pub images: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(enumeration = "PostType", tag = "11")]
    pub post_type: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Comment {
    #[prost(int32, tag = "1")]
    pub id: i32,
    #[prost(int32, tag = "2")]
    pub post_id: i32,
    #[prost(int32, tag = "3")]
    pub user_id: i32,
    #[prost(string, tag = "4")]
    pub content: ::prost::alloc::string::String,
    #[prost(int32, tag = "5")]
    pub likes: i32,
    #[prost(int64, tag = "6")]
    pub created_at: i64,
    #[prost(int64, optional, tag = "7")]
    pub updated_at: ::core::option::Option<i64>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PostType {
    Foodpost = 0,
    Sellpost = 1,
    Amusementpost = 2,
}
impl PostType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PostType::Foodpost => "FOODPOST",
            PostType::Sellpost => "SELLPOST",
            PostType::Amusementpost => "AMUSEMENTPOST",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "FOODPOST" => Some(Self::Foodpost),
            "SELLPOST" => Some(Self::Sellpost),
            "AMUSEMENTPOST" => Some(Self::Amusementpost),
            _ => None,
        }
    }
}
