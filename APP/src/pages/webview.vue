<script lang="ts">
export default {
  data() {},
  onLoad() {},
  onReady() {},
  onError() {
    uni.reLaunch({
      url: "/pages/inde",
    });
  },
  methods: {},
};
</script>

<template>
  <web-view
    src="https://jsj.top/f/DS8GPX?background=white&banner=show&embedded=true&inner_redirect=true"
  />
</template>

<style></style>
