interface Search {
  q: string
}
export interface Pagination {
  order_by?: string
  offset?: number
  limit?: number
  page?: number
  per_page?: number
}

export function fmt(query: AnyObject) {
  return Object.entries(query)
    .filter(([_, value]) => value !== undefined)
    .map(
      ([key, value]) =>
        `${encodeURIComponent(key)}=${encodeURIComponent(value as string)}`,
    )
    .join('&')
}
