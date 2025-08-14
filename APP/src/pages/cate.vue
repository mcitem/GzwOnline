<script setup lang="ts">
import type { ScrollViewOnScrollEvent } from "@uni-helper/uni-app-types";
import { Collections } from "@/api/category";
import CollectionCellVue from "@/components/CollectionCell.vue";
import { onLoad } from "@dcloudio/uni-app";
import { getCurrentInstance, ref } from "vue";

const instance = getCurrentInstance();
const current = ref(0);
const scrollTop = ref(0);
const scrollRightTop = ref(0);
const oldScrollTop = ref(0);
const tabbar = ref<
  {
    id: number;
    name: string;
    foods: Collection[];
  }[]
>([]);
onLoad(() => {
  Collections().then((res) => {
    tabbar.value = res;
  });
});
function switchMenu(index: number) {
  if (index == current.value) return;
  const query = uni.createSelectorQuery().in(instance?.proxy);
  query
    .select(`#item${index}`)
    .boundingClientRect((data) => {
      let { top } = data as UniApp.NodeInfo;
      top = top || 0;
      scrollRightTop.value = oldScrollTop.value + top - 20;
    })
    .exec();
  current.value = index;
}

function rightScroll(e: ScrollViewOnScrollEvent) {
  oldScrollTop.value = e.detail.scrollTop;
}
</script>

<template>
  <view class="h-[calc(100vh-50px)] flex flex-col">
    <view class="flex flex-1 overflow-hidden">
      <up-toast ref="uToastRef" />
      <scroll-view
        :scroll-top="scrollTop"
        scroll-y
        scroll-with-animation
        class="h-100vh w-200rpx"
      >
        <view
          v-for="(item, index) in tabbar"
          :key="index"
          class="u-tab-item"
          :class="[current == index ? 'u-tab-item-active' : '']"
          @click="switchMenu(index)"
        >
          <text style="font-size: 13px">
            {{ item.name }}
          </text>
        </view>
      </scroll-view>

      <scroll-view
        :scroll-top="scrollRightTop"
        scroll-y
        scroll-with-animation
        class="bg-[#ededed]"
        @scroll="rightScroll"
      >
        <view class="p-16rpx">
          <view v-for="(item, index) in tabbar" :key="index">
            <view
              v-if="item.foods.length"
              class="mb-30rpx bg-#fff p-16rpx rounded-8rpx"
            >
              <view
                :id="`item${index}`"
                class="text-[26rpx] color-[$u-main-color] font-bold"
                style="margin-bottom: 10px"
              >
                <text>{{ item.name }}</text>
              </view>
              <view class="last:min-h-[100%]">
                <CollectionCellVue
                  v-for="(item1, index1) in item.foods"
                  :key="index1"
                  :Collection="item1"
                />
              </view>
            </view>
          </view>
        </view>
      </scroll-view>
    </view>
  </view>
</template>

<style lang="scss" scoped>
.u-tab-item {
  height: 110rpx;
  background: #f6f6f6;
  box-sizing: border-box;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 26rpx;
  color: #444;
  font-weight: 400;
  line-height: 1;
}

.u-tab-item-active {
  position: relative;
  color: #000;
  font-size: 30rpx;
  font-weight: 600;
  background: #fff;
}

.u-tab-item-active::before {
  content: "";
  position: absolute;
  border-left: 4px solid #8c2d2c;
  height: 32rpx;
  left: 94%;
  top: 39rpx;
}
</style>
