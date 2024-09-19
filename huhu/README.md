
# Huhu Log Exploration

Re-implementation of my Python project to explore Apache Webserver logs files.

Rust has the distinction of having the fastest regex library on the planet; it
will be interesting to see how this goes.


## MVP

1. Read all lines from a single log file
2. Match hostnames using regex and count how often they appear.


## Next

1. Read lines of text from
    - Standard input
    - One or more log files
    - One or more log files compressed in zstd, gzip, etc.

2. Extract the useful parts of each line into a 'record'.
    - Simply using whitespace will give us hostnames and client IP addresses
    - Hand-written regex to extract timestamps, integers, paths, status codes
    - Regex builder (ie. from Python huhu project) to create regex from Apache
      log specification, ie. "%h %A..."

3. Group records in various ways
    - all
    - file
    - hostname
    - hostname + path
    - date
    - date + time bins
    - HTTP status codes and verbs

4. Summarise records in various ways
    - counts
    - error counts
    - page counts
    - bytes transferred
    - cpu time used
    - paths with highest cpu time
    - user IP hostnames
    - user IP countries


## Personal Interests

I run a small business hosting Django web applicaitons. I'm interested in
producing a few basic reports from my webserver logs. I have one log file
per day which is fine-grained grouping enough for me.

* Per client hostname
    - Basic hits
    - Page hits
    - Bytes transferred
    - CPU time used
    - Unique clients

* By hostname + path
    - Top 10 CPU used
    - Top 10 Bytes transferred
    - Top 10 404/500 errors

* Peak hits per second over some sliding window
    - Page hits per second (ie. from Django app)
    - Non-page hits per second (ie. directly from webserver)


## Historical Perspective

Until Google Analytics destroyed the market with its client-side 'logging', we
used [Analog](https://en.wikipedia.org/wiki/Analog_(program)) for years. Its
basic reports were:

1. General
    - Successful requests: 501,959 (8,629)
    - Average successful requests per day: 438 (1,232)
    - Successful requests for pages: 49,415 (756)
    - Average successful requests for pages per day: 43 (107)
    - Failed requests: 8,603 (179)
    - Redirected requests: 582 (6)
    - Distinct files requested: 4,860 (2,294)
    - Distinct hosts served: 25,959 (544)
    - Corrupt logfile lines: 317,387
    - Data transferred: 7.26 gigabytes (93.98 megabytes)
    - Average data transferred per day: 6.49 megabytes (13.43 megabytes)

2. Daily Report
    - Hits and page requests per day
    - Highlight busiest day

3. Day of the week report
    - Total hits and page requests per day of the week, ie. Monday, Tuesday, etc.

4. Hourly
    - As above, but for hours of the day, ie. 12AM, 1AM, 2AM, etc.

5. External search words - presumably from referral URL

6. Browser summary & operating system summary
    - Top 20 browsers and OS by page requests from user agent string.

7. Status code totals.
8. File size and extensions.
9. Internal search engine queries.
10. Top 20 paths by bytes and requests.
