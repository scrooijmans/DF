- <a href="https://opendal.apache.org/" class="breadcrumbs__link" aria-label="Home page"><img src="out_opendal/apps/migration/index_media/edf54a765e27bedcc87f5708545b58efaaa38a1a.svg" class="breadcrumbHomeIcon_q3uS" /></a>
- <a href="https://opendal.apache.org/category/applications/" class="breadcrumbs__link">Applications</a>
- Apps migration guide

# Apps migration guide

This repository no longer maintains the applications Oli and Ofs.

What changed

- Oli moved to: <a href="https://github.com/apache/opendal-oli" rel="noopener noreferrer" target="_blank">https://github.com/apache/opendal-oli</a>
- Ofs moved to: <a href="https://github.com/apache/opendal-ofs" rel="noopener noreferrer" target="_blank">https://github.com/apache/opendal-ofs</a>
- Oay has been removed.

Impact

- Crates remain the same (oli, ofs). Cargo users do not need to change dependency names.
- Source code, issue tracking and releases for Oli/Ofs are now in the new repositories.

How to migrate

- For issues or PRs regarding Oli/Ofs, open them in their new repositories.
- For building from source, clone the new repositories.

FAQ

- Why the move? To decouple release cadence and CI from the core project.
- Are there breaking changes? No API/CLI breaking changes are planned as part of the move.

Related

- Tracking: <a href="https://github.com/apache/opendal/issues/6689" rel="noopener noreferrer" target="_blank">https://github.com/apache/opendal/issues/6689</a>

<a href="https://github.com/apache/opendal/tree/main/website/docs/40-apps/migration.mdx" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_opendal/apps/migration/index_media/82254bca835e54e35c885c6543f8416b5aff021e.svg" class="iconEdit_evxu" />Edit this page</a>

Last updated on **Oct 18, 2025** by **Jintao Zhang**

<a href="https://opendal.apache.org/category/applications/" class="pagination-nav__link pagination-nav__link--prev"></a>

Previous

Applications

<a href="https://opendal.apache.org/apps/oay/" class="pagination-nav__link pagination-nav__link--next"></a>

Next

Oay (removed)
