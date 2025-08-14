<script setup lang="ts">
import { CollectionList } from "@/api/collection";
import CollectionCell from "@/components/CollectionCell.vue";
import { useGlobalDataStore } from "@/stores";
import { onLoad } from "@dcloudio/uni-app";
import { storeToRefs } from "pinia";
import { ref } from "vue";

const NavigateTo = getApp().NavigateTo;
const GlobalDatastore = useGlobalDataStore();
const { windowWidth, NavBar } = storeToRefs(GlobalDatastore);

const SwiperList = ref<string[]>([
  `${import.meta.env.VITE_OSS}1.jpg`,
  `${import.meta.env.VITE_OSS}2.jpg`,
]);
const NotionText = ref(
  "我们是岭南师范学院“骐骥”突击队，我们正在筹备一项关于广州湾线上历史博物馆项目。以数字技术重现历史瑰宝，让文化触手可及。为了更好地满足您的需求，我们特别设计了这份问卷，希望了解您对于这一创新尝试的看法和期待。-->>"
);

const FloorData = ref<
  {
    title: string;
    data: Collection[];
  }[]
>([
  {
    title: "热门藏品",
    data: [],
  },
  {
    title: "明星藏品",
    data: [],
  },
]);

const showActionSheet = ref(false);
const showActionSheetList = ref([
  { name: "高德地图" },
  { name: "腾讯地图" },
  { name: "百度地图" },
]);
function handleActionSheetSelect({ name }: { name: string }) {
  function to(url: string) {
    if (window.top && window.self !== window.top) {
      window.top.location.href = url;
    } else {
      window.location.href = url;
    }
  }
  switch (name) {
    case "高德地图":
      {
        to(
          "https://uri.amap.com/marker?position=110.363054,21.276229&name=%e5%b9%bf%e5%b7%9e%e6%b9%be%e5%8e%86%e5%8f%b2%e6%b0%91%e4%bf%97%e9%a6%86&src=mc.mcitem.com&coordinate=gaode&callnative=1"
        );
      }
      break;
    case "百度地图":
      {
        to(
          "http://api.map.baidu.com/geocoder?address=%E5%B9%BF%E5%B7%9E%E6%B9%BE%E5%8E%86%E5%8F%B2%E6%B0%91%E4%BF%97%E9%A6%86&output=html&src=webapp.mcitem.mc"
        );
      }
      break;
    case "腾讯地图": {
      to(
        "https://apis.map.qq.com/uri/v1/marker?marker=coord:21.276086,110.363003;title:%E5%B9%BF%E5%B7%9E%E6%B9%BE%E5%8E%86%E5%8F%B2%E6%B0%91%E4%BF%97%E9%A6%86;addr:%E5%B9%BF%E4%B8%9C%E7%9C%81%E6%B9%9B%E6%B1%9F%E5%B8%82%E8%B5%A4%E5%9D%8E%E5%8C%BA%E5%92%8C%E5%B9%B3%E8%B7%AF51%E5%8F%B7&referer=mc.mcitem.com"
      );
    }
    default: {
    }
  }
}
onLoad(() => {
  CollectionList({
    order_by: "ViewsCount",
    limit: 5,
  }).then((res) => {
    FloorData.value[0].data.push(...res);
  });
  CollectionList({
    order_by: "LikesCount",
    limit: 5,
  }).then((res) => {
    FloorData.value[1].data.push(...res);
  });
});
</script>

<template>
  <view class="page">
    <!-- nav -->
    <up-action-sheet
      round="10"
      cancel-text="取消"
      :actions="showActionSheetList"
      :show="showActionSheet"
      @close="showActionSheet = false"
      @select="handleActionSheetSelect as () => void"
    />
    <up-navbar
      class="nav"
      title="首页"
      title-style="color:#fff"
      :placeholder="true"
      bg-color="#8a2c2d"
      :height="NavBar"
      @left-click="NavigateTo('/pages/search')"
    >
      <template #left>
        <uni-icons type="search" size="27" color="#fff" />
      </template>
      <template #right>
        <div>
          <uni-icons
            type="map-filled"
            size="27"
            color="#fff"
            @click="NavigateTo('/pages/game')"
          />

          <uni-icons
            type="paperplane"
            size="27"
            color="#fff"
            @click="showActionSheet = true"
          />
        </div>
      </template>
    </up-navbar>
    <!-- end nav -->

    <!-- Notion -->

    <view v-if="NotionText" class="">
      <view class="mx-auto my-0">
        <up-notice-bar
          duration="5000"
          speed="80"
          :text="NotionText"
          icon=""
          mode="closable"
          color="#ffffff"
          bg-color="#8a2c2d"
          url="/pages/webview"
        />
      </view>
    </view>
    <!-- end Notion -->

    <!-- swiper -->
    <view
      v-if="SwiperList.length > 0 && windowWidth < 450"
      style="margin: 5px 0"
    >
      <up-swiper
        height="230"
        img-mode="aspectFill"
        :list="SwiperList"
        :indicator="true"
        previous-margin="5"
        next-margin="5"
        interval="6000"
        duration="850"
        circular
        radius="5"
        bg-color="#ffffff"
      />
    </view>
    <!-- end swiper -->

    <!-- 楼层区域 -->
    <view style="padding-bottom: 52px">
      <view v-for="(item, index) in FloorData" :key="index">
        <!-- 层级标题 -->

        <uni-section class="sec" :title="item.title" type="line" />

        <!-- 藏品列表 -->
        <view v-if="item.data && item.data.length > 0">
          <CollectionCell
            v-for="(jitem, jindex) in item.data"
            :key="jindex"
            :Collection="jitem"
          />
        </view>
      </view>
      <view
        style="
          text-align: center;
          margin-top: 3px;
          color: #777;
          font-size: 13px;
          display: flex;
          flex-direction: column;
          justify-content: center;
          gap: 3px;
        "
      >
        <view>
          © 2023-2025 版权所有
          <a class="link" href="mailto:19120748606@163.com" target="_blank"
            >违法和不良信息举报
          </a>
        </view>
        <view>
          <a class="link" href="https://beian.miit.gov.cn/" target="_blank"
            >粤ICP备2024292541号-1
          </a>
          <a class="link" href="https://beian.mps.gov.cn/" target="_blank">
            <img
              src="@/common/beian.png"
              height="15"
              width="15"
              style="vertical-align: sub"
            />
            粤公网安备44090202001181号
          </a>
        </view>
      </view>
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

.link {
  color: #777;
  text-decoration: none;
}

.link:hover {
  color: #8a2c2d;
}
</style>
