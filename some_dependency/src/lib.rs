pub fn useful_api() {}

// private module!
mod dont_look_at_this {
    mod really_nobody_should_care_about_this {
        pub struct Pwned;
        use std::{cell, ops};
        impl AsRef<Pwned> for cell::Ref<'_, Box<[u8]>> {
            fn as_ref(&self) -> &Pwned {
                &Pwned
            }
        }
        impl ops::Deref for Pwned {
            type Target = [u8];

            fn deref(&self) -> &Self::Target {
                &[21, 42, 63]
            }
        }
    }
}
