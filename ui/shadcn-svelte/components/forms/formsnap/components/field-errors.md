# FieldErrors - Formsnap

The container for validation errors for a Field, Fieldset, or ElementField.

The `FieldErrors` component renders the following structure by default (attributes omitted for brevity):

Notice that we're populating the fallback for the children snippet, so if you don't provide children content for the `FieldErrors` component, it will render a `<div>` element for each error in the `errors` array.

The `errors` are the errors for the [Field](https://formsnap.dev/docs/components/field), [Fieldset](https://formsnap.dev/docs/components/fieldset), or [ElementField](https://formsnap.dev/docs/components/element-field) that the `FieldErrors` component is associated with and must be used within the context of one of those components.

The errors container is automatically linked to the control of the field using the `aria-describedby` attribute when errors are present.

## Usage

### Basic Usage

By default, the `FieldErrors` component will render a `<div>` element with the errors for the field it is associated with.

### Custom Error Rendering

If you want to customize the rendering of the errors, you can access the errors using the `errors` snippet prop and render them however you'd like.

## API Reference

### Props

The `FieldErrors` component accepts all props that a standard HTML `<div>` element would accept along with a few additional props:

ref

type: HTMLElement | null

A reference to the underlying HTML element rendered by the `Description` component.

If provided, the `FieldErrors` component will not render an HTML element and will instead expect you to spread the snippet's `props` onto an element of your choosing.

See the [`child`](https://formsnap.dev/docs/composition/child) snippet documentation for more information.

...rest

type: HTMLAttributes<HTMLElement>

Any additional props provided to the `FieldErrors` component will be spread onto the underlying HTML element.

### Attributes

#### Field Errors Container

The following attributes are automatically applied to the container rendered by the `FieldErrors` component. This is also the shape of the `props` snippet prop when using the [child](https://formsnap.dev/docs/composition/child) snippet.

#### Error Elements

The following attributes are automatically applied to the individual error elements rendered by the `FieldErrors` component. This is also the shape of the `errorProps` snippet prop.
