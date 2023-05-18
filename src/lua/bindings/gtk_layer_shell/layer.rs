use crate::bind_c_enum;

bind_c_enum!(::gtk4_layer_shell::Layer as Layer with variants {
    Background as "background",
    Bottom as "bottom",
    Top as "top",
    Overlay as "overlay",
    EntryNumber as "entry_number",
});
