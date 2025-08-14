<script setup lang="ts">
import { ref } from "vue";

const img = `${import.meta.env.VITE_OSS}0.jpg`;
function handle() {
  if (timer) {
    clearInterval(timer);
  }
  handleSkip();
}
function handleSkip() {
  uni.reLaunch({
    url: "/pages/inde",
  });
}

const count = ref(4);
let timer: NodeJS.Timeout;
timer = setInterval(() => {
  count.value -= 1;
  if (count.value <= 0) {
    clearInterval(timer);
    console.log("time");
    handleSkip();
  }
}, 1000);
</script>

<template>
  <view @click="handle">
    <view class="fixed top-0 left-0 h-100vh w-full">
      <button class="floating-button">
        <span>跳过{{ count > 0 ? count : "" }}</span>
      </button>
      <image class="h-100vh w-full" mode="aspectFill" :src="img" />
    </view>
  </view>
</template>

<style lang="scss" scoped>
.floating-button {
  font-size: 14px;
  display: flex;
  text-align: center;
  align-items: center;
  justify-content: center;
  position: fixed;
  top: 20px;
  right: 20px;
  background-color: rgba(220, 220, 220, 0.3);
  color: #555;
  border-radius: 15px;
  width: 75px; //70px
  height: 40px;
  cursor: pointer;
  box-shadow: 0 2px 5px rgba(0, 0, 0, 0.2);
  z-index: 100;
}

.floating-button:hover {
  background-color: rgba(220, 220, 220, 0.5);
}
</style>
