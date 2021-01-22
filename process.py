import pymongo, os, sys

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
    
if __name__=="__main__":
    os._exit(0)