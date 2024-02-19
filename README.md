## Benchmarks

### Wizened (lodash)

     http_reqs......................: 58974   5893.776506/s
     iteration_duration.............: avg=6.77ms  min=609.45µs med=6.66ms max=58.8ms  p(99)=9.3ms  p(99.99)=56.3ms
     iterations.....................: 58974   5893.776506/s
     vus............................: 40      min=40        max=40
     vus_max........................: 40      min=40        max=40

---

### Not wizened (lodash)

     http_reqs......................: 1889    185.197476/s
     iteration_duration.............: avg=213.95ms min=50.16ms med=212.3ms  max=329.55ms p(99)=299.83ms p(99.99)=329.37ms
     iterations.....................: 1889    185.197476/s
     vus............................: 40      min=40       max=40
     vus_max........................: 40      min=40       max=40

---

### Wizened (empty)

     http_reqs......................: 244400  24436.014486/s
     iteration_duration.............: avg=1.63ms  min=93.25µs med=1.52ms max=47.39ms p(99)=5.03ms p(99.99)=27.16ms
     iterations.....................: 244400  24436.014486/s
     vus............................: 40      min=40         max=40
     vus_max........................: 40      min=40         max=40

---

### Not wizened (empty)

     http_reqs......................: 160810  16074.791915/s
     iteration_duration.............: avg=2.47ms  min=359.29µs med=2.26ms max=58.23ms p(99)=7.86ms p(99.99)=28.51ms
     iterations.....................: 160810  16074.791915/s
     vus............................: 40      min=40         max=40
     vus_max........................: 40      min=40         max=40

---

### Wizened (empty) with minimal QuickJS features

     http_reqs......................: 272228  27218.551184/s
     iteration_duration.............: avg=1.46ms  min=85.58µs med=1.35ms max=23.84ms p(99)=4.66ms p(99.99)=14.14ms
     iterations.....................: 272228  27218.551184/s
     vus............................: 40      min=40         max=40
     vus_max........................: 40      min=40         max=40
