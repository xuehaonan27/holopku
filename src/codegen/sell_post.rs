// This file is @generated by prost-build.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SellPost {
    #[prost(message, optional, tag = "1")]
    pub post: ::core::option::Option<super::post::Post>,
    #[prost(string, optional, tag = "2")]
    pub contact: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, tag = "3")]
    pub price: i32,
    #[prost(enumeration = "GoodsType", tag = "4")]
    pub goods_type: i32,
    #[prost(bool, tag = "5")]
    pub sold: bool,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum GoodsType {
    Ticket = 0,
    Book = 1,
    Display = 2,
    Computer = 3,
    Other = 4,
}
impl GoodsType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Ticket => "Ticket",
            Self::Book => "Book",
            Self::Display => "Display",
            Self::Computer => "Computer",
            Self::Other => "Other",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Ticket" => Some(Self::Ticket),
            "Book" => Some(Self::Book),
            "Display" => Some(Self::Display),
            "Computer" => Some(Self::Computer),
            "Other" => Some(Self::Other),
            _ => None,
        }
    }
}
