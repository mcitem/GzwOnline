<script setup lang="ts">
import infoCellVue from "@/components/infoCell.vue";
import infoList from "@/utils/infoList";

if (process.env.NODE_ENV === "development") {
  let j = "";
  for (let i = 0; i < infoList.length; i++) {
    j += infoList[i].title;
  }
  // console.log(j);
}
const poster = `${import.meta.env.VITE_OSS}-1.jpg`;
const video = `${import.meta.env.VITE_OSS}output.mp4`;
</script>

<template>
  <view>
    <video :poster="poster" :src="video" />
    <view class="container">
      <view class="item-title">
        探索广州湾，传承历史韵味——骐骥突击队邀您共赴文化之旅！
      </view>
    </view>
    <uni-section title="相关资讯" type="line" class="sec" />
    <view class="infocontain">
      <infoCellVue v-for="(item, index) in infoList" :info="item" />
    </view>
  </view>
</template>

<style lang="scss" scoped>
.sec ::v-deep .uni-section-header__decoration {
  background-color: #8a2c2d;
  &.line {
    height: 14px;
  }
}

video {
  width: 100%;
}
.container {
  display: flex;
  align-items: center;
  .item-title {
    display: flex;
    padding-left: 8px;
    padding-right: 3px;
    padding-top: 4px;
    justify-content: center;
    align-items: center;
    font-size: 22px;
  }
}

.infocontain {
  margin-top: 10px;
  display: flex;
  flex-direction: column;
}
</style>
