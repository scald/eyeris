# Eyeris API Documentation

## Base URL

```
http://localhost:3000/api/v1
```

## Endpoints

### Analyze Image

Analyzes an image using AI vision capabilities.

```http
POST /analyze
```

#### Request

- Method: `POST`
- Content-Type: `multipart/form-data`

##### Parameters

| Name  | Type   | In    | Description                                                 |
| ----- | ------ | ----- | ----------------------------------------------------------- |
| image | file   | form  | The image file to analyze                                   |
| model | string | query | (Optional) The model to use for analysis. Default: "gpt-4o" |

#### Response

```json
{
  "success": true,
  "message": "Analysis completed successfully",
  "data": {
    "analysis": "Detailed JSON analysis of the image",
    "token_usage": {
      "prompt_tokens": 123,
      "completion_tokens": 456,
      "total_tokens": 579
    }
  }
}
```

##### Error Response

```json
{
  "success": false,
  "message": "Error message describing what went wrong"
}
```

### Health Check

Check if the API is running and healthy.

```http
GET /health
```

#### Response

```json
{
  "success": true,
  "message": "Service is healthy"
}
```

## Error Codes

- `400 Bad Request`: Invalid request (missing image, invalid format)
- `500 Internal Server Error`: Server-side error

## Example Usage

### cURL

```bash
curl -X POST http://localhost:3000/api/v1/analyze \
  -F "image=@path/to/your/image.jpg" \
  -G -d "model=gpt-4o"
```

### Python

```python
import requests

url = "http://localhost:3000/api/v1/analyze"
files = {"image": open("path/to/your/image.jpg", "rb")}
params = {"model": "gpt-4o"}

response = requests.post(url, files=files, params=params)
print(response.json())
```

### JavaScript

```javascript
async function analyzeImage(imageFile) {
  const formData = new FormData();
  formData.append("image", imageFile);

  const response = await fetch(
    "http://localhost:3000/api/v1/analyze?model=gpt-4o",
    {
      method: "POST",
      body: formData,
    }
  );

  return await response.json();
}
```
