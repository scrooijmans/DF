- <a href="https://opendal.apache.org/" class="breadcrumbs__link" aria-label="Home page"><img src="out_opendal/community/release/reference/setup_gpg/index_media/edf54a765e27bedcc87f5708545b58efaaa38a1a.svg" class="breadcrumbHomeIcon_q3uS" /></a>
- <a href="https://opendal.apache.org/community/category/release/" class="breadcrumbs__link">Release</a>
- <a href="https://opendal.apache.org/community/category/reference/" class="breadcrumbs__link">Reference</a>
- Setup GPG key

On this page

# Setup GPG key

![](out_opendal/community/release/reference/setup_gpg/index_media/5a72c3ee6a9cc14296f901057d6eabf2b9b712b4.svg)note

This section is a brief from the <a href="https://infra.apache.org/openpgp.html" rel="noopener noreferrer" target="_blank">Cryptography with OpenPGP</a> guideline.

## Install GPG<a href="https://opendal.apache.org/community/release/reference/setup_gpg/#install-gpg" class="hash-link" aria-label="Direct link to Install GPG" translate="no" title="Direct link to Install GPG">â€‹</a>

For more details, please refer to <a href="https://www.gnupg.org/download/index.html" rel="noopener noreferrer" target="_blank">GPG official website</a>. Here shows one approach to install GPG with `apt`:

``` prism-code
sudo apt install gnupg2
```

## Generate GPG Key<a href="https://opendal.apache.org/community/release/reference/setup_gpg/#generate-gpg-key" class="hash-link" aria-label="Direct link to Generate GPG Key" translate="no" title="Direct link to Generate GPG Key">â€‹</a>

Attentions:

- Name is best to keep consistent with your full name of Apache ID;
- Email should be the Apache email;
- Name is best to only use English to avoid garbled.

Run `gpg --full-gen-key` and complete the generation interactively:

``` prism-code
gpg (GnuPG) 2.2.20; Copyright (C) 2020 Free Software Foundation, Inc.
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.

Please select what kind of key you want:
   (1) RSA and RSA (default)
   (2) DSA and Elgamal
   (3) DSA (sign only)
   (4) RSA (sign only)
  (14) Existing key from card
Your selection? 1 # input 1
RSA keys may be between 1024 and 4096 bits long.
What keysize do you want? (2048) 4096 # input 4096
Requested keysize is 4096 bits
Please specify how long the key should be valid.
         0 = key does not expire
      <n>  = key expires in n days
      <n>w = key expires in n weeks
      <n>m = key expires in n months
      <n>y = key expires in n years
Key is valid for? (0) 0 # input 0
Key does not expire at all
Is this correct? (y/N) y # input y

GnuPG needs to construct a user ID to identify your key.

Real name: Hulk Lin               # input your name
Email address: hulk@apache.org    # input your email
Comment:                          # input some annotations, optional
You selected this USER-ID:
    "Hulk <hulk@apache.org>"

Change (N)ame, (C)omment, (E)mail or (O)kay/(Q)uit? O # input O
We need to generate a lot of random bytes. It is a good idea to perform
some other action (type on the keyboard, move the mouse, utilize the
disks) during the prime generation; this gives the random number
generator a better chance to gain enough entropy.
We need to generate a lot of random bytes. It is a good idea to perform
some other action (type on the keyboard, move the mouse, utilize the
disks) during the prime generation; this gives the random number
generator a better chance to gain enough entropy.

# Input the security key
ââââââââââââââââââââââââââââââââââââââââââââââââââââââââ
â Please enter this passphrase                         â
â                                                      â
â Passphrase: _______________________________          â
â                                                      â
â       <OK>                              <Cancel>     â
ââââââââââââââââââââââââââââââââââââââââââââââââââââââââ
# key generation will be done after your inputting the key with the following output
gpg: key E49B00F626B marked as ultimately trusted
gpg: revocation certificate stored as '/Users/hulk/.gnupg/openpgp-revocs.d/F77B887A4F25A9468C513E9AA3008E49B00F626B.rev'
public and secret key created and signed.

pub   rsa4096 2022-07-12 [SC]
      F77B887A4F25A9468C513E9AA3008E49B00F626B
uid           [ultimate] hulk <hulk@apache.org>
sub   rsa4096 2022-07-12 [E]
```

## Upload your key to public GPG keyserver<a href="https://opendal.apache.org/community/release/reference/setup_gpg/#upload-your-key-to-public-gpg-keyserver" class="hash-link" aria-label="Direct link to Upload your key to public GPG keyserver" translate="no" title="Direct link to Upload your key to public GPG keyserver">â€‹</a>

Firstly, list your key:

``` prism-code
gpg --list-keys
```

The output is like:

``` prism-code
-------------------------------
pub   rsa4096 2022-07-12 [SC]
      F77B887A4F25A9468C513E9AA3008E49B00F626B
uid           [ultimate] hulk <hulk@apache.org>
sub   rsa4096 2022-07-12 [E]
```

Then, send your key id to key server:

``` prism-code
gpg --keyserver keys.openpgp.org --send-key <key-id> # e.g., F77B887A4F25A9468C513E9AA3008E49B00F626B
```

Among them, `keys.openpgp.org` is a randomly selected keyserver, you can use `keyserver.ubuntu.com` or any other full-featured keyserver.

## Check whether the key is created successfully<a href="https://opendal.apache.org/community/release/reference/setup_gpg/#check-whether-the-key-is-created-successfully" class="hash-link" aria-label="Direct link to Check whether the key is created successfully" translate="no" title="Direct link to Check whether the key is created successfully">â€‹</a>

Uploading takes about one minute; after that, you can check by your email at the corresponding keyserver.

Uploading keys to the keyserver is mainly for joining a <a href="https://infra.apache.org/release-signing.html#web-of-trust" rel="noopener noreferrer" target="_blank">Web of Trust</a>.

## Add your GPG public key to the KEYS document<a href="https://opendal.apache.org/community/release/reference/setup_gpg/#add-your-gpg-public-key-to-the-keys-document" class="hash-link" aria-label="Direct link to Add your GPG public key to the KEYS document" translate="no" title="Direct link to Add your GPG public key to the KEYS document">â€‹</a>

![](out_opendal/community/release/reference/setup_gpg/index_media/af50301a53fa555616a9b2279e7e25a1d566cb1a.svg)info

`SVN` is required for this step.

The svn repository of the release branch is: <a href="https://dist.apache.org/repos/dist/release/opendal" rel="noopener noreferrer" target="_blank">https://dist.apache.org/repos/dist/release/opendal</a>

Please always add the public key to KEYS in the release branch:

``` prism-code
svn co https://dist.apache.org/repos/dist/release/opendal opendal-dist
# As this step will copy all the versions, it will take some time. If the network is broken, please use svn cleanup to delete the lock before re-execute it.
cd opendal-dist
(gpg --list-sigs YOUR_NAME@apache.org && gpg --export --armor YOUR_NAME@apache.org) >> KEYS # Append your key to the KEYS file
svn add .   # It is not needed if the KEYS document exists before.
svn ci -m "add gpg key for YOUR_NAME" # Later on, if you are asked to enter a username and password, just use your apache username and password.
```

## Upload the GPG public key to your GitHub account<a href="https://opendal.apache.org/community/release/reference/setup_gpg/#upload-the-gpg-public-key-to-your-github-account" class="hash-link" aria-label="Direct link to Upload the GPG public key to your GitHub account" translate="no" title="Direct link to Upload the GPG public key to your GitHub account">â€‹</a>

- Enter <a href="https://github.com/settings/keys" rel="noopener noreferrer" target="_blank">https://github.com/settings/keys</a> to add your GPG key.
- Please remember to bind the email address used in the GPG key to your GitHub account (<a href="https://github.com/settings/emails" rel="noopener noreferrer" target="_blank">https://github.com/settings/emails</a>) if you find "unverified" after adding it.

<a href="https://github.com/apache/opendal/tree/main/website/community/release/reference/setup_gpg.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_opendal/community/release/reference/setup_gpg/index_media/82254bca835e54e35c885c6543f8416b5aff021e.svg" class="iconEdit_evxu" />Edit this page</a>

<a href="https://opendal.apache.org/community/release/reference/generate_release_note/" class="pagination-nav__link pagination-nav__link--prev"></a>

Previous

Generate release note

<a href="https://opendal.apache.org/community/category/committers/" class="pagination-nav__link pagination-nav__link--next"></a>

Next

Committers

- <a href="https://opendal.apache.org/community/release/reference/setup_gpg/#install-gpg" class="table-of-contents__link toc-highlight">Install GPG</a>
- <a href="https://opendal.apache.org/community/release/reference/setup_gpg/#generate-gpg-key" class="table-of-contents__link toc-highlight">Generate GPG Key</a>
- <a href="https://opendal.apache.org/community/release/reference/setup_gpg/#upload-your-key-to-public-gpg-keyserver" class="table-of-contents__link toc-highlight">Upload your key to public GPG keyserver</a>
- <a href="https://opendal.apache.org/community/release/reference/setup_gpg/#check-whether-the-key-is-created-successfully" class="table-of-contents__link toc-highlight">Check whether the key is created successfully</a>
- <a href="https://opendal.apache.org/community/release/reference/setup_gpg/#add-your-gpg-public-key-to-the-keys-document" class="table-of-contents__link toc-highlight">Add your GPG public key to the KEYS document</a>
- <a href="https://opendal.apache.org/community/release/reference/setup_gpg/#upload-the-gpg-public-key-to-your-github-account" class="table-of-contents__link toc-highlight">Upload the GPG public key to your GitHub account</a>
