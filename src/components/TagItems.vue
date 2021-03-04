<template>
    <v-list
        style="margin-top: 0px;margin-right:5px;"
        shaped
        color="grey darken-4"
    >
        <v-list-item
        v-for="tag in $root.tagList.open"
        :key="tag"
        link
        @click="routeTo('/tag/'+tag)"
        >
        <v-list-item-content>
            <v-list-item-subtitle># {{ tag }}</v-list-item-subtitle>
        </v-list-item-content>
        </v-list-item>

        <v-divider class="mx-3 my-5" v-if="$root.tagList.open.length"></v-divider>

        <v-list-item
        v-for="tag in $root.tagList.close"
        :key="tag"
        link
        @click="routeTo('/tag/'+tag)"
        >
        <v-list-item-content>
            <v-list-item-subtitle># {{ tag }}</v-list-item-subtitle>
        </v-list-item-content>
        </v-list-item>
    </v-list>
</template>

<script>

export default {
    data: () => ({
        tagList: {},
        timerID: null
    }),
    methods: {
        routeTo(path, params={}){
            this.$router.push({
                path,
                params
            })
        }
    },
    mounted() {
        this.global_.request.getTagList(this);
        this.timerID=setInterval(() => {
            this.global_.request.getTagList(this);
        }, 600000);
    },
    beforeDestroy(){
        clearInterval(this.timerID);
    },
    destroy() {
        clearInterval(this.timerID);
    }
}
</script>

