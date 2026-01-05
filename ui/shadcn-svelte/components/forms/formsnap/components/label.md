# Label - Formsnap

Renders a label element for a control.

The `Label` component must be used as a child of a [Control](https://formsnap.dev/docs/components/control) component. It renders a `<label>` element and includes the necessary attributes to associate it with the control.

## Usage

When using a `Label` inside a [Control](https://formsnap.dev/docs/components/control), you don't need to worry about the `for` attribute. Formsnap handles that for you.

## API Reference

### Props

ref

type: HTMLElement | null

A `$bindable` reference to the underlying `<label>` element rendered by the `Label` component.

If provided, the `Label` component will not render an HTML element and will instead expect you to spread the snippet's `props` onto an element of your choosing.

See the [`child`](https://formsnap.dev/docs/composition/child) snippet documentation for more information.

...rest

type: HTMLAttributes<HTMLLabelElement>

Any additional props provided to the `Label` component will be spread onto the underlying HTML element.

## Data Attributes

The following data attributes are automatically applied to the element rendered by the `Label` component.

Applied to the element for selection during styling or otherwise.

data-fs-error

type: '' | undefined

Applied to the element when a validation error exists on the field.
