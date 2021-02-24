import pymongo, os, sys, uuid, utils
from flask import jsonify
from interface import User

LOG_INFO=0
LOG_WARN=1
LOG_ERROR=2
LOG_UNDEFINED=3

RETURN_DATA=0
RETURN_CODE=-1

def connect_server(addr):
    try:
        config=utils.read_json_file("config.json")
        cli = pymongo.MongoClient(config["db_address"])
        if 'db_user' in config and 'db_pass' in config:
            db=cli.admin
            db.authenticate(config['db_user'],config['db_pass'])
    except BaseException as e:
        print(e)
        sys.exit("Cannot connect to dbserver")
    return cli

def check_db(cli):
    if "N2-Station" not in cli.list_database_names():
        setup_db(cli["N2-Station"])
        
def setup_db(db):
    utils.simple_log(db,LOG_INFO,"Database being set up")

def add_user_to_db(db, data):
    if "id" not in data:
        data["_id"]=str(uuid.uuid4()).replace('-','')
    if "name" not in data:
        data["name"]=str(uuid.uuid4())[0:8]
    if "password" not in data:
        data["password"]=str(uuid.uuid4())[0:8]
    if "email" not in data:
        data["email"]="random@debug.dev"
    if "rooms" not in data:
        data["rooms"]=[]
    if "limit" not in data:
        data["limit"]={ "room_count": 5 } # Config WIP
    if 'token' not in data:
        data["token"]={}
    
    if db["users"].find_one({"email":data["email"]}) != None:
        return (-1, RETURN_CODE)
    if db["users"].find_one({"name":data["name"]}) != None or db["users"].find_one({"name_lower":data["name"].lower()}) != None:
        return (-2, RETURN_CODE)
    
    #if 密码强度不合格:   WIP
        #return -3

    data["name_lower"]=data["name"].lower();
    data["password"]=utils.get_hashed_password(data["password"])
    
    db["users"].insert_one(data)
    return (data["_id"], RETURN_DATA)

def delete_user_from_db(db, data):
    method, data = utils.get_input(data,["id","name","email"])

    method=method.replace("id","_id",1)

    if method==None:
        return (-10, RETURN_CODE)

    db["users"].delete_one({method:data})

    return (0,RETURN_DATA)

def get_user_by_token(db,token):
    if token==None:
        raise RuntimeError('-10')
    t=db["tokens"].find_one({"token": token})
    if t == None:
        raise RuntimeError('-11')
    
    try:
        user=User(db,{"id":t["userID"]})
    except BaseException as e:
        raise RuntimeError(str(e))
    
    return user

if __name__=="__main__":
    os._exit(0)