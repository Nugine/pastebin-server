import requests
import json
import os

payload = {
    'title': "title",
    'lang': "rust",
    'content': "// content",
    'expiration': 100
}

json.dump(payload,open('post.txt','w'))
url = "http://localhost:8088/record"

resp = requests.post(url,json=payload)
print(resp.status_code)
print(resp.json())

bench = f'ab -n 1000000 -c 500 -p post.txt  -T "application/json" {url}'
print(bench)
os.system(bench)
os.system('tail server.log')
