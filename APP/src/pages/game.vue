<script setup lang="ts">
import type {
  CanvasOnTouchendEvent,
  CanvasOnTouchmoveEvent,
  CanvasOnTouchstartEvent,
} from "@uni-helper/uni-app-types";
import { useGlobalDataStore } from "@/stores";
import { onLoad } from "@dcloudio/uni-app";
import { storeToRefs } from "pinia";
import { computed, ref } from "vue";
import { getImageDimensions, shuffleArray } from "./game";

interface Item {
  id: number; // 触控identifier
  pos_x: number; // 坐标系x
  pos_y: number; // 坐标系y
  index: number; // 拼图块index
  x: number; // 移动中的x
  y: number; // 移动中的y
  dx: number;
  dy: number;
}

document.body.style.overflow = "hidden";

const uToastRef = ref<UniToastRef>();

const GlobalDatastores = useGlobalDataStore();
const { windowWidth, windowHeight } = storeToRefs(GlobalDatastores);

const ctx = uni.createCanvasContext("ctx");

const CanvasRenderList: any[] = [];

const CanvasSize_x = computed(() => {
  return windowWidth.value;
});
const CanvasSize_y = computed(() => {
  return windowHeight.value - 100;
});

const RenderStart_x = ref(10);
const RenderStart_y = ref(0);
const RenderEnd_y = ref(CanvasSize_y.value);

const RenderEnd_x = computed(() => {
  return CanvasSize_x.value - RenderStart_x.value;
});
const Render_x = computed(() => {
  return RenderEnd_x.value - RenderStart_x.value;
});
const Render_y = computed(() => {
  return RenderEnd_y.value - RenderStart_y.value;
});

const Render_fr_x = ref(3);
const Render_fr_y = ref(3);
const Render_gap = ref(1);

const size_x = computed(() => {
  return (
    (Render_x.value - Render_gap.value * (Render_fr_x.value - 1)) /
    Render_fr_x.value
  );
});
const size_y = computed(() => {
  return (
    (Render_y.value - Render_gap.value * (Render_fr_y.value - 1)) /
    Render_fr_y.value
  );
});

const Render_src = ref("");
const Render_arr = ref<number[]>([]);

// data-<

function loadCanvasRenderList({
  src,
  fr_x = 3,
  fr_y = 3,
  gap = 1,
}: {
  src: string;
  fr_x?: number;
  fr_y?: number;
  gap?: number;
}) {
  return new Promise(async (resolve, reject) => {
    CanvasRenderList.length = 0;
    Render_fr_x.value = fr_x;
    Render_fr_y.value = fr_y;
    Render_gap.value = gap;
    Render_src.value = src;
    const srcInfo = await getImageDimensions(Render_src.value);
    RenderStart_y.value =
      CanvasSize_y.value / 2 -
      ((srcInfo.height / srcInfo.width) * Render_x.value) / 2;
    RenderEnd_y.value =
      (srcInfo.height / srcInfo.width) * Render_x.value + RenderStart_y.value;
    if (Render_y.value + 1 > CanvasSize_y.value) {
      RenderStart_x.value = RenderStart_x.value + 3;
      loadCanvasRenderList({ src, fr_x, fr_y, gap });
      return;
    }
    for (let i = 0; i < Render_fr_y.value; i++) {
      const blockStart_y =
        i * size_y.value + Render_gap.value * i + RenderStart_y.value;
      const srcStart_y = (i * srcInfo.height) / Render_fr_y.value;
      for (let j = 0; j < Render_fr_x.value; j++) {
        const blockStart_x =
          j * size_x.value + Render_gap.value * j + RenderStart_x.value;
        const srcStart_x = (j * srcInfo.width) / Render_fr_x.value;
        CanvasRenderList.push([
          srcStart_x,
          srcStart_y,
          srcInfo.width / Render_fr_x.value,
          srcInfo.height / Render_fr_y.value,
          blockStart_x,
          blockStart_y,
          size_x.value,
          size_y.value,
          true,
        ]);
      }
    }

    // shuffle render list
    const arr = new Array(Render_fr_x.value * Render_fr_y.value) // @ts-ignore
      .fill()
      .map((item, index) => index);

    shuffleArray(arr, CanvasRenderList);
    Render_arr.value = [...arr];

    render();
    resolve("");
    uni.hideLoading();
  });
}
// utils->
// 转换子元素相对坐标
//相对于子元素中心的坐标
function getRelativeXY(x: number, y: number) {
  const { pos_x, pos_y } = getPos(x, y);

  if (pos_x == null || pos_y == null) {
    return {
      dx: 0,
      dy: 0,
    };
  }

  const dx =
    x -
    RenderStart_x.value -
    size_x.value / 2 -
    (pos_x - 1) * (size_x.value + Render_gap.value);

  const dy =
    y -
    RenderStart_y.value -
    size_y.value / 2 -
    (pos_y - 1) * (size_y.value + Render_gap.value);

  console.log(pos_x, pos_y, x, y, dx, dy);
  return { dx: -dx, dy: -dy };
}
// 画布坐标转换整数拼图块坐标系
function getPos(
  x: number,
  y: number
): {
  pos_x: number | null;
  pos_y: number | null;
} {
  let pos_x, pos_y: number | null;

  if (
    x <= RenderStart_x.value ||
    y <= RenderStart_y.value ||
    x >= RenderEnd_x.value ||
    y >= RenderEnd_y.value
  ) {
    return { pos_x: null, pos_y: null };
  }
  const relative_x = x - RenderStart_x.value;
  const relative_y = y - RenderStart_y.value;
  pos_x = Math.floor(relative_x / (size_x.value + Render_gap.value)) + 1;
  pos_y = Math.floor(relative_y / (size_y.value + Render_gap.value)) + 1;
  if (
    pos_x * (size_x.value + Render_gap.value) - Render_gap.value <
    relative_x
  ) {
    pos_x = null;
  }
  if (
    pos_y * (size_y.value + Render_gap.value) - Render_gap.value <
    relative_y
  ) {
    pos_y = null;
  }
  return { pos_x, pos_y };
}
function pos2index(pos_x: number, pos_y: number) {
  return pos_x + (pos_y - 1) * Render_fr_x.value;
}
function getIndex(x: number, y: number) {
  // condition: in render
  if (
    x <= RenderStart_x.value ||
    y <= RenderStart_y.value ||
    x >= RenderEnd_x.value ||
    y >= RenderEnd_y.value
  ) {
    return null;
  }
  const { pos_x, pos_y } = getPos(x, y);
  if (!pos_x || !pos_y) {
    return null;
  }
  const index = pos2index(pos_x, pos_y);
  return index;
}
// utils-<

const MovingList: Item[] = [];
function render() {
  ctx.clearRect(0, 0, CanvasSize_x.value, CanvasSize_y.value);
  for (let i = 0; i < CanvasRenderList.length; i++) {
    if (CanvasRenderList[i][8]) {
      ctx.drawImage(Render_src.value, ...CanvasRenderList[i].slice(0, 8));
    }
  }
  for (let i = 0; i < CanvasRenderList.length; i++) {
    if (!CanvasRenderList[i][8]) {
      for (let j = 0; j < MovingList.length; j++) {
        if (MovingList[j].index - 1 === i) {
          ctx.drawImage(
            Render_src.value,
            ...CanvasRenderList[i].slice(0, 4),
            MovingList[j].x - size_x.value / 2,
            MovingList[j].y - size_y.value / 2,
            size_x.value,
            size_y.value
          );
          break;
        }
      }
    }
  }
  ctx.draw();

  if (
    Render_arr.value.toString() ===
    new Array(Render_fr_x.value * Render_fr_y.value) // @ts-ignore
      .fill()
      .map((item, index) => index)
      .toString()
  ) {
    uToastRef.value?.show({
      type: "default",
      message: "恭喜挑战成功！\n分享链接让别人也来试试吧！",
    });
  }
}
function handleTouchStart(e: CanvasOnTouchstartEvent) {
  e.stopPropagation();
  const { x, y, identifier } = e.changedTouches[0];
  if (x == undefined || y == undefined || identifier == undefined) {
    return;
  }
  const { pos_x, pos_y } = getPos(x, y);
  let index = getIndex(x, y);
  if (pos_x && pos_y) {
    index = index as number;
    for (let i = 0; i < MovingList.length; i++) {
      if (MovingList[i].index === index) {
        return;
      }
      if (MovingList[i].id === identifier) {
        return;
      }
    }
    const { dx, dy } = getRelativeXY(x, y);
    MovingList.push({
      id: identifier,
      pos_x,
      pos_y,
      index,
      x,
      y,
      dx,
      dy,
    });
    CanvasRenderList[index - 1][8] = false;
  }
}

function handleTouchMove(e: CanvasOnTouchmoveEvent) {
  for (let i = 0; i < e.touches.length; i++) {
    // x,y为实时坐标
    const { x, y, identifier } = e.touches[i];
    if (x == undefined || y == undefined || identifier == undefined) {
      return;
    }
    // 需要加上相对于子item左上角的坐标

    for (let j = 0; j < MovingList.length; j++) {
      if (MovingList[j].id === identifier) {
        MovingList[j].x = x + MovingList[i].dx;
        MovingList[j].y = y + MovingList[i].dy;
      }
    }
  }
  render();
}

function handleTouchCancel() {
  for (let i = 0; i < MovingList.length; i++) {
    CanvasRenderList[MovingList[i].index - 1][8] = true;
    MovingList.splice(i, 1);
  }
  render();
}

function handleTouchEnd(e: CanvasOnTouchendEvent) {
  const { x, y, identifier } = e.changedTouches[0];
  if (x == undefined || y == undefined || identifier == undefined) {
    return;
  }
  const index = getIndex(x, y);
  for (let i = 0; i < MovingList.length; i++) {
    if (MovingList[i].id === identifier) {
      CanvasRenderList[MovingList[i].index - 1][8] = true;
      if (index) {
        // inedx Moving[i].index
        for (let j = 0; j < 4; j++) {
          [
            CanvasRenderList[index - 1][j],
            CanvasRenderList[MovingList[i].index - 1][j],
          ] = [
            CanvasRenderList[MovingList[i].index - 1][j],
            CanvasRenderList[index - 1][j],
          ];
        }
        [
          Render_arr.value[index - 1],
          Render_arr.value[MovingList[i].index - 1],
        ] = [
          Render_arr.value[MovingList[i].index - 1],
          Render_arr.value[index - 1],
        ];
      }
      MovingList.splice(i, 1);
      break; // 必须break
    }
  }

  if (e.touches.length === 0) {
    for (let i = 0; i < MovingList.length; i++) {
      CanvasRenderList[MovingList[i].index - 1][8] = true;
      MovingList.splice(i, 1);
    }
  }
  render();
}

onLoad(async (options) => {
  if (
    options &&
    options.id &&
    Number(options.id) > 0 &&
    Number(options.id) <= 10
  ) {
    Render_src.value = `${import.meta.env.VITE_OSS}game/${options.id}.png`;
  } else {
    Render_src.value = `${import.meta.env.VITE_OSS}game/${
      Math.floor(Math.random() * (10 - 1 + 1)) + 1
    }.png`;
  }
  uni.showLoading({
    title: "加载中",
  });
  await loadCanvasRenderList({ src: Render_src.value });
});

const frnumber_x = ref(3);
const frnumber_y = ref(3);
async function buttonclick() {
  await loadCanvasRenderList({
    src: Render_src.value,
    fr_x: frnumber_x.value,
    fr_y: frnumber_y.value,
  });
}
const ret = () => {
  console.log("hello");
  uni.reLaunch({
    url: "/pages/inde",
  });
};
</script>

<template>
  <up-toast ref="uToastRef" />
  <view class="">
    <view
      style="
        display: flex;
        text-align: center;
        flex-direction: column;
        justify-content: center;
        gap: 15px;
        background-color: #e7e7e7;
      "
      :style="
        (() => {
          return {
            height: `${windowHeight - CanvasSize_y}px`,
          };
        })()
      "
    >
      <view
        style="
          display: flex;
          justify-content: start;
          gap: 20px;
          padding: 0 25px;
          font-size: 18px;
          font-weight: 400;
        "
      >
        <uni-icons type="left" @click="ret" size="27"></uni-icons>
        <span style="line-height: 23px; font-size: 22px">拼图小游戏</span>
      </view>
      <view
        style="
          display: flex;
          text-align: center;
          justify-content: center;
          gap: 15px;
        "
      >
        <up-number-box
          v-model="frnumber_x"
          :min="1"
          :max="10"
          :disabled-input="true"
        />
        <up-number-box
          v-model="frnumber_y"
          :min="1"
          :max="10"
          :disabled-input="true"
        />
        <view
          style="display: flex; justify-content: center; align-items: center"
        >
          <up-button
            text="调整难度"
            type="primary"
            style="width: 78px; height: 30px"
            @click="buttonclick"
          />
        </view>
      </view>
    </view>

    <view class="container">
      <canvas
        canvas-id="ctx"
        :style="
          (() => {
            return {
              width: `${CanvasSize_x}px`,
              height: `${CanvasSize_y}px`,
              border: '1px solid #ededed',
            };
          })()
        "
        @touchstart="handleTouchStart"
        @touchmove="handleTouchMove"
        @touchend="handleTouchEnd"
        @touchcancel="handleTouchCancel"
      />
    </view>
  </view>
</template>

<style lang="scss" scoped>
.page {
  background-color: #ededed;
  height: 100vh;
}

.container {
  width: 100%;
  display: flex;
  justify-content: center;
  align-items: center;
  background-color: #ededed;
}
</style>
