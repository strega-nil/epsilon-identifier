use std::{borrow, cmp, fmt, ops, hash};

use atomizer::AtomProxy;

use unicode_normalization::UnicodeNormalization;

use super::*;

// FORMATTING TRAIT IMPLS
impl fmt::Debug for NfcString {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    <str as fmt::Debug>::fmt(self.as_str(), f)
  }
}
impl fmt::Display for NfcString {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    <str as fmt::Display>::fmt(self.as_str(), f)
  }
}

// CONVERSION TRAIT IMPLS
impl borrow::Borrow<NfcCmpString> for NfcString {
  fn borrow(&self) -> &NfcCmpString {
    NfcCmpString::from_str(self.as_str())
  }
}

impl ops::Deref for NfcStringBuf {
  type Target = NfcString;

  fn deref(&self) -> &NfcString {
    unsafe { self.ptr.as_ref() }
  }
}

impl borrow::Borrow<NfcString> for NfcStringBuf {
  fn borrow(&self) -> &NfcString {
    &**self
  }
}
impl borrow::Borrow<NfcCmpString> for NfcStringBuf {
  fn borrow(&self) -> &NfcCmpString {
    NfcCmpString::from_str(self.as_str())
  }
}

impl borrow::Borrow<NfcCmpString> for str {
  fn borrow(&self) -> &NfcCmpString {
    NfcCmpString::from_str(self)
  }
}

impl ToOwned for NfcString {
  type Owned = NfcStringBuf;

  fn to_owned(&self) -> NfcStringBuf {
    NfcStringBuf::from_nfc_string(self)
  }
}

impl<'a> From<&'a NfcString> for NfcStringBuf {
  fn from(s: &'a NfcString) -> Self {
    Self::from_nfc_string(s)
  }
}
impl<'a> From<&'a str> for NfcStringBuf {
  fn from(s: &'a str) -> Self {
    Self::new(s)
  }
}

impl AtomProxy<NfcStringBuf> for str {
  type Compare = NfcCmpString;

  fn to_owned(&self) -> NfcStringBuf {
    NfcStringBuf::new(self)
  }
  fn to_compare(&self) -> &Self::Compare {
    NfcCmpString::from_str(self)
  }
}

// ORDERING TRAIT IMPLS

impl cmp::PartialEq for NfcCmpString {
  fn eq(&self, other: &Self) -> bool {
    self.0.nfc().eq(other.0.nfc())
  }
}
impl cmp::Eq for NfcCmpString {}

impl cmp::PartialOrd for NfcCmpString {
  fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
    Some(self.0.nfc().cmp(other.0.nfc()))
  }
}
impl cmp::Ord for NfcCmpString {
  fn cmp(&self, other: &Self) -> cmp::Ordering {
    self.0.nfc().cmp(other.0.nfc())
  }
}

impl cmp::PartialEq for NfcString {
  fn eq(&self, other: &Self) -> bool {
    self.as_str() == other.as_str()
  }
}
impl cmp::Eq for NfcString {}

impl cmp::PartialOrd for NfcString {
  fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
    Some(self.cmp(other))
  }
}
impl cmp::Ord for NfcString {
  fn cmp(&self, other: &Self) -> cmp::Ordering {
    self.as_str().cmp(other.as_str())
  }
}

impl cmp::PartialEq<str> for NfcString {
  fn eq(&self, other: &str) -> bool {
    self.as_str().chars().eq(other.nfc())
  }
}
impl cmp::PartialEq<NfcCmpString> for NfcString {
  fn eq(&self, other: &NfcCmpString) -> bool {
    self.as_str().chars().eq(other.0.nfc())
  }
}
impl cmp::PartialEq<NfcString> for str {
  fn eq(&self, other: &NfcString) -> bool {
    *other == *self
  }
}
impl cmp::PartialEq<NfcString> for NfcCmpString {
  fn eq(&self, other: &NfcString) -> bool {
    *other == *self
  }
}

impl cmp::PartialOrd<str> for NfcString {
  fn partial_cmp(&self, other: &str) -> Option<cmp::Ordering> {
    Some(self.as_str().chars().cmp(other.nfc()))
  }
}
impl cmp::PartialOrd<NfcString> for str {
  fn partial_cmp(&self, other: &NfcString) -> Option<cmp::Ordering> {
    other.partial_cmp(self)
  }
}
impl cmp::PartialOrd<NfcCmpString> for NfcString {
  fn partial_cmp(&self, other: &NfcCmpString) -> Option<cmp::Ordering> {
    Some(self.as_str().chars().cmp(other.0.nfc()))
  }
}
impl cmp::PartialOrd<NfcString> for NfcCmpString {
  fn partial_cmp(&self, other: &NfcString) -> Option<cmp::Ordering> {
    other.partial_cmp(self)
  }
}

impl cmp::PartialEq for NfcStringBuf {
  fn eq(&self, other: &Self) -> bool {
    (**self).eq(other)
  }
}
impl cmp::Eq for NfcStringBuf {}

impl cmp::PartialOrd for NfcStringBuf {
  fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
    (**self).partial_cmp(other)
  }
}
impl cmp::Ord for NfcStringBuf {
  fn cmp(&self, other: &Self) -> cmp::Ordering {
    (**self).cmp(&**other)
  }
}

impl cmp::PartialEq<NfcString> for NfcStringBuf {
  fn eq(&self, other: &NfcString) -> bool {
    (**self).eq(other)
  }
}
impl cmp::PartialEq<NfcStringBuf> for NfcString {
  fn eq(&self, other: &NfcStringBuf) -> bool {
    other.eq(self)
  }
}
impl cmp::PartialEq<str> for NfcStringBuf {
  fn eq(&self, other: &str) -> bool {
    (**self).eq(other)
  }
}
impl cmp::PartialEq<NfcStringBuf> for str {
  fn eq(&self, other: &NfcStringBuf) -> bool {
    other.eq(self)
  }
}
impl cmp::PartialEq<NfcCmpString> for NfcStringBuf {
  fn eq(&self, other: &NfcCmpString) -> bool {
    (**self).eq(other)
  }
}
impl cmp::PartialEq<NfcStringBuf> for NfcCmpString {
  fn eq(&self, other: &NfcStringBuf) -> bool {
    other.eq(self)
  }
}

impl cmp::PartialOrd<NfcString> for NfcStringBuf {
  fn partial_cmp(&self, other: &NfcString) -> Option<cmp::Ordering> {
    (**self).partial_cmp(other)
  }
}
impl cmp::PartialOrd<NfcStringBuf> for NfcString {
  fn partial_cmp(&self, other: &NfcStringBuf) -> Option<cmp::Ordering> {
    other.partial_cmp(self)
  }
}
impl cmp::PartialOrd<str> for NfcStringBuf {
  fn partial_cmp(&self, other: &str) -> Option<cmp::Ordering> {
    (**self).partial_cmp(other)
  }
}
impl cmp::PartialOrd<NfcStringBuf> for str {
  fn partial_cmp(&self, other: &NfcStringBuf) -> Option<cmp::Ordering> {
    other.partial_cmp(self)
  }
}
impl cmp::PartialOrd<NfcCmpString> for NfcStringBuf {
  fn partial_cmp(&self, other: &NfcCmpString) -> Option<cmp::Ordering> {
    (**self).partial_cmp(other)
  }
}
impl cmp::PartialOrd<NfcStringBuf> for NfcCmpString {
  fn partial_cmp(&self, other: &NfcStringBuf) -> Option<cmp::Ordering> {
    other.partial_cmp(self)
  }
}

// HASH IMPLS

// We implement hashing this way so that it's guaranteed stable for us,
// and so that we can NFC-encode str as we write.
// We write a 0xFF at the end because it doesn't appear in utf-8, so that
// hash(("abc", "def")) ≠ hash(("abcdef", ""))

impl hash::Hash for NfcString {
  fn hash<H: hash::Hasher>(&self, h: &mut H) {
    let mut buff = [0u8; 4];
    for ch in self.as_str().chars() {
      h.write(ch.encode_utf8(&mut buff).as_bytes());
    }
    h.write_u8(0xFF);
  }
}

impl hash::Hash for NfcStringBuf {
  fn hash<H: hash::Hasher>(&self, h: &mut H) {
    (**self).hash(h)
  }
}

impl hash::Hash for NfcCmpString {
  fn hash<H: hash::Hasher>(&self, h: &mut H) {
    let mut buff = [0u8; 4];
    for ch in self.0.nfc() {
      h.write(ch.encode_utf8(&mut buff).as_bytes());
    }
    h.write_u8(0xFF);
  }
}
