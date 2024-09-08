#!/bin/bash

#Create user
curl -X POST \
-H "Content-Type: application/json" \
-d @json/create_user.json \
127.0.0.1:3000/user/create