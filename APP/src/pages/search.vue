<script setup lang="ts">
import { Search } from "@/api/collection";
import CollectionCell from "@/components/CollectionCell.vue";
import { ref, watch } from "vue";

const DataList = ref([]);
const pagingRef = ref<ZPagingRef>();
const q = ref("");

let throttled = "";
let timeoutId: NodeJS.Timeout | null = null;
let lastUpdateTime = 0;
const SetupdateTime = 1000; // ms

watch(q, (newValue) => {
  const now = Date.now();
  if (now - lastUpdateTime >= SetupdateTime && newValue) {
    update(newValue, now);

    if (timeoutId) {
      clearTimeout(timeoutId);
    }

    timeoutId = setTimeout(() => {
      // 重新获取最新值
      update(q.value, Date.now());
    }, SetupdateTime);
  }
});

function update(newValue: string, now: number) {
  if (throttled != newValue) {
    throttled = newValue;
    lastUpdateTime = now;
    if (pagingRef.value) {
      pagingRef.value.reload();
    }
  }
}

function queryList(pageNo: number, pageSize: number) {
  Search({ q: throttled, page: pageNo, per_page: pageSize }).then((res) => {
    if (pagingRef.value) {
      pagingRef.value.complete(res);
    }
  });
}
</script>

<template>
  <view class="_page">
    <view class="sea">
      <up-search v-model="q" placeholder="请输入关键字" :show-action="false" />
    </view>
    <z-paging
      ref="pagingRef"
      v-model="DataList"
      :refresher-enabled="false"
      :fixed="false"
      @query="queryList"
    >
      <CollectionCell
        v-for="(item, i) in DataList"
        :key="i"
        :Collection="item"
      />
    </z-paging>
  </view>
</template>

<style lang="scss">
._page {
  width: 100%;
  display: flex;
  flex-direction: column;
}
.sea {
  background-color: #000;
  background-color: #fff;
  padding-left: 10px;
  padding-bottom: 5px;
  padding-top: 3px;
  padding-right: 8px;
}
</style>
