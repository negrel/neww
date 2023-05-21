use gtk::traits::{BoxExt, OrientableExt};

use mlua::UserData;

use super::orientation::Orientation;
use crate::{
    add_field_getter, add_field_setter, add_gtk_orientable_fields, add_mapped_field_getter,
    add_upcast_methods, lua::bindings::gtk::Widget,
};

#[derive(Debug, Clone, Default)]
pub struct Box(pub gtk::Box);

impl UserData for Box {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        add_gtk_orientable_fields!(fields);

        add_field_getter!(fields, spacing, spacing);
        add_field_setter!(fields, spacing, set_spacing);
    }

    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        add_upcast_methods!(methods, Widget);

        methods.add_method("append", |_vm, this, child: Widget| {
            this.0.append(&child.0);
            Ok(())
        });
        methods.add_method(
            "insert_child_after",
            |_vm, this, (child, sibling): (Widget, Option<Widget>)| {
                let sibling = sibling.as_ref().map(|sib| &sib.0);
                this.0.insert_child_after(&child.0, sibling);
                Ok(())
            },
        );
        methods.add_method("prepend", |_vm, this, child: Widget| {
            this.0.prepend(&child.0);
            Ok(())
        });
        methods.add_method("remove", |_vm, this, child: Widget| {
            this.0.remove(&child.0);
            Ok(())
        });
    }
}
