import os, uuid, time, pymongo, utils

RETURN_DATA=0
RETURN_CODE=-1

class User:
    syncignore=["db","index","id"]
    def __init__(self, db, data):
        self.db=db
        self.index=utils.get_input(data, ["id","user","email"])
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
        self.user=user["user"]
        self.email=user["email"]
        self.password=user["password"]
        self.rooms=user["rooms"]
        self.limit=user["limit"]
        self.token=user["token"]

        self.index = ("id", self.id)
    
    '同步数据'
    def sync_data(self):
        data={}
        for k,v in vars(self).items():
            if k in self.syncignore:
                continue
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


    
    
if __name__=="__main__":
    os._exit(0)