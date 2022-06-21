<template>
  <login-main></login-main>
</template>

<script>
import LoginMain from "./components/LoginMain.vue";

export default {
  name: 'App',
  components: {
    LoginMain,
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
}
</style>
