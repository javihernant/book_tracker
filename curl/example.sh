#GET /
curl 127.0.0.1:3000

#Create user
curl -X POST \
-H "Content-Type: application/json" \
-d @curl/json/create_user.json \
127.0.0.1:3000/users