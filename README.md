# Overview

This is a repo representing some example skills with Rust, including automated testing, shell interface,
and a functioning REST server.

## Basic concept

This repo is more-or-less an H-Index calculator. The interface receives an array of citations (numbers) and
returns the 'H-Index' from the data set.

This calculation occurs in:
src/helpers/helper_h_index.rs

More info on the topic:
https://subjectguides.uwaterloo.ca/calculate-academic-footprint/YourHIndex

## The supplementary functionality is effectively 'different ways to get the result'

### There are two shell scripts in this repo.

exec_local.sh - This is an example shell implementation of passing arguments to the executable.

exec_server.sh - This is a REST server which can receive an array of citation numbers as a JSON payload
and then calculates the index based on the information.

## The general shell command follows this general pattern

cargo run -- (local/server) (data set)

If the command is run as a server, then the code will ignore supplementary numbers, with the expectation
that the data will be provided via REST POST.

Example Python client request:
```
response = requests.post(
    "http://127.0.0.1:8001/h_index",
    headers = {'Content-Type': 'application/json', 'Accept':'application/json'},
    json = {
        "citations": [7, 15, 20, 30, 33, 6, 5, 4]
    },
)
```
