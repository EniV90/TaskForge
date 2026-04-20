curl -X POST http://0.0.0.0:8001/api/v1/create \
-H "Content-Type: application/json" \
-d '{
  "email": "test123@gmail.com",
  "password": "password123"
}'

curl -u test@gmail.com:password -X GET \
http://0.0.0.0:8001/api/v1/auth/login