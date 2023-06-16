use crate::bind_c_enum;

bind_c_enum!(::gtk::Align as Align with variants {
    Fill as "fill",
    Start as "start",
    End as "end",
    Center as "center",
});
