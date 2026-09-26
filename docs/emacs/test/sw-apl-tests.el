;;; sw-apl-tests.el --- ERT tests for sw-apl-mode and ob-sw-apl  -*- lexical-binding: t; coding: utf-8 -*-

;; Copyright (c) 2026 Michael A Wright
;; SPDX-License-Identifier: MIT

;;; Commentary:

;; Run by `just test-emacs':
;;
;;   emacs --batch -L docs/emacs -l docs/emacs/test/sw-apl-tests.el \
;;         -f ert-run-tests-batch-and-exit
;;
;; The babel tests run the sw-apl in SW_APL_BIN, or target/release/sw-apl.

;;; Code:

(require 'ert)
(require 'org)
(require 'sw-apl-mode)
(require 'ob-sw-apl)

(defconst sw-apl-tests--binary
  (or (getenv "SW_APL_BIN")
      (expand-file-name "../../../target/release/sw-apl"
                        (file-name-directory (or load-file-name buffer-file-name))))
  "The sw-apl the babel tests run.")

(defun sw-apl-tests--face-at (text needle)
  "The face font-lock gives the first NEEDLE in TEXT, in sw-apl-mode."
  (with-temp-buffer
    (insert text)
    (sw-apl-mode)
    (font-lock-ensure)
    (goto-char (point-min))
    (search-forward needle)
    (let ((face (get-text-property (match-beginning 0) 'face)))
      (if (consp face) (car face) face))))

(ert-deftest sw-apl-mode-colours-each-kind ()
  (let ((line "R←+/⍳10 ¯2.5 'TEXT' ⎕IO ⍝ NOTE"))
    (should (eq (sw-apl-tests--face-at line "+") 'sw-apl-primitive-face))
    (should (eq (sw-apl-tests--face-at line "⍳") 'sw-apl-primitive-face))
    (should (eq (sw-apl-tests--face-at line "←") 'sw-apl-arrow-face))
    (should (eq (sw-apl-tests--face-at line "10") 'sw-apl-number-face))
    (should (eq (sw-apl-tests--face-at line "¯2.5") 'sw-apl-number-face))
    (should (eq (sw-apl-tests--face-at line "TEXT") 'font-lock-string-face))
    (should (eq (sw-apl-tests--face-at line "⎕IO") 'sw-apl-system-face))
    (should (eq (sw-apl-tests--face-at line "NOTE") 'font-lock-comment-face))
    (should-not (sw-apl-tests--face-at line "R"))))

(ert-deftest sw-apl-mode-colours-definitions-commands-and-labels ()
  (should (eq (sw-apl-tests--face-at "∇R←AVG X\n" "∇") 'sw-apl-del-face))
  (should (eq (sw-apl-tests--face-at "∇R←AVG X\n" "AVG") 'font-lock-function-name-face))
  (should (eq (sw-apl-tests--face-at ")LOAD 1 LIFE\n" ")LOAD") 'sw-apl-command-face))
  (should (eq (sw-apl-tests--face-at "LOOP:X←X+1\n" "LOOP") 'sw-apl-label-face))
  ;; A primitive in a string or a comment is not a primitive.
  (should (eq (sw-apl-tests--face-at "'A+B'" "+") 'font-lock-string-face))
  (should (eq (sw-apl-tests--face-at "⍝ A+B" "+") 'font-lock-comment-face)))

(defun sw-apl-tests--run (body &rest header)
  "Evaluate BODY as an sw-apl block with HEADER arguments, as org does."
  (unless (file-executable-p sw-apl-tests--binary)
    (ert-skip (format "no sw-apl at %s" sw-apl-tests--binary)))
  (let ((org-babel-sw-apl-command sw-apl-tests--binary)
        (org-confirm-babel-evaluate nil))
    (with-temp-buffer
      (org-mode)
      (insert (format "#+begin_src sw-apl %s\n%s\n#+end_src\n"
                      (mapconcat #'identity header " ") body))
      (goto-char (point-min))
      (org-babel-execute-src-block nil nil '((:results . "output silent"))))))

(ert-deftest ob-sw-apl-runs-a-block-in-b-by-default ()
  (should (equal (sw-apl-tests--run "2+3\n)DIALECT") "5\n(B) '75\n")))

(ert-deftest ob-sw-apl-takes-the-mode ()
  (should (equal (sw-apl-tests--run ")DIALECT" ":mode 70") "(A) '70\n"))
  (should (equal (sw-apl-tests--run "⍎'1+1'" ":mode 75") "2\n")))

(ert-deftest ob-sw-apl-shows-a-session-when-asked ()
  (should (equal (sw-apl-tests--run "2+3" ":echo yes") "      2+3\n5\n")))

(ert-deftest ob-sw-apl-reports-errors-as-apl-does ()
  (should (equal (sw-apl-tests--run "1 2+1 2 3")
                 "LENGTH ERROR\n      1 2+1 2 3\n         ^\n")))

(ert-deftest ob-sw-apl-refuses-a-session ()
  (skip-unless (file-executable-p sw-apl-tests--binary))
  (should-error (sw-apl-tests--run "1" ":session s") :type 'user-error))

(ert-deftest ob-sw-apl-defines-and-runs-a-function ()
  (should (equal (sw-apl-tests--run "∇R←AVG X\nR←(+/X)÷⍴X\n∇\nAVG 3 5 10") "6\n")))

(provide 'sw-apl-tests)
;;; sw-apl-tests.el ends here
