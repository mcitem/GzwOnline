import TsRequests from "./TsRequests";

export const requests = new TsRequests({
  baseURL: import.meta.env.VITE_API,
});

requests.interceptors.request = (config) => {
  if (config.loading === undefined || config.loading === true) {
    uni.showLoading({
      title: "请稍后...",
    });
  }
  return config;
};

requests.interceptors.response = (response) => {
  if (response.config.loading != false) {
    uni.hideLoading();
  }
  if (response.isSuccess === true) {
    return response;
  } else {
    return response;
  }
};

export default requests;
