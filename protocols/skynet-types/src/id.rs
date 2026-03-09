use serde::{Deserialize, Serialize};
use std::fmt;

/// 全局唯一身份标识，用于跨子网唯一标识用户
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AuthId([u8; 32]);

/// 用户在子网内的本地标识，用于隐私保护
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UserId([u8; 32]);

/// 子网唯一标识
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SubnetId([u8; 32]);

/// 频道/群组唯一标识
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ChannelId([u8; 32]);

/// 消息唯一标识
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MessageId([u8; 32]);

/// 资源唯一标识
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ResourceId([u8; 32]);

macro_rules! impl_id_type {
    ($name:ident, $doc:expr) => {
        impl $name {
            /// 从字节数组创建标识
            pub fn from_bytes(bytes: [u8; 32]) -> Self {
                Self(bytes)
            }

            /// 从任意字节数据使用 blake3 哈希创建标识
            pub fn hash<T: AsRef<[u8]>>(data: T) -> Self {
                let hash = blake3::hash(data.as_ref());
                Self(hash.into())
            }

            /// 获取标识的字节数组引用
            pub fn as_bytes(&self) -> &[u8; 32] {
                &self.0
            }

            /// 转换为字节数组
            pub fn into_bytes(self) -> [u8; 32] {
                self.0
            }

            /// 转换为十六进制字符串
            pub fn to_hex(&self) -> String {
                hex::encode(self.0)
            }

            /// 从十六进制字符串创建标识
            pub fn from_hex(hex_str: &str) -> Result<Self, hex::FromHexError> {
                let mut bytes = [0u8; 32];
                hex::decode_to_slice(hex_str, &mut bytes)?;
                Ok(Self(bytes))
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.to_hex())
            }
        }

        impl AsRef<[u8]> for $name {
            fn as_ref(&self) -> &[u8] {
                &self.0
            }
        }
    };
}

impl_id_type!(AuthId, "全局唯一身份标识，用于跨子网唯一标识用户");
impl_id_type!(UserId, "用户在子网内的本地标识，用于隐私保护");
impl_id_type!(SubnetId, "子网唯一标识");
impl_id_type!(ChannelId, "频道/群组唯一标识");
impl_id_type!(MessageId, "消息唯一标识");
impl_id_type!(ResourceId, "资源唯一标识");

impl SubnetId {
    /// 创建主网标识（全零）
    pub fn mainnet() -> Self {
        Self([0u8; 32])
    }

    /// 检查是否为主网标识
    pub fn is_mainnet(&self) -> bool {
        self.0 == [0u8; 32]
    }
}

impl Default for SubnetId {
    fn default() -> Self {
        Self::mainnet()
    }
}
