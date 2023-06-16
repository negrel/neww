#[macro_export]
macro_rules! add_method_no_args_no_return {
    ($methods:ident, $method_name:ident) => {
        $methods.add_method(stringify!($method_name), |_vm, this, ()| {
            this.0.$method_name();
            Ok(())
        });
    };
}

#[macro_export]
macro_rules! add_field_getter {
    ($fields:ident, $field_name:ident, $getter_name:ident) => {
        $fields.add_field_method_get(stringify!($field_name), |_vm, this| {
            Ok(this.0.$getter_name())
        });
    };
}

#[macro_export]
macro_rules! add_mapped_field_getter {
    ($fields:ident, $field_name:ident, $getter_name:ident, $convert:expr) => {
        $fields.add_field_method_get(stringify!($field_name), |_vm, this| {
            #[allow(clippy::redundant_closure_call)]
            Ok($convert(this.0.$getter_name()))
        });
    };
}

#[macro_export]
macro_rules! add_field_setter {
    ($fields:ident, $field_name:ident, $setter_name:ident) => {
        $fields.add_field_method_set(stringify!($field_name), |_vm, this, value| {
            this.0.$setter_name(value);
            Ok(())
        })
    };
}

#[macro_export]
macro_rules! add_mapped_field_setter {
    ($fields:ident, $field_name:ident, $setter_name:ident, $mapped_type:ident) => {
        $fields.add_field_method_set(stringify!($field_name), |vm, this, value| {
            use ::mlua::FromLua;
            this.0.$setter_name(&$mapped_type::from_lua(value, vm)?.0);
            Ok(())
        })
    };
}

#[macro_export]
macro_rules! add_child_accessors {
    ($fields:ident) => {
        add_mapped_field_getter!($fields, child, child, |widget: Option<gtk::Widget>| {
            widget.map(Widget)
        });
        $fields.add_field_method_set("child", |_vm, this, widget: Option<Widget>| {
            match widget {
                Some(w) => this.0.set_child(Some(&w.0)),
                None => this.0.set_child(None::<&gtk::Widget>),
            };
            Ok(())
        });
    };
}

#[macro_export]
macro_rules! add_downcast_method {
    ($methods:ident, $downcast_type:ident) => {
        $methods.add_method(
            &concat!("as_", stringify!($downcast_type)).to_lowercase(),
            |_vm, this, ()| {
                use ::gtk::prelude::Cast;
                let obj = this
                    .0
                    .clone()
                    .downcast::<::gtk::$downcast_type>()
                    .expect(concat!(
                        "Failed to downcast to ",
                        stringify!($downcast_type)
                    ));
                Ok($downcast_type(obj))
            },
        )
    };
}

#[macro_export]
macro_rules! add_upcast_methods {
    ($methods:ident $(, $upcast_type:ident)+) => {
        $(
            $methods.add_method(
                // e.g. "as_widget" for Widget upcast_type.
                &concat!("as_", stringify!($upcast_type)).to_lowercase(),
                |_vm, this, ()| {
                    use ::gtk::prelude::Cast;
                    let obj = this.0.clone().upcast::<gtk::$upcast_type>();
                    Ok($upcast_type(obj))
                },
            );
        )+

        // Add meta method so as_xxx is optional for upcast.
        $methods.add_meta_method(::mlua::MetaMethod::Index, |vm, this, key: String| {
            use ::mlua::chunk;
            let this = this.to_owned();
            let upcasts = vec![$(
                // e.g. "as_widget" for Widget upcast_type.
                concat!("as_", stringify!($upcast_type)).to_lowercase(),
            )*];
            let result = vm.load(chunk! {
                for _, upcast in pairs($upcasts) do
                    local ok, result = pcall(function()
                        return $this[upcast]($this)[$key]
                    end)
                    if ok then
                        return result
                    end
                end
                return nil
            }).eval::<::mlua::Value>()?;
            Ok(result)
        });

        // Add meta method so as_xxx is optional for upcast.
        $methods.add_meta_method(::mlua::MetaMethod::NewIndex, |vm, this, (key, value): (String, mlua::Value)| {
            use ::mlua::chunk;
            let this = this.to_owned();
            let upcasts = vec![$(
                // e.g. "as_widget" for Widget upcast_type.
                concat!("as_", stringify!($upcast_type)).to_lowercase(),
            )*];
            let result = vm.load(chunk! {
                for _, upcast in pairs($upcasts) do
                    local ok = pcall(function()
                        $this[upcast]($this)[$key] = $value
                    end)
                end
            }).eval::<::mlua::Value>()?;
            Ok(result)
        });
    };
}

#[macro_export]
macro_rules! add_connect_methods {
    ($methods:ident $(, $signal_name:literal as fn($args:ty) -> $returns:ty)+ ) => {
        $methods.add_method(
            "connect",
            |vm, this, (signal, func): (String, ::mlua::Function)| {
                let this = this.to_owned();
                vm.load(::mlua::chunk! {
                    local key = "connect_" .. $signal
                    // kebab-case to snake_case
                    key = string.gsub(key, "-", "_")
                    $this[key]($this, $func)
                })
                .exec()?;
                Ok(())
            },
        );

        $(
            $methods.add_method(
                // kebab-case to snake_case
                &concat!("connect_", $signal_name).replace("-", "_"),
                |_vm, this, func: ::mlua::Function| {
                    unsafe {
                        // Change function lifetime to static.
                        // This is safe because our lua VM is also static.
                        let func = ::std::mem::transmute::<
                            ::mlua::Function<'lua>,
                            ::mlua::Function<'static>,
                        >(func);
                        this.0.connect_closure(
                            $signal_name,
                            false,
                            ::gtk::glib::closure_local!(|this| {
                                let return_value = func
                                    .call::<$args, $returns>(Self(this))
                                    .expect("lua signal handler returned an unexpected error");
                                return_value
                            }),
                        );
                    }
                    Ok(())
                },
            );
        )+
    };
}

#[macro_export]
macro_rules! bind_c_enum {
    ($module:path as $type:ident with variants { $($variant:ident as $variant_str:literal ,)+ }) => {
        #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
        pub struct $type(pub $module);

        impl ToString for $type {
            fn to_string(&self) -> String {
                match self.0 {
                    $(
                        <$module>::$variant => $variant_str,
                    )+
                    _ => unreachable!(),
                }
                .to_owned()
            }
        }

        impl TryFrom<&str> for $type {
            type Error = ::anyhow::Error;

            fn try_from(value: &str) -> Result<Self, Self::Error> {
                let err_str = concat!("$type must be one of" $(, "\"", $variant_str, "\"")+, ", got {value:?}");

                match value {
                    $(
                        $variant_str => Ok(Self(<$module>::$variant)),
                    )+
                    _ => Err(anyhow::anyhow!(err_str)),
                }
            }
        }

        impl<'lua> ::mlua::ToLua<'lua> for $type {
            fn to_lua(self, vm: &'lua ::mlua::Lua) -> ::mlua::Result<mlua::Value<'lua>> {
                Ok(::mlua::Value::String(vm.create_string(&self.to_string())?))
            }
        }

        impl<'lua> ::mlua::FromLua<'lua> for $type {
            fn from_lua(lua_value: ::mlua::Value<'lua>, _vm: &'lua ::mlua::Lua) -> ::mlua::Result<Self> {
                let variant = match lua_value {
                    ::mlua::Value::String(str) => str,
                    _ => {
                        return Err(::mlua::Error::FromLuaConversionError {
                            from: lua_value.type_name(),
                            to: stringify!(<$module>),
                            message: Some("must be of type string".to_owned()),
                        })
                    }
                };

                let variant = variant
                    .to_str()
                    .map_err(|err| ::mlua::Error::FromLuaConversionError {
                        from: "string",
                        to: stringify!(<$module>),
                        message: Some(err.to_string()),
                    })?;

                $type::try_from(variant).map_err(|err| ::mlua::Error::FromLuaConversionError {
                    from: "string",
                    to: stringify!(<$module>),
                    message: Some(err.to_string()),
                })
            }
        }
        #[cfg(test)]
        mod test {
            use ::claims::assert_ok;

            use super::$type;

            #[test]
            fn from_string_to_string() {
                $(
                    let result = $type::try_from($variant_str);
                    assert_ok!(&result);
                    let variant = result.unwrap();
                    assert_eq!($type(<$module>::$variant), variant);
                    assert_eq!($variant_str, variant.to_string());
                )+
            }
        }
    };
}
