import os, uuid, hashlib

code2type=["INFO","WARN","ERROR","UNDEFINED"]

def simple_log(db, code, info):
    db["logs"].insert_one({ "time": int(time.time()), "type": code2type[3 if code>3 else code], "info": info })

def md5(text):
    hl = hashlib.md5()
    hl.update(text.encode(encoding='utf-8'))
    return hl.hexdigest()

def add_test_user_to_db(db, data, cache):
    if "id" not in data:
        data["_id"]=str(uuid.uuid4())
    if "name" not in data:
        data["name"]=str(uuid.uuid4())[0:8]
    if "pass" not in data:
        data["pass"]=md5(data["name"]+md5(str(uuid.uuid4())[0:9]))
    if "email" not in data:
        data["email"]="random@debug.dev"
    if "rooms" not in data:
        data["rooms"]=[]
    if "limit" not in data:
        data["limit"]={ "room_count": 5 }
    db["users"].insert_one(data)
    cache["userList"][data["_id"]]={ "id": data["_id"], "name": data["name"], "email": data["email"] }
    return cache
                          
if __name__=="__main__":
    os._exit(0)