- <a href="https://opendal.apache.org/" class="breadcrumbs__link" aria-label="Home page"><img src="out_opendal/community/release/verify/index_media/edf54a765e27bedcc87f5708545b58efaaa38a1a.svg" class="breadcrumbHomeIcon_q3uS" /></a>
- <a href="https://opendal.apache.org/community/category/release/" class="breadcrumbs__link">Release</a>
- Verify a release candidate

On this page

# Verify a release candidate

To verify a release candidate, the following checklist could be used:

- [ ] Download links are valid.
- [ ] Checksums and signatures.
- [ ] LICENSE/NOTICE files exist.
- [ ] No unexpected binary files.
- [ ] All source files have ASF headers.
- [ ] Can compile from source.

![](out_opendal/community/release/verify/index_media/5a72c3ee6a9cc14296f901057d6eabf2b9b712b4.svg)note

It is NOT necessary to run all checks to cast a vote for a release candidate.

However, you should clearly state which checks you did. The release manager needs to ensure that each check was done.

## Download links are valid<a href="https://opendal.apache.org/community/release/verify/#download-links-are-valid" class="hash-link" aria-label="Direct link to Download links are valid" translate="no" title="Direct link to Download links are valid">â€‹</a>

To verify the release candidate, you need to download the release candidate from the <a href="https://dist.apache.org/repos/dist/dev/opendal/" rel="noopener noreferrer" target="_blank">dist</a> directory.

Our current distribution contains many files, which we recommend downloading using svn.

Use the following command to download all artifacts, replace "\${release_version}-\${rc_version}" with the version ID of the version to be released:

``` prism-code
svn co https://dist.apache.org/repos/dist/dev/opendal/${release_version}-${rc_version}/
```

## Checksums and signatures<a href="https://opendal.apache.org/community/release/verify/#checksums-and-signatures" class="hash-link" aria-label="Direct link to Checksums and signatures" translate="no" title="Direct link to Checksums and signatures">â€‹</a>

Every file in a release candidate should have a checksum and signature file.

For example, if the release candidate is `0.46.0-rc1`, the checksum and signature file should be like:

``` prism-code
https://dist.apache.org/repos/dist/dev/opendal/0.46.0-rc1/apache-opendal-core-0.46.0-rc1-src.tar.gz.sha512
https://dist.apache.org/repos/dist/dev/opendal/0.46.0-rc1/apache-opendal-core-0.46.0-rc1-src.tar.gz.asc
```

### Verify checksums and signatures<a href="https://opendal.apache.org/community/release/verify/#verify-checksums-and-signatures" class="hash-link" aria-label="Direct link to Verify checksums and signatures" translate="no" title="Direct link to Verify checksums and signatures">â€‹</a>

GnuPG is recommended here. It can be installed with the following command:

``` prism-code
apt-get install gnupg
# or
yum install gnupg
# or
brew install gnupg
```

Firstly, import the OpenDAL release manager's public key:

``` prism-code
curl https://downloads.apache.org/opendal/KEYS > KEYS # Download KEYS
gpg --import KEYS # Import KEYS to local
```

Then, trust the public key:

``` prism-code
gpg --edit-key <KEY-used-in-this-version> # Edit the key
```

It will enter the interactive mode, use the following command to trust the key:

``` prism-code
gpg> trust
```

And then, select the level of trust, for example:

``` prism-code
Please decide how far you trust this user to correctly verify other users' keys
(by looking at passports, checking fingerprints from different sources, etc.)

  1 = I don't know or won't say
  2 = I do NOT trust
  3 = I trust marginally
  4 = I trust fully
  5 = I trust ultimately
  m = back to the main menu
```

Select `5` to trust the key ultimately.

Now, we could start the verification.

We've provided a script to verify the checksum and signature of the release candidate.

The script is in the `scripts` directory of our repository. You can download it directly from <a href="https://raw.githubusercontent.com/apache/opendal/main/scripts/verify.py" rel="noopener noreferrer" target="_blank">here</a>. Please put it in the same directory as the release candidate.

Run the script in a specific release candidate's folder:

``` prism-code
python ./verify.py
```

You will see the following output if the verification is successful:

``` prism-code
$ python ./verify.py
> Checking apache-opendal-bin-oli-0.41.3-src.tar.gz
gpg: Signature made äº  6/ 7 20:57:06 2024 CST
gpg:                using RSA key 8B374472FAD328E17F479863B379691FC6E298DD
gpg: Good signature from "Zili Chen (CODE SIGNING KEY) <tison@apache.org>" [unknown]
gpg: WARNING: This key is not certified with a trusted signature!
gpg:          There is no indication that the signature belongs to the owner.
Primary key fingerprint: 8B37 4472 FAD3 28E1 7F47  9863 B379 691F C6E2 98DD
> Success to verify the gpg sign for apache-opendal-bin-oli-0.41.3-src.tar.gz
apache-opendal-bin-oli-0.41.3-src.tar.gz: OK
> Success to verify the checksum for apache-opendal-bin-oli-0.41.3-src.tar.gz
.......
> Start checking LICENSE file in /Users/yan/Downloads/opendal-dev/apache-opendal-0.47.0-src
> LICENSE file exists in /Users/yan/Downloads/opendal-dev/apache-opendal-0.47.0-src
> Start checking NOTICE file in /Users/yan/Downloads/opendal-dev/apache-opendal-0.47.0-src
> NOTICE file exists in /Users/yan/Downloads/opendal-dev/apache-opendal-0.47.0-src
cargo 1.78.0 (54d8815d0 2024-03-26)
Start building opendal core
Success to build opendal core
openjdk version "22.0.1" 2024-04-16
OpenJDK Runtime Environment Temurin-22.0.1+8 (build 22.0.1+8)
OpenJDK 64-Bit Server VM Temurin-22.0.1+8 (build 22.0.1+8, mixed mode)
Start building opendal java binding
> Success to build opendal java binding
```

## Check the file content of the source package<a href="https://opendal.apache.org/community/release/verify/#check-the-file-content-of-the-source-package" class="hash-link" aria-label="Direct link to Check the file content of the source package" translate="no" title="Direct link to Check the file content of the source package">â€‹</a>

Unzip `apache-opendal-${release_version}-${rc_version}-src.tar.gz` and check the follows:

- LICENSE and NOTICE files are correct for the repository.
- All files have ASF license headers if necessary.
- Building is OK.

## Check the Maven artifacts of opendal-java<a href="https://opendal.apache.org/community/release/verify/#check-the-maven-artifacts-of-opendal-java" class="hash-link" aria-label="Direct link to Check the Maven artifacts of opendal-java" translate="no" title="Direct link to Check the Maven artifacts of opendal-java">â€‹</a>

Download the artifacts from `https://repository.apache.org/content/repositories/orgapacheopendal-${maven_artifact_number}/`.

You can check the follows:

- Checksum of JARs matches the bundled checksum file.
- Signature of JARs matches the bundled signature file.
- JARs are reproducible locally. This means you can build the JARs on your machine and verify the checksum is the same with the bundled one.

The reproducibility requires the same JDK distribution and the same Maven distribution. You should use <a href="https://adoptium.net/temurin/releases/?version=8" rel="noopener noreferrer" target="_blank">Eclipse Temurin JDK 8</a> and the bundled Maven Wrapper to make the same artifacts.

<a href="https://github.com/apache/opendal/tree/main/website/community/release/verify.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_opendal/community/release/verify/index_media/82254bca835e54e35c885c6543f8416b5aff021e.svg" class="iconEdit_evxu" />Edit this page</a>

<a href="https://opendal.apache.org/community/release/" class="pagination-nav__link pagination-nav__link--prev"></a>

Previous

Create a release

<a href="https://opendal.apache.org/community/category/reference/" class="pagination-nav__link pagination-nav__link--next"></a>

Next

Reference

- <a href="https://opendal.apache.org/community/release/verify/#download-links-are-valid" class="table-of-contents__link toc-highlight">Download links are valid</a>
- <a href="https://opendal.apache.org/community/release/verify/#checksums-and-signatures" class="table-of-contents__link toc-highlight">Checksums and signatures</a>
  - <a href="https://opendal.apache.org/community/release/verify/#verify-checksums-and-signatures" class="table-of-contents__link toc-highlight">Verify checksums and signatures</a>
- <a href="https://opendal.apache.org/community/release/verify/#check-the-file-content-of-the-source-package" class="table-of-contents__link toc-highlight">Check the file content of the source package</a>
- <a href="https://opendal.apache.org/community/release/verify/#check-the-maven-artifacts-of-opendal-java" class="table-of-contents__link toc-highlight">Check the Maven artifacts of opendal-java</a>
