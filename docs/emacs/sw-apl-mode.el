;;; sw-apl-mode.el --- Major mode for sw-apl source  -*- lexical-binding: t; coding: utf-8 -*-

;; Copyright (c) 2026 Michael A Wright
;; SPDX-License-Identifier: MIT

;;; Commentary:

;; A major mode for APL as sw-apl reads it: scripts (.apl) and saved
;; workspaces (.apl.ws), and the source blocks of an Org document
;; (see ob-sw-apl.el).  It colours what a reader tells apart at a
;; glance -- primitives, the arrows, numbers, strings, comments,
;; system names and commands, the del and the function it opens,
;; labels -- and nothing else: no indentation, no REPL.
;;
;; Every face is its own, named sw-apl-...-face, so a document
;; exported with htmlize carries a class per kind (org-sw-apl-...)
;; that its stylesheet can colour.
;;
;; A primitive is recognised by what it is not -- a letter, a digit, a
;; quote, punctuation, a name, a system name -- so this file holds no
;; copy of sw-apl's glyph table (data/glyphs.toml is the only one).
;;
;;   (add-to-list 'load-path "path/to/sw-apl/docs/emacs")
;;   (require 'sw-apl-mode)

;;; Code:

(defgroup sw-apl nil
  "Editing APL for sw-apl."
  :group 'languages)

(defface sw-apl-primitive-face '((t :inherit font-lock-builtin-face))
  "A primitive function or operator glyph.")
(defface sw-apl-arrow-face '((t :inherit font-lock-keyword-face))
  "Assignment and branch.")
(defface sw-apl-number-face '((t :inherit font-lock-constant-face))
  "A number, the high minus included.")
(defface sw-apl-system-face '((t :inherit font-lock-type-face))
  "Quad, quote-quad and the quad names.")
(defface sw-apl-command-face '((t :inherit font-lock-preprocessor-face))
  "A system command: a line that begins with a right parenthesis.")
(defface sw-apl-del-face '((t :inherit font-lock-keyword-face :weight bold))
  "The del and del-tilde that open and close a definition.")
(defface sw-apl-label-face '((t :inherit font-lock-variable-name-face))
  "A label at the start of a function line.")

(defconst sw-apl--name "[A-Za-z∆⍙][A-Za-z0-9∆⍙_̲]*"
  "A name: a letter, then letters, digits and underscores.")

(defconst sw-apl--not-primitive
  "][:alnum:][:space:]'∆⍙_̲⎕⍞∇⍫⍝¯()[;:←→"
  "What a primitive is not, as the inside of a bracket expression after
its caret: the closing bracket comes first, where it is literal.")

(defvar sw-apl-mode-syntax-table
  (let ((table (make-syntax-table)))
    (modify-syntax-entry ?' "\"" table)
    (modify-syntax-entry ?\" "." table)
    (modify-syntax-entry ?⍝ "<" table)
    (modify-syntax-entry ?\n ">" table)
    (dolist (c '(?∆ ?⍙ ?_ ?̲))
      (modify-syntax-entry c "w" table))
    table)
  "Quotes delimit strings and the lamp begins a comment.")

(defun sw-apl--header-name (limit)
  "Find the next function header before LIMIT, and match its name.
A header is the text after an opening del, up to any semicolon; its
result, if any, is before the left arrow, and the name is the only
word, the first of two (monadic) or the middle of three (dyadic)."
  (when (re-search-forward "^[ \t]*[∇⍫][ \t]*\\([^;\n∇⍫]*\\)" limit t)
    (let* ((start (match-beginning 1))
           (text (match-string 1))
           (arrow (string-match "←" text))
           (from (if arrow (1+ arrow) 0))
           (words '())
           (pos from))
      (while (string-match sw-apl--name text pos)
        (push (cons (match-beginning 0) (match-end 0)) words)
        (setq pos (match-end 0)))
      (setq words (nreverse words))
      (let ((name (pcase (length words)
                    (1 (nth 0 words))
                    (2 (nth 0 words))
                    (3 (nth 1 words)))))
        (if name
            (progn
              (set-match-data (list (+ start (car name)) (+ start (cdr name))))
              t)
          (sw-apl--header-name limit))))))

(defconst sw-apl-font-lock-keywords
  `(("^[ \t]*)[^\n]*" . 'sw-apl-command-face)
    ("^[ \t]*\\([∇⍫]\\)" 1 'sw-apl-del-face)
    ("[∇⍫][ \t]*$" 0 'sw-apl-del-face)
    (sw-apl--header-name 0 'font-lock-function-name-face)
    (,(concat "^[ \t]*\\(" sw-apl--name "\\):") 1 'sw-apl-label-face)
    ("⎕[A-Z]*\\|⍞" . 'sw-apl-system-face)
    ("\\(?:^\\|[^[:alnum:]∆⍙_̲]\\)\\(¯?\\(?:[0-9]+\\.?[0-9]*\\|\\.[0-9]+\\)\\(?:E¯?[0-9]+\\)?\\)"
     1 'sw-apl-number-face)
    ("[←→]" . 'sw-apl-arrow-face)
    (,(concat "[^" sw-apl--not-primitive "]") . 'sw-apl-primitive-face))
  "What sw-apl-mode colours, first match winning.")

;;;###autoload
(define-derived-mode sw-apl-mode prog-mode "sw-apl"
  "Major mode for APL as sw-apl reads it."
  :syntax-table sw-apl-mode-syntax-table
  (setq-local comment-start "⍝ ")
  (setq-local comment-start-skip "⍝+[ \t]*")
  (setq-local font-lock-defaults '(sw-apl-font-lock-keywords)))

;;;###autoload
(add-to-list 'auto-mode-alist '("\\.apl\\(?:\\.ws\\)?\\'" . sw-apl-mode))

(provide 'sw-apl-mode)
;;; sw-apl-mode.el ends here
