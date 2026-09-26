;;; ob-sw-apl.el --- Org Babel support for sw-apl  -*- lexical-binding: t; coding: utf-8 -*-

;; Copyright (c) 2026 Michael A Wright
;; SPDX-License-Identifier: MIT

;;; Commentary:

;; Source blocks in APL, run by sw-apl:
;;
;;   #+begin_src sw-apl :mode 75
;;         +/⍳10
;;   #+end_src
;;
;; Each block is a batch run of its own, `sw-apl --no-echo -f', in a
;; clear workspace: what one block defines the next does not see, so
;; a block that needs a workspace loads it, or takes the definitions
;; it needs by noweb reference.  The header arguments:
;;
;;   :mode 70 or 75     the mode, (A) '70 or (B) '75; 75 by default
;;   :echo yes          show each line as typed, indented six spaces,
;;                      so the result reads as a session transcript
;;   :library DIR       where libraries 0 and 1 are (sw-apl --library)
;;   :lib "2=DIR,NAME"  a library beyond them (sw-apl --lib)
;;
;; :session is refused: sw-apl runs a block and ends.  Blocks are
;; shown in sw-apl-mode, and tangle to .apl files.
;;
;;   (add-to-list 'load-path "path/to/sw-apl/docs/emacs")
;;   (require 'ob-sw-apl)
;;   (add-to-list 'org-babel-load-languages '(sw-apl . t))

;;; Code:

(require 'ob)
(require 'sw-apl-mode)

(defgroup ob-sw-apl nil
  "Org Babel support for sw-apl."
  :group 'org-babel)

(defcustom org-babel-sw-apl-command "sw-apl"
  "The sw-apl program that runs a block: a name on `exec-path', or a path."
  :type 'string
  :group 'ob-sw-apl)

(defvar org-babel-default-header-args:sw-apl
  '((:results . "output") (:session . "none") (:mode . "75"))
  "Default header arguments for sw-apl source blocks.")

(add-to-list 'org-src-lang-modes '("sw-apl" . sw-apl))
(with-eval-after-load 'ob-tangle
  (add-to-list 'org-babel-tangle-lang-exts '("sw-apl" . "apl")))

(defun org-babel-sw-apl--program ()
  "The sw-apl to run: `org-babel-sw-apl-command' found on `exec-path',
or as a path, or in ~/.local/softwarewrighter/bin, where a GUI Emacs
with a short PATH would not look."
  (let ((command org-babel-sw-apl-command))
    (or (and (file-name-absolute-p command) (file-executable-p command) command)
        (executable-find command)
        (let ((there (expand-file-name command "~/.local/softwarewrighter/bin")))
          (and (file-executable-p there) there))
        (user-error "sw-apl block: %S not found; set `org-babel-sw-apl-command'"
                    command))))

(defun org-babel-sw-apl--arguments (params file)
  "The command line for a run of FILE with the header arguments PARAMS."
  (let ((mode (format "%s" (or (cdr (assq :mode params)) "75")))
        (echo (cdr (assq :echo params)))
        (library (cdr (assq :library params)))
        (lib (cdr (assq :lib params))))
    (unless (member mode '("70" "75"))
      (user-error "sw-apl block: :mode is 70 or 75, not %s" mode))
    (append (list "--mode" mode)
            (unless (equal echo "yes") (list "--no-echo"))
            (when library (list "--library" (expand-file-name library)))
            (when lib (list "--lib" (format "%s" lib)))
            (list "-f" file))))

(defun org-babel-execute:sw-apl (body params)
  "Run BODY, an sw-apl block with header arguments PARAMS, and return
what it printed."
  (let ((session (cdr (assq :session params))))
    (when (and session (not (equal session "none")))
      (user-error "sw-apl blocks take no :session; each block runs on its own")))
  (let ((file (make-temp-file "ob-sw-apl-" nil ".apl"))
        (program (org-babel-sw-apl--program)))
    (unwind-protect
        (progn
          (with-temp-file file
            (set-buffer-file-coding-system 'utf-8-unix)
            (insert (org-babel-expand-body:generic body params) "\n"))
          (with-temp-buffer
            (let* ((coding-system-for-read 'utf-8)
                   (status (apply #'call-process program nil t nil
                                  (org-babel-sw-apl--arguments params file))))
              (unless (eq status 0)
                (user-error "sw-apl block: exit %s: %s" status (buffer-string)))
              (buffer-string))))
      (delete-file file))))

(provide 'ob-sw-apl)
;;; ob-sw-apl.el ends here
