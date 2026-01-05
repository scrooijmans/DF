# Legend - Formsnap

Provides a title for a group of related form controls.

You should always use the `Legend` component in conjunction with the [Fieldset](https://formsnap.dev/docs/components/fieldset) component to provide a title for a group of related form controls. See the the `Fieldset` component's [documentation](https://formsnap.dev/docs/components/fieldset) for more information on when to use a fieldset.

## Props

The `Legend` component renders a `<legend>` element and accepts all props that a standard HTML `<legend>` element would accept along with a few additional props:

ref

type: HTMLElement | null

A `$bindable` reference to the underlying HTML element rendered by the `Legend` component.

If provided, the `Legend` component will not render an HTML element and will instead expect you to spread the snippet's `props` onto an element of your choosing.

See the [`child`](https://formsnap.dev/docs/composition/child) snippet documentation for more information.

...rest

type: HTMLAttributes<HTMLLegendElement>

Any additional props provided to the `Legend` component will be spread onto the underlying HTML element.

## Data Attributes

The following attributes are automatically applied to the element rendered by the `Legend` component.

Applied to the element for selection during styling or otherwise.

data-fs-error

type: '' | undefined

Applied to the element when a validation error exists on the field.
