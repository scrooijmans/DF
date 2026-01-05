# Description - Formsnap

Provides an accessible description for a form field.

The `Description` component provides an accessible description for a field. It renders a `<div>` element and should be used to provide additional context or instructions for a form field.

Descriptions must be used within the context of a [Field](https://formsnap.dev/docs/components/field), [Fieldset](https://formsnap.dev/docs/components/fieldset), or [ElementField](https://formsnap.dev/docs/components/element-field) component and will automatically be linked to the [Control](https://formsnap.dev/docs/components/control) of the field using the `aria-describedby` attribute.

## Usage

## API Reference

### Props

The `Description` component accepts all props that a standard HTML `<div>` element would accept, along with a few additional props:

ref

type: HTMLElement | null

A reference to the underlying HTML element rendered by the `Description` component.

If provided, the `Description` component will not render an HTML element and will instead expect you to spread the snippet's `props` onto an element of your choosing.

See the [`child`](https://formsnap.dev/docs/composition/child) snippet documentation for more information.

...rest

type: HTMLAttributes<HTMLElement>

Any additional props provided to the `Description` component will be spread onto the underlying HTML element.

### Data Attributes

The following data attributes are automatically applied to the `<div>` element rendered by the `Description` component.

data-fs-description

type: ''

Applied to the description element for selection during styling or otherwise.

data-fs-error

type: '' | undefined

Applied to the description element when a validation error exists on the field.
