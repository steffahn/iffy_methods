pub fn useful_api() {}

// private module!
mod dont_look_at_this {
    mod really_nobody_should_care_about_this {
        pub struct Pwned;
        use std::{cell, fmt};
        impl fmt::Debug for Pwned {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("[21, 42, 63]")
            }
        }
        impl AsRef<Pwned> for cell::Ref<'_, Box<[u8]>> {
            fn as_ref(&self) -> &Pwned {
                &Pwned
            }
        }
    }
}
