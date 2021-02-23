<template>
    <v-container fluid fill-height class="align-center justify-center">
        <v-row
            align="center"
            justify="center"
            class="fill-height"
        >
            <v-col sm="12" md="11" class="pb-1 fill-height">
                <v-card tile class="pt-5 pl-3 pr-3 pb-8" height="95%">
                    <p class="title ml-3 mb-0">{{ room.title }}</p>
                    <p class="caption ml-3 mb-2">{{ room.desc }}</p>
                    <v-row
                        align="center"
                        justify="center"
                    >
                        <v-col sm="12" md="8" class="col-me pl-6">
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
                            <v-gravatar v-if="room" class=" ml-sm-8 mt-1 mb-1 mt-md-5 mb-md-5" :size="60" :email="$root.roomList[$route.params.id].user.email" style="border-radius: 60px;" />
                            <p v-if="room" style="font-size: 25px; margin-left: 120px; margin-top: -75px;">{{$root.roomList[$route.params.id].user.name}}</p>
                        </v-col>
                        <v-spacer />
                        <v-col sm="12" md="4" class="col-me pb-8">
                            <v-card tile id="chatBox" class="pt-2 pb-6 ml-md-0 mr-md-0 ml-sm-12 mr-sm-12 ml-1 mr-1" height="200px">
                                <v-card outline id="chatContent" height="80%" class="ml-2 mr-2 mt-0 mb-0">
                                </v-card>
                                <v-row class="py-5 py-sm-0 mx-auto">
                                <v-col cols="9" class="pt-0 pr-0">
                                    <v-text-field color="white" label="Commit" class="ml-2 mr-2 pt-0 mt-0"></v-text-field>
                                </v-col>
                                <v-spacer/>
                                <v-col cols="3" class="pt-0 pr-0 pl-4">
                                    <v-btn icon color="white"><v-icon color="white">mdi-arrow-up-thick</v-icon></v-btn>
                                </v-col>
                                </v-row>
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
            this.setChatboxHeight('chatBox');
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

