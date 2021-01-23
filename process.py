import pymongo, os, sys, uuid, utils

LOG_INFO=0
LOG_WARN=1
LOG_ERROR=2
LOG_UNDEFINED=3

def connect_server(addr):
    try:
        cli = pymongo.MongoClient("mongodb://localhost:27017/")
    except BaseException as e:
        print(e)
        sys.exit("Cannot connect to dbserver")
    return cli["N2-Station"]

def check_db(db):
    if "N2-Station" not in cli.list_database_names():
        setup_db()
        
def setup_db(db):
    utils.simple_log(db,LOG_INFO,"Database being set up")

def add_user_to_db(db, data, cache):
    if "id" not in data:
        data["_id"]=str(uuid.uuid4()).replace('-','')
    if "user" not in data:
        data["name"]=str(uuid.uuid4())[0:8]
    if "pass" not in data:
        data["pass"]=md5(data["name"]+md5(str(uuid.uuid4())[0:9]))
    if "email" not in data:
        data["email"]="random@debug.dev"
    if "rooms" not in data:
        data["rooms"]=[]
    if "limit" not in data:
        data["limit"]={ "room_count": 5 } # Config WIP
    
    if db["users"].find_one({"email":data["email"]}) != None:
        return (-1, -1)
    if db["users"].find_one({"user":data["user"]}) != None:
        return (-2, -2)
    
    #if 密码强度不合格:   WIP
        #return -3
        
    data["pass"]=utils.md5(utils.md5(data["pass"]))
    
    db["users"].insert_one(data)
    if "userList" not in cache:
        cache["userList"]={}
    cache["userList"][data["_id"]]={ "id": data["_id"], "user": data["user"], "email": data["email"] }
    return (data["_id"], 0)
    
if __name__=="__main__":
    os._exit(0)