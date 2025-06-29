# ssh CLI for terminal
ssh CLI for terminals. Maybe a application later......

for now I want to write this thing with rust.

Below are what I think.

1. change different rsa keys for different entries, and save credentials with encoding.(while maybe I would not add the encoding functions at this very beginning.)
2. ssh......Maybe it's a good idea to put one inside this in case some systems don't install the ssh originally.
3. cross-platform key synchronisation, I can write one of ip I suppose.
4. export credentials with json format incase computer reset.

Future development?

Nah.

maybe a interface, and a portable server?

I'm not good at web and anything about interface.

# Modules

`config`: Handles configuration management and entry encoding.

`ssh_client`: Manages SSH connections.

`sync`: Handles synchronization logic.

# How to use

 `add`: Add or update an SSH entry with name, IP, username, authentication mode, and credentials.

```bash
ssh_cli add myserver 192.168.1.10 user --password mypass --auth password
```

the auth mode have three types, password, RSA, and both.

You need to input the RSA code just the same as the password.

`remove`: Remove an SSH entry by name.

```bash
Ssh_cli remove myserver
```

`list`: List all stored SSH entries.

```bash
ssh_cli list
```

`connect`: Connect to a host using a stored entry.

```bash
ssh_cli connect myserver
```

`export`: Export the configuration to a specified file path.

```bash
ssh_cli export backup.json
```

`import`: Import configuration from a specified file path.

```bash
ssh_cli import backup.json
```

`sync-send`: Send the configuration to a target IP for synchronization.

```bash
ssh_cli sync-send 192.168.1.20
```

`sync-recv`: Receive configuration from another device (blocks until received).

```bash
ssh_cli sync-recv
```



