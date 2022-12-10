(defn find-marker
  [input len]
  ((fn [input idx len]
     (if (= len (count (set (first input))))
       idx
       (recur (rest input) (inc idx) len))) (partition len 1 input) len len))


(let [input (slurp "input.txt")]
  (println (find-marker input 4))
  (println (find-marker input 14)))
