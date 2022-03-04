# git clone ssh://git@fedora:2222/tmp/repos/username/project.git

# cat ~/.ssh/id_ed25519.pub >> ssh-data/authorized_keys
# mkdir -p repos/username/project.git
# cd repos/username/project.git
# git init --bare -b main && cd ../../..

# podman build -t example -v $PWD/ssh-data:/etc/ssh-data:Z --squash . -f ssh.dockerfile
# podman run -d --rm -p 2222:22 -v $PWD/repos:/tmp/repos:Z -v $PWD/ssh-data:/etc/ssh-data:Z localhost/example

FROM alpine:latest

RUN apk update && apk add --no-cache git openssh

RUN adduser -s /bin/sh -D git && echo "git:$(( $RANDOM % 50 + 1 ))" | chpasswd


RUN ssh-keygen -t rsa -f /etc/ssh/ssh_host_rsa_key -q -P ""
RUN cat /etc/ssh/ssh_host_rsa_key > /tmp/id_rsa
RUN cat /etc/ssh/ssh_host_rsa_key.pub > /tmp/id_rsa.pub
RUN echo 'AuthorizedKeysFile /etc/ssh-data/authorized_keys' > /etc/ssh/sshd_config

RUN cat /etc/ssh/ssh_host_rsa_key.pub >> /etc/ssh-data/authorized_keys

USER git

RUN install -d -m 0700 -o git ~/.ssh

RUN cat /tmp/id_rsa > ~/.ssh/id_rsa
RUN cat /tmp/id_rsa.pub > ~/.ssh/id_rsa.pub


RUN chmod 0600 ~/.ssh/*

RUN eval `ssh-agent -s` && ssh-agent $SHELL && ssh-add ~/.ssh/id_rsa


RUN ssh-keygen -lf /etc/ssh-data/authorized_keys

USER root
EXPOSE 22
CMD ["/usr/sbin/sshd", "-D"]