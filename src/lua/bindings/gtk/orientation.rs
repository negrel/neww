use crate::bind_c_enum;

bind_c_enum!(::gtk::Orientation as Orientation with variants {
    Horizontal as "horizontal",
    Vertical as "vertical",
});
