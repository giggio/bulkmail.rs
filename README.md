# Bulkmail

[![Docker Stars](https://img.shields.io/docker/stars/giggio/bulkmail.svg)](https://hub.docker.com/r/giggio/bulkmail/)
[![Docker Pulls](https://img.shields.io/docker/pulls/giggio/bulkmail.svg)](https://hub.docker.com/r/giggio/bulkmail/)

This app reads a csv file and sends an email with a formatted
[Handlebars](https://handlebarsjs.com/) file.

This can be run on Linux for AMD64 and ARMv7.

## Upstream Links

* Docker Registry @ [giggio/bulkmail](https://hub.docker.com/r/giggio/bulkmail/)
* GitHub @ [giggio/bulkmail](https://github.com/giggio/bulkmail)

## Quick Start

You need to mount a volume somewhere, and the files will be read/writen there.
Run it like this (example for Gmail):

````bash
bulkmail send /path/to/input.csv "Subject here" /path/to/template.handlebars yourusername@gmail.com smtp.gmail.com:465 -u yourusername@gmail.com -p 'your password in here'
````

If the e-mail address is the same as the user you can omit it.

````bash
bulkmail send /path/to/input.csv "Subject here" /path/to/template.handlebars yourusername@gmail.com smtp.gmail.com:465 -p 'your password in here'
````

Or, in Docker:

````bash
docker run --rm -ti -v /path/to/files/:/data giggio/bulkmail # arguments
````

### Csv file details

The csv file has to have a header, and at least one column titled `email_address`.
See an example of a input file [here](https://github.com/giggio/bulkmail.rs/blob/main/test/input1.csv).

### Template file details

We are using Handlebars to render the message body. You can see an example that works with the above
csv file [here](https://github.com/giggio/bulkmail.rs/blob/main/test/input1.handlebars).

### Detailed commands

There are two commands: `send` and `write`. The former sends the message, the
latter writes a text file with the outputs and sends nothing.

All commands have a `-v` option for verbose output, and you can get help by
running `docker run --rm giggio/bulkmail --help`.

#### Sending bulk e-mails

To view available args run:

````bash
docker run --rm giggio/bulkmail send --help
````

This command has a simulated argument, which will make it not send the e-mail
but write to stdout instead.

#### Writing e-mails to a file

To view available args run:

````bash
docker run --rm giggio/bulkmail write --help
````

This command has a simulated argument, which will not read or write from/to
any files, instead it will read from an in-memory csv and write to stdout.

#### E-mail options

Commands that send e-mail will do so using SMTP. You have to supply the values
like server, port, sender and destination e-mail addresses etc. Authentication
information is optional, but most mail servers will require it.

## Contributing

Questions, comments, bug reports, and pull requests are all welcome.  Submit them at
[the project on GitHub](https://github.com/giggio/bulkmail/).

Bug reports that include steps-to-reproduce (including code) are the
best. Even better, make them in the form of pull requests.

## Author

[Giovanni Bassi](https://github.com/giggio)

## License

Licensed under the MIT license.
