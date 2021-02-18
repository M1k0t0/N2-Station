<template>
  <v-container fill-height class="align-center justify-center">
    <v-snackbar
    absolute
    top
    :timeout=1145141919
    v-model="error_snackbar"
    >
      {{ error_msg }}
      <v-btn
        color="pink"
        text
        @click="error_snackbar = false"
      >
        Close
      </v-btn>
    </v-snackbar>
    <v-snackbar
    absolute
    top
    :timeout=1145141919
    v-model="info_snackbar"
    >
      操作成功
      <v-btn
        color="pink"
        text
        @click="info_snackbar = false"
      >
        Close
      </v-btn>
    </v-snackbar>
    <v-row justify="center" align="center">
      <v-col xs="6" md="6" sm="10" class="col-me">
      <v-card class="pt-6 pl-6 pr-6 pb-0">
      
      <v-form
        ref="form"
        v-model="valid"
        lazy-validation
      >
        <v-text-field
          v-model="name"
          :counter="16"
          :rules="nameRules"
          label="Username"
          required
          color="white"
          @click="block_button_for_attention=false;"
          v-if="action=='register'"
        ></v-text-field>

        <v-text-field
          v-model="email"
          :rules="emailRules"
          label="E-mail (for gravatar)"
          required
          color="white"
          @click="block_button_for_attention=false;"
          v-if="action=='register'"
        ></v-text-field>

        <v-text-field
          v-model="password"
          :rules="passwordRules"
          label="Password"
          type="password"
          required
          color="white"
          @click="block_button_for_attention=false;"
          v-if="action=='register'"
        ></v-text-field>

        <!-- <v-checkbox
          v-model="checkbox"
          :rules="[v => !!v || 'You must agree to continue!']"
          label="No password reset system! Do you agree?"
          required
          color="white"
          v-if="action=='register'"
        ></v-checkbox> -->

        <v-row class="pb-6" v-if="action=='register'">
          <v-col>
            <v-btn
              color="white"
              @click="action='login';"
              icon
            >
              <v-icon>mdi-account</v-icon>
            </v-btn>
          </v-col>
          <v-spacer></v-spacer>
          <v-col class="text-end">
            <v-btn
              :disabled="!valid || !email || !name || !password || loading || block_button_for_attention"
              color="success"
              @click="register"
            >
              register
            </v-btn>
          </v-col>
        </v-row>

        <v-text-field
          v-model="credentials"
          label="Login credentials"
          required
          color="white"
          v-if="action=='login'"
          @click="block_button_for_attention=false;"
          :error="block_button_for_attention && !info_snackbar"
        ></v-text-field>

        <v-text-field
          v-model="password"
          label="Password"
          type="password"
          required
          color="white"
          v-if="action=='login'"
          @click="block_button_for_attention=false;"
          :error="block_button_for_attention && !info_snackbar"
        ></v-text-field>
        <v-row class="pb-6" v-if="action=='login'">
          <v-col>
            <v-btn
              color="white"
              @click="action='register'; resetValidation()"
              icon
            >
              <v-icon>mdi-account-plus</v-icon>
            </v-btn>
          </v-col>
          <v-spacer></v-spacer>
          <v-col class="text-end">
            <v-btn
              :loading="loading"
              :disabled="!password || !credentials || loading || block_button_for_attention"
              color="success"
              @click="getToken()"
            >
              login
            </v-btn>
          </v-col>
        </v-row>
      </v-form>
      </v-card>
      </v-col>
    </v-row>
  </v-container>
</template>

<script>
import global_ from './Global';
import axios from 'axios';
axios.defaults.withCredentials = true;

export default {
  data: () => ({
    valid: false,
    name: '',
    nameRules: [
      v => !!v || 'Name is required',
      v => (v && v.length <= 16 && v.indexOf('@')==-1) || 'Name must be less than 16 characters without "@"',
    ],
    email: '',
    emailRules: [
      v => !!v || 'E-mail is required',
      v => /.+@.+\..+/.test(v) || 'E-mail must be valid',
    ],
    password: '',
    passwordRules:[
      v => !!v || 'Password is required',
      v => /^.*(?=.{8,16})(?=.*\d)(?=.*[A-Za-z]{2,}).*$/.test(v)  || 'Password must be /^.*(?=.{8,16})(?=.*\\d)(?=.*[A-Za-z]{2,}).*$/'
    ],
    checkbox: false,
    action: 'login',
    credentials: null,
    error_msg: '',
    ok: false,
    loading: false,
    error_snackbar: false,
    info_snackbar: false,
    block_button_for_attention: false
  }),
  methods: {
    validate () {
      this.$refs.form.validate()
    },
    reset () {
      this.$refs.form.reset()
    },
    resetValidation () {
      this.$refs.form.resetValidation()
      this.valid=false;
    },
    register(){
      this.loading=true;
      var data={"email":this.email,"name":this.name,"pass":this.password};
      axios
      .post(this.$root.backend+'/api/auth/register',data)
      .then(response => {
        if(response.data.status!=0){
          this.error_msg='注册失败，'+global_.get_err_msg(response.data.action,response.data.status);
          this.error_snackbar=true;
        }else{
          this.ok=true;
          this.info_snackbar=true;
        }
        this.block_button_for_attention=true;
      })
      .catch(error => {
          console.log(error);
          this.errored = true;
      })
      .finally(() => {
        this.loading = false;
        setTimeout(() => this.routeTo('/panel/rooms'), 1000)
      });
    },
    getToken(){
      this.loading=true;
      var data;
      if(this.credentials.indexOf('@')!=-1)
        data={"email":this.credentials,"pass":this.password};
      else
        data={"name":this.credentials,"pass":this.password};
      axios
      .post(this.$root.backend+'/api/auth/getToken',data)
      .then(response => {
        if(response.data.status!=0){
          this.error_msg='登录失败，'+global_.get_err_msg(response.data.action,response.data.status);
          this.error_snackbar=true;
        }else{
          this.ok=true;
          this.info_snackbar=true;
        }
        this.block_button_for_attention=true;
      })
      .catch(error => {
          console.log(error);
          this.errored = true;
      })
      .finally(() => {
        this.loading = false;
        setTimeout(() => this.routeTo('/panel/rooms'), 1000)
      });
    },
    routeTo(base, data=''){
      this.$router.push({
          path: base+data,
      })
    }
  },
  mounted(){
    if(this.global_.getCookie('Authorization')){
      this.error_msg='已登录，正在跳转...';
      this.error_snackbar=true;
      setTimeout(() => this.routeTo('/panel/rooms'), 1000)
    }
    this.$set(this.$root.bread,1,{
        text: 'Login',
        disabled: true,
        href: '#/login',
    });
  },
}
</script>