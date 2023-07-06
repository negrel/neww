use std::string::FromUtf8Error;

use anyhow::{anyhow, Context};
use derive_builder::Builder;
use quick_xml::{events::BytesStart, name::QName, Reader};
use serde::Serialize;

use crate::{parse_object, Object};

/// Neww is the root tag (like <html>) of a .neww file.
#[derive(Debug, Clone, PartialEq, Eq, Builder)]
pub struct Neww {
    #[builder(default)]
    pub meta: Option<Meta>,
    #[builder(default)]
    pub interface: Interface,
}

/// Meta contains meta tags that aren't related to UI.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Meta {}

/// Interface contains UI related tags that will be transpiled to GTK UI.
#[derive(Debug, Serialize, Clone, PartialEq, Eq, Default)]
#[serde(rename = "interface")]
pub struct Interface {
    #[serde(rename = "object")]
    pub windows: Vec<Object>,
}

pub fn parse_neww(neww: &str) -> Result<Neww, anyhow::Error> {
    let mut reader = Reader::from_str(neww);
    reader.trim_text(true);

    loop {
        let event = reader
            .read_event()
            .context("failed to read event")?
            .into_owned();

        match event {
            quick_xml::events::Event::Start(tag) => {
                let name =
                    qname_to_string(tag.name()).context("failed to convert tag to String")?;

                if name == "neww" {
                    return parse_neww_tag(tag, &mut reader);
                } else {
                    return Err(anyhow!("unexpected tag: got <{name}>, expected <neww>"));
                }
            }
            quick_xml::events::Event::End(_tag) => return Err(anyhow!("unexpected end tag")),
            quick_xml::events::Event::Empty(_tag) => return Err(anyhow!("unexpected empty tag")),
            quick_xml::events::Event::Text(_txt) => return Err(anyhow!("unexpected text")),
            quick_xml::events::Event::Decl(_)
            | quick_xml::events::Event::PI(_)
            | quick_xml::events::Event::DocType(_)
            | quick_xml::events::Event::Comment(_)
            | quick_xml::events::Event::CData(_) => continue,
            quick_xml::events::Event::Eof => return Err(anyhow!("unexpected end of file")),
        }
    }
}

fn parse_neww_tag(
    start: BytesStart<'static>,
    reader: &mut Reader<&[u8]>,
) -> Result<Neww, anyhow::Error> {
    let tag_name = qname_to_string(start.name()).context("failed to convert tag to String")?;
    let mut neww = NewwBuilder::default();

    loop {
        let event = reader
            .read_event()
            .context("failed to read event")?
            .into_owned();

        match event {
            quick_xml::events::Event::Start(tag) => {
                let name =
                    qname_to_string(tag.name()).context("failed to convert tag to String")?;

                match name.as_str() {
                    "meta" => {
                        neww.meta(Some(
                            parse_meta_tag(tag, reader).context("failed to parse meta tag")?,
                        ));
                    }
                    "interface" => {
                        neww.interface(
                            parse_interface_tag(tag, reader)
                                .context("failed to parse interface tag")?,
                        );
                    }
                    _ => {
                        return Err(anyhow!(
                            "unexpected start tag: got <{name}>, expected <meta> or <interface>"
                        ))
                    }
                }
            }
            quick_xml::events::Event::End(tag) => {
                let name =
                    qname_to_string(tag.name()).context("failed to convert tag to String")?;

                if name == tag_name {
                    return neww.build().context("invalid neww tag");
                } else {
                    return Err(anyhow!(
                        "unexpected end tag: got </{name}>, expected </{tag_name}>"
                    ));
                }
            }
            quick_xml::events::Event::Empty(_tag) => return Err(anyhow!("unexpected empty tag")),
            quick_xml::events::Event::Text(_txt) => return Err(anyhow!("unexpected text")),
            quick_xml::events::Event::Decl(_)
            | quick_xml::events::Event::PI(_)
            | quick_xml::events::Event::DocType(_)
            | quick_xml::events::Event::Comment(_)
            | quick_xml::events::Event::CData(_) => continue,
            quick_xml::events::Event::Eof => return Err(anyhow!("unexpected end of file")),
        }
    }
}

fn parse_meta_tag(
    start: BytesStart<'static>,
    reader: &mut Reader<&[u8]>,
) -> Result<Meta, anyhow::Error> {
    let tag_name = qname_to_string(start.name()).context("failed to convert tag to String")?;
    let mut meta = Meta {};

    Ok(meta)
}

fn parse_interface_tag(
    start: BytesStart<'static>,
    reader: &mut Reader<&[u8]>,
) -> Result<Interface, anyhow::Error> {
    let tag_name = qname_to_string(start.name()).context("failed to convert tag to String")?;
    let mut interface = Interface {
        windows: Vec::new(),
    };

    loop {
        let event = reader
            .read_event()
            .context("failed to read event")?
            .into_owned();

        match event {
            quick_xml::events::Event::Start(tag) => {
                let object =
                    parse_object(tag, reader).context("failed to parse interface children")?;

                match object.class.as_str() {
                    "GtkWindow" => {
                        if let Some(id) = &object.id {
                            if id.as_str() == "main" {
                                interface.windows.insert(0, object.clone());
                                continue;
                            }
                        }

                        interface.windows.push(object)
                    }
                    _ => {
                        return Err(anyhow!(
                            "unexpected interface child tag: only <window> tag are accepted"
                        ))
                    }
                }
            }
            quick_xml::events::Event::End(tag) => {
                let name =
                    qname_to_string(tag.name()).context("failed to convert tag to String")?;

                if name == tag_name {
                    return Ok(interface);
                } else {
                    return Err(anyhow!(
                        "unexpected end tag: got </{name}>, expected </{tag_name}>"
                    ));
                }
            }
            quick_xml::events::Event::Empty(_) => todo!(),
            quick_xml::events::Event::Text(_) => todo!(),
            quick_xml::events::Event::Decl(_)
            | quick_xml::events::Event::PI(_)
            | quick_xml::events::Event::DocType(_)
            | quick_xml::events::Event::Comment(_)
            | quick_xml::events::Event::CData(_) => continue,
            quick_xml::events::Event::Eof => return Err(anyhow!("unexpected end of file")),
        }
    }
}
fn qname_to_string(name: QName) -> Result<String, FromUtf8Error> {
    String::from_utf8(name.as_ref().to_vec())
}
