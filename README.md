# Hurricane Service Implementation

Project created and build using the following tools:
 - cargo 1.53.0 (4369396ce 2021-04-27)
 - rustc 1.53.0 (53cb7b09b 2021-06-17)
 - rustup 1.24.3 (ce5817a94 2021-05-31)

Cargo package dependencies:
 - rocket 0.5.0-rc.1
 - csv 1.1

# How to run

Start the webserver by running 'cargo run' inside the root folder.
This boots up a webserver which listens on the following URL: http://127.0.0.1:8000

Send a HTTP GET Request to the following URLs to get answers from the server.
This can be done by a tool like Postman or by simply opening the URL inside a browser.

http://127.0.0.1:8000/hurricanes/propability_by_month?month=6 (Valid values for the month parameter range from 1 to 12)

http://127.0.0.1:8000/hurricanes/year_and_month_with_most_hurricanes

# How to start unittests

!ToDo!