#!/bin/zsh

# Example request from client:
# requests.post(
#        "http://127.0.0.1:8001/h_index",
#        headers = {'Content-Type': 'application/json', 'Accept':'application/json'},
#        json = {
#            "citations": [7, 15, 20, 30, 33, 6, 5, 4]
#        },
#    )

cargo run -- server