use anyhow::anyhow;
use mlua::{FromLua, ToLua};

use crate::bind_c_enum;

bind_c_enum!(::gtk::pango::WrapMode as WrapMode with variants {
    Word as "word",
    Char as "char",
    WordChar as "word_char",
});
