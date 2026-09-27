;;; literate.el --- Run, tangle and publish docs/literate/*.org  -*- lexical-binding: t; coding: utf-8 -*-

;; Copyright (c) 2026 Michael A Wright
;; SPDX-License-Identifier: MIT

;;; Commentary:

;; Loaded by `emacs --batch' from the repository root, by
;; scripts/literate.sh and scripts/check-literate.sh.  Blocks run the
;; release build, target/release/sw-apl, so a document's recorded
;; results are this checkout's.
;;
;;   (sw-apl-literate-run FILE)       run every block, record results
;;   (sw-apl-literate-tangle FILE)    write the files it tangles to
;;   (sw-apl-literate-export FILE)    FILE.html beside it, coloured

;;; Code:

(defconst sw-apl-literate-root default-directory
  "The repository root, which every file name here is relative to.
Visiting a document moves `default-directory' to the document's own.")

(defun sw-apl-literate--path (file)
  "FILE, relative to the repository root."
  (expand-file-name file sw-apl-literate-root))

(require 'package)
(package-initialize)
(require 'org)
(require 'ox-html)
(require 'ob-tangle)
(add-to-list 'load-path (sw-apl-literate--path "docs/emacs"))
(require 'ob-sw-apl)
(unless (require 'htmlize nil t)
  (error "literate: htmlize is not installed (M-x package-install RET htmlize)"))

(setq org-babel-sw-apl-command (sw-apl-literate--path "target/release/sw-apl")
      org-confirm-babel-evaluate nil
      org-src-preserve-indentation t
      org-html-htmlize-output-type 'css
      org-html-validation-link nil
      org-export-with-sub-superscripts '{}
      org-export-time-stamp-file nil
      make-backup-files nil
      ;; Every document opens with the same way about.
      org-html-preamble t
      org-html-preamble-format
      '(("en" "<nav class=\"literate\"><a href=\"index.html\">Literate APL</a> · <a href=\"../\">sw-apl in your browser</a> · <a href=\"https://github.com/sw-vibe-coding/sw-apl\">Repository</a></nav>")))
(org-babel-do-load-languages 'org-babel-load-languages '((sw-apl . t)))

(defun sw-apl-literate--visit (file)
  "Visit FILE in org-mode, from its own directory."
  (find-file (sw-apl-literate--path file))
  (org-mode))

(defun sw-apl-literate-run (file)
  "Run every block in FILE that evaluates, recording its result."
  (sw-apl-literate--visit file)
  (org-babel-execute-buffer)
  (save-buffer))

(defun sw-apl-literate-tangle (file)
  "Write the files FILE tangles to."
  (org-babel-tangle-file (sw-apl-literate--path file)))

(defun sw-apl-literate-export (file)
  "Export FILE to HTML beside it, with htmlize's classes for colour."
  (sw-apl-literate--visit file)
  ;; Org names what it links to with random references; seeded, they
  ;; come out the same every time, so an export can be checked.
  (random (file-name-nondirectory file))
  (org-html-export-to-html))

(provide 'literate)
;;; literate.el ends here
