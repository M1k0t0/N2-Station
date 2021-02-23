<template>
    <v-container fluid fill-height class="align-center justify-center">
        <v-row
            align="center"
            justify="center"
            class="fill-height"
        >
            <v-col sm="12" md="11" class="pb-1 fill-height">
                <v-card tile class="px-6 pt-3 on-phone-card-padding-bottom on-desktop-card-padding-bottom" height="95%">
                    <v-row
                        align="stretch"
                        justify="center"
                        style="flex-wrap: wrap;"
                    >
                        <v-col sm="12" md="8" class="col-me">
                            <p class="title mb-0">{{ room.title }}</p>
                            <p class="caption mb-2">{{ room.desc }}</p>
                            <!-- <video
                            class="pl-sm-12 pr-sm-12 pt-sm-0 pb-sm-0 no-outline"
                            width="100%"
                            height="auto"
                            id="videoElement" controls autoplay>
                                Your browser is too old which doesn't support HTML5 video.
                            </video> -->
                            <div 
                            class="pt-sm-0 pb-sm-0 no-outline"
                            id="videoFrame"
                            ></div>
                            <v-card flat>
                                <v-row
                                    align="center"
                                    justify="start"
                                >
                                    <v-col cols="2">
                                        <v-gravatar 
                                            v-if="room" 
                                            class="" 
                                            :size="55" 
                                            :email="$root.roomList[$route.params.id].user.email" 
                                            style="display:inline; border-radius: 60px;" 
                                        />
                                    </v-col>
                                    <v-col cols="5">
                                        <p 
                                            v-if="room" 
                                            style="display:inline;font-size: 25px;"
                                        >
                                            {{$root.roomList[$route.params.id].user.name}}
                                        </p>
                                    </v-col>
                                </v-row>
                            </v-card>
                        </v-col>
                        <v-spacer />
                        <v-col sm="12" md="4" class="col-me pb-0 on-phone-chat-box">
                            <v-card tile id="chatBox" style="height:100%;" class="d-flex on-phone-chat-box-flex-reverse on-desktop-chat-box-flex" color="#36393f">
                                <v-card flat id="chatContent" height="80%" class="d-flex on-phone-chat-box-flex-reverse pa-5" color="#36393f">
                                    <p class="body-2">
                                        ck
                                        <font color="#dcddde">：这里还没建设完成哦，暂时无法使用w</font>
                                    </p>
                                </v-card>
                                <v-card color="#36393f" height="20%">
                                    <v-row align="center" class="mx-auto">
                                    <v-col cols="9" class="on-phone-chat-box-textarea">
                                        <v-textarea 
                                            :rows="2"
                                            no-resize
                                            solo 
                                            clearable
                                            hide-details
                                            color="white" 
                                            background-color="#40444b"
                                            label="发个弹幕呗~" 
                                            class="no-outline mb-auto"
                                        ></v-textarea>
                                    </v-col>
                                    <v-spacer/>
                                    <v-col cols="3" class="on-phone-chat-box-textarea">
                                        <v-btn icon color="white"><v-icon color="white">mdi-arrow-up-thick</v-icon></v-btn>
                                    </v-col>
                                    </v-row>
                                </v-card>
                            </v-card>
                        </v-col>
                    </v-row>
                </v-card>
            </v-col>
        </v-row>
    </v-container>
</template>

<script>

import flvjs from 'flv.js';
import DPlayer from 'dplayer';

export default {
    data: () => ({
        player: {},
        room: null
    }),
    methods: {
        setSource(id){
            this.player.source = this.global_.pullAddress+"/live?port="+this.global_.pullAddressRTMPPort+"&app=rtmp&stream="+id;
            this.$nextTick(() => this.pullVideo());

        },
        loadRoom(id){
            this.setSource(id);
            // this.setChatboxHeight('chatBox');
        },
        pullVideo(){
            if (this.player.source && flvjs.isSupported()) {
                var videoFrame = document.getElementById('videoFrame');
                var videoSource = this.player.source;
                var flvPlayer = null;
                this.$root.DPlayer = new DPlayer({
                    container: videoFrame,
                    video: {
                        url: 'live.flv',
                        type: 'customFlv',
                        live: true,
                        playbackSpeed: [1],
                        customType: {
                            customFlv: function (video) {
                                flvPlayer = flvjs.createPlayer({
                                    type: 'flv',
                                    url: videoSource
                                });
                                flvPlayer.attachMediaElement(video);
                                flvPlayer.load();
                            }
                        }
                    }
                });
                this.$root.DPlayer.play();
                this.$root.flvPlayer=flvPlayer;
            }
        },
        setChatboxHeight(id){
            this.$nextTick(() => {
                if(document.documentElement.clientWidth>=960){
                    document.getElementById(id).style.height=document.getElementById('videoFrame').clientHeight-24+'px';
                }else
                    document.getElementById(id).style.height=document.documentElement.clientHeight-document.getElementById('videoFrame').clientHeight-24+'px';
            })
        }
    },
    mounted() {
        this.room=this.$root.roomList[this.$route.params.id];
        if(!this.room){
            this.global_.request.asyncGetRoomList(this).then(() => {
                this.room=this.$root.roomList[this.$route.params.id];
                this.loadRoom(this.$route.params.id);
                this.$set(this.$root.bread,1,{
                    text: this.room.title,
                    disabled: true,
                    href: '#/live/'+this.$route.params.id,
                });
            })
        }else{
            this.loadRoom(this.$route.params.id);
            this.$set(this.$root.bread,1,{
                text: this.room.title,
                disabled: true,
                href: '#/live/'+this.$route.params.id,
            });
        }
    },
    beforeRouteUpdate (to, from, next) {
        if(this.$root.flvPlayer) this.$root.flvPlayer.destroy();
        if(this.$root.DPlayer) this.$root.DPlayer.destroy();
        this.id = to.params.id;
        this.room=this.$root.roomList[this.id];
        this.loadRoom(this.id);
        this.$set(this.$root.bread,1,{
            text: this.room.title,
            disabled: true,
            href: '#/live/'+this.$route.params.id,
        });
        next();
    }
}
</script>

<style>
@media screen and (max-width:960px){
    .on-phone-chat-box{
        min-height: 400px;
        margin-bottom: 48px!important;
    }
    .on-phone-card-padding-bottom{
        padding-bottom: 16px!important;
    }
    .on-phone-chat-box-flex-reverse{
        flex-direction:column-reverse;
    }
    .on-phone-chat-box-textarea{
        padding-top: 6px;
    }
}
@media screen and (min-width:960px){
    .on-desktop-card-padding-bottom{
        padding-bottom: 50px!important;
    }
    .on-desktop-chat-box-flex{
        flex-direction:column;
    }
}
</style>
