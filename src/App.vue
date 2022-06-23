<template>
  <n-message-provider>
    <login-main></login-main>
  </n-message-provider>

</template>

<script>
import LoginMain from "./components/LoginMain.vue";
import {NMessageProvider} from 'naive-ui'
export default {
  name: 'App',
  components: {
    LoginMain,
    NMessageProvider
  }
}
</script>

<style>
html,body,#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin: auto;
  height: 100%;
  overflow: hidden;
  user-select: none;
}
</style>
