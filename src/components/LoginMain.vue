<template>
  <div class="login-form">
    <div>
      <img src="@/assets/img.png" style="width: 80%;margin-top: 30px"/>
    </div>
    <div class="form">
      <n-form ref="formRef" :model="model" :rules="rules" label-placement="left">
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
      <n-button block type="warning">登录</n-button>
      <n-checkbox-group v-model:value="model.checkboxGroupValue" class="option">

          <n-checkbox value="Option 1" size="small" style="width: 50%">
            开机自启
          </n-checkbox>
          <n-checkbox value="Option 2" size="small">
            记住密码
          </n-checkbox>
          <n-checkbox value="Option 3" size="small">
            自动登录
          </n-checkbox>

      </n-checkbox-group>
      <div style="text-align: left;margin-top: 10px">
        <n-button text color="#ffeeee">
          <template #icon>
            <n-icon size="20">
              <desktop-cursor24-regular />
            </n-icon>
          </template>
          本机ipv4地址: {{status.data.online_ip}}
        </n-button>
        <n-button text color="#ffeeee">
          <template #icon>
            <n-icon size="20">
              <wallet />
            </n-icon>
          </template>
          账户余额: {{status.data.user_balance}}
        </n-button>
        <n-button text color="#ffeeee">
          <template #icon>
            <n-icon size="20">
              <network-check24-filled />
            </n-icon>
          </template>
          已用流量: {{(status.data.sum_bytes/1000000).toFixed(2)}}MB
        </n-button>
        <n-button text color="#ffeeee">
          <template #icon>
            <n-icon size="20">
              <timer-sharp />
            </n-icon>
          </template>
          已用时长: {{(status.data.sum_seconds/60/60).toFixed(2)}}小时
        </n-button>
      </div>



    </div>
  </div>
</template>
<script setup>
import {onMounted, reactive, ref} from "vue";
import {NForm, NFormItem, NInput, NButton, NCheckboxGroup, NCheckbox, NIcon} from 'naive-ui'
import {DesktopCursor24Regular, NetworkCheck24Filled} from "@vicons/fluent"
import {Wallet, TimerSharp} from "@vicons/ionicons5"
import {http} from "@tauri-apps/api"
import {ResponseType} from "@tauri-apps/api/http";
const model = reactive({
  username: "",
  password: ""
})
const rules = ref([])
const status = reactive({
  data: {
    ServerFlag: 4294967040,
    add_time: 1655825466,
    all_bytes: 291808928,
    bytes_in: 297415874,
    bytes_out: 33180516,
    checkout_date: 0,
    domain: "",
    error: "ok",
    keepalive_time: 1655830792,
    online_ip: "10.135.150.220",
    real_name: "",
    remain_seconds: 0,
    sum_bytes: 88711022999,
    sum_seconds: 8544644,
    sysver: "1.01.20190701",
    user_balance: 53.548458,
    user_charge: 13.151463,
    user_mac: "",
    user_name: "by2006111",
    wallet_balance: 0
  }
})
onMounted(() => {
  http.fetch("https://gw.buaa.edu.cn/cgi-bin/rad_user_info?callback=a", {responseType: ResponseType.Text}).then(res => {
    let rawString = res.data;
    let result = rawString.slice(0, -1).slice(2)
    result = JSON.parse(result)
    status.data = result
  })
})



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