<script lang="ts">
// import VConsole from "vconsole";
// import { storeToRefs } from "pinia";
// import useGlobalDataStore from "@/stores";
export default {
  onLaunch() {
    // Vconsole
    // if (process.env.NODE_ENV === "development") {
    //   const vconsole = new VConsole();
    // }
    // const store = useGlobalDataStore();
    // const { windowWidth } = storeToRefs(store);
  },
  methods: {
    NavigateTo(url: string) {
      uni.navigateTo({
        url,
      });
    },
  },
  onHide() {},
  onBackPress(options: Page.BackPressOption) {
    if (options.from === "navigateBack") {
      return void 0;
    }
  },
};
</script>

<style lang="scss">
@import "uview-plus/index.scss";

._page {
  // #ifndef APP-PLUS
  height: calc(100vh - 44px); //原生导航栏
  // #endif
  // #ifdef APP-PLUS
  height: calc(100vh);
  // #endif
  background-color: #ededed;
}
.page {
  // #ifndef APP-PLUS
  height: calc(100vh - 44px - 50px); //原生导航栏+tabbar
  // #endif
  // #ifdef APP-PLUS
  height: calc(100vh);
  // #endif
  background-color: #ededed;
}
._tap {
  -webkit-tap-highlight-color: rgba(0, 0, 0, 0);
}
</style>
