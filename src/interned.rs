//! Interned strings

use std::fmt::Display;

#[cfg(feature = "arcintern")]
type Inner = internment::ArcIntern<String>;
#[cfg(not(feature = "arcintern"))]
type Inner = internment::Intern<String>;

/// An interned string.
///
/// `InternedString` is backed by the [internment] crate: either by an `Intern`
/// or an `ArcIntern` depending on what features are configured. By default, it
/// uses an `Intern`, which leaks its allocations. This is usually fine, because
/// Pandoc filters generally process one document and then exit. If leaking is
/// not acceptable, enable the `arcintern` feature to cause `InternedString`s to
/// be backed by `ArcIntern`, which adds some overhead but prevents memory
/// leaks.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InternedString {
    inner: Inner,
}

impl AsRef<str> for InternedString {
    fn as_ref(&self) -> &str {
        self.inner.as_str()
    }
}

impl std::ops::Deref for InternedString {
    type Target = str;
    fn deref(&self) -> &str {
        self.as_ref()
    }
}

impl Display for InternedString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

impl From<InternedString> for String {
    fn from(v: InternedString) -> Self {
        v.as_ref().into()
    }
}

impl From<String> for InternedString {
    fn from(value: String) -> Self {
        InternedString {
            inner: Inner::new(value),
        }
    }
}

impl From<&str> for InternedString {
    fn from(value: &str) -> Self {
        InternedString {
            inner: Inner::from_ref(value),
        }
    }
}

impl serde::Serialize for InternedString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let str: &str = self.as_ref();
        str.serialize(serializer)
    }
}

impl<'de> serde::Deserialize<'de> for InternedString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct StrVisitor;

        impl<'de> serde::de::Visitor<'de> for StrVisitor {
            type Value = InternedString;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a string")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(InternedString::from(v))
            }
        }

        deserializer.deserialize_str(StrVisitor)
    }
}
