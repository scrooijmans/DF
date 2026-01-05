<img src="out_scichartv4/typedoc/enums/enumericformat_media/d804b495cb9b84b9007a25b5d85f9ae674004cde.gif" style="display:none;" width="1" height="1" />

Search

- Preparing search index...
- The search index is not available

<a href="https://www.scichart.com/documentation/js/v4/typedoc/index.html" class="title">TypeDoc API Documentation</a> for SciChart <a href="https://www.scichart.com/javascript-chart-features" class="title" target="blank">JavaScript Charts</a>

<a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/enumericformat.html#" class="tsd-widget options no-caption" data-toggle="options">Options</a>

All

- Public
- Public/Protected
- All

Inherited

- [Globals](https://www.scichart.com/documentation/js/v4/typedoc/index.html)
- [ENumericFormat](https://www.scichart.com/documentation/js/v4/typedoc/enums/enumericformat.html)

# Enumeration ENumericFormat

Go to [JavaScript Charting Documentation](https://www.scichart.com/documentation/js/v4/intro)

Go to <a href="https://scichart.com/demo" target="_blank">JavaScript Chart Examples</a>

Go to <a href="https://github.com/abtsoftware/scichart.js.examples" target="_blank">SciChart.js on Github</a>

Enumeration constants for standard formatting strings

## Index

### Enumeration members

- <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/enumericformat.html#date_ddmm" class="tsd-kind-icon">Date_DDMM</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/enumericformat.html#date_ddmmhhmm" class="tsd-kind-icon">Date_DDMMHHMM</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/enumericformat.html#date_ddmmyy" class="tsd-kind-icon">Date_DDMMYY</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/enumericformat.html#date_ddmmyyyy" class="tsd-kind-icon">Date_DDMMYYYY</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/enumericformat.html#date_hhmm" class="tsd-kind-icon">Date_HHMM</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/enumericformat.html#date_hhmmss" class="tsd-kind-icon">Date_HHMMSS</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/enumericformat.html#date_ssms" class="tsd-kind-icon">Date_SSms</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/enumericformat.html#decimal" class="tsd-kind-icon">Decimal</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/enumericformat.html#engineering" class="tsd-kind-icon">Engineering</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/enumericformat.html#exponential" class="tsd-kind-icon">Exponential</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/enumericformat.html#noformat" class="tsd-kind-icon">NoFormat</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/enumericformat.html#scientific" class="tsd-kind-icon">Scientific</a>
- <a href="https://www.scichart.com/documentation/js/v4/typedoc/enums/enumericformat.html#significantfigures" class="tsd-kind-icon">SignificantFigures</a>

## Enumeration members

### Date_DDMM

Date_DDMM: = "Date_DDMM"

Format as a date in format DD/MM

### Date_DDMMHHMM

Date_DDMMHHMM: = "Date_DDMMHHMM"

Format as a date in format DD/MM HH:MM

### Date_DDMMYY

Date_DDMMYY: = "Date_DDMMYY"

Format as a date in format DD/MM/YY

### Date_DDMMYYYY

Date_DDMMYYYY: = "Date_DDMMYYYY"

Format as a date in format DD/MM/YYYY

### Date_HHMM

Date_HHMM: = "Date_HHMM"

Format as a date in format HH:MM

### Date_HHMMSS

Date_HHMMSS: = "Date_HHMMSS"

Format as a date in format HH:MM:SS

### Date_SSms

Date_SSms: = "Date_SSms"

Format as a date in format SS.ms

### Decimal

Decimal: = "Decimal"

Format to a specified number of decimal places

### Engineering

Engineering: = "Engineering"

Engineering notation, eg 12.32K, 1.5M, 2.7G

feature  
You can pass your custom prefixes as [IEngineeringPrefix](https://www.scichart.com/documentation/js/v4/typedoc/interfaces/iengineeringprefix.html)

note  
`labelPrecision` will not only represent the number of decimal places, but the number of significant figures, when needed.

### Exponential

Exponential: = "Exponential"

Format using Exponential notation to a specified number of significant figures eg 1.0E0, 1.5E1, 2.7E2 Note that this will ALWAYS be base 10, even when used on a Logarithmic axis

### NoFormat

NoFormat: = "NoFormat"

No format, return the string representation unchanged

### Scientific

Scientific: = "Scientific"

Format using Scientific notation to a specified number of significant figures eg 1.0x10^1, 1.5x10^2, 2.7x10^3 On a Logarithmic axis, the base will be the same as the axis logarithmic base

### SignificantFigures

SignificantFigures: = "SignificantFigures"

Format to a specified number of significant figures

## Legend

- Constructor
- Property
- Method
- Accessor

<!-- -->

- Inherited constructor
- Inherited property
- Inherited method
- Inherited accessor

<!-- -->

- Property
- Method

<!-- -->

- Protected property
- Protected method

<!-- -->

- Static property
- Static method

Generated using <a href="https://typedoc.org/" target="_blank">TypeDoc</a>
