<template>
    <v-container fluid>
            <v-row
                    align="center"
                    justify="center"
            >
            <v-col sm="12" md="10" class="col-me">
                <v-card shaped tile>
                    <div class="pl-4 pb-2 pt-2">
                        <h1># N2 Station</h1>
                        <br />
                        <p>一个实现 N2 Frontend API 的多直播源直播站</p>
                        <p>Github项目: <a href="https://github.com/M1k0t0/N2-Station">https://github.com/M1k0t0/N2-Station</a></p>
                        <p>拉流播放器基于 <a href="https://github.com/bilibili/flv.js">flv.js</a></p>
                        <p>项目仍在编写中，觉得顶的话就给个Star吧！</p>
                    </div>
                </v-card>
            </v-col>

            <v-col sm="12" md="10" class="col-me">
                <v-card tile>
                    <h3 class="pt-5 pl-4 pr-4">### DEBUG MODE ###</h3>
                    <v-text-field
                        v-model="$root.backend"
                        label="Backend API Address"
                        prepend-icon="mdi-link"
                        append-icon="mdi-water"
                        append-outer-icon="mdi-send"
                        @click:append="resetBackendAddress()"
                        @click:append-outer="flushBackendAPI()"
                        class="pt-5 pl-4 pr-4"
                    ></v-text-field>
                    <v-switch
                    v-model="$root.sfmode"
                    :label="`API Format: ${getAPIFormat()}`"
                    class="pt-0 pl-4 pr-4"
                    @change="resetBackendAddress()"
                    ></v-switch>
                </v-card>
            </v-col>
        </v-row>
    </v-container>
</template>
<script>
import global_ from './Global';
import axios from 'axios';

export default {
    data: () => ({

    }),
    methods: {
        resetBackendAddress(){
            this.$root.backend=this.$root.sfmode ? global_.BackendAddress : global_.debugBackendAddress;
            this.flushBackendAPI();
        },
        flushBackendAPI(){
            this.getRoomList();
            if(this.$root.backend==global_.debugBackendAddress) this.$root.sfmode=false;
            else this.$root.sfmode=true;
        },
        getRoomList(){
            axios
            .get(this.$root.backend+'/api/info/room')
            .then(response => {
                this.$root.roomList = this.$root.sfmode ? response.data.rooms : response.data.data;
            })
            .catch(error => {
                console.log(error);
                this.errored = true;
            })
            .finally(() => this.loading = false);
        },
        getAPIFormat(){
            return this.$root.sfmode?"SF":"CK";
        }
    },
    mounted() {
        this.$root.bread.splice(1,1);
    }
}
</script>
