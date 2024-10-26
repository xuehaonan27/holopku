// This file is @generated by prost-build.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FoodPost {
    #[prost(message, optional, tag = "1")]
    pub post: ::core::option::Option<super::post::Post>,
    #[prost(enumeration = "Place", tag = "2")]
    pub place: i32,
    #[prost(int32, tag = "3")]
    pub score: i32,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Place {
    JiaYuan = 0,
    YiYuan = 1,
    ShaoYuan = 2,
    YanNan = 3,
    NongYuan = 4,
    XueYi = 5,
    XueWu = 6,
    Other = 7,
}
impl Place {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Place::JiaYuan => "JiaYuan",
            Place::YiYuan => "YiYuan",
            Place::ShaoYuan => "ShaoYuan",
            Place::YanNan => "YanNan",
            Place::NongYuan => "NongYuan",
            Place::XueYi => "XueYi",
            Place::XueWu => "XueWu",
            Place::Other => "Other",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "JiaYuan" => Some(Self::JiaYuan),
            "YiYuan" => Some(Self::YiYuan),
            "ShaoYuan" => Some(Self::ShaoYuan),
            "YanNan" => Some(Self::YanNan),
            "NongYuan" => Some(Self::NongYuan),
            "XueYi" => Some(Self::XueYi),
            "XueWu" => Some(Self::XueWu),
            "Other" => Some(Self::Other),
            _ => None,
        }
    }
}
