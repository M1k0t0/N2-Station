<template>
<v-container class="pa-0" align="center" height="90%">
    <v-list
    two-line
    subheader
    color="grey darken-4"
    rounded
    class="mb-6"
    >
    <v-subheader class="overline">ROOM SETTINGS</v-subheader>

    <div style="padding-left:15px!important;padding-right:30px!important;">
    <v-list-item link class="pr-0" 
    style="min-height:40px!important;" 
    :input-value="$root.panelMenuIndex==0"
    @click="routeTo('/panel','/rooms'); $root.panelMenuIndex=0"
    >
        <v-list-item-content class="pt-0 pb-0">
        <v-list-item-title class="body-2">My Rooms</v-list-item-title>
        </v-list-item-content>
    </v-list-item>
    </div>
    <div style="padding-left:15px!important;padding-right:30px!important;">
    <v-list-item link class="pr-0" 
    style="min-height:40px!important;"
    :input-value="$root.panelMenuIndex==1"
    @click="routeTo('/panel','/createRoom'); $root.panelMenuIndex=1"
    >
        <v-list-item-content class="pt-0 pb-0">
        <v-list-item-title class="body-2">Create Room</v-list-item-title>
        </v-list-item-content>
    </v-list-item>
    </div>
    <div style="padding-left:15px!important;padding-right:30px!important;">
    <v-list-item link class="pr-0" 
    style="min-height:40px!important;" 
    :input-value="$root.panelMenuIndex==2"
    @click="routeTo('/panel','/editRoom'); $root.panelMenuIndex=2"
    >
        <v-list-item-content class="pt-0 pb-0">
        <v-list-item-title class="body-2">Edit Room</v-list-item-title>
        </v-list-item-content>
    </v-list-item>
    </div>

    </v-list>

    <v-divider></v-divider>

    <v-list
    two-line
    subheader
    color="grey darken-4"
    rounded
    class="mt-8"
    >
    <v-subheader class="overline">USER SETTINGS</v-subheader>

    <div style="padding-left:15px!important;padding-right:30px!important;">
    <v-list-item link class="pr-0" 
    style="min-height:40px!important;" 
    :input-value="$root.panelMenuIndex==3"
    @click="routeTo('/panel','/changeInfo'); $root.panelMenuIndex=3"
    >
        <v-list-item-content class="pt-0 pb-0">
        <v-list-item-title class="body-2">Credentials</v-list-item-title>
        </v-list-item-content>
    </v-list-item>
    </div>
    <div style="padding-left:15px!important;padding-right:30px!important;">
    <v-list-item link class="pr-0" 
    style="min-height:40px!important;" 
    :input-value="$root.panelMenuIndex==4"
    @click="routeTo('/panel','/changePassword'); $root.panelMenuIndex=4"
    >
        <v-list-item-content class="pt-0 pb-0">
        <v-list-item-title class="body-2">Password</v-list-item-title>
        </v-list-item-content>
    </v-list-item>
    </div>
    <div style="padding-left:15px!important;padding-right:30px!important;">
    <v-list-item link class="pr-0" 
    style="min-height:40px!important;" 
    :input-value="$root.panelMenuIndex==5"
    @click="routeTo('/panel','/2FA'); $root.panelMenuIndex=5"
    >
        <v-list-item-content class="pt-0 pb-0">
        <v-list-item-title class="body-2">2FA (?)</v-list-item-title>
        </v-list-item-content>
    </v-list-item>
    </div>
    <div style="padding-left:15px!important;padding-right:30px!important;">
    <v-list-item link class="pr-0" style="min-height:40px!important;" :input-value="$root.panelMenuIndex==6">
        <v-list-item-content class="pt-0 pb-0">
        <v-list-item-title class="body-2"><font color="#f04746"><b>Logout</b></font></v-list-item-title>
        </v-list-item-content>
    </v-list-item>
    </div>

    </v-list>

</v-container>
</template>

<script>
export default {
    methods: {
        routeTo(base, data=''){
            this.$router.push({
                path: base+data,
            })
        }
    }
}
</script>