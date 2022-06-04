FROM ubuntu:22.04

ADD install.sh /tmp

RUN chmod u+x /tmp/install.sh
RUN /tmp/install.sh