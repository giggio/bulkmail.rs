use crate::{
    args::Send,
    args::Smtp,
    destinations::get_destinations,
    mail,
    render::{render, Mail},
};

pub fn send(send: Send) -> Result<(), Option<String>> {
    let destinations = get_destinations(&send.file_path, send.simulate)?;
    let mails = render(
        destinations,
        &send.subject,
        &send.template_file_path,
        send.simulate,
    )?;
    send_mails(mails, send.simulate, send.smtp)?;
    Ok(())
}

fn send_mails(mails: Vec<Mail>, simulate: bool, smtp: Smtp) -> Result<(), String> {
    for mail_message in mails {
        mail::send_mail(
            simulate,
            &mail_message.to,
            &mail_message.subject,
            &mail_message.body,
            &smtp,
        )?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    mod xx {
        use pretty_assertions::assert_eq;

        use super::super::*;
        #[test]
        fn average_calculated_with_single_item() {
            assert_eq!(true, true);
        }
    }
}
