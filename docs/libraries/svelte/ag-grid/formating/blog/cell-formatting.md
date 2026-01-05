# Formatting numbers, strings and currency values in ag-Grid

14 July 2020   |   [How To](https://blog.ag-grid.com/tag/how-to/)

![](https://blog.ag-grid.com/content/images/2020/07/formatter-blog-image-1.png)

Formatting values is a must-have to deliver a great user experience in any application. This is why valueFormatters are one of the most used features of ag-Grid, giving users a great degree of flexibility with how their data is displayed.

What is a `valueFormatter`? The short, pithy and high-level answer is: _something that makes the data easier to understand for people._ The valueFormatter is a tool built to help people understand the data better, not the machine. To find out more, check out our [documentation](https://ag-grid.com/javascript-grid-value-formatters?ref=blog.ag-grid.com).

In this post, I'll illustrate how to format numbers, strings and currencies, along with some useful _gotchas_ and examples. So grab a grid, grab some coffee and let's get formatting!

To illustrate, I'll be using the following example:

You’ll see three pairs of columns, each pair consists of one unformatted column and a duplicate column using a value formatter.

Let's go over these left to right, with each pair getting progressively more complex.

#### **In this blog, we'll be going over:**

- [Number formatting with an inline valueFormartter](#inline-valueFormatter)
- [String formatting with an external valueFormatter](#external-valueFormatter)
- [Formatting the filter values](#filter-valueFormatter)
- [Formatting Currency by Passing Parameters to a valueFormatter](#passing-parameters)

---

Recently I also made a video covering all the subjects in this and the next valueFormatters blog. So if you don't like the sound of my typing and prefer the sound of my voice, check it out!

[Watch On YouTube](https://www.youtube.com/watch?v=n_lypNK0n_k&ref=blog.ag-grid.com)

---

## Number formatting with an inline valueFormatter

Let’s start with formatting some numeric values. Take a look at the Number and Number Formatted columns in the screenshot below.

![](https://blog.ag-grid.com/content/images/2020/07/image.png)

toFixed() even rounds the numeric values

The column definitions for these are as follows:

```
    {
      field: 'number',
    },
    {
      headerName: 'Number Formatted',
      field: 'number',
      valueFormatter: params => params.data.number.toFixed(2),
    },
```

Number & Number Formatted column definitions

In this case, we're formatting the value using the simplest version of a `valueFormatter` - an arrow function chopping a numeric value down to two decimal places, defined inline as part of the column definition. I love this example because it encapsulates most cases I've come across, a single transformation that makes complex data more human-friendly.

---

## String formatting with an external valueFormatter

When you have more complex formatting logic, you can use an external `valueFormatter`. Let’s format a string value by capitalizing its first letter to illustrate this. This is shown in the ‘String formatted’ column in the screenshot below:

![](https://blog.ag-grid.com/content/images/2020/07/image-2.png)

In short, turning “apple” to “Apple”

This formatting is implemented using the column definition code below:

```
    {
      field: 'string',
    },
    {
      headerName: 'String Formatted',
      field: 'string',
      valueFormatter: stringFormatter,
      filter: 'agSetColumnFilter',
      filterParams: {
        valueFormatter: stringFormatter,
      },
    },
```

String & String Formatted column definitions

This time we see that the `valueFormatter` is a reference to a formatter defined elsewhere:

```
function stringFormatter(params) {
  var fruit = params.value;
  var firstChar = fruit.slice(0, 1).toUpperCase();
  return firstChar + fruit.slice(1);
}
```

String formatter function - this time I'm assigning the value to variable called fruit to avoid confusion—hopefully.

And here it is. Note that just like the inline function above, the function — when invoked — will take params from the ag-Grid from which it extracts the relevant data. In this case it’s `params.value` — `params.data.string` would work as well and be more specific — and then slice off the first character, uppercase it and return the uppercase character with the tail of the initial string.

In the code above, you may notice that there's more than just a `valueFormatter` in String Formatted's column definitions. There's also a filter, `agSetColumnFilter` which is ag-Grid's standard set filter and `filterParams` in which we've also declared the `valueFormatter` again. This is related to filtering of formatted values, which we'll cover next.

---

## Formatting the filter values

```
filter: 'agSetColumnFilter',
      filterParams: {
        valueFormatter: stringFormatter,
      },
```

By default, a `valueFormatter` will only be used for formatting the values in ag-Grid cells, and the filtering values will be generated from the underlying unformatted data. In order to format the filtering values for this column the same way as the column cell values, we need to pass the `valueFormatter` to the filter.

Please see this illustrated in the two screenshots below - first screenshot shows formatted (capitalized) values in the filter when the `valueFormatter` is passed to the filter, while the second screenshot shows the default behavior with unformatted values.

![](https://blog.ag-grid.com/content/images/2020/06/fruitFilterFormatter.gif)

String Formatted filter

To see the difference for yourself, modify the sample by commenting-out the filterParams and opening up the `setFilter` in the String Formatted column, you'll note that the filtering values are not capitalized.

![](https://blog.ag-grid.com/content/images/2020/06/Jun-09-2020-16-38-03.gif)

String Formatted filter without valueFormatter in the filter

---

## Formatting Currency by Passing Parameters to a valueFormatter

Now let’s step it up a notch to something we all use, want, and need more of, MONEY! In other words, let's look at how to quickly format currency values.

![](https://blog.ag-grid.com/content/images/2020/06/image-8.png)

Dum-dah-dah-dum Dum-dah-dum Dah-dum!

Currency formatting is illustrated in the Currency and Currency Formatted columns in the sample, as shown below:

![](https://blog.ag-grid.com/content/images/2020/07/image-3.png)

Chopping off the decimals, adding a dollar sign(as a parameter) & an apostrophe for legibility.

Let's have a look at how to implement this using the code for the Currency and Currency Formatted columns:

```
    {
      field: 'currency',
    },
    {
      headerName: 'Currency Formatted',
      field: 'currency',
      valueFormatter: params => currencyFormatter(params.data.currency, '

Currency & Currency Formatted column definitions

In the code segment above you can see how you can pass parameters to your formatter. In this case we're passing a currency symbol: '$'.

Once again, we're referencing a `valueFormatter` declared elsewhere. This time I've chosen to pass it the data as `params.data.currency`, though in this case `params.value` would work just as well.

Let's take a closer look at the `valueFormatter`  definition:

```

function currencyFormatter(currency, sign) {
var sansDec = currency.toFixed(0);
var formatted = sansDec.replace(/\B(?=(\d{3})+(?!\d))/g, ',');
return sign + `${formatted}`;
}

```


Currency formatter function

In this `valueFormatter` I've decided to chop off the decimal tail, and used this [solution](https://stackoverflow.com/questions/2901102/how-to-print-a-number-with-commas-as-thousands-separators-in-javascript?ref=blog.ag-grid.com) from Stack Overflow to add a comma after the third element from the right.

In this column we also have a filter, this time it's ag-Grid's `agNumberColumnFilter` in the `filterParams`, I've chosen to suppress the AND/OR conditions and limited the filtering options to greater than to keep things simple  — find out more about these features on our [documentation](https://www.ag-grid.com/javascript-grid-filter-provided-simple/?ref=blog.ag-grid.com).

* * *

So that was part one of `valueFormatters`, but there's still something missing. Dates! In the next blog, I'll go through how `valueFormatters` can be used with date data, and how to configure your grid to sort and filter formatted date values.  

To find out more about `valueForamtters` and to get an in-depth understanding of how they work, check out our example-filled documentation **[here](https://www.ag-grid.com/javascript-grid-value-formatters/?ref=blog.ag-grid.com#value-formatters)**. You can also find our documentation for [**sorting**](https://www.ag-grid.com/javascript-grid-sorting/?ref=blog.ag-grid.com) & [**filtering**](https://www.ag-grid.com/javascript-grid-filtering-overview/?ref=blog.ag-grid.com) on the main ag-Grid site as well.

Lastly if you're new to ag-Grid and want to see what all the hubbub is about, why not try it out — for free — by checking out our **[getting started guides](https://www.ag-grid.com/javascript-grid-getting-started/?ref=blog.ag-grid.com)**.

Read more posts about...),
      filter: 'agNumberColumnFilter',
      filterParams: {
        suppressAndOrCondition: true,
        filterOptions: ['greaterThan'],
      },
    },
```

Currency & Currency Formatted column definitions

In the code segment above you can see how you can pass parameters to your formatter. In this case we're passing a currency symbol: '$'.

Once again, we're referencing a `valueFormatter` declared elsewhere. This time I've chosen to pass it the data as `params.data.currency`, though in this case `params.value` would work just as well.

Let's take a closer look at the `valueFormatter`  definition:

urltomarkdowncodeblockplaceholder50.16552542680798332

Currency formatter function

In this `valueFormatter` I've decided to chop off the decimal tail, and used this [solution](https://stackoverflow.com/questions/2901102/how-to-print-a-number-with-commas-as-thousands-separators-in-javascript?ref=blog.ag-grid.com) from Stack Overflow to add a comma after the third element from the right.

In this column we also have a filter, this time it's ag-Grid's `agNumberColumnFilter` in the `filterParams`, I've chosen to suppress the AND/OR conditions and limited the filtering options to greater than to keep things simple  — find out more about these features on our [documentation](https://www.ag-grid.com/javascript-grid-filter-provided-simple/?ref=blog.ag-grid.com).

---

So that was part one of `valueFormatters`, but there's still something missing. Dates! In the next blog, I'll go through how `valueFormatters` can be used with date data, and how to configure your grid to sort and filter formatted date values.  

To find out more about `valueForamtters` and to get an in-depth understanding of how they work, check out our example-filled documentation **[here](https://www.ag-grid.com/javascript-grid-value-formatters/?ref=blog.ag-grid.com#value-formatters)**. You can also find our documentation for [**sorting**](https://www.ag-grid.com/javascript-grid-sorting/?ref=blog.ag-grid.com) & [**filtering**](https://www.ag-grid.com/javascript-grid-filtering-overview/?ref=blog.ag-grid.com) on the main ag-Grid site as well.

Lastly if you're new to ag-Grid and want to see what all the hubbub is about, why not try it out — for free — by checking out our **[getting started guides](https://www.ag-grid.com/javascript-grid-getting-started/?ref=blog.ag-grid.com)**.

Read more posts about...
