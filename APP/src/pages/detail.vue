<script lang="ts">
export default {
  options: { styleIsolation: "shared" },
};
</script>

<script setup lang="ts">
import {
  CollectionDetail,
  CollectionPhotos,
  handleCollectionLike,
} from "@/api/collection";

import { useGlobalDataStore } from "@/stores/GlobalData";
import { onLoad, onShow } from "@dcloudio/uni-app";
import { storeToRefs } from "pinia";
import { reactive, type Reactive, ref } from "vue";
import { FeedBack } from "@/api/feedback";
const popupRef = ref();
const toastRef = ref();

const GlobalDataStore = useGlobalDataStore();
const { day, month, year } = storeToRefs(GlobalDataStore);
const id = ref(0);
const title = ref("");
const subtitle = ref("");
const description = ref("");
const text = ref("");
const urls2: Reactive<string[]> = reactive([]);

function preview(index: number) {
  uni.previewImage({
    current: 0,
    urls: urls2,
  });
}
const Liked = ref(false);
const Like = ref(0);
function handleLike() {
  handleCollectionLike(id.value, Liked, Like)
    .then()
    .catch(() => {
      if (!toastRef.value) {
        return;
      }
      toastRef.value.show({
        type: "default",
        message: "今天已经点过赞了哦",
        duration: 820,
      });
    });
}
// 反馈按钮
const model = reactive({
  content: "",
  contact: "",
});

function Commit() {
  if (!toastRef.value || !popupRef.value) {
    return;
  }
  if (model.content == "") {
    toastRef.value.show({
      type: "default",
      message: "请留下您的宝贵意见",
      duration: 720,
    });
    return;
  }
  toastRef.value.show({
    type: "default",
    message: "感谢您的反馈",
    duration: 820,
  });
  popupRef.value.close();

  FeedBack({
    content: model.content,
    contact: model.contact,
  });

  model.contact = model.content = "";
}

onLoad(async (option) => {
  if (!option) {
    uni.navigateBack();
    return;
  }
  const _ = await CollectionDetail(option.id).then((res) => {
    id.value = option.id;
    urls2.push(`${import.meta.env.VITE_OSS}full/${res.main_image}`);

    title.value = res.title;
    subtitle.value = res.subtitle;
    description.value = res.description;
    text.value = res.supplement;
    Like.value = res.likes_count;
  });
  CollectionPhotos(option.id).then((res) => {
    res.forEach((item) => {
      urls2.push(`${import.meta.env.VITE_OSS}/full/${item}`);
    });
  });
  const res = uni.getStorageSync(`Like${option.id}`);
  if (res && res == `${year.value}${month.value}${day.value}`) {
    Liked.value = true;
  }
});
onShow(() => {});
</script>

<template>
  <view class="h-full">
    <view class="container">
      <view style="width: 100%">
        <up-swiper
          :list="urls2"
          radius="0"
          height="300"
          indicator
          :autoplay="false"
          @click="preview"
        />
      </view>
      <view class="info">
        <view class="title">
          {{ title }}
        </view>
        <view class="subtitle">
          {{ subtitle }}
        </view>
        <view class="base" style="white-space: pre-wrap">
          {{ description ? description.replace(/\\n/g, "\n") : "" }}
        </view>
        <view class="desc" style="white-space: pre-wrap">
          {{ text ? text.replace(/\\n/g, "\n") : "" }}
        </view>
      </view>
    </view>
    <view class="footer">
      <view class="footer_container">
        <view class="">
          <button
            class="museum-detail__button museum-detail__button--feedback"
            @click="popupRef ? popupRef.open() : null"
          >
            问题反馈
          </button>
        </view>
        <view class="thump-up" @tap.stop @click="handleLike">
          <up-icon
            size="25"
            :name="Liked ? 'thumb-up-fill' : 'thumb-up'"
            :label="Like"
          />
        </view>
      </view>
    </view>
    <up-toast ref="toastRef" />
    <uni-popup ref="popupRef" type="bottom">
      <view style="height: 205px; background-color: #fff; padding-top: 10px">
        <up-input
          v-model="model.contact"
          style="padding-left: 10px; margin-bottom: 5px"
          placeholder="您的联系方式"
          maxlength="20"
          border="none"
        />
        <up-textarea v-model="model.content" placeholder="您的宝贵意见" count />
        <view style="margin-top: 15px">
          <up-button
            type="primary"
            text="确定"
            style="width: 88px; background-color: #8c2a2c; border: #8c2a2c"
            shape="circle"
            @click="Commit"
          />
        </view>
      </view>
    </uni-popup>
  </view>
</template>

<style scoped lang="scss">
.container {
  display: flex;
  flex-direction: column;
  align-items: center;
  .info {
    width: 95%;
    margin-top: 20rpx;
    margin-bottom: 120rpx;
    .title {
      font-size: 36rpx;
      margin-top: 10rpx;
    }
    .subtitle {
      font-size: 28rpx;
      color: #999;
    }
    .base {
      font-size: 28rpx;
      color: #333;
    }
    .desc {
      font-size: 32rpx;
      margin: 20rpx 0;
      line-height: 48rpx;
    }
  }
}

.footer {
  position: fixed;
  bottom: 0;
  left: 0;
  right: 0;
  padding: 10rpx;
  background-color: #f7f7fa;
  border-top: 1px solid #ccc;
  .footer_container {
    display: flex;
    justify-content: space-around;
    align-items: center;
    .museum-detail__button {
      border: none;
      border-radius: 10rpx;
      font-size: 28rpx;
      color: #fff;
      cursor: pointer;
      background-color: #8a2c2d;
    }
  }
}
</style>
