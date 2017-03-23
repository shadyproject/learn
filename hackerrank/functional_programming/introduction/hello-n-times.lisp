(defun fn (n) (dotimes (x n) (format t "Hello World~%")))

(fn (parse-integer (read-line)))
