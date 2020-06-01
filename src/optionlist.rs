use std::ffi;
use std::os::raw;

pub struct OptionList {
    inner: Option<Box<ffi::CStr>>,
}

const NULL_TERMINATED_STR: &str = "\0";

impl OptionList {
    // pub fn new<S: AsRef<str>>(s: S) -> Result<OptionList, PdfError> {
    //     let sref = s.as_ref();
    //     if sref.is_empty() {
    //         Ok(OptionList { inner: None })
    //     } else {
    //         Ok(OptionList {
    //             inner: Some(ffi::CString::new(sref)?.into_boxed_c_str()),
    //         })
    //     }
    // }

    pub fn as_ptr(&self) -> *const raw::c_char {
        self.inner
            .as_ref()
            .map(|inner| inner.as_ptr())
            .unwrap_or(NULL_TERMINATED_STR.as_ptr() as _)
    }
}

impl<T> From<T> for OptionList
where
    T: AsRef<str>,
{
    fn from(o: T) -> Self {
        let sref = o.as_ref();
        if sref.is_empty() {
            OptionList { inner: None }
        } else {
            OptionList {
                inner: ffi::CString::new(sref)
                    .map(ffi::CString::into_boxed_c_str)
                    .ok(),
            }
        }
    }
}
