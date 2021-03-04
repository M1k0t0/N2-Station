<template>
    <v-card>
        <v-subheader>{{ $route.params.tag }}</v-subheader>

        <v-list two-line>
        <template v-for="room in $root.roomList">
            <v-list-item
            :key="room.id"
            link
            v-if="room.tag.indexOf($route.params.tag)!=-1"
            @click="$root.sfmode?routeTo('/live/'+room['stream_id']):routeTo('/live/'+room['id'])"
            >

            <v-list-item-avatar color="grey darken-1">
                <v-btn 
                icon 
                style="margin-left:2px;">
                    <img
                        src=""
                        style="width:40px!important; height:40px!important; margin-top:0px;"
                        class="ml-6"
                    >
                    
                    <!-- <v-icon v-if="item.status=='open'" class="mx-auto my-auto">mdi-broadcast</!-->
                    <!-- <v-icon v-if="item.status=='close'" class="mx-auto my-auto">mdi-broadcast-off</v-icon> --> -->
                </v-btn>
            </v-list-item-avatar>
            
            <v-list-item-content>
                <v-list-item-title>{{ room.title }}</v-list-item-title>

                <v-list-item-subtitle>
                {{ room.desc }}
                </v-list-item-subtitle>
            </v-list-item-content>
            </v-list-item>
            
            <v-divider
            :key="`divider-${room.id}`"
            inset
            v-if="room.tag.indexOf($route.params.tag)!=-1"
            ></v-divider>
        </template>
        </v-list>
    </v-card>
</template>
<script>
export default {
    methods: {
        routeTo(path, params={}){
            this.$router.push({
                path,
                params
            })
        }
    },
    mounted(){
        this.$set(this.$root.bread,1,{
            text: '#'+this.$route.params.tag+' rooms',
            disabled: true,
            href: '#/tag/'+this.$route.params.tag,
        });
    },
    beforeRouteUpdate (to, from, next) {
        this.tag = to.params.tag;
        this.$set(this.$root.bread,1,{
            text: '#'+this.tag+' rooms',
            disabled: true,
            href: '#/tag/'+this.tag,
        });
        next();
    }
}
</script>

