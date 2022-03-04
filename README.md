# basics

## Usage

### Add project manually
```bash
# cd to liveoverflow
cat ~/.ssh/id_ed25519.pub >> ssh-data/authorized_keys
mkdir -p repos/username/project.git
cd repos/username/project.git
git init --bare -b main && cd ../../..
```

### ssh server
```bash
podman build -t example -v $PWD/ssh-data:/etc/ssh-data:Z --squash . -f ssh.dockerfile
podman run -d --rm -p 2222:22 -v $PWD/repos:/tmp/repos:Z -v $PWD/ssh-data:/etc/ssh-data:Z localhost/example
```

### server

```bash
cargo run
# Started http server: 127.0.0.1:3000
```

### Try Git clone
`# git clone ssh://git@fedora:2222/tmp/repos/username/project.git`

### web client

- [https://localhost:3000/login](https://localhost:3000/login)
- [https://localhost:3000/register](https://localhost:3000/register)
- [https://localhost:3000/account](https://localhost:3000/account)
- [https://localhost:3000/home](https://localhost:8080/home)