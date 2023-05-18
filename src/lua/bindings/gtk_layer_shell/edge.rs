use crate::bind_c_enum;

bind_c_enum!(::gtk4_layer_shell::Edge as Edge with variants {
    Left as "left",
    Right as "right",
    Bottom as "bottom",
    Top as "top",
});
