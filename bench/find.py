import requests
import json
import os

payload = {
    'title': "title",
    'lang': "rust",
    'content': "// content",
    'expiration': 3600
}

url = "http://localhost:8088/record"
resp = requests.post(url, json=payload)
print(resp.status_code)

resp=resp.json()
print(resp)

url = url+'/'+resp['key']

bench = f'ab -n 100000 -c 100 {url}'
print(bench)
os.system(bench)