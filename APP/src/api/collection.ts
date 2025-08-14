import type { Ref } from 'vue'
import { fmt, type Pagination } from '@/@types/query'
import pinia from '@/stores'
import { useGlobalDataStore } from '@/stores/GlobalData'
import requests from '@/utils/requests'
import { storeToRefs } from 'pinia'

export function CollectionList(query: Pagination): Promise<Collection[]> {
  return new Promise(async (resolve, reject) => {
    const res = await requests.get<Collection[]>({
      url: `/collection/list?${fmt(query)}`,
    })
    resolve(res.data.data)
  })
}
export function CollectionLike(id: number): Promise<any> {
  return new Promise(async (resolve, reject) => {
    const res = await requests.get({
      url: `/collection/${id}/like`,
      loading: false,
    })
    resolve(res.data)
  })
}

const sotre = useGlobalDataStore(pinia)
const { day, month, year } = storeToRefs(sotre)

export function handleCollectionLike(
  id: number,
  Liked: Ref<boolean>,
  Like: Ref<number>,
): Promise<boolean> {
  return new Promise(async (resolve, reject) => {
    if (Liked.value) {
      reject(false)
      return
    }
    CollectionLike(id).then(() => {
      Liked.value = true
      Like.value += 1
      uni.setStorageSync(
        `Like${id}`,
        `${year.value}${month.value}${day.value}`,
      )
      resolve(true)
    })
  })
}

export function CollectionDetail(id: number) {
  return new Promise<Collection>(async (resolve, reject) => {
    const res = await requests.get<Collection>({
      url: `/collection/${id}`,
    })
    resolve(res.data.data)
  })
}

export function CollectionPhotos(id: number) {
  return new Promise<Photos>(async (resolve, reject) => {
    const res = await requests.get<Photos>({
      url: `/collection/${id}/photos`,
    })
    resolve(res.data.data)
  })
}

export function Search(query: Pagination & { q: string }) {
  return new Promise<Collection[]>(async (reslove) => {
    const res = await requests.get<Collection[]>({
      url: `/collection/search?${fmt(query)}`,
    })
    reslove(res.data.data)
  })
}
