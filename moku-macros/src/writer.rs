use syn::{Item, ItemMod};

use crate::metadata::Metadata;

/// Given a fully formed and validated Metadata, write the auto-generated elements of a
/// state_machine to a module.
pub fn write_state_machine(metadata: Metadata, module: ItemMod) -> ItemMod {
    let mut writer = Writer { metadata, module };
    writer.write()
}

struct Writer {
    metadata: Metadata,
    module: ItemMod,
}

impl Writer {
    fn write(self) -> ItemMod {
        self.module
    }

    fn write_state_enter(&mut self) {
        for item in &self.module.content.as_ref().unwrap().1 {
            if let Item::Impl(imp) = item {}
        }
    }
}
