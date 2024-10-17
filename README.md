# Canary

## Working with Merge Queues

This repository uses GitHub Merge Queues to control the delivery of
Docker images.

*N.B.*: GitHub Merge Queues are a fairly new feature, so there's still
some unintuitive behavior to work through. For example, please note
that "required status checks" must pass *both* before enqueueing a PR is
allowed *and* for an enqueued PR to successfully merge. To facilitate
running separate checks for PR readiness (*i.e.* light-weight tests to check if 
a PR ready to be merged) and validation (*i.e.* run heavy-weight tests 
running immediately before merging), we give two jobs the same name
even though they fire on different events.

The job `⚡PR Ready` is a special name we use for both jobs. The
emoiji character is specifically used to distinguish it as a special
type of job. The workflow should only ever call into other workflows,
giving us the most flexibility with workflow reuse.

Below, you'll find resources for working with and
understanding merge queues. It's pretty light for now, so feel free
to add more links.

* [GitHub Actions Patterns](https://github.com/orgs/community/discussions/103114#discussioncomment-8359045)
