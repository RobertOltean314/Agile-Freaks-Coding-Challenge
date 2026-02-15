# Agile-Freaks-Coding-Challenge

## Overview

You have been hired by a company that builds an app for coffee addicts. You are
responsible for taking the user&#39;s location and returning a list of the three closest
coffee shops.

## Input

The coffee shop list is a comma separated file with rows of the following form:
`Name,Y Coordinate,X Coordinate`
The quality of data in this list of coffee shops may vary. Malformed entries should
cause the
program to exit appropriately.
Your program will be executed directly from the command line and will be provided
three
arguments in the following order:
`&lt;user x coordinate&gt; &lt;user y coordinate&gt; &lt;shop data url&gt;`
Notice that the data file will be read from an network location (ex:
https://raw.githubusercontent.com/Agilefreaks/test_oop/master/coffee_shops.csv)

## Output

Write a program that takes the user&#39;s coordinates encoded as listed above and prints
out a
newlineseparated list of the three closest coffee shops (including distance from the
user) in
order of closest to farthest. These distances should be rounded to four decimal
places.
Assume all coordinates lie on a plane.
The output should be very simple no UI is required.

## Example

Using the
https://raw.githubusercontent.com/Agilefreaks/test_oop/master/coffee_shops.csv
**Input**
`47.6 -122.4 coffee_shops.csv`
**Expected output**

```
Starbucks Seattle2,0.0645
Starbucks Seattle,0.0861
Starbucks SF,10.0793
```
