<template>
    <v-container fill-height class="pa-12">
        <!-- <v-row justify="start" align="center" class="ma-0"> -->
        <transition-group name="flip-list" tag="div" class="row ma-0 align-center justify-start">
        <v-col 
        xs="3" md="4" sm="6" 
        v-for="room of sortedRoomList"
        :key="room.id"
        >
        <v-card class="pa-6" height="350px">
            <v-card-text class="pb-0">
            <div>
                {{ room.id }}
            </div>
            <p class="display-1 text--primary">
                {{ room.title }}
            </p>
            </v-card-text>
            <div class="ml-4">
                <v-chip
                v-for="(tag,ind) in room.tag" 
                :key="ind" 
                :color="randColor(room.id, ind)">
                    {{ tag }}
                </v-chip>
            </div>
            <v-card-text>
                描述：{{ room.desc }}
                <br />
                状态：
                <span v-if="room.status=='open' && haveAnOpenedRoom(room.id)"><b>开启📢</b></span>
                <span v-if="room.status=='close'"><b>关闭🚬</b></span>
            </v-card-text>

            <v-card-actions>
            <v-btn
                class="mt-12"
                color="green lighten-2"
                @click="toggleLiveStatus(room.id)"
                :disabled="OpenedRoom!='' && OpenedRoom!=room.id"
                :loading="loading && processingRoom==room.id"
            >
                Toggle Status
            </v-btn>
            <v-btn
                class="mt-12 ml-4 pr-4"
                color="warning"
                @click="editRoomInfo(room.id)"
            >
                Edit
            </v-btn>
            </v-card-actions>
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
        processingRoom: ''
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
            })
        },
        updateUserRoomList(){
            if(this.$route.params.pos=='rooms'){
                if(this.$root.userRoomList.status==-10 || this.$root.userRoomList.status==-11){
                    console.log('redirecting');
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
            }
            this.global_.request.getRoomList(this);
        },
        routeTo(base, data=''){
            this.$router.push({
                path: base+data,
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
                this.global_.request.closeRoom(this,id);
            else if(this.$root.userRoomList.data[id].status=='close')
                this.global_.request.openRoom(this,id);
            setTimeout(() => {
                this.requestUserRoomList();
                this.processingRoom='';
            }, 1000); // it's stupid
        }
    },
    mounted() {
        this.$set(this.$root.bread,1,{
            text: 'UserPanel',
            disabled: false,
            href: '#/panel/rooms',
        });
        this.requestUserRoomList();
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
        this.pos = to.params.pos;
        this.$set(this.$root.bread,1,{
            text: 'UserPanel',
            disabled: false,
            href: '#/panel/rooms',
        });
        this.requestUserRoomList();
        next();
    }
}
</script>