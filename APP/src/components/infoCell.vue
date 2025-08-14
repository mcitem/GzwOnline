<script setup lang="ts">
interface InfoTypes {
  url: string;
  title: string;
  ori: string;
  time: string;
  img?: string;
}
const props = defineProps<{ info: InfoTypes }>();

function to(url: string) {
  if (window.top && window.self !== window.top) {
    window.top.location.href = url;
  } else {
    window.location.href = url;
  }
}

function handle() {
  to(props.info.url);
}
</script>

<template>
  <view class="cell" @click="handle">
    <view class="text">
      <view class="title">
        {{ props.info.title }}
      </view>
      <view class="desc">
        {{ props.info.ori + (props.info.time ? `  ${props.info.time}` : "") }}
      </view>
    </view>

    <view v-if="props.info.img" class="imgcontain">
      <image
        class="img"
        :src="props.info.img"
        mode="aspectFill"
        :lazy-load="true"
      />
    </view>
  </view>
</template>

<style lang="scss" scoped>
@import "@/common/songti.css";
.cell {
  padding-top: 8px;
  display: flex;
  justify-content: space-between;
  border-bottom: 1px solid #ededed;
  .text {
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    .title {
      font-family: "songti";
      font-size: 20px;
      line-height: 28px;
      padding-left: 10px;
      display: -webkit-box;
      overflow: hidden;
      line-clamp: 2;
      -webkit-line-clamp: 2;
      -webkit-box-orient: vertical;
    }
    .desc {
      margin-top: 10px;
      padding-left: 10px;
      padding-bottom: 10px;
      color: #b5b4b8;
      font-size: 15px;
      display: flex;
      justify-content: start;
      white-space: pre-wrap;
    }
  }
  .imgcontain {
    margin-left: 5px;
    margin-right: 10px;
    .img {
      border-radius: 4px;
      width: 100px;
      height: 80px;
    }
  }
}
</style>
