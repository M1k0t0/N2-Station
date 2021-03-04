<template>
    <v-container fill-height class="pa-12">
        <v-dialog
        v-if="overlayRoom"
        v-model="overlayRoom"
        max-width="600"
        >
        <v-card>
            <v-card-title class="headline">
            直播相关信息
            </v-card-title>

            <v-card-text>
                <p class="mb-0">推流码率不要设的太高，否则观众也容易卡，一般2k足矣。</p>
                <p class="mb-0">但串流游戏需要稍高一些（动态画面容易糊）</p>
                <p class="mb-0">帧数设定为30即可，60也容易糊。</p>
                <p class="mb-0">推流地址：{{ global_.pushAddress+'/rtmp' }}</p>
                <p class="mb-0">流密钥：{{ overlayRoom }}?user={{ getRoomOwner(overlayRoom).name }}&amp;pass=你的密码</p>
            </v-card-text>

            <v-card-actions>
            <v-spacer></v-spacer>

            <v-btn
                color="green darken-1"
                text
                @click="overlayRoom = null"
            >
                懂了，这就去和obs对线
            </v-btn>
            </v-card-actions>
        </v-card>
        </v-dialog>
        <!-- <v-row justify="start" align="center" class="ma-0"> -->
        <transition-group name="flip-list" tag="div" class="row ma-0 align-center justify-start">
        <v-col 
        xs="3" md="4" sm="6" 
        v-for="room of sortedRoomList"
        :key="room.id"
        >
        <v-card class="pa-6 d-flex flex-column" height="350px" style="border-radius:16px;">
            <v-overlay
            absolute
            opacity=0.8
            :value="confirmOverlayRoom==room.id"
            >
            <v-progress-circular
                indeterminate
                size="64"
                v-if="!confirmDialog"
            ></v-progress-circular>
            <v-card elevation=4 v-if="confirmDialog">            
            <v-card-title class="headline grey lighten-2" style="background-color:#72767d!important;">
                是否要删除该房间？
            </v-card-title>
            <v-card-text class="mt-3">
                {{room.id}} 将会永久失去！（真的很久！）
            </v-card-text>
            <v-card-actions>
                <v-btn
                    color="error"
                    @click="deleteRoom(room.id)"
                >
                    确认
                </v-btn>
                <v-spacer></v-spacer>
                <v-btn
                    color="warning"
                    @click="confirmOverlayRoom=''"
                >
                    取消
                </v-btn>
            </v-card-actions>
            </v-card>
            </v-overlay>
            <v-card-text class="pb-0" style="max-height: 30%;">
            <div class="d-flex">
                <p class="mb-0 mr-auto pt-1">
                    {{ room.id }}
                </p>
                <v-btn 
                    icon 
                    x-small 
                    color="grey" 
                    class="mr-n2" 
                    @click="confirmOverlayRoom=room.id;"
                    :disable="!confirmDialog"
                >
                    <v-icon>mdi-close</v-icon>
                </v-btn>
            </div>
            <p class="display-1 text--primary" v-line-clamp:20="1">
                {{ room.title }}
            </p>
            </v-card-text>
            <div class="ml-4" v-line-clamp:20="2">
                <v-chip
                v-for="(tag,ind) in room.tag"
                class="mr-1 mb-1"
                :key="ind" 
                :color="randColor(room.id, ind)">
                    {{ tag }}
                </v-chip>
            </div>
            <v-card-text v-line-clamp:20="3">
                描述：{{ room.desc }}
                <br />
                状态：
                <span v-if="room.status=='open' && haveAnOpenedRoom(room.id)"><b>开启📢</b></span>
                <span v-if="room.status=='close'"><b>关闭🚬</b></span>
            </v-card-text>

            <v-card-actions class="d-flex mt-auto align-self-start" style="width: 100%">
            <v-btn
                color="green lighten-2 mr-auto"
                @click="toggleLiveStatus(room.id)"
                :disabled="OpenedRoom!='' && OpenedRoom!=room.id"
                :loading="loading && processingRoom==room.id"
                style="border-radius:7px;"
            >
                Toggle
            </v-btn>
            <v-btn
                class="text-center"
                color="warning"
                @click="editRoomInfo(room.id)"
                style="border-radius:7px;"
            >
                Edit
            </v-btn>
            </v-card-actions>
        </v-card>

        </v-col>
        <v-col 
        xs="3" md="4" sm="6"
        key="createRoom"
        v-if="renderButton"
        >
        <v-card class="pa-0 d-flex flex-column" height="350px" style="border-radius:16px;">
            <v-btn
            tile
            color="grey darken-4"
            width="100%"
            height="100%"
            style="opacity:0.7;border-radius:16px;"
            @click="routeTo('/panel/createRoom')"
            >
            <v-icon x-large style="font-size:55px;">mdi-plus-circle-outline</v-icon>
            </v-btn>
        </v-card>
        </v-col>
        </transition-group>
        <!-- </v-row> -->
    </v-container>
</template>

<script>
import axios from 'axios';
axios.defaults.withCredentials = true;

export default {
    data: () => ({
        userRoomList: [],
        error_msg: '',
        colors: ["primary","deep-purple accent-4","orange","green","red","pink","cyan"],
        vis: [0,0,0,0,0,0,0,0],
        colorSetting: {},
        OpenedRoom: '',
        loading: false,
        processingRoom: '',
        overlayRoom: null,
        confirmOverlayRoom: '',
        confirmDialog: true,
        renderButton: false
    }),
    methods: {
        requestUserRoomList(){
            this.$set(this.$root,'userRoomList',{});
            axios
            .get(this.$root.backend+'/api/user/rooms')
            .then(response => {
                this.$set(this.$root,'userRoomList',response.data); // full data returned
            })
            .catch(error => {
                console.log(error);
            })
            .finally(() => {
                this.updateUserRoomList();
                this.loading=false;
                this.confirmOverlayRoom='';
                this.confirmDialog=true;
            })
        },
        updateUserRoomList(){
            if(!this.global_.getCookie('Authorization') || this.$root.userRoomList.status==-10 || this.$root.userRoomList.status==-11){
                // console.log('redirecting');
                this.error_msg="cookie错误";
                this.global_.delCookie('Authorization');
                this.routeTo('/login');
            }else this.error_msg=this.global_.get_err_msg(this.$root.userRoomList.action,this.$root.userRoomList.status);
            this.OpenedRoom='';
            this.userRoomList=[];
            for(var i in this.$root.userRoomList.data){ // unsafe
                var tmp=this.$root.userRoomList.data[i];
                tmp.id=i;
                this.userRoomList.push(tmp);
            }
            this.global_.request.getRoomList(this);
            this.renderButton=true;
        },
        routeTo(path, params={}){
            this.$router.push({
                path,
                params
            })
        },
        randColor(id,num){
            if(this.colorSetting[id]==undefined) this.colorSetting[id]={};
            if(this.colorSetting[id][num]==undefined){
                var rn;
                if(this.vis[0]==7) this.vis=[0,0,0,0,0,0,0,0];
                for(rn=Math.floor(Math.random()*7)+1;this.vis[rn];){
                    rn=Math.floor(Math.random()*7)+1;
                }
                this.vis[rn]=1;
                this.vis[0]+=1;
                this.colorSetting[id][num]=this.colors[rn-1];
            }
            return this.colorSetting[id][num];
        },
        haveAnOpenedRoom(id){
            this.OpenedRoom=id;
            return true;
        },
        toggleLiveStatus(id){
            this.processingRoom=id;
            this.loading=true;
            if(this.$root.userRoomList.data[id].status=='open')
                this.global_.request.closeRoom(this,id)
                .then(() => {
                    this.requestUserRoomList();
                    this.processingRoom='';
                });
            else if(this.$root.userRoomList.data[id].status=='close'){
                this.overlayRoom=id;
                this.global_.request.openRoom(this,id)
                .then(() => {
                    this.requestUserRoomList();
                    this.processingRoom='';
                });
            }
        },
        getRoomOwner(id){
            return this.$root.roomList[id].user;
        },
        deleteRoom(id){
            this.confirmDialog=false;
            this.global_.request.deleteRoom(this,id)
            .then(() => {
                this.requestUserRoomList();
            });
        }
    },
    created(){
        this.$root.panelMenuIndex=0;
    },
    mounted() {
        this.requestUserRoomList();
        // this.$set(this.$root,'userRoomList',{"action":"getUserRoomList","data":{"Test":{"_id":"Test","desc":"\u4e00\u4e2a\u7528\u6765\u6d4b\u8bd5\u7684\u623f\u95f4","image":"default","status":"close","tag":["\u6d4b\u8bd5","Tag\u6d4b\u8bd5","\ud83d\ude00","wwwwwwwww","abababababababab"],"time":{"createTime":1612151938,"openTime":1613658131,"stopTime":1613664149},"title":"\u54fc \u54fc \u554a\u554a\u554a\u554a\u554a\u554a\u554a\u554a\u554a\u554a\u554a\u554a\u554a\u554a\u554a\u554a\u554a\u554a\u554a"},"Test2":{"_id":"Test2","desc":"\u4e00\u4e2a\u7528\u6765\u6d4b\u8bd5\u7684\u623f\u95f42","image":"default","status":"close","tag":["Tag\u6d4b\u8bd52","Tag\u6d4b\u8bd5"],"time":{"createTime":1612287215,"openTime":1613721175,"stopTime":1613721186},"title":"\u6d4b\u8bd5\u623f\u95f42"}},"status":0});
        // this.updateUserRoomList(); // DEBUG
    },
    computed: {
        sortedRoomList() {
            return this.userRoomList.slice().sort((e1,e2) => {
                return e1.status=="open"?-1:e2.status=="open"?1:0;
            });
        },
    },
    beforeRouteUpdate (to, from, next) {
        this.$root.panelMenuIndex=0;
        this.requestUserRoomList();
        next();
    }
}
</script>