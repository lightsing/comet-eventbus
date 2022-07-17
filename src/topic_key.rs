use std::borrow::Cow;
use std::fmt::{Debug, Display, Formatter};
use std::ops::Deref;
use std::str::Utf8Error;

/// Wrapper of bytes represent a `Topic`
///
/// ## Example:
/// ```
/// use comet_eventbus::TopicKey;
///
/// // create topic from str literal
/// TopicKey::from("my awsome topic");
///
/// // crate topic from bytes literal
/// TopicKey::from(b"deafbeef");
///
/// // create topic from Vec<u8>
/// TopicKey::from(vec![0xde, 0xaf, 0xbe, 0xef]);
/// ```
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct TopicKey(Cow<'static, [u8]>);

impl TopicKey {
    /// try parse topic key as an utf-8 str
    pub fn try_as_str(&self) -> Result<&str, Utf8Error> {
        std::str::from_utf8(self.as_ref())
    }
}

impl AsRef<[u8]> for TopicKey {
    fn as_ref(&self) -> &[u8] {
        self.0.deref()
    }
}

impl Deref for TopicKey {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        self.0.deref()
    }
}

impl From<Vec<u8>> for TopicKey {
    fn from(value: Vec<u8>) -> Self {
        Self(Cow::from(value))
    }
}

impl From<&'static Vec<u8>> for TopicKey {
    fn from(value: &'static Vec<u8>) -> Self {
        Self(Cow::from(value))
    }
}

impl From<&'static [u8]> for TopicKey {
    fn from(value: &'static [u8]) -> Self {
        Self(Cow::from(value))
    }
}

impl From<&'static str> for TopicKey {
    fn from(value: &'static str) -> Self {
        Self(Cow::from(value.as_bytes()))
    }
}

impl Display for TopicKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            &self.try_as_str().unwrap_or(&hex::encode(self.as_ref()))
        )
    }
}

impl Debug for TopicKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("TopicKey")
            .field(&self.try_as_str().unwrap_or(&hex::encode(self.as_ref())))
            .finish()
    }
}
