use anyhow::{anyhow, Context};
use quick_xml::{
    events::{BytesEnd, BytesStart, Event},
    name::QName,
    Reader, Writer,
};

#[derive(Debug)]
pub struct Ui {
    pub gtk_ui: String,
    scripts: Vec<String>,
}

#[derive(Debug, thiserror::Error)]
#[error("{cause}: at line {line}")]
pub struct ParseError {
    pub line: usize,
    pub cause: anyhow::Error,
}

const NEWW_PREFIX: &[u8] = b"neww";

/// Parse a neww XML file and return a valid GTK UI XML file.
pub fn parse(src: &str) -> Result<Ui, ParseError> {
    let mut reader = Reader::from_str(src);

    // Parse and add context if an error occurred.
    parse_with_reader(&mut reader).map_err(|err| {
        let lines = src.lines();
        let error_position = reader.buffer_position();

        let mut position = 0;
        let mut err_line = 0;
        for (i, line) in lines.enumerate() {
            position += line.len();
            if position > error_position {
                err_line = i;
                break;
            }
        }

        ParseError {
            line: err_line,
            cause: err,
        }
    })
}

enum NewwTag {
    Script,
}

impl TryFrom<&BytesStart<'_>> for NewwTag {
    type Error = anyhow::Error;

    fn try_from(value: &BytesStart) -> Result<Self, Self::Error> {
        match value.name().as_ref() {
            b"neww:script" => Ok(NewwTag::Script),
            _ => Err(anyhow!("unknown neww tag")),
        }
    }
}

fn parse_with_reader(reader: &mut Reader<&[u8]>) -> Result<Ui, anyhow::Error> {
    // Parsed result
    let mut gtk_ui = Writer::new(Vec::new());
    let mut scripts = Vec::new();

    // Process XML to convert neww file into GTK UI XML.
    loop {
        let event = reader.read_event()?;
        match event {
            Event::Start(ref start_tag) => {
                // If neww tag, process it
                if is_neww_start_tag(start_tag) {
                    match NewwTag::try_from(start_tag).context("Failed to parse neww tag")? {
                        NewwTag::Script => scripts.push(parse_neww_script(reader)?),
                    }
                    // Don't add it to GTK UI XML
                    continue;
                }
            }
            Event::Empty(ref empty_tag) => {
                if is_neww_start_tag(empty_tag) {
                    match NewwTag::try_from(empty_tag).context("Failed to parse neww tag")? {
                        NewwTag::Script => return Err(anyhow!("neww:script can't be empty")),
                    }
                }
            }
            Event::End(ref end_tag) => {
                if is_neww_end_tag(end_tag) {
                    continue;
                }
            }
            Event::Eof => break,
            Event::Text(_)
            | Event::Comment(_)
            | Event::CData(_)
            | Event::Decl(_)
            | Event::DocType(_)
            | Event::PI(_) => {}
        };

        // Add tag to gtk_ui xml
        gtk_ui.write_event(event)?;
    }

    Ok(Ui {
        gtk_ui: String::from_utf8(gtk_ui.into_inner())?,
        scripts,
    })
}

fn is_neww_qname(name: &QName) -> bool {
    match name.prefix() {
        Some(prefix) => prefix.into_inner() == NEWW_PREFIX,
        None => false,
    }
}

fn is_neww_start_tag(tag: &BytesStart) -> bool {
    is_neww_qname(&tag.name())
}

fn is_neww_end_tag(tag: &BytesEnd) -> bool {
    is_neww_qname(&tag.name())
}

// Parse content of a single neww:script tag.
fn parse_neww_script(reader: &mut Reader<&[u8]>) -> Result<String, anyhow::Error> {
    let mut script_content = "".to_owned();
    loop {
        let event = reader.read_event()?;
        match event {
            Event::Text(text) => script_content = String::from_utf8(text.to_vec())?,
            Event::End(end_tag) => match end_tag.local_name().as_ref() {
                b"script" => return Ok(script_content),
                _ => return Err(anyhow!("neww:script tags can only contain text")),
            },
            Event::Start(_)
            | Event::Empty(_)
            | Event::Comment(_)
            | Event::CData(_)
            | Event::Decl(_)
            | Event::PI(_)
            | Event::DocType(_)
            | Event::Eof => {
                return Err(anyhow!("neww:script tags can only contain text"));
            }
        }
    }
}

#[cfg(test)]
mod test {
    use claims::{assert_err, assert_ok};

    #[test]
    fn simple_gtk_ui_is_valid() {
        let gtk_ui = r#"
<?xml version="1.0" encoding="UTF-8"?>
<interface>
	<object class="GtkBox" id="body">
		<property name="orientation">vertical</property>
		<property name="halign">start</property>
		<child>
			<object class="GtkLabel">
				<property name="label">Hello world!</property>
			</object>
		</child>
	</object>
</interface>
        "#;

        let result = super::parse(gtk_ui);
        assert_ok!(&result);

        let ui = result.unwrap();
        assert_eq!(gtk_ui, ui.gtk_ui);
    }

    #[test]
    fn gtk_ui_with_neww_script_tag_is_valid() {
        let gtk_ui = r#"
<?xml version="1.0" encoding="UTF-8"?>
<interface>
	<neww:script>
		println("Hello world")
	</neww:script>
	<object class="GtkBox" id="body">
		<property name="orientation">vertical</property>
	</object>
</interface>
        "#;

        let result = super::parse(gtk_ui);
        assert_ok!(&result);

        let ui = result.unwrap();
        assert_eq!(
            r#"
<?xml version="1.0" encoding="UTF-8"?>
<interface>
	
	<object class="GtkBox" id="body">
		<property name="orientation">vertical</property>
	</object>
</interface>
        "#,
            ui.gtk_ui
        );
    }

    #[test]
    fn gtk_ui_with_unknown_neww_tag_is_invalid() {
        let gtk_ui = r#"
<?xml version="1.0" encoding="UTF-8"?>
<interface>
	<neww:foo>
		println("Hello world")
	</neww:foo>
	<object class="GtkBox" id="body">
		<property name="orientation">vertical</property>
	</object>
</interface>
        "#;

        let result = super::parse(gtk_ui);
        assert_err!(&result);

        let err = result.unwrap_err();
        assert_eq!(4, err.line, "error line number doesn't match");
    }

    #[test]
    fn gtk_ui_with_empty_neww_script_tag_is_invalid() {
        let gtk_ui = r#"
<?xml version="1.0" encoding="UTF-8"?>
<interface>
	<neww:script/>
	<object class="GtkBox" id="body">
		<property name="orientation">vertical</property>
	</object>
</interface>
        "#;

        let result = super::parse(gtk_ui);
        assert_err!(&result);

        let err = result.unwrap_err();
        assert_eq!(4, err.line, "error line number doesn't match");
    }
}
