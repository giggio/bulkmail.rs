FROM alpine:3.14 as bins
ARG PLATFORM
COPY target/output/bulkmail /app/bulkmail
RUN apk add binutils && strip /app/bulkmail

FROM scratch
LABEL maintainer="giggio@giggio.net"
ENTRYPOINT [ "/bulkmail" ]
COPY --from=bins /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
COPY --from=bins /app/bulkmail .
