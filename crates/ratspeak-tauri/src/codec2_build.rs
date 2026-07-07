//! Fork build label surfaced in About dialogs and `api_version`.

/// Label shown under the app version on Codec2-enabled builds.
pub const BUILD_LABEL: &str = "Codec2";

/// Version string for native About dialogs (macOS application version field).
pub fn about_version_line(base: &str) -> String {
    format!("{base}\n{BUILD_LABEL}")
}
