use std::time::Duration;

use crate::Result;

pub struct Clipboard {
    handle: x11_clipboard::Clipboard,
}

impl Clipboard {
    pub fn get(&self) -> Result<String> {
        let text = self.handle.load(
            self.handle.getter.atoms.clipboard,
            self.handle.getter.atoms.utf8_string,
            self.handle.getter.atoms.property,
            Duration::from_secs(3),
        )?;

        Ok(String::from_utf8(text)?)
    }

    pub fn set(&self, text: impl AsRef<str>) -> Result<()> {
        self.handle.store(
            self.handle.setter.atoms.clipboard,
            self.handle.setter.atoms.utf8_string,
            text.as_ref(),
        )?;
        Ok(())
    }
}

impl Default for Clipboard {
    fn default() -> Self {
        Self {
            handle: x11_clipboard::Clipboard::new().expect("unable to access clipboard"),
        }
    }
}
