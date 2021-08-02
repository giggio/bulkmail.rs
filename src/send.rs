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
    if simulate {
        println!("----------------------");
    }
    for mail_message in mails {
        if simulate {
            println!(
                "Would send message:
To: {}
Subject: {}
Body:
{}
----------------------",
                &mail_message.to, &mail_message.subject, &mail_message.body
            )
        } else {
            mail::send_mail(
                simulate,
                &mail_message.to,
                &mail_message.subject,
                &mail_message.body,
                &smtp,
            )?;
        }
    }
    Ok(())
}
