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
    ($fields:ident, $field_name:ident, $setter_name:ident, $convert:expr) => {
        $fields.add_field_method_set(stringify!($field_name), |vm, this, value| {
            #[allow(clippy::redundant_closure_call)]
            this.0.$setter_name($convert(vm, this, value)?);
            Ok(())
        })
    };
}

#[macro_export]
macro_rules! add_method {
    ($methods:ident, $method_name:ident, $obj_method_name:ident $(, $args:ident)*) => {
        $methods.add_method(stringify!($method_name), |_vm, this, ($($args, )*)| {
            Ok(this.0.$obj_method_name($($args, )*))
        })
    };
}

#[macro_export]
macro_rules! add_downcast_method {
    ($methods:ident, $downcast_type:ident) => {
        $methods.add_method(
            &concat!("as_", stringify!($downcast_type)).to_lowercase(),
            |_vm, this, ()| {
                let obj = this
                    .0
                    .clone()
                    .downcast::<gtk::$downcast_type>()
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
                    let obj = this.0.clone().upcast::<gtk::$upcast_type>();
                    Ok($upcast_type(obj))
                },
            );
        )+
        // Add meta method so as_xxx is not mandatory.
        $methods.add_meta_method(MetaMethod::Index, |vm, this, key: String| {
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
            }).eval::<mlua::Value>()?;
            Ok(result)
        });
    };
}
