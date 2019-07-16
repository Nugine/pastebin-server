import requests
import json
import os

payload = {
    'title': "title",
    'lang': "rust",
    'content': "// content",
    'expiration': 3
}

url = "http://localhost:8088/record"

resp = requests.post(url,json=payload)
print(resp.status_code)
print(resp.json())
