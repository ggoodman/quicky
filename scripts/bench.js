// @ts-check

import { check } from "k6";
import http from "k6/http";

/** @type {import("k6/options").Options} */
export const options = {
  // stages: [
  //   { duration: "1m", target: 40 },
  //   { duration: "20m", target: 40 },
  // ],
  duration: "10s",
  vus: 40,
  summaryTrendStats: ["avg", "min", "med", "max", "p(99)", "p(99.99)"],
  thresholds: {
    http_req_failed: ["rate<0.01"], // http errors should be less than 1%
    http_req_duration: ["p(99)<50"], // 99% of requests should be below 50ms
  },
};

const expectBody = JSON.stringify({ hello: "world" });

export default function () {
  const res = http.get(`http://localhost:8080/wasmtime`, {
    timeout: "5s",
  });
  check(res, {
    "status was 200": (r) => r.status == 200,
    "body is '4'": (r) => r.body == "4",
  });
  // sleep(1);
}
