<template>
  <div class="login-form">
    <div>
      <img src="@/assets/img.png" style="width: 70%;margin-top: 15px"/>
    </div>
    <div class="form" style="text-align: left">
      <n-form ref="formRef" :model="model" :rules="rules" label-placement="left" v-show="!has_login">
        <n-form-item path="username" label="学号">
          <n-input v-model:value="model.username" @keydown.enter.prevent placeholder="请输入学号"/>
        </n-form-item>
        <n-form-item path="password" label="密码" >
          <n-input
              v-model:value="model.password"
              type="password"
              placeholder="请输入密码"
              @keydown.enter.prevent
          />
        </n-form-item>
      </n-form>
      <n-button block type="warning" @click="login" v-show="!has_login">登录</n-button>
      <n-checkbox-group v-model:value="checkboxGroupValue" v-show="!has_login" class="option">
          <n-checkbox value="auto_start" size="small" style="width: 50%">
            开机自启
          </n-checkbox>
          <n-checkbox value="remember_password" size="small">
            记住密码
          </n-checkbox>
          <n-checkbox value="auto_login" size="small">
            自动登录
          </n-checkbox>
      </n-checkbox-group>
      <div v-if="has_login">
        <n-result status="success" title="在线" :description="status.data.user_name" size="small"></n-result>
      </div>
      <div style="text-align: left;margin-top: 15px;" v-show="has_login">
        <n-alert title="客户端信息" type="warning" :show-icon="false" style="opacity: 0.8">
          <div style="display: flex">
            <n-icon size="22" style="margin-top: 3px">
              <cloud-checkmark16-filled />
            </n-icon>
            <div style="line-height: 28px;margin-left: 10px">
              在线ip: {{status.data.online_ip}}
            </div>
          </div>
          <div style="display: flex">
            <n-icon size="22" style="margin-top: 3px">
              <wallet />
            </n-icon>
            <div style="line-height: 28px;margin-left: 10px">
              账户余额: {{status.data.user_balance}}
            </div>
          </div>
          <div style="display: flex">
            <n-icon size="22" style="margin-top: 3px">
              <network-check24-filled />
            </n-icon>
            <div style="line-height: 28px;margin-left: 10px">
              已用流量: {{(status.data.sum_bytes/1000000).toFixed(2)}}MB
            </div>

          </div>
          <div style="display: flex">
            <n-icon size="22" style="margin-top: 3px">
              <timer-sharp />
            </n-icon>
            <div style="line-height: 28px;margin-left: 10px">
              已用时长: {{(status.data.sum_seconds/60/60).toFixed(2)}}小时
            </div>

          </div>


        </n-alert>
        <n-button block type="error" style="margin-top: 10px" @click="logout">注销</n-button>
      </div>
    </div>
  </div>
</template>
<script setup>
import {onMounted, reactive, ref, watch} from "vue";
import {NForm, NFormItem, NInput, NButton, NCheckboxGroup, NCheckbox, NIcon, NResult, NAlert} from 'naive-ui'
import { NetworkCheck24Filled, CloudCheckmark16Filled} from "@vicons/fluent"
import {Wallet, TimerSharp} from "@vicons/ionicons5"
import {http, invoke} from "@tauri-apps/api"
import {ResponseType} from "@tauri-apps/api/http";
import { useMessage } from 'naive-ui'
import {enable, disable, isEnabled} from 'tauri-plugin-autostart-api'
const model = reactive({
  username: "",
  password: ""
})
const rules = ref([])
const status = reactive({
  data: {
  }
})
const checkboxGroupValue = ref([])
watch(() => checkboxGroupValue.value, (n) => {
  option.auto_start = n.indexOf("auto_start") > -1
  option.auto_login = n.indexOf("auto_login") > -1
  option.remember_password = n.indexOf("remember_password") > -1
  if (option.auto_login) {
    option.remember_password = true
    if (n.indexOf("remember_password") === -1) {
      n.push("remember_password")
    }
  }
  if (option.remember_password) {
    option.password = model.password;
  }
  isEnabled().then(enabled => {
    if (option.auto_login && !enabled) {
      enable()
    }
    if (!option.auto_login && enabled) {
      disable()
    }
  })

  option.username = model.username;
  invoke("update_option", {payload: option})
})
// const notOnlineRes = {
//   "client_ip": "10.135.150.220",
//   "ecode": 0,
//   "error": "not_online_error",
//   "error_msg": "",
//   "online_ip": "10.135.150.220",
//   "res": "not_online_error",
//   "srun_ver": "SRunCGIAuthIntfSvr V1.18 B20190701",
//   "st": 1655878120
// }
const option = reactive({
  auto_start: false,
  auto_login: false,
  remember_password: false,
  password: "",
  username: ""
})
const has_login = ref(false)
onMounted(() => {
  http.fetch("https://gw.buaa.edu.cn/cgi-bin/rad_user_info?callback=a", {responseType: ResponseType.Text}).then(res => {
    let rawString = res.data;
    let result = rawString.slice(0, -1).slice(2)
    result = JSON.parse(result)
    has_login.value = result.res !== "not_online_error";
    status.data = result
    fetchOption().then(() => {
      if (option.auto_login && !has_login.value) {
        login()
      }
    })
  })

})
const fetchOption = () => {
  return invoke("fetch_option").then(c_option => {
    option.auto_start = c_option.auto_start;
    option.auto_login = c_option.auto_login;
    option.remember_password = c_option.remember_password;
    option.password = c_option.password;
    option.username = c_option.username;
    model.username = c_option.username;
    model.password = c_option.password;
    checkboxGroupValue.value = []
    if (option.auto_start) {
      checkboxGroupValue.value.push("auto_start")
    }
    if (option.auto_login) {
      checkboxGroupValue.value.push("auto_login")
    }
    if (option.remember_password) {
      checkboxGroupValue.value.push("remember_password")
    }
  })
}
const message = useMessage();
const login = () => {
  if (!model.username || !model.password) {
    message.warning("请填写正确的学号和姓名")
  }
  invoke("login", {username: model.username, password: model.password, clientIp: status.data.client_ip ? status.data.client_ip : status.data.online_ip}).then(res => {
    if (res.error === "ok") {
      has_login.value = true
      if (res.ploy_msg) {
        message.info(res.ploy_msg, { duration: 5000 })
      }
      http.fetch("https://gw.buaa.edu.cn/cgi-bin/rad_user_info?callback=a", {responseType: ResponseType.Text}).then(res => {
        console.log(res)
        let rawString = res.data;
        let result = rawString.slice(0, -1).slice(2)
        result = JSON.parse(result)
        has_login.value = result.res !== "not_online_error";
        status.data = result
      })
      let new_option = JSON.parse(JSON.stringify(option))
      new_option.username = model.username
      if (option.remember_password) {
        new_option.password = model.password
      } else {
        new_option.password = ""
      }
      invoke("update_option", {payload: new_option}).then(() => {
        fetchOption()
      })
    } else {
      message.error(
          res.error + ": " + res.error_msg,
          { duration: 5000 }
      )
    }

  })
}
const logout = () => {
  invoke("logout", {username: status.data.user_name, ip: status.data.online_ip}).then(() => {
    has_login.value = false
    message.success(
        "已注销",
        { duration: 5000 }
    )
  })
}

</script>

<style scoped>
.login-form {
  height: 100%;
  background: #C6FFDD;  /* fallback for old browsers */
  background: -webkit-linear-gradient(to top, #f7797d, #FBD786, #C6FFDD);  /* Chrome 10-25, Safari 5.1-6 */
  background: linear-gradient(to top, #f7797d, #FBD786, #C6FFDD); /* W3C, IE 10+/ Edge, Firefox 16+, Chrome 26+, Opera 12+, Safari 7+ */
}
.form {
  padding-left: 20px;
  width: 85%;
  padding-top: 20px;
}
.option {
  text-align: justify;
  margin-top: 15px;
  font-size: 12px;
}
</style>