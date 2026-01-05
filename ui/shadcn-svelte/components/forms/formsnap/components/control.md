# Control - Formsnap

Associates a label with and provides necessary attributes for a form control.

In the context of a form, a **_control_** refers to any interactive element such as an input field, a select dropdown, or a button. This includes custom components like select dropdowns or checkboxes that function as buttons but still serve as form inputs, typically activated by clicking on a label or pressing a key.

Each control and its label should be wrapped in its own `Control` component. This is important for accessibility, as it ensures that the label is associated with the control, and that the label is announced to screen readers when the control receives focus.

##### Why a separate component?

A common question is _"why we can't just include this logic in the various `Field` components?"_.

Doing so would limit the `Field` component to a single control, which would prevent them from being used for checkbox groups, radio groups, and other multi-control components. The APIs are flexible enough that you could create your own custom wrapper components to make them more convenient to use for your specific use case.

The `Control` component doesn't render an element itself, it strictly provides context and attributes for the control via a snippet prop and state for the [Label](https://formsnap.dev/docs/components/label).

## Usage

+page.svelte

## API Reference

### Props

Optionally provide a unique id for the form item/control. If not provided, a unique ID will be generated for you.

This is useful when another library automatically generates IDs for form items. You can pass that ID to the `id` prop and the label will be associated with that control.

The children snippet is used to provide attributes for the control element/component.

required

## Composition

Since the `Control` component doesn't render an HTML element, it's a common practice to create a wrapper component around it to have consistent styling and behavior across your forms.

For example, you may want to automatically include the [Label](https://formsnap.dev/docs/components/label) for each item, and you want the label and children content to be wrapped in a `<div>`.

Here's how you might do just that:

CustomControl.svelte
