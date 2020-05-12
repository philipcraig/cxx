#[repr(C)]
pub struct RustOption<T> {
    repr: Option<T>,
}

impl<T> RustOption<T> {
    pub fn new() -> Self {
        RustOption { repr: Option::None }
    }

    pub fn from(v: Option<T>) -> Self {
        RustOption { repr: v }
    }

    pub fn from_ref(v: &Option<T>) -> &Self {
        unsafe { &*(v as *const Option<T> as *const RustOption<T>) }
    }

    pub fn into_option(self) -> Option<T> {
        self.repr
    }

    pub fn as_option(&self) -> &Option<T> {
        &self.repr
    }

    pub fn as_mut_option(&mut self) -> &mut Option<T> {
        &mut self.repr
    }

    pub fn is_some(&self) -> bool {
        self.repr.is_some()
    }

    pub fn is_none(&self) -> bool {
        self.repr.is_none()
    }
}
