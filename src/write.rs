use std::fs;

use crate::{
    args::Write,
    destinations::get_destinations,
    render::{render, Mail},
};
#[cfg(test)]
use mockall::automock;
use std::fmt::Write as _;

#[cfg(windows)]
const LINE_ENDING: &str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &str = "\n";

pub fn write(write: Write) -> Result<(), Option<String>> {
    let destinations = get_destinations(&write.file_path, write.simulate)?;
    let mails = render(
        destinations,
        &write.subject,
        &write.template_file_path,
        write.simulate,
    )?;
    write_mails(
        &FileWriter {},
        mails,
        &write.output_file_path,
        write.simulate,
    )?;
    Ok(())
}

fn write_mails(
    writer: &dyn WritesFile,
    mails: Vec<Mail>,
    file_path: &str,
    simulate: bool,
) -> Result<(), String> {
    let mut text = String::new();
    let _ = write!(text, "----------------------{LINE_ENDING}");
    for mail in mails {
        let _ = write!(text, "To: {}{LINE_ENDING}Subject: {}{LINE_ENDING}{}{LINE_ENDING}----------------------{LINE_ENDING}",
            mail.to, mail.subject, mail.body);
    }
    if simulate {
        println!("Results would be writen to '{file_path}' and are as follows:\n{text}");
    } else {
        writer
            .write(file_path, text)
            .map_err(|err| format!("Could not write to output file: {err}"))?;
    }
    Ok(())
}

#[cfg_attr(test, automock)]
trait WritesFile {
    fn write(&self, file_path: &str, text: String) -> Result<(), std::io::Error>;
}
struct FileWriter {}
impl WritesFile for FileWriter {
    fn write(&self, file_path: &str, text: String) -> Result<(), std::io::Error> {
        fs::write(file_path, text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn writes() {
        let mut writer = MockWritesFile::new();
        writer
            .expect_write()
            .withf(move |file_path, text| {
                file_path == "my_path"
                    && text
                        == "----------------------
To: a to
Subject: a subject
a body
----------------------
"
            })
            .times(1)
            .return_once(|_, _| Ok(()));
        let wrote = write_mails(
            &writer,
            vec![Mail {
                body: "a body".to_string(),
                subject: "a subject".to_string(),
                to: "a to".to_string(),
            }],
            "my_path",
            false,
        );
        assert!(wrote.is_ok());
    }
}
