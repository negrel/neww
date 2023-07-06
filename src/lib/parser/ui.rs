use std::string::FromUtf8Error;

use anyhow::{anyhow, Context};
use quick_xml::{events::BytesStart, name::QName, reader::Reader};
use serde::Serialize;

#[derive(Debug, Serialize, Clone, PartialEq, Eq)]
#[serde(rename = "object")]
pub struct Object {
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "@class")]
    pub class: String,
    #[serde(rename = "property", default, skip_serializing_if = "Vec::is_empty")]
    pub properties: Vec<Property>,
    #[serde(rename = "child", default, skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<Child>,
}

#[derive(Debug, Serialize, Clone, PartialEq, Eq)]
#[serde(rename = "child")]
pub struct Child {
    pub object: Object,
}

#[derive(Debug, Serialize, Clone, PartialEq, Eq)]
#[serde(rename = "property")]
pub struct Property {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "$value")]
    pub value: String,
}

pub fn parse_object_empty_tag(
    start: BytesStart<'static>,
    _reader: &mut Reader<&[u8]>,
) -> Result<Object, anyhow::Error> {
    let tag_name = qname_to_string(start.name()).context("failed to convert tag to String")?;
    let mut object = Object {
        id: None,
        class: "Gtk".to_owned() + &tag_name[..1].to_uppercase() + &tag_name[1..],
        properties: Vec::new(),
        children: Vec::new(),
    };

    // Parse attributes.
    for attr in start.attributes() {
        let attr = attr.context("failed to parse attribute")?;

        let name =
            qname_to_string(attr.key).context("failed to convert attribute name to String")?;
        let value = String::from_utf8(attr.value.to_vec())
            .context("failed to convert attribute value to String")?;

        match name.as_str() {
            "id" => object.id = Some(value),
            _ => object.properties.push(Property { name, value }),
        }
    }

    Ok(object)
}

pub fn parse_object(
    start: BytesStart<'static>,
    reader: &mut Reader<&[u8]>,
) -> Result<Object, anyhow::Error> {
    let tag_name = qname_to_string(start.name()).context("failed to convert tag to String")?;
    let mut object = parse_object_empty_tag(start, reader)?;

    loop {
        let event = reader
            .read_event()
            .context("failed to read event")?
            .into_owned();

        match event {
            quick_xml::events::Event::Start(tag) => {
                let child = parse_object(tag, reader)?;
                object.children.push(Child { object: child });
            }
            quick_xml::events::Event::End(tag) => {
                let name =
                    qname_to_string(tag.name()).context("failed to convert tag to String")?;

                if name == tag_name {
                    return Ok(object);
                } else {
                    return Err(anyhow!(
                        "unexpected end tag: got </{name}>, expected </{tag_name}>"
                    ));
                }
            }
            quick_xml::events::Event::Empty(tag) => {
                let child = parse_object_empty_tag(tag, reader)?;
                object.children.push(Child { object: child });
            }
            quick_xml::events::Event::Text(txt) => object.properties.push(Property {
                name: "label".to_owned(),
                value: String::from_utf8(txt.into_inner().to_vec())
                    .context("failed to convert inner text to String")?,
            }),
            quick_xml::events::Event::Comment(_) => continue,
            quick_xml::events::Event::CData(_)
            | quick_xml::events::Event::Decl(_)
            | quick_xml::events::Event::PI(_)
            | quick_xml::events::Event::DocType(_)
            | quick_xml::events::Event::Eof => {
                return Err(anyhow!(
                    "unexpected event occured: got {event:?}, expected another tag or text"
                ))
            }
        }
    }
}

fn qname_to_string(name: QName) -> Result<String, FromUtf8Error> {
    String::from_utf8(name.as_ref().to_vec())
}

#[cfg(test)]
mod test {
    use crate::{parse_neww_ui, Child, Object, Property};

    #[test]
    fn parse_nested_objects() {
        let window = parse_neww_ui(
            r#"
          <window id="win1">
            <box orientation="horizontal">
              <!-- NOTE: space before "world" label will be trimmed -->
              <label>Hello</label><label> world</label>
              <button>Exit</button>
            </box>
          </window>"#,
        )
        .unwrap();

        assert_eq!(
            window,
            Some(Object {
                id: Some("win1".to_owned()),
                class: "GtkWindow".to_owned(),
                properties: Vec::new(),
                children: vec![Child {
                    object: Object {
                        id: None,
                        class: "GtkBox".to_owned(),
                        properties: vec![Property {
                            name: "orientation".to_owned(),
                            value: "horizontal".to_owned()
                        }],
                        children: vec![
                            Child {
                                object: Object {
                                    id: None,
                                    class: "GtkLabel".to_owned(),
                                    properties: vec![Property {
                                        name: "label".to_owned(),
                                        value: "Hello".to_owned()
                                    }],
                                    children: Vec::new()
                                }
                            },
                            Child {
                                object: Object {
                                    id: None,
                                    class: "GtkLabel".to_owned(),
                                    properties: vec![Property {
                                        name: "label".to_owned(),
                                        value: "world".to_owned()
                                    }],
                                    children: Vec::new()
                                }
                            },
                            Child {
                                object: Object {
                                    id: None,
                                    class: "GtkButton".to_owned(),
                                    properties: vec![Property {
                                        name: "label".to_owned(),
                                        value: "Exit".to_owned()
                                    }],
                                    children: Vec::new()
                                }
                            }
                        ]
                    }
                }]
            })
        );
    }
}
