SSH Keys from Remotes
=====================

## Warning: Still unstable—before 1.0, automatic updates may lock you out of systems.

[![Build Status](https://travis-ci.com/liamdawson/ssh-keys-from-remotes.svg?branch=master)](https://travis-ci.com/liamdawson/ssh-keys-from-remotes)

_**Disclaimer:** No serious security testing has been performed. Use at your own risk!_

Allow ssh login via keys automatically updated from an external source (e.g. GitHub).

Rationale
---------

It can be challenging keeping SSH keys up to date across infrequently accessed machines.
On the other hand, services like GitHub and Gitlab are easier to remember, and publish the SSH
public keys for each user via easily-accessed URLs. Alternatively, you can centralise your
public keys in a place you control and rely upon that.

`ssh-keys-from-remotes` accepts a basic mapping of a UNIX user to remote lists of keys, and
fetches these **when an SSH login is attempted** for mapped users. It also **caches** keys
when accessed, to ensure the keys are still recognised when there are connectivity issues.

This model introduces new points of attack in your system, but may be an acceptable tradeoff
vs the risk of a) losing access to infrequently accessed machines, or b) accidentally leaving
compromised public keys on the system.

There are better ways to accomplish this if you have access to provisioning systems like Chef
or Ansible, or if you want to enable access for GitHub/Gitlab teams. Otherwise, this tool
provides a low-maintenance, low friction means of keeping your keys up to date.

Installation
------------

The preferred approach is to use the repository (to ensure unattended updates work):

```shell
# Debian-based distros (using apt):

sudo apt-key adv --fetch-keys https://repo.skfr.ldaws.com/signing-key.pgp
sudo apt-add-repository 'deb https://repo.skfr.ldaws.com/ stable main'
sudo apt install ssh-keys-from-remotes
```

Binary and `deb` releases are also published to [GitHub Releases](https://github.com/liamdawson/ssh-keys-from-remotes/releases)
whenever a new release is created. Support for other environments is not planned at this
time, but is negotiable.

Usage
-----

Replace the following lines in `/etc/ssh/sshd_config`:

```diff
-#AuthorizedKeysCommand none
-#AuthorizedKeysCommandUser nobody
+AuthorizedKeysCommand /usr/bin/ssh-keys-from-remotes pull %u
+AuthorizedKeysCommandUser ssh_keys_from_remotes
```

Edit `/etc/ssh-keys-remotes.toml` to map local users to remote keys. You may use
multiple urls per user in an array format (e.g. allowing multiple GitHub users to log in
to one local user account).

**Please** only use `https://` urls, or you risk _easily_ letting an attacker control
the list of valid keys.

Finally, **reload** your SSH server with `sudo systemctl reload ssh`, and test your SSH
configuration in a new terminal before closing the current connection.

Planned Work
------------

* [ ] Add tests to confirm and document expected behaviour
* [ ] Add convenience config properties for GitHub and Gitlab
* [ ] Support TTLs on cache
