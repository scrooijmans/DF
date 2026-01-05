Identifying and Handling Missing Well Log Data Prior to Machine Learning
Part 2 in a series going from Exploratory Data Analysis to Machine Learning with Well Log Data

Andy McDonald
Oct 2, 2021
11 min read
Share
Photo by Markus Spiske on Unsplash
Photo by Markus Spiske on Unsplash
Machine learning and Artificial Intelligence are becoming popular within the geoscience and petrophysics domains. Especially over the past decade. Machine learning is a subdivision of Artificial Intelligence and is the process by which computers can learn and make predictions from data without being explicitly programmed to do so. We can use machine learning in a number of ways within petrophysics, including automating outlier detection, property prediction, facies classification, etc.

This series of articles will look at taking a dataset from basic well log measurements through to petrophysical property prediction. These articles were originally presented as Jupyter Notebooks at a workshop on Machine Learning and Artificial Intelligence at the 2021 SPWLA Conference. They have since been expanded and updated to form these articles. The series will consist of the following, and links will be included once they have been released.

Initial Data Exploration of selected wells from the Volve field dataset
Identification & Dealing With Missing Data (this article)
Detection of outliers / anomalous data points using manual and automated methods
Prediction of key reservoir properties using Machine Learning
This article on identifying missing data within well log measurements is a culmination of previous work and the specific articles can be found below:

Using the MissingNo library to Identify Missing Values
Visualising Well Data Coverage Using Matplotlib
Also, if you want to see how the missingno library works for identifying missing data, check out my YouTube video that goes over this library and its features.

https://youtu.be/Wdvwer7h-8w

Identification & Dealing With Missing Data
Missing values are a common problem within datasets. Within well log datasets, data can be missing for a number of reasons, including tool/sensor failure, data vintage, telemetry issues, stick and pull, and missing by choice. These issues are described in detail in McDonald (2021).

Within the Python world, there are a number of useful functions from easy to use libraries that we can take advantage of to identify missing data, these methods include:

Pandas Dataframe Summaries (e.g. .describe(), and .info())
MissingNo Library
Visualisations
The process of handling missing data can be controversial. A number of petrophysicists, data scientists, and others argue that the in-filling of data could lead to the addition of greater uncertainty in the final results, whilst others suggest the data should be filled in. Filling in of missing values can be done using simple linear interpolation, filling in with the mean, and also extend to using machine learning algorithms to predict what the missing values could be. As always, you should check your data after applying any missing data imputation techniques.

Within this article, we are going to first identify missing data and then use a number of techniques to remove the affected rows and columns. Data removal will be demonstrated using Variable Discarding and Listwise Deletion methods.

Data
The dataset that we will use for this article comes from the popular Volve Field dataset that was released in 2018 to foster research and learning. The released data includes:

Well Logs
Petrophysical Interpretations
Reports (geological, completion, petrophysical, core, etc)
Core Measurements
Seismic Data
Geological Models
and more…
The Volve Field is located some 200 km west of Stavanger in the Norwegian Sector of the North Sea. Hydrocarbons were discovered within the Jurassic aged Hugin Formation in 1993. Oil production began in 2008 and lasted for 8 years (twice as long as planned) until 2016, when production ceased. In total 63 MMBO were produced over the field’s lifetime and reached a plateau of 56,000 B/D.

Further details about the Volve Field and the entire dataset can be found at: https://www.equinor.com/en/what-we-do/norwegian-continental-shelf-platforms/volve.html

The data is licensed under the Equinor Open Data Licence.

Importing Libraries & Data
The first step is to import the libraries that we will require for working with the data. For this notebook, we will be using:

pandas for loading and storing the data
matplotlib and seaborn for visualising the data
numpy for a number of calculation methods
missingno to visualise where missing data exists
import pandas as pd
import matplotlib.pyplot as plt
import missingno as msno
import numpy as np
Next, we will load the data using the pandas read_csv function and assign it to the variable df. The data will now be stored within a structured object known as a dataframe.

df = pd.read_csv('data/spwla_volve_data.csv')
As seen in the previous article, we can call upon a few methods to check the data contents and initial quality.

The .head() method allows us to view the first 5 rows of the dataframe.

df.head()
Header rows from the dataframe using the df.head() method. Image by the author.
Header rows from the dataframe using the df.head() method. Image by the author.
The .describe() method provides us some summary statistics. To identify if we have missing data using this method, we need to look at the count row. If we assume that MD (measured depth) is the most complete column, we have 27,845 data points. Now, if we look at DT and DTS, we can see we only have 5,493 and 5,420 data points respectively. A number of other columns also have lower numbers, namely: RPCELM, PHIF, SW, VSH.

df.describe()
Dataframe summary statistics using the df.describe() method - click to zoom in. Image by the author.
Dataframe summary statistics using the df.describe() method – click to zoom in. Image by the author.
To gain a clearer insight, we can call upon the info() method to see how many non-null values exist for each column. Right away we can see the ones highlighted previously have lower numbers of non-null values.

df.info()
This returns the following:

<class 'pandas.core.frame.DataFrame'>
RangeIndex: 27845 entries, 0 to 27844
Data columns (total 16 columns):
wellName 27845 non-null object
MD 27845 non-null float64
BS 27845 non-null float64
CALI 27845 non-null float64
DT 5493 non-null float64
DTS 5420 non-null float64
GR 27845 non-null float64
NPHI 27845 non-null float64
RACEHM 27845 non-null float64
RACELM 27845 non-null float64
RHOB 27845 non-null float64
RPCEHM 27845 non-null float64
RPCELM 27600 non-null float64
PHIF 27736 non-null float64
SW 27736 non-null float64
VSH 27844 non-null float64
dtypes: float64(15), object(1)
memory usage: 3.4+ MB
Using missingno to Visualise Data Sparsity
The missingno library is designed to take a dataframe and allow you to visualise where gaps may exist.

We can simply call upon the .matrix() method and pass in the dataframe object. When we do, we generate a graphical view of the dataframe.

In the plot below, we can see that there are significant gaps within the DT and DTS columns, with minor gaps in the RPCELM, PHIF, and SW columns.

The sparkline to the right-hand side of the plot provides an indication of data completeness. If the line is at the maximum value (to the right) it shows that data row as being complete.

msno.matrix(df)
Missingno matrix plot illustrating where the missing data values occur. Image by author
Missingno matrix plot illustrating where the missing data values occur. Image by author
Another plot we can call upon is the bar plot, which provides a graphical summary of the number of points in each column.

msno.bar(df)
Missingno bar plot of the data within the selected wells. Image by author.
Missingno bar plot of the data within the selected wells. Image by author.
Using matplotlib to Create a Custom Data Coverage Plot
We can generate our own plots to show how the data sparsity varies across each of the wells. In order to do this, we need to manipulate the dataframe.

First, we create a copy of the dataframe to work on separately and then replace each column with a value of 1 if the data is non-null.

To make our plot work, we need to increment each column’s value by 1. This allows us to plot each column as an offset to the previous one.

data_nan = df.copy()
for num, col in enumerate(data_nan.columns[2:]):
data_nan[col] = data_nan[col].notnull() \* (num + 1)
data_nan[col].replace(0, num, inplace=True)
When we view the header of the dataframe we now have a series of columns with increasing values from 1 to 14.

data_nan.head()
Dataframe containing base values ready for plotting data coverage. Image by author
Dataframe containing base values ready for plotting data coverage. Image by author
Next, we can group the dataframe by the wellName column.

grouped = data_nan.groupby('wellName')
We can then create multiple subplots for each well using the new dataframe. Rather than creating subplots within subplots, we can shade from the previous column’s max value to the current column’s max value if the data is present. If data is absent, it will be displayed as a gap.

#Setup the labels we want to display on the x-axis
labels = ['BS', 'CALI', 'DT', 'DTS', 'GR', 'NPHI', 'RACEHM',
'RACELM', 'RHOB', 'RPCEHM', 'RPCELM',
'PHIF', 'SW', 'VSH']

#Setup the figure and the subplots
fig, axs = plt.subplots(3, 2, figsize=(20,20))
#Loop through each well and column in the grouped dataframe
for (name, well), ax in zip(grouped, axs.flat):
ax.set_xlim(0,9)

    #Setup the depth range
    ax.set_ylim(well.MD.max() + 50, well.MD.min() - 50)
    ax.set_ylim(well.MD.max() + 50, well.MD.min() - 50)

    # Create multiple fill betweens for each curve# This is between
    # the number representing null values and the number representing
    # actual values
    ticks = []
    ticks_labels = []
    for i, curve in enumerate(labels):
        ax.fill_betweenx(well.MD, i, well[curve], facecolor='lightblue')
        ticks.append(i)
        ticks_labels.append(i+0.5)

    # add extra value on to ticks
    ticks.append(len(ticks))

    #Setup the grid, axis labels and ticks
    ax.grid(axis='x', alpha=0.5, color='black')
    ax.set_ylabel('DEPTH (m)', fontsize=18, fontweight='bold')

    #Position vertical lines at the boundaries between the bars
    ax.set_xticks(ticks, minor=False)

    #Position the curve names in the centre of each column
    ax.set_xticks(ticks_labels, minor=True)

    #Setup the x-axis tick labels
    ax.set_xticklabels(labels,  rotation='vertical', minor=True,
                       verticalalignment='bottom', fontsize=14)

    ax.set_xticklabels('', minor=False)
    ax.tick_params(axis='x', which='minor', pad=-10)
    ax.tick_params(axis='y', labelsize=14 )

    #Assign the well name as the title to each subplot
    ax.set_title(name, fontsize=16, fontweight='bold')

plt.tight_layout()
plt.subplots_adjust(hspace=0.15, wspace=0.25)

# plt.savefig('missingdata.png', dpi=200)

plt.show()
view rawmed_missing_matplotlib.py hosted with ❤ by GitHub
Subplots showing the data coverage in each of the well logging curves from the five selected wells. Blue represents where data is present, white indicates missing values. Image by author.
Subplots showing the data coverage in each of the well logging curves from the five selected wells. Blue represents where data is present, white indicates missing values. Image by author.
From the plot, we can not only see the data range of each well, but we can also see that 2 of the 5 wells have missing DT and DTS curves, 2 of the wells have missing data within RPCELM, and 2 of the wells have missing values in the PHIF and SW curves.

Dealing With Missing Data
Discarding Variables
Variable discarding can be used in situations where missing values are extant within a variable, which in turn can make that variable unfit for the intended use. As such it can be removed from the dataset. If this is done, it can have wide implications for machine learning modelling, especially if the variable is important and present within other wells.

Within our example dataset, both DT and DTS are missing in two of the wells. We have the option to remove these wells from the dataset, or we can remove these two columns for all of the wells.

The following is an example of how we remove the two curves from the dataframe. For this we can pass in a list of the columns names to the drop() function, the axis, which we want to drop data along, in this case the columns (axis=1), and the inplace=True argument allows us to physically remove these values from the dataframe.

df.drop(df[['DT', 'DTS']], axis=1, inplace=True)
If we view the header of the dataframe, we will see that we have removed the required columns.

df.head()

However, if we call upon the info method…

df.info()
Which returns the result below. We can see that we still have a number of logging curves/columns with missing values. Namely RPCELM, PHIF, SW and VSH. The last three of which are petrophysical outputs and may only be present over the zones of interest.

<class 'pandas.core.frame.DataFrame'>
RangeIndex: 27845 entries, 0 to 27844
Data columns (total 14 columns):
wellName 27845 non-null object
MD 27845 non-null float64
BS 27845 non-null float64
CALI 27845 non-null float64
GR 27845 non-null float64
NPHI 27845 non-null float64
RACEHM 27845 non-null float64
RACELM 27845 non-null float64
RHOB 27845 non-null float64
RPCEHM 27845 non-null float64
RPCELM 27600 non-null float64
PHIF 27736 non-null float64
SW 27736 non-null float64
VSH 27844 non-null float64
dtypes: float64(13), object(1)
memory usage: 3.0+ MB
Discarding NaNs using Listwise Deletion
Listwise deletion, also known as case deletion, is a common and convenient approach to dealing with incomplete datasets. The method removes all rows (cases) where there are one or more missing values in the features.

In Python we can drop missing values from our pandas dataframe by calling upon a special function called dropna(). This will remove any NaN (Not a Number) values from the dataframe. The inplace=True argument allows us to physically remove these values from the dataframe without having to assign it to a new variable.

df.dropna(inplace=True)
If we call upondf.info() we will now see that our dataset has reduced to 27,491 non-null values for each column.

<class 'pandas.core.frame.DataFrame'>
Int64Index: 27491 entries, 0 to 27844
Data columns (total 14 columns):
wellName 27491 non-null object
MD 27491 non-null float64
BS 27491 non-null float64
CALI 27491 non-null float64
GR 27491 non-null float64
NPHI 27491 non-null float64
RACEHM 27491 non-null float64
RACELM 27491 non-null float64
RHOB 27491 non-null float64
RPCEHM 27491 non-null float64
RPCELM 27491 non-null float64
PHIF 27491 non-null float64
SW 27491 non-null float64
VSH 27491 non-null float64
dtypes: float64(13), object(1)
memory usage: 3.1+ MB
Summary
Now that we have removed the missing values, we can move on to the next step which is identifying and dealing with outliers and bad data.

This short article has shown three separate ways to visualise missing data. The first is by interrogating the dataframe using pandas, the second, by using the missingno library, and thirdly by creating a custom visualisation with matplotlib.

In the end, we covered two ways in which missing data can be removed from the dataframe. The first by discarding variables, and the second by discarding missing values within the rows.

The examples shown in this article illustrate a basic workflow for dealing with missing values. The data should always be QC’d thoroughly at each stage to ensure it is still fit for purpose.

Thanks for reading!

If you have found this article useful, please feel free to check out my other articles looking at various aspects of Python and well log data. You can also find my code used in this article and others at GitHub.

If you want to get in touch you can find me on LinkedIn or at my website.

Interested in learning more about python and well log data or petrophysics? Follow me on Medium.

Join Medium with my referral link – Andy McDonald
