use crate::LibError;

impl Default for LibError {
    fn default() -> Self {
        Self::Other("Undefined".into())
    }
}

unsafe impl Send for LibError {}
unsafe impl Sync for LibError {}

