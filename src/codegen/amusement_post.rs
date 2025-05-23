// This file is @generated by prost-build.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AmusementPost {
    #[prost(message, optional, tag = "1")]
    pub post: ::core::option::Option<super::post::Post>,
    #[prost(int32, tag = "2")]
    pub people_all: i32,
    #[prost(int32, tag = "3")]
    pub people_already: i32,
    #[prost(enumeration = "GameType", tag = "4")]
    pub game_type: i32,
    #[prost(int64, tag = "5")]
    pub start_time: i64,
    #[prost(string, tag = "6")]
    pub amuse_place: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub contact: ::prost::alloc::string::String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum GameType {
    WolfKill = 0,
    JvBen = 1,
    BloodTower = 2,
    Karaok = 3,
    BoardGame = 4,
    Sports = 5,
    Riding = 6,
    Other = 7,
}
impl GameType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::WolfKill => "WolfKill",
            Self::JvBen => "JvBen",
            Self::BloodTower => "BloodTower",
            Self::Karaok => "Karaok",
            Self::BoardGame => "BoardGame",
            Self::Sports => "Sports",
            Self::Riding => "Riding",
            Self::Other => "Other",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "WolfKill" => Some(Self::WolfKill),
            "JvBen" => Some(Self::JvBen),
            "BloodTower" => Some(Self::BloodTower),
            "Karaok" => Some(Self::Karaok),
            "BoardGame" => Some(Self::BoardGame),
            "Sports" => Some(Self::Sports),
            "Riding" => Some(Self::Riding),
            "Other" => Some(Self::Other),
            _ => None,
        }
    }
}
