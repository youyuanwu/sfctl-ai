```ps1
$env:GEMINI_API_KEY = 'my-key'

curl "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash:generateContent" \
  -H 'Content-Type: application/json' \
  -H 'X-goog-api-key: GEMINI_API_KEY' \
  -X POST \
  -d '{
    "contents": [
      {
        "parts": [
          {
            "text": "Explain how AI works in a few words"
          }
        ]
      }
    ]
  }'

$headers = @{
  "Content-Type" = "application/json"
  "X-goog-api-key" = $env:GEMINI_API_KEY
}

$body = @{
  contents = @(
    @{
      parts = @(
        [ordered]@{
          text = "Explain how AI works in a few words"
        }
      )
    }
  )
} | ConvertTo-Json -Depth 5

$resp = Invoke-RestMethod `
  -Uri "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash:generateContent" `
  -Headers $headers `
  -Method Post `
  -Body $body

$resp | ConvertTo-Json -Depth 10
```