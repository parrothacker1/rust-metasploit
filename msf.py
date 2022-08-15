#!/home/gitpod/.pyenv/shims/python
import msgpack,requests
requests.packages.urllib3.disable_warnings()

data=["auth.login","user","password"]
packed_data=msgpack.packb(data);

req=requests.post("https://127.0.0.1:4040/api",data=packed_data,headers={"Content-Type":"binary/message-pack"},verify=False)
unpacked_data=msgpack.unpackb(req.content,strict_map_key=False)

data=["db.status",unpacked_data[b"token"]]
packed_data=msgpack.packb(data)

req=requests.post("https://127.0.0.1:4040/api",data=packed_data,headers={"Content-Type":"binary/message-pack"},verify=False)
print(msgpack.unpackb(req.content,strict_map_key=False))
