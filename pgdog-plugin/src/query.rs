use crate::bindings::{Parameter, Query};

use std::alloc::{alloc, dealloc, Layout};
use std::ffi::{CStr, CString};
use std::ptr::{copy, null};

impl Query {
    /// Get query text.
    pub fn query(&self) -> &str {
        debug_assert!(self.query != null());
        unsafe { CStr::from_ptr(self.query) }.to_str().unwrap()
    }

    /// Create new query to pass it over the FFI boundary.
    pub fn new(query: CString) -> Self {
        Self {
            len: query.as_bytes().len() as i32,
            query: query.into_raw(),
            num_parameters: 0,
            parameters: null(),
        }
    }

    /// Add parameters.
    pub fn parameters(&mut self, params: &[Parameter]) {
        let layout = Layout::array::<Parameter>(params.len()).unwrap();
        let parameters = unsafe { alloc(layout) };

        unsafe {
            copy(params.as_ptr(), parameters as *mut Parameter, params.len());
        }
        self.parameters = parameters as *const Parameter;
        self.num_parameters = params.len() as i32;
    }

    /// Get parameter at offset if one exists.
    pub fn parameter(&self, index: usize) -> Option<Parameter> {
        if index < self.num_parameters as usize {
            unsafe { Some(*(self.parameters.offset(index as isize))) }
        } else {
            None
        }
    }

    /// Free memory allocated for parameters, if any.
    ///
    /// SAFETY: This is not to be used by plugins.
    /// This is for internal pgDog usage only.
    pub unsafe fn drop(&mut self) {
        if !self.parameters.is_null() {
            for index in 0..self.num_parameters {
                if let Some(mut param) = self.parameter(index as usize) {
                    param.drop();
                }
            }
            let layout = Layout::array::<Parameter>(self.num_parameters as usize).unwrap();
            unsafe {
                dealloc(self.parameters as *mut u8, layout);
                self.parameters = null();
            }
        }
    }
}
