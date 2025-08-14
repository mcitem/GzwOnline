<script setup lang="ts">
import { handleCollectionLike } from "@/api/collection";
import { useGlobalDataStore } from "@/stores/GlobalData";
import { storeToRefs } from "pinia";
import { computed, onMounted, ref, toRef } from "vue";
import { NavigateTo } from "@/utils";

const { Collection } = defineProps<{ Collection: Collection }>();

const GlobalDataStore = useGlobalDataStore();
const { windowWidth, day, month, year } = storeToRefs(GlobalDataStore);

const Like = toRef(Collection, "likes_count");

const Liked = ref(false);

function handleLike() {
  handleCollectionLike(Collection.id, Liked, Like);
}

const img = computed(() => import.meta.env.VITE_OSS + Collection.main_image);

const DescDisplay = computed(() => {
  const temp = (windowWidth.value * 200) / 750 >= 80;
  return temp;
});

onMounted(() => {
  const res = uni.getStorageSync(`Like${Collection.id}`);
  if (res && res == `${year.value}${month.value}${day.value}`) {
    Liked.value = true;
  }
});

defineExpose({
  Collection,
});
</script>

<template>
  <view
    class="col-box"
    @click="NavigateTo(`/pages/detail?id=${Collection.id}`)"
  >
    <view class="img-box">
      <up-lazy-load
        :image="img"
        class="img"
        border-radius="14"
        height="200rpx"
        img-mode="scaleToFill"
      />
    </view>
    <view class="info-box">
      <view class="title">
        {{ Collection.title }}
      </view>
      <view v-if="Collection.subtitle" class="subtitle">
        {{ Collection.subtitle }}
      </view>
      <view v-if="Collection.description && DescDisplay" class="desc">
        {{ Collection.description }}
      </view>
      <view class="foot">
        <view class="foot-text"> 浏览 {{ Collection.views_count }} </view>
        <view class="thump-up" @tap.stop @click="handleLike">
          <up-icon
            :name="Liked ? 'thumb-up-fill' : 'thumb-up'"
            :label="Like"
            color="#8a2a2d"
            label-size="10"
          />
        </view>
      </view>
    </view>
  </view>
</template>

<style lang="scss" scoped>
.col-box {
  display: flex;
  padding: 10px 5px;
  border-bottom: 1px solid #f0f0f0;
  background-color: #fff;
  box-shadow: 0 0 3rpx 0 rgba(0, 0, 0, 0.3);

  .img-box {
    margin-right: 10rpx;
    border-radius: 10rpx;

    .img {
      width: 200rpx;
      height: 200rpx;
      border-radius: 11rpx;
      display: block;
    }
  }

  .info-box {
    height: 200rpx;
    display: flex;
    flex-direction: column;
  }
  .title {
    font-size: 19px;
    display: -webkit-box;
    line-clamp: 1;
    -webkit-line-clamp: 1;
    -webkit-box-orient: vertical;
    overflow: hidden;
  }
  .subtitle {
    color: $u-info;
    margin-top: 3px;
    display: -webkit-box;
    overflow: hidden;
    line-clamp: 1;
    -webkit-line-clamp: 1;
    -webkit-box-orient: vertical;
  }
  .desc {
    margin-top: 3px;
    display: -webkit-box;
    overflow: hidden;
    line-clamp: 1;
    -webkit-line-clamp: 1;
    -webkit-box-orient: vertical;
  }
  .foot {
    display: flex;
    justify-content: flex-start;
    gap: 10px;
    color: #808080;
    font-size: 10px;
    margin-top: auto;
    .thump-up {
      display: flex;
      gap: 2px;
    }
    .foot-text {
      justify-content: center;
      align-items: center;
      display: flex;
    }
  }
}
</style>
