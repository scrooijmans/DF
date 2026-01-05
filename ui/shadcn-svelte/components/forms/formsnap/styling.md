# Styling - Formsnap

Easily style the various parts of your forms.

Formsnap doesn't ship with any styles by default, but it does provide a number of ways to style the various parts of your form. You can use the `class` prop to apply classes to the various components, or you can use the `data` attributes to style the components using CSS.

## Data Attributes

Data attributes are applied to the various parts of your form so that you can easily style them using those attributes as selectors on a parent element or at the global level.

- Attribute: data-fs-error
  - Description: Applied to all the formsnap elements within a field if the field has a validation error. Using this attribute, you can customize the appearance of the input, label, etc. when the field has a validation error.
- Attribute: data-fs-control
  - Description: Applied to the form control element used within a Control context.
- Attribute: data-fs-label
  - Description: Applied to the <label> element rendered by the Label component.
- Attribute: data-fs-field-errors
  - Description: Applied to the FieldErrors container <div> element.
- Attribute: data-fs-field-error
  - Description: Applied to the individually rendered <div> elements for each of the errors in the FieldErrors component.
- Attribute: data-fs-description
  - Description: Applied to the <div> element rendered by the Description component.
- Attribute: data-fs-fieldset
  - Description: Applied to the <fieldset> element rendered by the Fieldset component.
- Attribute: data-fs-legend
  - Description: Applied to the <legend> element rendered by the Legend component.

Here's an example of how you might use these data attributes to style the various parts of your form:

app.pcss

## CSS Frameworks

If you're using a CSS framework like TailwindCSS or UnoCSS, you can simply pass the `class` prop to the various components that render HTML elements under the hood. For example:
