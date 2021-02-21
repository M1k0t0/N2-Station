<template>
    <v-container fluid class="align-center justify-center fill-height">
            <v-row
                align="center"
                justify="center"
            >
                <v-col sm="12" md="8" id="videoFrame" class="col-me">
                    <video
                    class="no-outline"
                    width="100%"
                    height="auto"
                    id="videoElement" controls autoplay>
                        Your browser is too old which doesn't support HTML5 video.
                    </video>
                    <v-gravatar v-if="$root.roomList[$route.params.id]" :size="60" :email="$root.roomList[$route.params.id].user.email" style="border-radius: 60px;" />
                    <p v-if="$root.roomList[$route.params.id]" style="font-size: 25px; margin-left: 130px; margin-top: -83px;">{{$root.roomList[$route.params.id].user.name}}</p>
                </v-col>
                <v-spacer />
                <v-col sm="12" md="4" class="col-me fill-height" min-height="300px" style="flex-grow: 1;">
                    <v-card class="fill-height">
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

    </v-container>
</template>

<script>

import flvjs from 'flv.js';

export default {
    data: () => ({
        player: {}
    }),
    methods: {
        setSource(id){
            this.player.source = 'https://live.4g.cx/live?port=1935&app=rtmp&stream='+id;
            this.$nextTick(() => this.pullVideo());

        },
        loadRoom(id){
            this.setSource(id);
            this.setChatboxHeight('chatBox');
        },
        pullVideo(){
            if (this.player.source && flvjs.isSupported()) {
                var videoElement = document.getElementById('videoElement');
                this.$root.flvPlayer = flvjs.createPlayer({
                    type: 'flv',
                    url: this.player.source,
                    hasAudio: true,
                    hasVideo: true,
                    isLive: true
                });
                this.$root.flvPlayer.attachMediaElement(videoElement);
                this.$root.flvPlayer.load();
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
        this.loadRoom(this.$route.params.id);
        this.$set(this.$root.bread,1,{
            text: this.$root.roomList[this.$route.params.id].title,
            disabled: true,
            href: '#/live/'+this.$route.params.id,
        });
    },
    beforeRouteUpdate (to, from, next) {
        if(this.$root.flvPlayer) this.$root.flvPlayer.destroy();
        this.id = to.params.id;
        this.loadRoom(this.id);
        this.$set(this.$root.bread,1,{
            text: this.$root.roomList[this.id].title,
            disabled: true,
            href: '#/live/'+this.id,
        });
        next();
    }
}
</script>