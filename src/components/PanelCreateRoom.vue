<template>
    <v-stepper
    v-model="e6"
    vertical
    class="ma-12"
    style="background-color:#36393f;"
    non-linear
    >
    <v-stepper-step
        :complete="e6 > 1"
        step="1"
        color="#7289da"
        editable
        class="py-5 mb-1"
    >
        房间ID
        <small>也是推流ID，不可更改，请不要用奇怪的字符</small>
    </v-stepper-step>

    <v-stepper-content step="1">
        <v-text-field
        autofocus
        v-model="id"
        class="mr-8"
        solo
        label="e.g. ckLive"
        clearable
        prepend-inner-icon="mdi-link"
        ></v-text-field>
        <v-btn
        color="#7289da"
        @click="e6 = 2"
        >
        下一步
        </v-btn>
    </v-stepper-content>

    <v-stepper-step
        :complete="e6 > 2"
        step="2"
        color="#7289da"
        editable
        class="py-5 my-1"
    >
        标题、描述
    </v-stepper-step>

    <v-stepper-content step="2">
        <v-card
        color="grey lighten-1"
        class="mb-12"
        height="200px"
        ></v-card>
        <v-btn
        color="#7289da"
        @click="e6 = 3"
        >
        下一步
        </v-btn>
        <v-btn text @click="e6--;">
        返回
        </v-btn>
    </v-stepper-content>

    <v-stepper-step
        :complete="e6 > 3"
        step="3"
        color="#7289da"
        editable
        class="py-5 my-1"
    >
        直播标签
    </v-stepper-step>

    <v-stepper-content step="3">
        <v-card
        color="grey lighten-1"
        class="mb-12"
        height="200px"
        ></v-card>
        <v-btn
        color="#7289da"
        @click="e6 = 4"
        >
        下一步
        </v-btn>
        <v-btn text @click="e6--;">
        返回
        </v-btn>
    </v-stepper-content>

    <v-stepper-step 
    step="4"
    color="#7289da"
    editable
    class="py-5 my-1"
    >
        缩略图
        <small>可能会重新设计，暂定64x64</small>
    </v-stepper-step>
    <v-stepper-content step="4">
        <v-card
        color="grey lighten-1"
        class="mb-12"
        height="200px"
        ></v-card>
        <v-btn
        color="#7289da"
        @click="e6 = 1"
        >
        完成
        </v-btn>
        <v-btn text @click="e6--;">
        返回
        </v-btn>
    </v-stepper-content>
    </v-stepper>
</template>
<script>
export default {
    data: () => ({
        e6: 1,
    }),
}
</script>
