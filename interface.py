import os, uuid, time, pymongo, utils

RETURN_DATA=0
RETURN_CODE=-1

class User:
    syncignore=["db","index","id"]
    def __init__(self, db, data):
        self.db=db
        self.index=utils.get_input(data, ["id","name","email"])
        if self.index[0]==None:
            raise RuntimeError('-10')
        self.update_data(self.index[0], self.index[1])

    '更新数据'
    def update_data(self, method=None, data=None):
        if method==None or data==None:
            try:
                method="id"
                data=self.id
            except:
                raise RuntimeError('-10')
        method=method.replace("id","_id",1)
        user=self.db["users"].find_one({method:data})
        if user==None:
            raise RuntimeError('-1')

        self.id=user["_id"]
        self.name=user["name"]
        self.email=user["email"]
        self.password=user["password"]
        self.rooms=user["rooms"]
        self.streaming=user["streaming"]
        self.limit=user["limit"]
        self.token=user["token"]

        self.index = ("id", self.id)
    
    '同步数据'
    def sync_data(self,endpoint=[]):
        if len(endpoint)==0:
            data={}
            for k,v in vars(self).items():
                if k in self.syncignore:
                    continue
                data[k]=v
            data={ "$set": data }
        else:
            data={}
            for k,v in vars(self).items():
                if k in endpoint:
                    data[k]=v
            data={ "$set": data }

        self.db["users"].update_one({ "_id": self.id }, data)

    '生成Token'
    def generate_token(self):
        t=str(uuid.uuid4()).replace('-','')
        self.token[t]={ "time": int(time.time()), "permission": -1 } # Permission system work in progress
        self.sync_data()

        self.db['tokens'].insert_one({ "token": t, "time": int(time.time()), "userID": self.id })

        return t

    def create_room(self, data):
        data["time"]={ "createTime": int(time.time()), "openTime": -1, "stopTime": -1 }
        data["status"]="close"

        id=data["id"]
        del data["id"]
        data["_id"]=id

        self.rooms[id]=data
        self.sync_data(['rooms'])

        data["userID"]=self.id

        self.db['rooms'].insert_one(data)
    
    def delete_room(self, data):
        self.close_room(data)

        del self.rooms[data["id"]]
        self.sync_data(['rooms'])

        self.db['rooms'].delete_one({"_id":data["id"]})
    
    def open_room(self, data):
        self.rooms[data["id"]]["status"]="open"
        self.rooms[data["id"]]["time"]["openTime"]=int(time.time())
        self.streaming.append(data["id"])
        self.sync_data(['rooms','streaming'])

        self.db['rooms'].update_one({"_id":data["id"]},{ 
            "$set": { 
                "status": "open", 
                "time": { 
                    "createTime": self.rooms[data["id"]]["time"]["createTime"],
                    "openTime": int(time.time()),
                    "stopTime": self.rooms[data["id"]]["time"]["stopTime"]
                }
            }
        })

    def close_room(self, data):
        if data["id"] not in self.streaming:
            return
        self.rooms[data["id"]]["status"]="close"
        self.rooms[data["id"]]["time"]["stopTime"]=int(time.time())
        del self.streaming[self.streaming.index(data["id"])]
        self.sync_data(['rooms','streaming'])

        self.db['rooms'].update_one({"_id":data["id"]},{ 
            "$set": { 
                "status": "close", 
                "time": { 
                    "createTime": self.rooms[data["id"]]["time"]["createTime"],
                    "openTime": self.rooms[data["id"]]["time"]["openTime"],
                    "stopTime": int(time.time()) 
                }
            }
        })

if __name__=="__main__":
    os._exit(0)