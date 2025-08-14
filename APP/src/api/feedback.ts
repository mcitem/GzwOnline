import requests from "@/utils/requests";

export function FeedBack(data: FeedBack) {
  return requests.post<void>({
    loading: false,
    url: "/feedback/new",
    header: {
      "Content-Type": "application/json",
    },
    data: data,
  });
}
