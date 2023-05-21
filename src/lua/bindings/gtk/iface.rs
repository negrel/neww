#[macro_export]
macro_rules! add_gtk_orientable_fields {
    ($fields:ident) => {
        add_mapped_field_getter!($fields, orientation, orientation, Orientation);
    };
}
