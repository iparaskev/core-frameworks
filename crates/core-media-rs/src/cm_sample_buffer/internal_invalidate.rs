use core_foundation::base::TCFType;

use crate::cm_sample_buffer::{internal_base::CMSampleBuffer, CMSampleBufferRef};

impl CMSampleBuffer {
    pub(super) fn internal_is_valid(&self) -> bool {
        extern "C" {
            pub fn CMSampleBufferIsValid(sampleBuffer: CMSampleBufferRef) -> bool;
        }
        unsafe { CMSampleBufferIsValid(self.as_concrete_TypeRef()) }
    }
}
