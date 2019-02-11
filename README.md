SSH Keys from Remotes
=====================

[![Build Status](https://travis-ci.com/liamdawson/ssh-keys-from-remotes.svg?branch=master)](https://travis-ci.com/liamdawson/ssh-keys-from-remotes)

_**Disclaimer:** No serious security testing has been performed. Use at your own risk!_

Allow ssh login via keys from an external source (e.g. GitHub).

Installation
------------

The preferred approach is to use the repository (to ensure updates come
through in a timely fashion):

```shell
# Debian-based distros (using apt):

sudo apt-key adv --fetch-keys http://ssh-keys-from-remotes-repo.s3-website-us-east-1.amazonaws.com/signing-key.pgp
sudo apt-add-repository 'deb http://ssh-keys-from-remotes-repo.s3-website-us-east-1.amazonaws.com/ stable main'
sudo apt install ssh-keys-from-remotes
```

Binary and `deb` releases are also published to [GitHub Releases](https://github.com/liamdawson/ssh-keys-from-remotes/releases)
whenever a new release is created.

Usage
-----

