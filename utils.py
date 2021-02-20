import os, uuid, hashlib, time, bcrypt, datetime, time_uuid, json, sys, requests

code2type=["INFO","WARN","ERROR","UNDEFINED"]

def simple_log(db, code, info):
    db["logs"].insert_one({ "time": int(time.time()), "type": code2type[3 if code>3 else code], "info": info })

def simple_reply(action, status, desc=None):
    if desc!=None:
        return {"action": action, "status": int(status), "desc": str(desc)}
    return {"action": action, "status": int(status)}

def md5(text):
    hl = hashlib.md5()
    hl.update(text.encode(encoding='utf-8'))
    return hl.hexdigest()

def get_hashed_password(plain_text_password):
    return bcrypt.hashpw(plain_text_password.encode('utf8'), bcrypt.gensalt())

def check_password(plain_text_password, hashed_password):
    return bcrypt.checkpw(plain_text_password.encode('utf8'), hashed_password)

def get_timestamp_from_uuid(plain_uuid):
    uuidObj = uuid.UUID('{'+str(plain_uuid)+'}')
    ts = time_uuid.TimeUUID(bytes=uuidObj.bytes).get_timestamp()
    return ts

def get_input(d,l=None):
    for key in d:
        if d[key] != None and d[key] != '' and (True if l == None else key in l):
            return (key,d[key])
    return (None, None)

def get_date_after(days):
    return datetime.datetime.today() + datetime.timedelta(days=int(days))

def delete_key(data, keyList):
    for i in keyList:
        try:
            del data[i]
        except:
            pass
    return data

def read_json_file(name):
    with open(sys.path[0]+'/'+name,'r') as f:
        return json.load(f)

def get_stat():
    return json.loads(requests.get('https://n2station.live/stat').text) # url should be configurable

if __name__=="__main__":
    os._exit(0)