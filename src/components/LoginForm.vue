<template>
  <v-container fill-height>
    <v-row align="center" justify="center">
      <v-col xs="4" md="6" sm="10" class="col-me">
      <v-card>
      <v-form
        ref="form"
        v-model="valid"
        lazy-validation
        class="pt-6 pl-6 pr-6 pb-0"
      >
        <v-text-field
          v-model="name"
          :counter="10"
          :rules="nameRules"
          label="Username"
          required
          color="white"
          v-if="action=='register'"
        ></v-text-field>

        <v-text-field
          v-model="email"
          :rules="emailRules"
          label="E-mail (for gravatar)"
          required
          color="white"
          v-if="action=='register'"
        ></v-text-field>

        <v-text-field
          v-model="password"
          :rules="passwordRules"
          label="Password"
          type="password"
          required
          color="white"
          v-if="action=='register'"
        ></v-text-field>

        <v-checkbox
          v-model="checkbox"
          :rules="[v => !!v || 'You must agree to continue!']"
          label="No password reset system! Do you agree?"
          required
          color="white"
          v-if="action=='register'"
        ></v-checkbox>

        <v-row>
          <v-col>
            <v-btn
              color="white"
              @click="action='login';"
              v-if="action=='register'"
              icon
            >
              <v-icon>mdi-account</v-icon>
            </v-btn>
          </v-col>
          <v-spacer></v-spacer>
          <v-col class="text-end">
            <v-btn
              :disabled="!valid || !email || !name || !password"
              color="success"
              @click="register"
              v-if="action=='register'"
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
        ></v-text-field>

        <v-text-field
          v-model="password"
          label="Password"
          type="password"
          required
          color="white"
          v-if="action=='login'"
        ></v-text-field>
        <v-row class="pb-6">
          <v-col>
            <v-btn
              color="white"
              @click="action='register'; resetValidation()"
              v-if="action=='login'"
              icon
            >
              <v-icon>mdi-account-plus</v-icon>
            </v-btn>
          </v-col>
          <v-spacer></v-spacer>
          <v-col class="text-end">
            <v-btn
              :disabled="!password || !credentials"
              color="success"
              @click="login"
              v-if="action=='login'"
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
  export default {
    data: () => ({
      valid: false,
      name: '',
      nameRules: [
        v => !!v || 'Name is required',
        v => (v && v.length <= 10) || 'Name must be less than 10 characters',
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
      credential: null
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
    },
    mounted(){
        this.$set(this.$root.bread,1,{
            text: 'Login',
            disabled: true,
            href: '#/login',
        });
    },
  }
</script>