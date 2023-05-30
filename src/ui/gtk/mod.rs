use serde::Serialize;

pub fn serialize(iface: &Interface) -> Result<String, quick_xml::DeError> {
    quick_xml::se::to_string(iface)
}

#[derive(Debug, Serialize)]
#[serde(rename = "interface")]
pub struct Interface {
    #[serde(default, rename = "object")]
    pub objects: Vec<Object>,
}

#[derive(Debug, Serialize)]
pub struct Object {
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "@class")]
    pub class: String,
    #[serde(default, rename = "property", skip_serializing_if = "Vec::is_empty")]
    pub properties: Vec<Property>,
    #[serde(default, rename = "child", skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<Child>,
}

#[derive(Debug, Serialize)]
pub struct Child {
    pub object: Object,
}

#[derive(Debug, Serialize)]
pub struct Property {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "$value")]
    pub value: String,
}

#[cfg(test)]
mod test {
    use claims::assert_ok;

    use super::*;

    #[test]
    fn empty_gtk_ui() {
        let result = serialize(&Interface { objects: vec![] });
        assert_ok!(&result);

        assert_eq!("<interface/>", result.unwrap());
    }

    #[test]
    fn single_object_gtk_ui() {
        let result = serialize(&Interface {
            objects: vec![Object {
                id: None,
                class: "GtkLabel".to_owned(),
                properties: vec![],
                children: vec![],
            }],
        });
        assert_ok!(&result);

        assert_eq!(
            r#"<interface><object class="GtkLabel"/></interface>"#,
            result.unwrap()
        );
    }

    #[test]
    fn single_object_with_property_gtk_ui() {
        let result = serialize(&Interface {
            objects: vec![Object {
                id: None,
                class: "GtkLabel".to_owned(),
                properties: vec![Property {
                    name: "label".to_owned(),
                    value: "Hello world".to_owned(),
                }],
                children: vec![],
            }],
        });
        assert_ok!(&result);

        assert_eq!(
            r#"<interface><object class="GtkLabel"><property name="label">Hello world</property></object></interface>"#,
            result.unwrap()
        );
    }

    #[test]
    fn object_with_children_gtk_ui() {
        let result = serialize(&Interface {
            objects: vec![Object {
                id: None,
                class: "GtkBox".to_owned(),
                properties: vec![Property {
                    name: "orientation".to_owned(),
                    value: "horizontal".to_owned(),
                }],
                children: vec![
                    Child {
                        object: Object {
                            id: None,
                            class: "GtkLabel".to_owned(),
                            properties: vec![Property {
                                name: "label".to_owned(),
                                value: "Hello".to_owned(),
                            }],
                            children: vec![],
                        },
                    },
                    Child {
                        object: Object {
                            id: None,
                            class: "GtkLabel".to_owned(),
                            properties: vec![Property {
                                name: "label".to_owned(),
                                value: "world".to_owned(),
                            }],
                            children: vec![],
                        },
                    },
                ],
            }],
        });
        assert_ok!(&result);

        assert_eq!(
            r#"<interface><object class="GtkBox"><property name="orientation">horizontal</property><child><object class="GtkLabel"><property name="label">Hello</property></object></child><child><object class="GtkLabel"><property name="label">world</property></object></child></object></interface>"#,
            result.unwrap()
        );
    }
}
