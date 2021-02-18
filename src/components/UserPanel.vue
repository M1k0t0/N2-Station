<template>
    <router-view></router-view>
</template>

<script>
export default {
    mounted() {
        this.$set(this.$root.bread,1,{
            text: 'UserPanel',
            disabled: false,
            href: '#/panel/rooms',
        });
        this.global_.request.getRoomList(this);
    },
    beforeRouteUpdate (to, from, next) {
        this.$set(this.$root.bread,1,{
            text: 'UserPanel',
            disabled: false,
            href: '#/panel/rooms',
        });
        this.global_.request.getRoomList(this);
        next();
    }
}
</script>
