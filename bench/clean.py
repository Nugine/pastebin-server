import requests
import json
import os
import time

def f(n):
    payload = {
        'title': "title",
        'lang': "rust",
        'content': "// content",
        'expiration': n
    }

    json.dump(payload,open('post.txt','w'))
    url = "http://localhost:8088/record"


    bench = f'ab -n 15000 -c 100 -p post.txt -T "application/json" {url}'
    print(bench)
    os.system(bench)

for i in range(20,10,-1):
    f(i)

for i in range(20,0,-1):
    time.sleep(1)
    print(f'wait {i} s')

os.system('cat server.log | rg "CLEAN "')
