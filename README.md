# Bulkmail

[![Docker Stars](https://img.shields.io/docker/stars/giggio/bulkmail.svg)](https://hub.docker.com/r/giggio/bulkmail/)
[![Docker Pulls](https://img.shields.io/docker/pulls/giggio/bulkmail.svg)](https://hub.docker.com/r/giggio/bulkmail/)
[![ImageLayers](https://images.microbadger.com/badges/image/giggio/bulkmail.svg)](https://microbadger.com/#/images/giggio/bulkmail)

This app reads a csv file and sends an email with a formatted
[Handlebars](https://handlebarsjs.com/) file.

This can be run on Linux for AMD64 and ARMv7.

## Upstream Links

* Docker Registry @ [giggio/bulkmail](https://hub.docker.com/r/giggio/bulkmail/)
* GitHub @ [giggio/bulkmail](https://github.com/giggio/bulkmail)

## Quick Start

You need to mount a volume somewhere, and the files will be read/writen there.
Run it like this:

````bash
docker run --rm -ti -v `pwd`/data:/data giggio/bulkmail # arguments
````

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

#### Alerting

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
