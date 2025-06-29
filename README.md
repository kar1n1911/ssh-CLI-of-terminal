# ssh CLI for terminal
ssh CLI for terminals. Maybe a application later......

Why?

The termius try to charge me for 10$ just for sync! And I can not even get my data export due to what they called security reasons!

So now I want to write this thing with rust.

Below are what I think.

1. change different rsa keys for different entries, and save credentials with encoding.(while maybe I would not add the encoding functions at this very beginning.)
2. ssh......Maybe it's a good idea to put one inside this in case some systems don't install the ssh originally.
3. cross-platform key synchronisation, I can write one of ip I suppose.
4. export credentials with json format in case of computer reset.

Future development?

Nah.

maybe a interface, and a portable server?

I'm not good at web and anything about interface.

So everyone can write a interface, PLEASE, HELP YOURSELF.

Plus, I don't have any server or other computer for me to test this thing, so you know where to find me for any issues.

And, I would only deal with proper use with wrong outcome.

You use it the wrong way and want me to handle that? You can do it yourself, don't bother me.

And about self build.

```bash
    cargo build --release
```

then find your compiled file at target/release/ssh-cli.


# Modules

`config`: Handles configuration management and entry encoding.

`ssh_client`: Manages SSH connections.

`sync`: Handles synchronization logic.

# How to use

 `add`: Add or update an SSH entry with name, IP, username, authentication mode, and credentials.

The auth mode have three types, password, RSA, and both.

You need to input the RSA code just the same as the password.

This is the most complicated method of all the methods.

If you want to auth with password:

```bash
ssh_cli add myserver 192.168.1.10 user password --password mypass
```



If you want to use RSA without password

```bash
ssh_cli add myserver 192.168.1.10 user rsa --rsa myrsa
```



If you want to use both

```bash
ssh_cli add myserver 192.168.1.10 user both --password mypass --rsa myrsa
```





`remove`: Remove an SSH entry by name. Not revcoverable.

```bash
Ssh_cli remove myserver
```

`list`: List all stored SSH entries. Will not list you any password.

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

`import`: Import configuration from a specified file path. And maybe it will only import, not add.

Use it wisely.

```bash
ssh_cli import backup.json
```

`sync-send`: Send the configuration to a target IP for synchronization. NOT TESTED.

```bash
ssh_cli sync-send 192.168.1.20
```

`sync-recv`: Receive configuration from another device (blocks until received). NOT TESTED.

```bash
ssh_cli sync-recv
```



