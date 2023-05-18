use crate::bind_c_enum;

bind_c_enum!(::gtk::Justification as Justification with variants {
    Left as "left",
    Right as "right",
    Center as "center",
    Fill as "fill",
});
