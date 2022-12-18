#lang racket

(define input
  (map (lambda (line) (map string->number (string-split line ",")))
       (with-input-from-file "input.txt" port->lines)))

(define (sides cube)
  (list (list (+ (first cube) 1) (second cube) (third cube))
        (list (- (first cube) 1) (second cube) (third cube))
        (list (first cube) (+ (second cube) 1) (third cube))
        (list (first cube) (- (second cube) 1) (third cube))
        (list (first cube) (second cube) (+ (third cube) 1))
        (list (first cube) (second cube) (- (third cube) 1))))

(define (exposed-sides cube scan)
  (remove* scan (sides cube)))

(define (count-exposed-sides cube scan)
  (length (exposed-sides cube scan)))

(define (out-of-bounds cube)
  (or (< (first cube) -1)
      (< (second cube) -1)
      (< (third cube) -1)
      (< 22 (first cube))
      (< 22 (second cube))
      (< 22 (third cube))))

(define (make-outside scan)
  (define outside (mutable-set empty))
  (define (make-outside* scan cube)
    (unless (or (out-of-bounds cube) (set-member? outside cube) (member cube scan))
      (set-add! outside cube)
      (for-each (lambda (side) (make-outside* scan side)) (sides cube))))
  (make-outside* scan (list 0 0 0))
  outside)

(define (part1 scan)
  (apply + (map (lambda (cube) (count-exposed-sides cube scan)) scan)))

(define (part2 scan)
  (let ([outside (make-outside scan)]
        [air (apply append (map (lambda (cube) (exposed-sides cube scan)) scan))])
    (length (filter (lambda (cube) (set-member? outside cube)) air))))

(displayln (part1 input))
(displayln (part2 input))
