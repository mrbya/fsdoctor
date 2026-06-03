/// Hash algorithm used by `FSDoctor`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HashAlgorithm {
    /// BLAKE3 hash.
    Blake3,
}

impl HashAlgorithm {
    /// Stable db/display name.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Blake3 => "blake3",
        }
    }
}

/// Digest of a regular file.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FileDigest {
    /// Hash algorithm.
    pub algorithm: HashAlgorithm,

    /// Raw digest bytes.
    pub bytes: [u8; 32],
}

impl FileDigest {
    /// Creates a BLAKE3 file digest.
    #[must_use]
    pub const fn blake3(bytes: [u8; 32]) -> Self {
        Self {
            algorithm: HashAlgorithm::Blake3,
            bytes,
        }
    }

    /// Returns lowercase hexadecimal hash representation.
    ///
    /// This is intended for display/export only.
    /// Internal storage should use `bytes`.
    #[must_use]
    pub fn to_hex(self) -> String {
        let mut hex = String::with_capacity(64);

        for byte in self.bytes {
            use std::fmt::Write as _;

            let _ = write!(&mut hex, "{byte:02x}");
        }

        hex
    }
}
