import requests from "@/utils/requests";

export function GetCateList() {
  return new Promise<Category[]>(async (reslove, reject) => {
    const res = await requests.get<Category[]>({
      url: "/category/list",
    });
    reslove(res.data.data);
  });
}

export function Collections() {
  return new Promise<Tabbar[]>(async (resolve, reject) => {
    const res = await requests.get<Tabbar[]>({
      url: "/category/collections",
    });
    resolve(res.data.data);
  });
}

export function Collection_with_id(id: number) {
  return new Promise<Tabbar>(async (reslove, reject) => {
    const res = await requests.get<Tabbar>({
      url: `/category/collections/${id}`,
      loading: false,
    });
    reslove(res.data.data);
  });
}
