/// <reference types="vite/client" />

declare module "*.vue" {
  import type { DefineComponent } from "vue";
  // eslint-disable-next-line @typescript-eslint/no-explicit-any, @typescript-eslint/ban-types
  const component: DefineComponent<{}, {}, any>;
  export default component;
}

/// <reference types='@dcloudio/types' />
import "vue";
declare module "@vue/runtime-core" {
  type Hooks = App.AppInstance & Page.PageInstance;

  interface ComponentCustomOptions extends Hooks {}
}

declare module "vue" {
  type Hooks = App.AppInstance & Page.PageInstance;
  interface ComponentCustomOptions extends Hooks {}
}

interface ImportMetaEnv {
  readonly VITE_API: string;
  readonly VITE_OSS: string;
}

declare global {
  interface APIResponse<T> {
    meta: {
      code: number;
      message: string;
      status: number;
    };
    data: T;
  }
}
