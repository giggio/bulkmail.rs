use std::collections::HashMap;

use handlebars::Handlebars;

pub fn render(
    destinations: Vec<HashMap<String, String>>,
    subject: &str,
    template_file_path: &str,
    simulate: bool,
) -> Result<Vec<Mail>, String> {
    let mut handlebars = Handlebars::new();
    if simulate {
        handlebars
            .register_template_string(
                "t1",
                "<p>Hello, {{ name }}!</p>
<p>Email is: {{ email_address }}</p>
<p>{{ other_property }}</p>
Other text...",
            )
            .map_err(|err| format!("Error when parsing template: {}", err))?;
    } else {
        handlebars
            .register_template_file("t1", template_file_path)
            .map_err(|err| format!("Error when parsing template: {}", err))?;
    }
    let mut mails: Vec<Mail> = vec![];
    for destination in destinations.into_iter() {
        if destination.get("email_address").is_none() {
            return Err("Missing email_address field.".to_owned());
        }
        mails.push(Mail {
            to: destination["email_address"].clone(),
            subject: subject.to_owned(),
            body: handlebars
                .render("t1", &destination)
                .map_err(|err| format!("Error when rendering template: {}", err))?,
        })
    }
    Ok(mails)
}

pub struct Mail {
    pub body: String,
    pub to: String,
    pub subject: String,
}
