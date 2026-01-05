# Quick start - Formsnap

Since Formsnap is built on top of [Superforms](https://superforms.rocks/), you'll need to install it as well as a schema validation library of your choice. We'll use [Zod](https://zod.dev/).

Before diving into this tutorial, it's important to be confident with [Superforms](https://superforms.rocks/), as Formsnap is built on top of it and uses the same APIs.

### Define a Zod schema

This schema will represent the shape of our form data. It's used to validate the form data on the client (optional) and server, along with some other useful things.

src/routes/settings/schema.ts

Looking at the schema above, we know we'll need a few different input types to represent the different data types. Here's how we'll map the schema to input types:

- `email` -> `<input type="email">`
- `bio` -> `<textarea>`
- `theme` -> `<input type="radio">`
- `language` -> `<select>`
- `marketingEmails` -> `<input type="checkbox>`
- `allergies` -> `<input type="checkbox">` (group/multiple)

Of course, there are other ways to represent the data, but this is the approach we'll take for this tutorial.

### Return the form from a load function

In Superforms fashion, we'll return the form from a load function to seamlessly merge our `PageData` and `ActionData`.

src/routes/settings/+page.server.ts

### Setup the form in the page component

Now that we have our form in the `PageData` object, we can use it, along with the schema we defined earlier, to setup the form in our page component.

src/routes/settings/+page.svelte

We'll initialize the super form using `superForm` and pass in the form from the `PageData`. We'll also enable client-side validation by passing the `validators` option. Then, we'll setup the form using the `enhance` function, which will progressively enhance the form with client-side validation and other features.

### Constructing a form field

You can think of form fields as the building blocks of your form. Each property of the schema will have a corresponding form field, which will be responsible for displaying the error messages and description.

We'll start with the `email` field and work our way down.

src/routes/settings/+page.svelte

We pass the `form` and `name` to the `Field` component, which will be used to setup the context for the field. The `name` is typed to the keys of the schema, so it's type-safe.

Now let's add the remaining parts of the field:

src/routes/settings/+page.svelte

We've first added the [Control](https://formsnap.dev/docs/components/control) component. `Control`s are used to represent a form control and its label. They keep the control and label in sync via the `props` snippet prop, which is spread onto the control. Inside the `Control`, we've added the [Label](https://formsnap.dev/docs/components/label) component, which will automatically associate itself with the control the `props` are spread onto. We've also added the control itself, which is an `input` that we're binding to the `email` property of the form data.

The [Description](https://formsnap.dev/docs/components/description) component is optional, but it's useful for providing additional context to the user about the field. It'll be synced with the `aria-describedby` attribute on the input, so it's accessible to screen readers.

The [FieldErrors](https://formsnap.dev/docs/components/field-errors) component is used to display validation errors to the user. It also is synced with the `aria-describedby` attribute on the input, which can receive multiple IDs, so that screen readers are able to read the error messages in addition to the description.

And that's really all it takes to setup a form field. Let's continue on with the rest of the fields.

### Add remaining form fields

src/routes/settings/+page.svelte

You may have noticed for the `allergies` and `theme` fields, we used the [Fieldset](https://formsnap.dev/docs/components/fieldset) and [Legend](https://formsnap.dev/docs/components/legend) components. These are used to group related fields together and provide a title for the group, which is great for accessibility and organization. Additionally, we only use a single [FieldError](https://formsnap.dev/docs/components/field-errors) and [Description](https://formsnap.dev/docs/components/description) component for the entire group, and use an [Control](https://formsnap.dev/docs/components/control) for each field in the group to associate the label with the control.

Now that you've built your first form, you're ready to start building more complex forms with Formsnap & Superforms. Be sure to check out the rest of the documentation to learn more about the different components and APIs available to you.
