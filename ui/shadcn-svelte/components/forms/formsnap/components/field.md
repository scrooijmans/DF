# Field - Formsnap

Provides the necessary context for a form field.

The `Field` component provides the necessary context for its children to react to changes in the form state, as well as provides necessary information about the field, such as the ids needed for aria attributes, and a lot more.

Each `Field` creates its own context, and the children of the field only access the immediate parent's context.

##### Note

The `Field` component doesn't render an element, it strictly provides context.

## API Reference

### Props

The form object returned from calling `superForm` in your component.

required

The path to the field in the form object.

required

### Snippet Props

The following snippet props are provided to the `children` snippet for convenience and ease of composition when using the `Field` component.

The value of the value store of the field.

errors

type: string\[\] | undefined

The value of the errors store for the field.

constraints

type: Record<string, unknown>

The constraints for the field.

Whether the field is tainted or not.

## Composition

Since the `Field` component doesn't render any HTML elements, it's a common practice to create a wrapper component around it to have consistent styling and behavior across your forms.

For example, you may always want to render the [FieldErrors](https://formsnap.dev/docs/components/field-errors) component for every field. Instead of manually including it every time, you can create a wrapper `<CustomField />` component that includes it automatically.

To maintain the type safety of the component, we'll need to use some generics, which eslint sometimes complains about, so if you see a warning, it's likely a false positive and you can ignore it.

CustomField.svelte
