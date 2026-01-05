# Apache OpenDALâ„¢ Downloads

The official Apache OpenDAL releases are provided as source artifacts.

## Releases<a href="https://opendal.apache.org/download/#releases" class="hash-link" aria-label="Direct link to Releases" translate="no" title="Direct link to Releases">â€‹</a>

The latest source release is <a href="https://www.apache.org/dyn/closer.lua/opendal/0.54.1/apache-opendal-core-0.54.1-src.tar.gz?action=download" rel="noopener noreferrer" target="_blank">0.54.1</a> (<a href="https://downloads.apache.org/opendal/0.54.1/apache-opendal-core-0.54.1-src.tar.gz.asc" rel="noopener noreferrer" target="_blank">asc</a>, <a href="https://downloads.apache.org/opendal/0.54.1/apache-opendal-core-0.54.1-src.tar.gz.sha512" rel="noopener noreferrer" target="_blank">sha512</a>).

For older releases, please check the <a href="https://archive.apache.org/dist/opendal/" rel="noopener noreferrer" target="_blank">archive</a>.

For even older releases during the incubating phase, please check the <a href="https://archive.apache.org/dist/incubator/opendal/" rel="noopener noreferrer" target="_blank">incubator archive</a>.

## Notes<a href="https://opendal.apache.org/download/#notes" class="hash-link" aria-label="Direct link to Notes" translate="no" title="Direct link to Notes">â€‹</a>

- When downloading a release, please verify the OpenPGP compatible signature (or failing that, check the SHA-512); these should be fetched from the main Apache site.
- The KEYS file contains the public keys used for signing release. It is recommended that (when possible) a web of trust is used to confirm the identity of these keys.
- Please download the <a href="https://downloads.apache.org/opendal/KEYS" rel="noopener noreferrer" target="_blank">KEYS</a> as well as the .asc signature files.

### To verify the signature of the release artifact<a href="https://opendal.apache.org/download/#to-verify-the-signature-of-the-release-artifact" class="hash-link" aria-label="Direct link to To verify the signature of the release artifact" translate="no" title="Direct link to To verify the signature of the release artifact">â€‹</a>

You will need to download both the release artifact and the .asc signature file for that artifact. Then verify the signature by:

- Download the KEYS file and the .asc signature files for the relevant release artifacts.
- Import the KEYS file to your GPG keyring:

``` prism-code
gpg --import KEYS
```

- Verify the signature of the release artifact using the following command:

``` prism-code
gpg --verify <artifact>.asc <artifact>
```

### To verify the checksum of the release artifact<a href="https://opendal.apache.org/download/#to-verify-the-checksum-of-the-release-artifact" class="hash-link" aria-label="Direct link to To verify the checksum of the release artifact" translate="no" title="Direct link to To verify the checksum of the release artifact">â€‹</a>

You will need to download both the release artifact and the .sha512 checksum file for that artifact. Then verify the checksum by:

``` prism-code
shasum -a 512 -c <artifact>.sha512
```

- <a href="https://opendal.apache.org/download/#releases" class="table-of-contents__link toc-highlight">Releases</a>
- <a href="https://opendal.apache.org/download/#notes" class="table-of-contents__link toc-highlight">Notes</a>
  - <a href="https://opendal.apache.org/download/#to-verify-the-signature-of-the-release-artifact" class="table-of-contents__link toc-highlight">To verify the signature of the release artifact</a>
  - <a href="https://opendal.apache.org/download/#to-verify-the-checksum-of-the-release-artifact" class="table-of-contents__link toc-highlight">To verify the checksum of the release artifact</a>
