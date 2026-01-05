# ElementField - Formsnap

Provides the necessary context for a form field that represents a single element in an array.

The `ElementField` component is used to treat each element of an array as a separate form field. It's useful when you have a dynamic list of items that you want to treat as separate fields in your form.

`ElementField`s should be used within the context of a [Field](https://formsnap.dev/docs/components/field) or [Fieldset](https://formsnap.dev/docs/components/fieldset) component. `ElementField`s create their own context to scope the errors and other states of the field.

## Usage

Here's an example of how you might use the `ElementField` component to create a dynamic list of URLs in a form.

+page.svelte

We're able to display errors for each element of the array, as well as array-level errors for the entire fieldset.

Check out the [Dynamic Fields](https://formsnap.dev/docs/recipes/dynamic-fields) recipe for more advanced usage of the `ElementField` component.

##### Note

The `ElementField` component doesn't render an element, it strictly provides context for its children.

## API Reference

### Props

The form object returned from calling `superForm` in your component.

required

name

type: FormPathLeaves<T>

The path to the field in the form object.

required

### Snippet Props

The following snippet props are provided to the `children` snippet for convenience and ease of composition when using the `ElementField` component.

The value of the value store of the field.

errors

type: string\[\] | undefined

The value of the errors store for the field.

constraints

type: Record<string, unknown>

The constraints for the field.

Whether the field is tainted or not.

## Composition

Since the `ElementField` component doesn't render any HTML elements, it's common practice to create a wrapper component around it to have consistent styling and behavior across your forms.

For example, you may always want to render the [FieldErrors](https://formsnap.dev/docs/components/field-errors) component for every field. Rather than manually including it each time, you can create a wrapper `<CustomElementField />` component that includes it automatically.

To maintain the type safety of the component, we'll need to use some generics, which eslint sometimes complains about, so if you see a warning, it's likely a false positive and you can ignore it.

CustomElementField.svelte
