(defun f (n list) 
  (dolist (item list)
    (dotimes (x n) (format t "~a~%" item)))

  )
(defun read-list ()
  (let ((n (read *standard-input* nil)))
    (if (null n)
        nil
        (cons n (read-list)))))

(format t "~{~d~%~}" (f (read) (read-list)))
