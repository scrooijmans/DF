# Fieldset - Formsnap

Groups related form controls or fields and extends the Field component.

The `Fieldset` component is used to follow the [W3C Grouping Controls](https://www.w3.org/WAI/tutorials/forms/grouping/#associating-related-controls-with-fieldset) recommendation for associating related form controls. It renders a `<fieldset>` element and should always be used in conjunction with the [Legend](https://formsnap.dev/docs/components/legend) component to provide a title for the group.

This component automatically includes the [Field](https://formsnap.dev/docs/components/field) component, so you don't need to worry about wrapping it yourself, just be sure to pass the `form` and `name` props to the `Fieldset` as you would with the `Field` component.

## When to use a fieldset

### Radio Groups

When you have a group of radio buttons related to a single field, you should use a `Fieldset` to group them together.

### Checkbox Groups

When you have a group of checkboxes related to a single field, typically used for multiple selections, you should use a `Fieldset` to group them together.

### Grouped Form Sections

When you have a large form with multiple sections containing related fields, such as a "Billing Address" and a "Shipping Address", you should use a `<fieldset>` to group the related fields together. You won't use the `Fieldset` component directly in this case, since it doesn't represent a field on the form.

## API Reference

### Props

The `Fieldset` component renders a `<fieldset>` element and accepts the following props:

The form object returned from calling `superForm` in your component.

required

The path to the field in the form object.

required

ref

type: HTMLElement | null

A `$bindable` reference to the underlying HTML element rendered by the `Fieldset` component.

If provided, the `Fieldset` component will not render an HTML element and will instead expect you to spread the snippet's `props` onto an element of your choosing.

See the [`child`](https://formsnap.dev/docs/composition/child) snippet documentation for more information.

...rest

type: HTMLAttributes<HTMLFieldSetElement>

Any additional props provided to the `Fieldset` component will be spread onto the underlying HTML element.

### Snippet Props (children)

The following snippet props are provided to the `children` snippet for convenience and ease of composition when using the `ElementField` component.

The value of the value store of the field.

errors

type: string\[\] | undefined

The value of the errors store for the field.

constraints

type: Record<string, unknown>

The constraints for the field.

Whether the field is tainted or not.

### Snippet Props (child)

The following snippet props are provided to the `child` snippet for convenience and ease of composition when using the `ElementField` component.

props

type: Record<string, unknown>

The props to spread onto your custom `<fieldset>` element/component.

The value of the value store of the field.

errors

type: string\[\] | undefined

The value of the errors store for the field.

constraints

type: Record<string, unknown>

The constraints for the field.

Whether the field is tainted or not.

### Data Attributes

The following data attributes are automatically applied to the `<fieldset>` element rendered by the `Fieldset` component.

data-fs-fieldset

type: ''

Applied to the element for selection during styling or otherwise.

data-fs-error

type: '' | undefined

Applied to the element when a validation error exists on the field.
