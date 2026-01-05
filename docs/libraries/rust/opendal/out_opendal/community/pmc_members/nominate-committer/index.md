- <a href="https://opendal.apache.org/" class="breadcrumbs__link" aria-label="Home page"><img src="out_opendal/community/pmc_members/nominate-committer/index_media/edf54a765e27bedcc87f5708545b58efaaa38a1a.svg" class="breadcrumbHomeIcon_q3uS" /></a>
- <a href="https://opendal.apache.org/community/category/pmc-members/" class="breadcrumbs__link">PMC Members</a>
- Nominate Committer

On this page

# Nominate Committer

This document mainly introduces how a PMC member nominates a new committer.

## Start vote about the candidate<a href="https://opendal.apache.org/community/pmc_members/nominate-committer/#start-vote-about-the-candidate" class="hash-link" aria-label="Direct link to Start vote about the candidate" translate="no" title="Direct link to Start vote about the candidate">â€‹</a>

Start a vote about the candidate via sending email to: <a href="mailto:private@opendal.apache.org" rel="noopener noreferrer" target="_blank">private@opendal.apache.org</a>:

- candidate_name: The full name of the candidate.
- candidate_github_id: The GitHub id of the candidate.

Title:

``` prism-code
[VOTE] Add candidate ${candidate_name} as a new committer
```

Content:

``` prism-code
Hi, All OpenDAL PMC members.
  
I'd like to take this chance to call the vote for inviting committed
contributor ${candidate_name} (GitHub id: ${candidate_github_id}) as a new committer of Apache 
OpenDAL.

${candidate_contributions}

${candidate_name}'s great contributions can be found:

- GitHub Account: https://github.com/${candidate_github_id}
- GitHub Pull Requests: https://github.com/apache/opendal/pulls?q=is%3Apr+author%3A${candidate_github_id}
- GitHub Issues: https://github.com/apache/opendal/issues?q=is%3Aissue+involves%3A${candidate_github_id}

Please make your valuable evaluation on whether we could invite ${candidate_name} as a
committer:

[ +1 ] Agree to add ${candidate_name} as a committer of OpenDAL.
[  0 ] Have no sense.
[ -1 ] Disagree to add ${candidate_name} as a committer of OpenDAL, because .....

This vote starts from the moment of sending and will be open for 3 days.
 
Thanks and best regards,

${your_name}
```

Example: <a href="https://lists.apache.org/thread/j16lvkyrmvg8wyf3z4gqpjky5m594jhy" rel="noopener noreferrer" target="_blank">https://lists.apache.org/thread/j16lvkyrmvg8wyf3z4gqpjky5m594jhy</a> (Private Link)

After at least 3 `+1` binding vote and no veto, claim the vote result:

Title:

``` prism-code
[RESULT][VOTE] Add candidate ${candidate_name} as a new committer
```

Content:

``` prism-code
Hi, all:

The vote for "Add candidate ${candidate_name} as a new committer" has PASSED and closed now.

The result is as follows:

4 binding +1 Votes:
- voter names

Vote thread: https://lists.apache.org/thread/j16lvkyrmvg8wyf3z4gqpjky5m594jhy

Then I'm going to invite ${candidate_name} to join us.

Thanks for everyone's support!

${your_name}
```

## Send invitation to the candidate<a href="https://opendal.apache.org/community/pmc_members/nominate-committer/#send-invitation-to-the-candidate" class="hash-link" aria-label="Direct link to Send invitation to the candidate" translate="no" title="Direct link to Send invitation to the candidate">â€‹</a>

Send an invitation to the candidate and cc <a href="mailto:private@opendal.apache.org" rel="noopener noreferrer" target="_blank">private@opendal.apache.org</a>:

Title:

``` prism-code
Invitation to become OpenDAL Committer: ${candidate_name}
```

Content:

``` prism-code
Hello ${candidate_name},

The OpenDAL PMC hereby offers you committer privileges
to the project. These privileges are offered on the
understanding that you'll use them reasonably and with
common sense. We like to work on trust rather than
unnecessary constraints. 

Being a committer enables you to more easily make 
changes without needing to go through the patch 
submission process.

Being a committer does not require you to 
participate any more than you already do. It does 
tend to make one even more committed. You will 
probably find that you spend more time here.

Of course, you can decline and instead remain as a 
contributor, participating as you do now.

A. This personal invitation is a chance for you to 
accept or decline in private.  Either way, please 
let us know in reply to the [private@opendal.apache.org] 
address only.

B. If you accept, the next step is to register an iCLA:
    1. Details of the iCLA and the forms are found 
    through this link: https://www.apache.org/licenses/#clas

    2. Instructions for its completion and return to 
    the Secretary of the ASF are found at
    https://www.apache.org/licenses/#submitting

    3. When you transmit the completed iCLA, request 
    to notify the Apache OpenDAL and choose a 
    unique Apache ID. Look to see if your preferred 
    ID is already taken at 
    https://people.apache.org/committer-index.html
    This will allow the Secretary to notify the PMC 
    when your iCLA has been recorded.

When recording of your iCLA is noted, you will 
receive a follow-up message with the next steps for 
establishing you as a committer.

With the expectation of your acceptance, welcome!

${your_name} (as represents of The Apache OpenDAL PMC)
```

## Add the candidate to the committer list<a href="https://opendal.apache.org/community/pmc_members/nominate-committer/#add-the-candidate-to-the-committer-list" class="hash-link" aria-label="Direct link to Add the candidate to the committer list" translate="no" title="Direct link to Add the candidate to the committer list">â€‹</a>

After the candidate accepts the invitation and the iCLA is recorded, add the candidate to the committer list by <a href="https://whimsy.apache.org/roster/committee/opendal" rel="noopener noreferrer" target="_blank">whimsy roster tools</a>

<img src="out_opendal/community/pmc_members/nominate-committer/index_media/a90fafe544afa770fc6708f77c64f9c90315d73a.png" class="img_KBPg" decoding="async" loading="lazy" width="1598" height="494" />

## Announcement<a href="https://opendal.apache.org/community/pmc_members/nominate-committer/#announcement" class="hash-link" aria-label="Direct link to Announcement" translate="no" title="Direct link to Announcement">â€‹</a>

Once the nominee accepts the invitation and the commit bit is granted, it's encouraged to send an announcement email to <a href="mailto:dev@opendal.apache.org" rel="noopener noreferrer" target="_blank">dev@opendal.apache.org</a> to welcome the new committers. Here is an email template:

``` prism-code
Hello, everyone

On behalf of the Apache OpenDAL PMC, I'm happy to announce that
${candidate_name} has accepted the invitation to become a committer of
Apache OpenDAL.

Welcome, and thank you for your contributions!

${your name}
```

<a href="https://github.com/apache/opendal/tree/main/website/community/pmc_members/nominate-committer.md" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_opendal/community/pmc_members/nominate-committer/index_media/82254bca835e54e35c885c6543f8416b5aff021e.svg" class="iconEdit_evxu" />Edit this page</a>

<a href="https://opendal.apache.org/community/pmc_members/onboarding/" class="pagination-nav__link pagination-nav__link--prev"></a>

Previous

Onboarding

<a href="https://opendal.apache.org/community/pmc_members/nominate-pmc-member/" class="pagination-nav__link pagination-nav__link--next"></a>

Next

Nominate PMC Member

- <a href="https://opendal.apache.org/community/pmc_members/nominate-committer/#start-vote-about-the-candidate" class="table-of-contents__link toc-highlight">Start vote about the candidate</a>
- <a href="https://opendal.apache.org/community/pmc_members/nominate-committer/#send-invitation-to-the-candidate" class="table-of-contents__link toc-highlight">Send invitation to the candidate</a>
- <a href="https://opendal.apache.org/community/pmc_members/nominate-committer/#add-the-candidate-to-the-committer-list" class="table-of-contents__link toc-highlight">Add the candidate to the committer list</a>
- <a href="https://opendal.apache.org/community/pmc_members/nominate-committer/#announcement" class="table-of-contents__link toc-highlight">Announcement</a>
