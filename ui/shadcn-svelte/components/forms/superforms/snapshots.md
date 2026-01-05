# Snapshots

A nice SvelteKit feature is [snapshots](https://kit.svelte.dev/docs/snapshots), which saves and restores data when the user navigates on the site. This is perfect for saving the form state, and with Superforms, you can take advantage of this in one line of code, as an alternative to a [tainted form message](https://superforms.rocks/concepts/tainted). Note that it only works for browser history navigation though.

## Usage

```
const { form, capture, restore } = superForm(data.form);

export const snapshot = { capture, restore };
```

The export has to be on a `+page.svelte` page to work, it cannot be in a component.

## Test it out

Modify the form below without submitting, then click the browser back button and then forward again. The form should be restored to its intermediate state.

Found a typo or an inconsistency? Make a quick correction [here](https://github.com/ciscoheat/superforms-web/tree/main/src/routes/concepts/snapshots/+page.md)!
