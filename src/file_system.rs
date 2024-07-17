use wasm_bindgen::JsCast;
use web_sys::FileSystemFileHandle;

pub use errors::*;

mod errors {
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum FileSystemError {
        NotAFileSystemFileHandle,
        InvalidErrorType,
        JsError(FileSystemJsError),
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum FileSystemJsError {
        /// Thrown if none of the below errors matched on JS side.
        UnknownError,
        /// Thrown if this is an unknown error from file picker.
        ShowSaveFilePickerUnknown,
        /// Thrown if the user dismisses the file picker without selecting or inputting a file, or if
        /// the user agent deems any selected files too sensitive or dangerous.
        ShowSaveFilePickerAbort,
        /// Thrown if the call was blocked by the same-origin policy or it was not called via a user
        /// interaction such as a button press.
        ShowSaveFilePickerSecurity,
        /// Thrown if accept types can't be processed, which may happen if:
        ///
        /// - Any key string of the accept options of any item in types options can't parse a valid
        /// MIME type.
        /// - Any value string(s) of the accept options of any item in types options is invalid, for
        /// example, if it does not start with . and if end with ., or if it contains any invalid code
        /// points and its length is more than 16.
        /// - The types options is empty and the excludeAcceptAllOption options is true.
        ShowSaveFilePickerType,
        /// Thrown if this is an unknown error from create writable.
        CreateWritableUnknown,
        /// Thrown if the PermissionStatus.state for the handle is not 'granted' in readwrite mode.
        CreateWritableNotAllowed,
        /// Thrown if current entry is not found.
        CreateWritableNotFound,
        /// Thrown if the browser is not able to acquire a lock on the file associated with the file handle.
        CreateWritableNoModificationAllowed,
        /// Thrown if implementation-defined malware scans and safe-browsing checks fails.
        CreateWritableAbort,
        /// Thrown if this is an unknown error from write.
        WriteUnknown,
        /// Thrown if PermissionStatus.state is not granted.
        WriteNotAllowed,
        /// Thrown if the new size of the file is larger than the original size of the file, and exceeds
        /// the browser's storage quota.
        WriteQuotaExceeded,
        /// Thrown if data is undefined, or if position or size aren't valid.
        WriteType,
        /// Thrown if this is an unknown error from close.
        CloseUnknown,
        /// The stream you are trying to close is locked.
        CloseType,
    }

    impl std::error::Error for FileSystemError {}
    impl std::fmt::Display for FileSystemError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::NotAFileSystemFileHandle => write!(f, "Not a filesystem file handle"),
                Self::InvalidErrorType => write!(f, "Invalid error type"),
                Self::JsError(err) => write!(f, "JS Error: {}", err),
            }
        }
    }
    impl std::error::Error for FileSystemJsError {}
    impl std::fmt::Display for FileSystemJsError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::UnknownError => write!(f, "Unknown error"),
                Self::ShowSaveFilePickerUnknown => write!(f, "Show save file picker unknown error"),
                Self::ShowSaveFilePickerAbort => write!(f, "Show save file picker abort error"),
                Self::ShowSaveFilePickerSecurity => {
                    write!(f, "Show save file picker security error")
                }
                Self::ShowSaveFilePickerType => write!(f, "Show save file picker type error"),
                Self::CreateWritableUnknown => write!(f, "Show create writable unknown error"),
                Self::CreateWritableNotAllowed => write!(f, "Create writable not allowed error"),
                Self::CreateWritableNotFound => write!(f, "Create writable not found error"),
                Self::CreateWritableNoModificationAllowed => {
                    write!(f, "Create writable no modification allowed error")
                }
                Self::CreateWritableAbort => write!(f, "Create writable abort error"),
                Self::WriteUnknown => write!(f, "Write unknown error"),
                Self::WriteNotAllowed => write!(f, "Write not allowed error"),
                Self::WriteQuotaExceeded => write!(f, "Write quota exceeded error"),
                Self::WriteType => write!(f, "Write type error"),
                Self::CloseUnknown => write!(f, "Close unknown error"),
                Self::CloseType => write!(f, "Close type error"),
            }
        }
    }
}

mod inner {
    use wasm_bindgen::prelude::*;
    use web_sys::FileSystemFileHandle;

    #[wasm_bindgen(module = "/assets/file_system.js")]
    extern "C" {
        #[wasm_bindgen(catch)]
        pub async fn save_to_file(
            handle: Option<FileSystemFileHandle>,
            file_data: String,
        ) -> Result<JsValue, JsValue>;
    }
}

pub async fn save_to_file(
    handle: Option<FileSystemFileHandle>,
    file_data: String,
) -> Result<FileSystemFileHandle, FileSystemError> {
    let result = inner::save_to_file(handle, file_data).await;

    use FileSystemError as FSE;
    use FileSystemJsError as FSJE;

    match result {
        Ok(handle) => match handle.dyn_into::<FileSystemFileHandle>() {
            Ok(handle) => Ok(handle),
            Err(_) => Err(FSE::NotAFileSystemFileHandle),
        },
        Err(err) => match err.as_f64().map(|x| x as u32) {
            Some(0) => Err(FSE::JsError(FSJE::UnknownError)),
            Some(1) => Err(FSE::JsError(FSJE::ShowSaveFilePickerUnknown)),
            Some(2) => Err(FSE::JsError(FSJE::ShowSaveFilePickerAbort)),
            Some(3) => Err(FSE::JsError(FSJE::ShowSaveFilePickerSecurity)),
            Some(4) => Err(FSE::JsError(FSJE::ShowSaveFilePickerType)),
            Some(5) => Err(FSE::JsError(FSJE::CreateWritableUnknown)),
            Some(6) => Err(FSE::JsError(FSJE::CreateWritableNotAllowed)),
            Some(7) => Err(FSE::JsError(FSJE::CreateWritableNotAllowed)),
            Some(8) => Err(FSE::JsError(FSJE::CreateWritableNoModificationAllowed)),
            Some(9) => Err(FSE::JsError(FSJE::CreateWritableAbort)),
            Some(10) => Err(FSE::JsError(FSJE::WriteUnknown)),
            Some(11) => Err(FSE::JsError(FSJE::WriteNotAllowed)),
            Some(12) => Err(FSE::JsError(FSJE::WriteQuotaExceeded)),
            Some(13) => Err(FSE::JsError(FSJE::WriteType)),
            Some(14) => Err(FSE::JsError(FSJE::CloseUnknown)),
            Some(15) => Err(FSE::JsError(FSJE::CloseType)),
            // Explicit remaining case
            Some(_) => Err(FSE::InvalidErrorType),
            None => Err(FSE::InvalidErrorType),
        },
    }
}
