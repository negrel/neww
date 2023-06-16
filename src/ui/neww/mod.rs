use quick_xml::de::DeError;
use serde::Deserialize;
use std::{boxed::Box as StdBox, path::PathBuf};

use super::gtk;

pub fn deserialize(src: &str) -> Result<Neww, DeError> {
    quick_xml::de::from_str(src)
}

macro_rules! struct_tag {
    ($name:ident as $tag_name:literal $decl:tt) => {
        #[derive(Debug, Deserialize, PartialEq)]
        #[serde(rename = $tag_name)]
        pub struct $name $decl
    };
}

macro_rules! enum_tag {
    ($name:ident $decl:tt) => {
        #[derive(Debug, Deserialize, PartialEq)]
        #[serde(rename_all = "lowercase")]
        pub enum $name $decl
    };
}

macro_rules! enum_attr {
    ($name:ident $decl:tt) => {
        #[derive(Debug, Deserialize, Hash, PartialEq, Eq)]
        #[serde(rename_all = "lowercase")]
        pub enum $name $decl
    };
}

macro_rules! component {
    ($name:ident as $tag_name:literal {
        $(
            $(#[$field_attr:meta])*
            $field:ident : $ty:ty,
        )*
    }) => {
        struct_tag!($name as $tag_name {
            #[serde(rename = "@id")]
            id: Option<String>,

            #[serde(rename = "@class")]
            css_classes: Option<String>,

            // We're adding common widget attributes / properties with macros as
            // serde(flatten) & serde(rename = "$value") doesn't work together
            // for now.
            #[serde(rename = "@hexpand")]
            hexpand: Option<bool>,

            #[serde(rename = "@vexpand")]
            vexpand: Option<bool>,

            #[serde(rename = "@opacity")]
            opacity: Option<f64>,

            #[serde(rename = "@halign")]
            halign: Option<String>,

            #[serde(rename = "@valign")]
            valign: Option<String>,

            #[serde(rename = "@justify")]
            justify: Option<String>,

            #[serde(rename = "@wrap_mode")]
            wrap_mode: Option<String>,

            #[serde(rename = "@wrap")]
            wrap: Option<bool>,

            $(
                $(#[$field_attr])*
                $field: $ty,
            )*
        });
    };
}

// Compute properties Vector based on common widget properties.
macro_rules! widget_attributes_into_gtk_props {
    ($self:ident) => {{
        let mut properties = Vec::new();

        // IDs css classes are handled per component.
        if let Some(id) = $self.id.clone() {
            properties.push(gtk::Property {
                name: "name".to_owned(),
                value: id,
            })
        }

        if let Some(hexpand) = $self.hexpand {
            properties.push(gtk::Property {
                name: "hexpand".to_owned(),
                value: hexpand.to_string(),
            });
        }

        if let Some(vexpand) = $self.vexpand {
            properties.push(gtk::Property {
                name: "vexpand".to_owned(),
                value: vexpand.to_string(),
            });
        }

        if let Some(opacity) = $self.opacity {
            properties.push(gtk::Property {
                name: "opacity".to_owned(),
                value: opacity.to_string(),
            });
        }

        if let Some(halign) = $self.halign {
            properties.push(gtk::Property {
                name: "halign".to_owned(),
                value: halign.to_string(),
            });
        }

        if let Some(valign) = $self.valign {
            properties.push(gtk::Property {
                name: "valign".to_owned(),
                value: valign.to_string(),
            });
        }

        if let Some(justify) = $self.justify {
            properties.push(gtk::Property {
                name: "justify".to_owned(),
                value: justify.to_string(),
            });
        }

        if let Some(wrap_mode) = $self.wrap_mode {
            properties.push(gtk::Property {
                name: "wrap_mode".to_owned(),
                value: wrap_mode.to_string(),
            });
        }

        if let Some(wrap) = $self.wrap {
            properties.push(gtk::Property {
                name: "wrap".to_owned(),
                value: wrap.to_string(),
            });
        }

        properties
    }};
}

struct_tag!(Neww as "neww" {
    pub meta: Option<Meta>,
    pub interface: Interface,
});

struct_tag!(Meta as "meta" {
    #[serde(default, rename = "$value")]
    pub children: Vec<MetaTag>,
});

impl Meta {
    pub fn scripts(&self) -> impl Iterator<Item = &Script> {
        self.children
            .iter()
            .filter(|tag| matches!(tag, MetaTag::Script(_)))
            .map(|el| match el {
                MetaTag::Script(script) => script,
                _ => unreachable!(),
            })
    }

    pub fn styles(&self) -> impl Iterator<Item = &Style> {
        self.children
            .iter()
            .filter(|tag| matches!(tag, MetaTag::Style(_)))
            .map(|el| match el {
                MetaTag::Style(style) => style,
                _ => unreachable!(),
            })
    }
}

enum_tag!(MetaTag {
    Script(Script),
    Style(Style),
});

struct_tag!(Script as "script" {
    #[serde(default, rename = "$text")]
    pub inline: Option<String>,

    #[serde(default, rename = "@src")]
    pub source_path: Option<PathBuf>,
});

struct_tag!(Style as "style" {
    #[serde(default, rename = "$text")]
    pub inline: Option<String>,

    #[serde(default, rename = "@src")]
    pub source_path: Option<PathBuf>,
});

struct_tag!(Interface as "interface" {
    #[serde(default, rename = "$value")]
    objects: Vec<Object>,
});

#[allow(clippy::from_over_into)]
impl Into<gtk::Interface> for Interface {
    fn into(self) -> gtk::Interface {
        gtk::Interface {
            objects: self
                .objects
                .into_iter()
                .map(Into::<gtk::Object>::into)
                .collect(),
        }
    }
}

enum_tag!(Object {
    Window(StdBox<Window>),
    Label(StdBox<Label>),
    Button(StdBox<Button>),
    Box(StdBox<Box>),
    Image(StdBox<Image>),
    Scale(StdBox<Scale>),
});

#[allow(clippy::from_over_into)]
impl Into<gtk::Object> for Object {
    fn into(self) -> gtk::Object {
        match self {
            Object::Window(w) => w.into(),
            Object::Label(l) => l.into(),
            Object::Button(b) => b.into(),
            Object::Box(b) => b.into(),
            Object::Image(i) => i.into(),
            Object::Scale(s) => s.into(),
        }
    }
}

#[allow(clippy::from_over_into)]
impl Into<gtk::Child> for Object {
    fn into(self) -> gtk::Child {
        gtk::Child {
            object: self.into(),
        }
    }
}

component!(Window as "window" {
    #[serde(default, rename = "@layer")]
    is_layer: bool,

    #[serde(default, rename = "$value")]
    child: Option<Object>,
});

#[allow(clippy::from_over_into)]
impl Into<gtk::Object> for StdBox<Window> {
    fn into(self) -> gtk::Object {
        let mut children = vec![];
        if let Some(child) = self.child {
            children.push(child.into())
        }
        gtk::Object {
            id: self.id.clone(),
            class: "GtkWindow".to_owned(),
            properties: widget_attributes_into_gtk_props!(self),
            object_properties: vec![],
            children,
        }
    }
}

component!(Label as "label" {
    #[serde(default, rename = "$text")]
    label: String,
});

#[allow(clippy::from_over_into)]
impl Into<gtk::Object> for StdBox<Label> {
    fn into(self) -> gtk::Object {
        let mut properties = widget_attributes_into_gtk_props!(self);
        properties.push(gtk::Property {
            name: "label".to_owned(),
            value: self.label,
        });

        gtk::Object {
            id: self.id,
            class: "GtkLabel".to_owned(),
            properties,
            object_properties: vec![],
            children: vec![],
        }
    }
}

component!(Button as "button" {
    #[serde(default, rename = "$text")]
    label: String,
});

#[allow(clippy::from_over_into)]
impl Into<gtk::Object> for StdBox<Button> {
    fn into(self) -> gtk::Object {
        let mut properties = widget_attributes_into_gtk_props!(self);
        properties.push(gtk::Property {
            name: "label".to_owned(),
            value: self.label,
        });

        gtk::Object {
            id: self.id,
            class: "GtkButton".to_owned(),
            properties,
            object_properties: vec![],
            children: vec![],
        }
    }
}

component!(Box as "box" {
    #[serde(rename = "@orientation")]
    orientation: Option<Orientation>,

    #[serde(default, rename = "$value")]
    children: Vec<Object>,
});

#[allow(clippy::from_over_into)]
impl Into<gtk::Object> for StdBox<Box> {
    fn into(self) -> gtk::Object {
        let mut properties = widget_attributes_into_gtk_props!(self);
        if let Some(orientation) = self.orientation {
            properties.push(gtk::Property {
                name: "orientation".to_owned(),
                value: orientation.to_string(),
            });
        }

        gtk::Object {
            id: self.id,
            class: "GtkBox".to_owned(),
            properties,
            object_properties: vec![],
            children: self
                .children
                .into_iter()
                .map(Into::<gtk::Child>::into)
                .collect(),
        }
    }
}

enum_attr!(Orientation {
    Horizontal,
    Vertical,
});

impl ToString for Orientation {
    fn to_string(&self) -> String {
        match self {
            Orientation::Horizontal => "horizontal",
            Orientation::Vertical => "vertical",
        }
        .to_owned()
    }
}

component!(Image as "image" {
    #[serde(rename = "@file")]
    file: String,
});

#[allow(clippy::from_over_into)]
impl Into<gtk::Object> for StdBox<Image> {
    fn into(self) -> gtk::Object {
        let mut properties = widget_attributes_into_gtk_props!(self);
        properties.push(gtk::Property {
            name: "file".to_owned(),
            value: self.file,
        });

        gtk::Object {
            id: self.id,
            class: "GtkImage".to_owned(),
            properties,
            object_properties: vec![],
            children: vec![],
        }
    }
}

component!(Scale as "scale" {
    #[serde(default, rename = "@value")]
    range_value: usize,

    #[serde(default, rename = "@min")]
    range_min: usize,

    #[serde(rename = "@max")]
    range_max: Option<usize>,

    #[serde(rename = "@step")]
    range_step: Option<usize>,
});

#[allow(clippy::from_over_into)]
impl Into<gtk::Object> for StdBox<Scale> {
    fn into(self) -> gtk::Object {
        let properties = widget_attributes_into_gtk_props!(self);
        let range_min = self.range_min;
        let range_max = self.range_max.unwrap_or(100);
        let range_step = self.range_max.unwrap_or(1);

        gtk::Object {
            id: self.id,
            class: "GtkScale".to_owned(),
            properties,
            children: vec![],
            object_properties: vec![gtk::ObjectProperty {
                name: "adjustment".to_owned(),
                object: gtk::Object {
                    id: None,
                    class: "GtkAdjustment".to_owned(),
                    properties: vec![
                        gtk::Property {
                            name: "lower".to_owned(),
                            value: range_min.to_string(),
                        },
                        gtk::Property {
                            name: "upper".to_owned(),
                            value: range_max.to_string(),
                        },
                        gtk::Property {
                            name: "step-increment".to_owned(),
                            value: range_step.to_string(),
                        },
                        gtk::Property {
                            name: "value".to_owned(),
                            value: self.range_value.to_string(),
                        },
                    ],
                    object_properties: vec![],
                    children: vec![],
                },
            }],
        }
    }
}

#[cfg(test)]
mod test {

    use claims::assert_ok;

    use super::*;

    #[test]
    fn empty_interface_neww_ui() {
        let result = deserialize("<neww><interface/></neww>");
        assert_ok!(&result);
        assert_eq!(
            Neww {
                meta: None,
                interface: Interface { objects: vec![] }
            },
            result.unwrap()
        );
    }

    #[test]
    fn single_window_object_neww_ui() {
        let result = deserialize("<neww><interface><window/></interface></neww>");
        assert_ok!(&result);
        assert_eq!(
            Neww {
                meta: None,
                interface: Interface {
                    objects: vec![Object::Window(StdBox::new(Window {
                        child: None,
                        id: None,
                        css_classes: None,
                        hexpand: None,
                        vexpand: None,
                        opacity: None,
                        is_layer: false,
                        halign: None,
                        valign: None
                    }))]
                }
            },
            result.unwrap()
        );
    }

    #[test]
    fn single_window_object_with_attributes_neww_ui() {
        let result = deserialize(
            r#"<neww><interface><window id="window-1" class="window pop-up" /></interface></neww>"#,
        );
        assert_ok!(&result);
        assert_eq!(
            Neww {
                meta: None,
                interface: Interface {
                    objects: vec![Object::Window(StdBox::new(Window {
                        id: Some("window-1".to_owned()),
                        css_classes: Some("window pop-up".to_owned()),
                        hexpand: None,
                        vexpand: None,
                        opacity: None,
                        child: None,
                        halign: None,
                        valign: None,
                        is_layer: false
                    }))]
                }
            },
            result.unwrap()
        );
    }

    #[test]
    fn window_with_box_with_label_and_button_neww_ui() {
        let result = deserialize(
            r#"
            <neww>
                <interface>
                    <window>
                        <box orientation="vertical">
                            <label>Hello world!</label>
                            <button>Button text</button>
                        </box>
                    </window>
                </interface>
            </neww>"#,
        );
        assert_ok!(&result);
        assert_eq!(
            Neww {
                meta: None,
                interface: Interface {
                    objects: vec![Object::Window(StdBox::new(Window {
                        id: None,
                        css_classes: None,
                        hexpand: None,
                        vexpand: None,
                        opacity: None,
                        halign: None,
                        valign: None,
                        is_layer: false,
                        child: Some(Object::Box(StdBox::new(Box {
                            id: None,
                            css_classes: None,
                            orientation: Some(Orientation::Vertical),
                            hexpand: None,
                            vexpand: None,
                            opacity: None,
                            halign: None,
                            valign: None,
                            children: vec![
                                Object::Label(StdBox::new(Label {
                                    id: None,
                                    css_classes: None,
                                    hexpand: None,
                                    vexpand: None,
                                    opacity: None,
                                    halign: None,
                                    valign: None,
                                    label: "Hello world!".to_owned(),
                                })),
                                Object::Button(StdBox::new(Button {
                                    id: None,
                                    css_classes: None,
                                    hexpand: None,
                                    vexpand: None,
                                    opacity: None,
                                    halign: None,
                                    valign: None,
                                    label: "Button text".to_owned(),
                                }))
                            ],
                        }))),
                    }))]
                },
            },
            result.unwrap()
        );
    }

    #[test]
    fn window_with_nested_widgets_and_extra_attributes_neww_ui() {
        let result = deserialize(
            r#"
            <neww>
                <interface>
                    <window>
                        <box hexpand="true">
                            <label>Hello world!</label>
                            <button>Button text</button>
                            <image hexpand="true" file="/dev/null" />
                        </box>
                    </window>
                </interface>
            </neww>"#,
        );
        assert_ok!(&result);
        assert_eq!(
            Neww {
                meta: None,
                interface: Interface {
                    objects: vec![Object::Window(StdBox::new(Window {
                        id: None,
                        css_classes: None,
                        hexpand: None,
                        vexpand: None,
                        opacity: None,
                        halign: None,
                        valign: None,
                        is_layer: false,
                        child: Some(Object::Box(StdBox::new(Box {
                            id: None,
                            css_classes: None,
                            orientation: None,
                            hexpand: Some(true),
                            vexpand: None,
                            opacity: None,
                            halign: None,
                            valign: None,
                            children: vec![
                                Object::Label(StdBox::new(Label {
                                    id: None,
                                    css_classes: None,
                                    hexpand: None,
                                    vexpand: None,
                                    label: "Hello world!".to_owned(),
                                    opacity: None,
                                    halign: None,
                                    valign: None,
                                })),
                                Object::Button(StdBox::new(Button {
                                    id: None,
                                    css_classes: None,
                                    hexpand: None,
                                    vexpand: None,
                                    label: "Button text".to_owned(),
                                    opacity: None,
                                    halign: None,
                                    valign: None,
                                })),
                                Object::Image(StdBox::new(Image {
                                    id: None,
                                    css_classes: None,
                                    hexpand: Some(true),
                                    vexpand: None,
                                    opacity: None,
                                    halign: None,
                                    valign: None,
                                    file: "/dev/null".to_owned()
                                }))
                            ],
                        }))),
                    }))]
                },
            },
            result.unwrap()
        );
    }
}
