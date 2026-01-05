- <a href="https://opendal.apache.org/" class="breadcrumbs__link" aria-label="Home page"><img src="out_opendal/community/release/index_media/edf54a765e27bedcc87f5708545b58efaaa38a1a.svg" class="breadcrumbHomeIcon_q3uS" /></a>
- <a href="https://opendal.apache.org/community/category/release/" class="breadcrumbs__link">Release</a>
- Create a release

On this page

# Create a release

This document mainly introduces how the release manager releases a new version of Apache OpenDALâ„¢ in accordance with the Apache requirements.

## Introduction<a href="https://opendal.apache.org/community/release/#introduction" class="hash-link" aria-label="Direct link to Introduction" translate="no" title="Direct link to Introduction">â€‹</a>

`Source Release` is the key point which Apache values, and is also necessary for an ASF release.

Please remember that publishing software has legal consequences.

This guide complements the foundation-wide policies and guides:

- <a href="https://www.apache.org/legal/release-policy.html" rel="noopener noreferrer" target="_blank">Release Policy</a>
- <a href="https://infra.apache.org/release-distribution" rel="noopener noreferrer" target="_blank">Release Distribution Policy</a>
- <a href="https://infra.apache.org/release-publishing.html" rel="noopener noreferrer" target="_blank">Release Creation Process</a>

## Some Terminology of release<a href="https://opendal.apache.org/community/release/#some-terminology-of-release" class="hash-link" aria-label="Direct link to Some Terminology of release" translate="no" title="Direct link to Some Terminology of release">â€‹</a>

In the context of our release, we use several terms to describe different stages of the release process.

Here's an explanation of these terms:

- `opendal_version`: the version of OpenDAL to be released, like `0.46.0`.
- `release_version`: the version of release candidate, like `0.46.0-rc.1`.
- `rc_version`: the minor version for voting round, like `rc.1`.
- `maven_artifact_number`: the number for Maven staging artifacts, like `1010`. The number can be found by searching "opendal" on <a href="https://repository.apache.org/#stagingRepositories" rel="noopener noreferrer" target="_blank">https://repository.apache.org/#stagingRepositories</a>. And the Maven staging artifacts will be created automatically when we push a git tag to GitHub for now.

## Preparation<a href="https://opendal.apache.org/community/release/#preparation" class="hash-link" aria-label="Direct link to Preparation" translate="no" title="Direct link to Preparation">â€‹</a>

![](out_opendal/community/release/index_media/0f0874621de662630fd5d7f09c89455c4ace26e6.svg)caution

This section is the requirements for individuals who are new to the role of release manager.

Refer to [Setup GPG Key](https://opendal.apache.org/community/release/reference/setup_gpg/) to make sure the GPG key has been set up.

## Start discussion about the next release<a href="https://opendal.apache.org/community/release/#start-discussion-about-the-next-release" class="hash-link" aria-label="Direct link to Start discussion about the next release" translate="no" title="Direct link to Start discussion about the next release">â€‹</a>

Start a discussion at <a href="https://github.com/apache/opendal/discussions/categories/general" rel="noopener noreferrer" target="_blank">OpenDAL Discussion General</a>:

Title:

``` prism-code
[DISCUSS] Release Apache OpenDAL ${release_version}
```

Content:

``` prism-code
Hello, Apache OpenDAL Community,

This is a call for a discussion to release Apache OpenDAL version ${opendal_version}.

The change lists about this release:

https://github.com/apache/opendal/compare/v${opendal_last_version}...main

Please leave your comments here about this release plan. We will bump the version in the repo and start the release process after the discussion.

Thanks

${name}
```

## Start a tracking issue about the next release<a href="https://opendal.apache.org/community/release/#start-a-tracking-issue-about-the-next-release" class="hash-link" aria-label="Direct link to Start a tracking issue about the next release" translate="no" title="Direct link to Start a tracking issue about the next release">â€‹</a>

Start a <a href="https://github.com/apache/opendal/issues/new?template=3-new-release.md" rel="noopener noreferrer" target="_blank">tracking issue on GitHub</a> for the upcoming release to track all tasks that need to be completed.

## Release List<a href="https://opendal.apache.org/community/release/#release-list" class="hash-link" aria-label="Direct link to Release List" translate="no" title="Direct link to Release List">â€‹</a>

Update the version list in the `dev/src/release/package.rs` file.

For example:

- If there is any breaking change, please bump the `minor` version instead of the `patch` version.
- If this package is not ready for release, please skip.

## GitHub Side<a href="https://opendal.apache.org/community/release/#github-side" class="hash-link" aria-label="Direct link to GitHub Side" translate="no" title="Direct link to GitHub Side">â€‹</a>

### Bump version in project<a href="https://opendal.apache.org/community/release/#bump-version-in-project" class="hash-link" aria-label="Direct link to Bump version in project" translate="no" title="Direct link to Bump version in project">â€‹</a>

Run `just update-version` to bump the version in the project.

### Update docs<a href="https://opendal.apache.org/community/release/#update-docs" class="hash-link" aria-label="Direct link to Update docs" translate="no" title="Direct link to Update docs">â€‹</a>

- Update `CHANGELOG.md`, refer to [Generate Release Note](https://opendal.apache.org/community/release/reference/generate_release_note/) for more information.
- Update `core/src/docs/upgrade.md` if there are breaking changes in `core`
- Make sure every released bindings' `upgrade.md` has been updated.
  - java: `bindings/java/upgrade.md`
  - node.js: `bindings/nodejs/upgrade.md`
  - python: `bindings/python/upgrade.md`

### Generate dependencies list<a href="https://opendal.apache.org/community/release/#generate-dependencies-list" class="hash-link" aria-label="Direct link to Generate dependencies list" translate="no" title="Direct link to Generate dependencies list">â€‹</a>

Download and setup `cargo-deny`. You can refer to <a href="https://embarkstudios.github.io/cargo-deny/cli/index.html" rel="noopener noreferrer" target="_blank">cargo-deny</a>.

Running `python3 ./scripts/dependencies.py generate` to update the dependency list of every package.

### Push release candidate tag<a href="https://opendal.apache.org/community/release/#push-release-candidate-tag" class="hash-link" aria-label="Direct link to Push release candidate tag" translate="no" title="Direct link to Push release candidate tag">â€‹</a>

After bump version PR gets merged, we can create a GitHub release for the release candidate:

- Create a tag at `main` branch on the `Bump Version` / `Patch up version` commit: `git tag -s "v0.46.0-rc.1"`, please correctly check out the corresponding commit instead of directly tagging on the main branch.
- Push tags to GitHub: `git push --tags`.

![](out_opendal/community/release/index_media/5a72c3ee6a9cc14296f901057d6eabf2b9b712b4.svg)note

Pushing a Git tag to GitHub repo will trigger a GitHub Actions workflow that creates a staging Maven release on <a href="https://repository.apache.org" rel="noopener noreferrer" target="_blank">https://repository.apache.org</a> which can be verified on voting.

### Check the GitHub action status<a href="https://opendal.apache.org/community/release/#check-the-github-action-status" class="hash-link" aria-label="Direct link to Check the GitHub action status" translate="no" title="Direct link to Check the GitHub action status">â€‹</a>

After pushing the tag, we need to check the GitHub action status to make sure the release candidate is created successfully.

- Python: <a href="https://github.com/apache/opendal/actions/workflows/ci_bindings_python.yml" rel="noopener noreferrer" target="_blank">Bindings Python CI</a>
- Java: <a href="https://github.com/apache/opendal/actions/workflows/ci_bindings_java.yml" rel="noopener noreferrer" target="_blank">Bindings Java CI</a> and <a href="https://github.com/apache/opendal/actions/workflows/release_java.yml" rel="noopener noreferrer" target="_blank">Bindings Java Release</a>
- Node.js: <a href="https://github.com/apache/opendal/actions/workflows/ci_bindings_nodejs.yml" rel="noopener noreferrer" target="_blank">Bindings Node.js CI</a>

In the most cases, it would be great to rerun the failed workflow directly when you find some failures. But if a new code patch is needed to fix the failure, you should create a new release candidate tag, increase the rc number and push it to GitHub.

## ASF Side<a href="https://opendal.apache.org/community/release/#asf-side" class="hash-link" aria-label="Direct link to ASF Side" translate="no" title="Direct link to ASF Side">â€‹</a>

If any step in the ASF Release process fails and requires code changes, we will abandon that version and prepare for the next one. Our release page will only display ASF releases instead of GitHub Releases.

Additionally, we should also drop the staging Maven artifacts on <a href="https://repository.apache.org" rel="noopener noreferrer" target="_blank">https://repository.apache.org</a>.

### Create an ASF Release<a href="https://opendal.apache.org/community/release/#create-an-asf-release" class="hash-link" aria-label="Direct link to Create an ASF Release" translate="no" title="Direct link to Create an ASF Release">â€‹</a>

After GitHub Release has been created, we can start to create ASF Release.

- Checkout to released tag. (e.g. `git checkout v0.46.0-rc.1`, tag is created in the previous step)
- Use the release script to create a new release: `just release`
  - This script will generate the release candidate artifacts under `dist`, including:
    - `apache-opendal-{package}-{version}-src.tar.gz`
    - `apache-opendal-{package}-{version}-src.tar.gz.asc`
    - `apache-opendal-{package}-{version}-src.tar.gz.sha512`
- Push the newly created branch to GitHub

This script will create a new release under `dist`.

For example:

``` prism-code
dist
âââ apache-opendal-bindings-c-0.44.2-src.tar.gz
âââ apache-opendal-bindings-c-0.44.2-src.tar.gz.asc
âââ apache-opendal-bindings-c-0.44.2-src.tar.gz.sha512
...
âââ apache-opendal-core-0.45.0-src.tar.gz
âââ apache-opendal-core-0.45.0-src.tar.gz.asc
âââ apache-opendal-core-0.45.0-src.tar.gz.sha512
âââ apache-opendal-integrations-dav-server-0.0.0-src.tar.gz
âââ apache-opendal-integrations-dav-server-0.0.0-src.tar.gz.asc
âââ apache-opendal-integrations-dav-server-0.0.0-src.tar.gz.sha512
âââ apache-opendal-integrations-object_store-0.42.0-src.tar.gz
âââ apache-opendal-integrations-object_store-0.42.0-src.tar.gz.asc
âââ apache-opendal-integrations-object_store-0.42.0-src.tar.gz.sha512

1 directory, 60 files
```

### Upload artifacts to the SVN dist repo<a href="https://opendal.apache.org/community/release/#upload-artifacts-to-the-svn-dist-repo" class="hash-link" aria-label="Direct link to Upload artifacts to the SVN dist repo" translate="no" title="Direct link to Upload artifacts to the SVN dist repo">â€‹</a>

![](out_opendal/community/release/index_media/af50301a53fa555616a9b2279e7e25a1d566cb1a.svg)info

SVN is required for this step.

The svn repository of the dev branch is: <a href="https://dist.apache.org/repos/dist/dev/opendal" rel="noopener noreferrer" target="_blank">https://dist.apache.org/repos/dist/dev/opendal</a>

First, checkout OpenDAL to local directory:

``` prism-code
# As this step will copy all the versions, it will take some time. If the network is broken, please use svn cleanup to delete the lock before re-execute it.
svn co https://dist.apache.org/repos/dist/dev/opendal opendal-dist-dev
```

Then, upload the artifacts:

> The `${release_version}` here should be like `0.46.0-rc.1`

``` prism-code
cd opendal-dist-dev
# create a directory named by version
mkdir ${release_version}
# copy source code and signature package to the versioned directory
cp ${repo_dir}/dist/* ${release_version}/
# check svn status
svn status
# add to svn
svn add ${release_version}
# check svn status
svn status
# commit to SVN remote server
svn commit -m "Prepare for ${release_version}"
```

Visit <a href="https://dist.apache.org/repos/dist/dev/opendal/" rel="noopener noreferrer" target="_blank">https://dist.apache.org/repos/dist/dev/opendal/</a> to make sure the artifacts are uploaded correctly.

### Close the Nexus staging repo<a href="https://opendal.apache.org/community/release/#close-the-nexus-staging-repo" class="hash-link" aria-label="Direct link to Close the Nexus staging repo" translate="no" title="Direct link to Close the Nexus staging repo">â€‹</a>

To verify the Maven staging artifacts in the next step, close the Nexus staging repo as below.

1.  Open <a href="https://repository.apache.org/#stagingRepositories" rel="noopener noreferrer" target="_blank">https://repository.apache.org/#stagingRepositories</a> with your Apache ID login.
2.  Find the artifact `orgapacheopendal-${maven_artifact_number}`, click the "Close" button.

The `close` operation means that the artifacts are ready for voting.

![](out_opendal/community/release/index_media/0f0874621de662630fd5d7f09c89455c4ace26e6.svg)caution

If the vote failed, click "Drop" to drop the staging Maven artifacts.

### Rescue<a href="https://opendal.apache.org/community/release/#rescue" class="hash-link" aria-label="Direct link to Rescue" translate="no" title="Direct link to Rescue">â€‹</a>

If you accidentally published wrong or unexpected artifacts, like wrong signature files, wrong sha256 files, please cancel the release for the current `release_version`, *increase th RC counting* and re-initiate a release with the new `release_version`. And remember to delete the wrong artifacts from the SVN dist repo. Additionally, you should also drop the staging Maven artifacts on <a href="https://repository.apache.org" rel="noopener noreferrer" target="_blank">https://repository.apache.org</a>.

## Voting<a href="https://opendal.apache.org/community/release/#voting" class="hash-link" aria-label="Direct link to Voting" translate="no" title="Direct link to Voting">â€‹</a>

OpenDAL requires votes from both the OpenDAL Community.

Start a VOTE at <a href="https://github.com/apache/opendal/discussions/categories/general" rel="noopener noreferrer" target="_blank">OpenDAL Discussion General</a>:

Title:

``` prism-code
[VOTE] Release Apache OpenDAL ${release_version} - Vote Round 1
```

Content:

``` prism-code
Hello, Apache OpenDAL Community,

This is a call for a vote to release Apache OpenDAL version ${opendal_version}.

The tag to be voted on is ${opendal_version}.

The release candidate:

https://dist.apache.org/repos/dist/dev/opendal/${release_version}/

Keys to verify the release candidate:

https://downloads.apache.org/opendal/KEYS

Git tag for the release:

https://github.com/apache/opendal/releases/tag/v${release_version}

Maven staging repo:

https://repository.apache.org/content/repositories/orgapacheopendal-${maven_artifact_number}/

Pypi testing repo:

https://test.pypi.org/project/opendal/

Website staged:

https://opendal-v${release_version | replace('.', '-')}.staged.apache.org/

Please download, verify, and test.

The VOTE will be open for at least 72 hours and until the necessary
number of votes are reached.

- [ ] +1 approve
- [ ] +0 no opinion
- [ ] -1 disapprove with the reason

To learn more about apache opendal, please see https://opendal.apache.org/

Checklist for reference:

- [ ] Download links are valid.
- [ ] Checksums and signatures.
- [ ] LICENSE/NOTICE files exist
- [ ] No unexpected binary files
- [ ] All source files have ASF headers
- [ ] Can compile from source

Use our verify.py to assist in the verify process:

    svn co https://dist.apache.org/repos/dist/dev/opendal/${release_version}/ opendal-dev
    cd opendal-dev
    curl -sSL https://github.com/apache/opendal/raw/v${release_version}/scripts/verify.py -o verify.py
    python verify.py

Thanks

${name}
```

Example: <a href="https://github.com/apache/opendal/discussions/5211" rel="noopener noreferrer" target="_blank">https://github.com/apache/opendal/discussions/5211</a>

The vote should be open for **at least 72 hours** except the following cases:

1.  Security issues
2.  The wild user affected bug fixes
3.  Any other emergency cases

The Release manager should claim the emergency cases in the vote email if he wants to vote it rapidly.

> Tips: The 72 hours is the minimum time for voting, so we can ensure that community members from various time zones can participate in the verification process.

After at least 3 `+1` binding vote (<a href="https://people.apache.org/phonebook.html?project=opendal" rel="noopener noreferrer" target="_blank">from OpenDAL PMC member</a>) and more +1 bindings than -1 bindings, claim the vote result:

Title:

``` prism-code
[RESULT][VOTE] Release Apache OpenDAL ${release_version} - Vote Round 1
```

Content:

``` prism-code
Hello, Apache OpenDAL Community,

The vote to release Apache OpenDAL ${release_version} has passed.

The vote PASSED with 3 +1 binding and 1 +1 non-binding votes, no +0 or -1 votes:

Binding votes:

- xxx
- yyy
- zzz

Non-Binding votes:

- aaa

Vote thread: ${vote_thread_url}

Thanks

${name}
```

It's better to use the real name or the public name which is displayed on the voters' profile page, or Apache ID of the voter, to show who voted in the vote result email, and avoid using nicknames, it will make the vote result hard for others to track the voter. We should make sure the binding votes are from the people who have the right to vote the binding one.

Example: <a href="https://lists.apache.org/thread/xk5myl10mztcfotn59oo59s4ckvojds6" rel="noopener noreferrer" target="_blank">https://lists.apache.org/thread/xk5myl10mztcfotn59oo59s4ckvojds6</a>

## Official Release<a href="https://opendal.apache.org/community/release/#official-release" class="hash-link" aria-label="Direct link to Official Release" translate="no" title="Direct link to Official Release">â€‹</a>

### Push the release git tag<a href="https://opendal.apache.org/community/release/#push-the-release-git-tag" class="hash-link" aria-label="Direct link to Push the release git tag" translate="no" title="Direct link to Push the release git tag">â€‹</a>

``` prism-code
# Checkout the tags that passed VOTE
git checkout ${release_version}
# Tag with the opendal version
git tag -s ${opendal_version}
# Push tags to GitHub to trigger releases
git push origin ${opendal_version}
```

### Publish artifacts to SVN RELEASE branch<a href="https://opendal.apache.org/community/release/#publish-artifacts-to-svn-release-branch" class="hash-link" aria-label="Direct link to Publish artifacts to SVN RELEASE branch" translate="no" title="Direct link to Publish artifacts to SVN RELEASE branch">â€‹</a>

``` prism-code
svn mv https://dist.apache.org/repos/dist/dev/opendal/${release_version} https://dist.apache.org/repos/dist/release/opendal/${opendal_version} -m "Release ${opendal_version}"
```

### Release Maven artifacts<a href="https://opendal.apache.org/community/release/#release-maven-artifacts" class="hash-link" aria-label="Direct link to Release Maven artifacts" translate="no" title="Direct link to Release Maven artifacts">â€‹</a>

1.  Open <a href="https://repository.apache.org/#stagingRepositories" rel="noopener noreferrer" target="_blank">https://repository.apache.org/#stagingRepositories</a>.
2.  Find the artifact `orgapacheopendal-${maven_artifact_number}`, click the "Release" button.

It will take some time to sync the Maven artifacts to the Maven Central.

![](out_opendal/community/release/index_media/0f0874621de662630fd5d7f09c89455c4ace26e6.svg)caution

If the vote failed, click "Drop" to drop the staging Maven artifacts.

### Check the language binding artifacts<a href="https://opendal.apache.org/community/release/#check-the-language-binding-artifacts" class="hash-link" aria-label="Direct link to Check the language binding artifacts" translate="no" title="Direct link to Check the language binding artifacts">â€‹</a>

We need to check the language binding artifacts in the language package repo to make sure they are released successfully.

- Python: <a href="https://pypi.org/project/opendal/" rel="noopener noreferrer" target="_blank">https://pypi.org/project/opendal/</a>
- Java: <a href="https://repository.apache.org/#nexus-search;quick~opendal" rel="noopener noreferrer" target="_blank">https://repository.apache.org/#nexus-search;quick~opendal</a>
- Node.js: <a href="https://www.npmjs.com/package/opendal" rel="noopener noreferrer" target="_blank">https://www.npmjs.com/package/opendal</a>

For Java binding, if we cannot find the latest version of artifacts in the repo, we need to check the `orgapacheopendal-${maven_artifact_number}` artifact status in staging repo.

For non-Java bindings, if we cannot find the latest version of artifacts in the repo, we need to check the GitHub action status.

### Create a GitHub Release<a href="https://opendal.apache.org/community/release/#create-a-github-release" class="hash-link" aria-label="Direct link to Create a GitHub Release" translate="no" title="Direct link to Create a GitHub Release">â€‹</a>

- Click <a href="https://github.com/apache/opendal/releases/new" rel="noopener noreferrer" target="_blank">here</a> to create a new release.
- Pick the git tag of this release version from the dropdown menu.
- Make sure the branch target is `main`.
- Generate the release note by clicking the `Generate release notes` button.
- Add the release note from every component's `upgrade.md` if there are breaking changes before the content generated by GitHub. Check them carefully.
- Publish the release.

### Send the announcement<a href="https://opendal.apache.org/community/release/#send-the-announcement" class="hash-link" aria-label="Direct link to Send the announcement" translate="no" title="Direct link to Send the announcement">â€‹</a>

Start an announcement to <a href="https://github.com/apache/opendal/discussions/categories/announcements" rel="noopener noreferrer" target="_blank">OpenDAL Discussion Announcements</a> and send the same content to `announce@apache.org`.

> Tips: Please follow the <a href="https://infra.apache.org/committer-email.html" rel="noopener noreferrer" target="_blank">Committer Email</a> guide to make sure you have already set up the email SMTP. Otherwise, your email cannot be sent to the announcement mailing list.

Instead of adding breaking changes, let's include the new features as "notable changes" in the announcement.

Title:

``` prism-code
[ANNOUNCE] Release Apache OpenDAL ${opendal_version}
```

Content:

``` prism-code
Hi all,

The Apache OpenDAL community is pleased to announce
that Apache OpenDAL ${opendal_version} has been released!

OpenDAL is a data access layer that allows users to easily and efficiently
retrieve data from various storage services in a unified way.

The notable changes since ${opendal_version} include:

1. xxxxx
2. yyyyyy
3. zzzzzz

Please refer to the change log for the complete list of changes:
https://github.com/apache/opendal/releases/tag/v${opendal_version}

Apache OpenDAL website: https://opendal.apache.org/

Download Links: https://opendal.apache.org/download

OpenDAL Resources:
- Issue: https://github.com/apache/opendal/issues
- Mailing list: dev@opendal.apache.org

Thanks
On behalf of Apache OpenDAL community
```

Example: <a href="https://lists.apache.org/thread/oy77n55brvk72tnlb2bjzfs9nz3cfd0s" rel="noopener noreferrer" target="_blank">https://lists.apache.org/thread/oy77n55brvk72tnlb2bjzfs9nz3cfd0s</a>

## Post release<a href="https://opendal.apache.org/community/release/#post-release" class="hash-link" aria-label="Direct link to Post release" translate="no" title="Direct link to Post release">â€‹</a>

After the official release out, you may perform a few post-actions.

### Remove the old releases<a href="https://opendal.apache.org/community/release/#remove-the-old-releases" class="hash-link" aria-label="Direct link to Remove the old releases" translate="no" title="Direct link to Remove the old releases">â€‹</a>

Remove the old releases if any. You only need the latest release there, and older releases are available through the Apache archive.

To clean up old releases, run:

``` prism-code
# 1. Get the list of releases
svn ls https://dist.apache.org/repos/dist/release/opendal
# 2. Delete each release (except for the last one)
svn del -m "Archiving OpenDAL release X.Y.Z" https://dist.apache.org/repos/dist/release/opendal/X.Y.Z
```

<a href="https://github.com/apache/opendal/tree/main/website/community/release/release.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_opendal/community/release/index_media/82254bca835e54e35c885c6543f8416b5aff021e.svg" class="iconEdit_evxu" />Edit this page</a>

<a href="https://opendal.apache.org/community/category/release/" class="pagination-nav__link pagination-nav__link--prev"></a>

Previous

Release

<a href="https://opendal.apache.org/community/release/verify/" class="pagination-nav__link pagination-nav__link--next"></a>

Next

Verify a release candidate

- <a href="https://opendal.apache.org/community/release/#introduction" class="table-of-contents__link toc-highlight">Introduction</a>
- <a href="https://opendal.apache.org/community/release/#some-terminology-of-release" class="table-of-contents__link toc-highlight">Some Terminology of release</a>
- <a href="https://opendal.apache.org/community/release/#preparation" class="table-of-contents__link toc-highlight">Preparation</a>
- <a href="https://opendal.apache.org/community/release/#start-discussion-about-the-next-release" class="table-of-contents__link toc-highlight">Start discussion about the next release</a>
- <a href="https://opendal.apache.org/community/release/#start-a-tracking-issue-about-the-next-release" class="table-of-contents__link toc-highlight">Start a tracking issue about the next release</a>
- <a href="https://opendal.apache.org/community/release/#release-list" class="table-of-contents__link toc-highlight">Release List</a>
- <a href="https://opendal.apache.org/community/release/#github-side" class="table-of-contents__link toc-highlight">GitHub Side</a>
  - <a href="https://opendal.apache.org/community/release/#bump-version-in-project" class="table-of-contents__link toc-highlight">Bump version in project</a>
  - <a href="https://opendal.apache.org/community/release/#update-docs" class="table-of-contents__link toc-highlight">Update docs</a>
  - <a href="https://opendal.apache.org/community/release/#generate-dependencies-list" class="table-of-contents__link toc-highlight">Generate dependencies list</a>
  - <a href="https://opendal.apache.org/community/release/#push-release-candidate-tag" class="table-of-contents__link toc-highlight">Push release candidate tag</a>
  - <a href="https://opendal.apache.org/community/release/#check-the-github-action-status" class="table-of-contents__link toc-highlight">Check the GitHub action status</a>
- <a href="https://opendal.apache.org/community/release/#asf-side" class="table-of-contents__link toc-highlight">ASF Side</a>
  - <a href="https://opendal.apache.org/community/release/#create-an-asf-release" class="table-of-contents__link toc-highlight">Create an ASF Release</a>
  - <a href="https://opendal.apache.org/community/release/#upload-artifacts-to-the-svn-dist-repo" class="table-of-contents__link toc-highlight">Upload artifacts to the SVN dist repo</a>
  - <a href="https://opendal.apache.org/community/release/#close-the-nexus-staging-repo" class="table-of-contents__link toc-highlight">Close the Nexus staging repo</a>
  - <a href="https://opendal.apache.org/community/release/#rescue" class="table-of-contents__link toc-highlight">Rescue</a>
- <a href="https://opendal.apache.org/community/release/#voting" class="table-of-contents__link toc-highlight">Voting</a>
- <a href="https://opendal.apache.org/community/release/#official-release" class="table-of-contents__link toc-highlight">Official Release</a>
  - <a href="https://opendal.apache.org/community/release/#push-the-release-git-tag" class="table-of-contents__link toc-highlight">Push the release git tag</a>
  - <a href="https://opendal.apache.org/community/release/#publish-artifacts-to-svn-release-branch" class="table-of-contents__link toc-highlight">Publish artifacts to SVN RELEASE branch</a>
  - <a href="https://opendal.apache.org/community/release/#release-maven-artifacts" class="table-of-contents__link toc-highlight">Release Maven artifacts</a>
  - <a href="https://opendal.apache.org/community/release/#check-the-language-binding-artifacts" class="table-of-contents__link toc-highlight">Check the language binding artifacts</a>
  - <a href="https://opendal.apache.org/community/release/#create-a-github-release" class="table-of-contents__link toc-highlight">Create a GitHub Release</a>
  - <a href="https://opendal.apache.org/community/release/#send-the-announcement" class="table-of-contents__link toc-highlight">Send the announcement</a>
- <a href="https://opendal.apache.org/community/release/#post-release" class="table-of-contents__link toc-highlight">Post release</a>
  - <a href="https://opendal.apache.org/community/release/#remove-the-old-releases" class="table-of-contents__link toc-highlight">Remove the old releases</a>
