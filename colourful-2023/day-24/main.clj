(require '[clojure.string :as str])
(import 'java.math.BigInteger)

;; ))))))))))))))))))

(defn parseData [input]
  (vec (map (fn [line]
              (vec (map #(BigInteger. %)
                        (re-seq #"-?\d+" line))))
            (str/split input #"\n"))))

(defn findLuckyRock [input]
  (let [process-builder (ProcessBuilder. (into ["python" "./helper.py" input]))
        process (.start process-builder)]
    (with-open [reader (java.io.BufferedReader. (java.io.InputStreamReader. (.getInputStream process)))]
      (doseq [line (line-seq reader)]
        (println line)))))

(defn findHailIntersections [input min max]
  (let [data (parseData input)]
    (reduce (fn [total h1]
              (reduce (fn [total h2]
                        (let [a1 (nth h1 4)
                              b1 (- (nth h1 3))
                              c1 (- (* (nth h1 4) (nth h1 0))
                                    (* (nth h1 3) (nth h1 1)))
                              a2 (nth h2 4)
                              b2 (- (nth h2 3))
                              c2 (- (* (nth h2 4) (nth h2 0))
                                    (* (nth h2 3) (nth h2 1)))]
                          (if (= (* a1 b2) (* a2 b1))
                            total
                            (let [x (/ (- (* c1 b2) (* c2 b1))
                                       (- (* a1 b2) (* a2 b1)))
                                  y (/ (- (* c2 a1) (* c1 a2))
                                       (- (* a1 b2) (* a2 b1)))]
                              (if (and (<= min x max)
                                       (<= min y max)
                                       (every? (fn [hs]
                                                 (and (>= (* (- x (nth hs 0)) (nth hs 3)) 0)
                                                      (>= (* (- y (nth hs 1)) (nth hs 4)) 0)))
                                               [h1 h2]))
                                (+ 1 total)
                                total))))) total (take-while #(not= % h1) data)))
            0 data)))

(defn main []
  (let [input (slurp "./input.txt")
        res1 (findHailIntersections input 200000000000000 400000000000000)]
    (println (format "Result 1: %s" res1)))
  (findLuckyRock "./input.txt"))
(main)