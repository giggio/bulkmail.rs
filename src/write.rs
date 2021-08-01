use std::fs;

use crate::{
    args::Write,
    destinations::get_destinations,
    render::{render, Mail},
};

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
    write_mails(mails, &write.output_file_path, write.simulate)?;
    Ok(())
}

fn write_mails(mails: Vec<Mail>, file_path: &str, simulate: bool) -> Result<(), String> {
    let mut text = String::new();
    text.push_str(&format!("----------------------{}", LINE_ENDING));
    for mail in mails {
        text.push_str(&format!(
            "To: {}{}Subject: {}{}{}{}----------------------{}",
            mail.to, LINE_ENDING, mail.subject, LINE_ENDING, mail.body, LINE_ENDING, LINE_ENDING
        ));
    }
    if simulate {
        println!(
            "Results would be writen to '{}' and are as follows:\n{}",
            file_path, text
        );
    } else {
        fs::write(file_path, text)
            .map_err(|err| format!("Could not write to output file: {}", err))?;
    }
    Ok(())
}
