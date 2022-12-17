<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { DouYinModel } from "../model/dy_model";

const input = ref("");

const name = ref("");
const url = ref("");

async function getVideoInfo() {
  let info = await invoke("get_dy_info", { input: input.value }) as DouYinModel;
  name.value = info.name;
  url.value = info.url;
  // console.log(JSON.stringify(info));
}
</script>

<template>
  <div class="card">
    <div class="parse-content">
      <div class="parse">
        <input class="parse-input" v-model="input" placeholder="请输入视频分享链接..." />
        <button type="button" @click="getVideoInfo()">一键解析</button>
      </div>
      <div class="parse-info">
        <p><span style="font-weight: 600;">视频名称：</span>{{ name }} </p>
        <div><span style="font-weight: 600;">视频链接：</span>{{url}}</div>
      </div>
    </div>
  </div>
</template>

<style>
.parse {
  display: flex;
  flex-direction: row;
  width: 100%;
  justify-content: center;
  align-items: center;
}

.parse-input {
  width: 300px;
  margin-right: 10px;
}

.card {
  display: flex;
  align-items: center;
  justify-content: center;
}

.parse-content {
  width: 500px;
  padding: 10px;
  display: flex;
  align-items: flex-start;
  flex-direction: column;
}

.parse-info {
  display: flex;
  flex: 1;
  text-align: left;
  justify-content: left;
  flex-direction: column;
  width: 500px;
  margin-top: 20px;
  word-break: break-all;
}
</style>