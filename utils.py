import os, uuid, hashlib, time, bcrypt

code2type=["INFO","WARN","ERROR","UNDEFINED"]

def simple_log(db, code, info):
    db["logs"].insert_one({ "time": int(time.time()), "type": code2type[3 if code>3 else code], "info": info })

def simple_reply(action, status):
    return {"action": action,"status": status}

def md5(text):
    hl = hashlib.md5()
    hl.update(text.encode(encoding='utf-8'))
    return hl.hexdigest()

def get_hashed_password(plain_text_password):
    return bcrypt.hashpw(plain_text_password, bcrypt.gensalt())

def check_password(plain_text_password, hashed_password):
    return bcrypt.checkpw(plain_text_password, hashed_password)
                          
if __name__=="__main__":
    os._exit(0)