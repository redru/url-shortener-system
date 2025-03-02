curl -X POST http://localhost:8000/shorten \
  -H "Content-Type: application/json" \
  -d '{"long_url": "https://www.example.com/very/long/url/that/needs/shortening-3"}'
