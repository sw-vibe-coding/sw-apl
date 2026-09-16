;;; apl-input.el --- sw-apl Quail input method  -*- lexical-binding: t; coding: utf-8 -*-

;; Copyright (c) 2026 Michael A Wright
;; SPDX-License-Identifier: MIT

;;; Commentary:

;; A self-contained input method for typing the APL\360 glyph set
;; used by sw-apl.  Backtick is the prefix; positions follow the
;; classic APL typeball keyboard (the same positions Dyalog and
;; gnu-apl-mode use).  See docs/input-methods.md.
;;
;;   (load-file "path/to/sw-apl/docs/emacs/apl-input.el")
;;   (set-input-method "sw-apl")   ; or C-\ then sw-apl

;;; Code:

(require 'quail)

(quail-define-package
 "sw-apl" "UTF-8" "⍴" t
 "sw-apl input method: backtick prefix, classic APL keyboard positions."
 nil t nil nil nil nil nil nil nil nil t)

(quail-define-rules
 ;; number row
 ("`2" "¯") ("`3" "<") ("`4" "≤") ("`5" "=") ("`6" "≥")
 ("`7" ">") ("`8" "≠") ("`9" "∨") ("`0" "∧") ("`-" "×") ("`=" "÷")
 ;; top row
 ("`q" "?") ("`e" "∊") ("`r" "⍴") ("`t" "~") ("`y" "↑")
 ("`u" "↓") ("`i" "⍳") ("`o" "○") ("`p" "*") ("`[" "←") ("`]" "→")
 ;; home row
 ("`s" "⌈") ("`d" "⌊") ("`f" "_") ("`g" "∇") ("`h" "∆")
 ("`j" "∘") ("`k" "'") ("`l" "⎕")
 ;; bottom row
 ("`b" "⊥") ("`n" "⊤") ("`m" "|") ("`," "⍝") ("`." "⍀") ("`/" "⌿")
 ;; shifted
 ("`@" "⍫") ("`#" "⍒") ("`$" "⍋") ("`%" "⌽") ("`^" "⍉") ("`&" "⊖")
 ("`*" "⍟") ("`(" "⍱") ("`)" "⍲") ("`_" "!") ("`+" "⌹") ("`{" "⍞")
 ("`H" "⍙") ("`!" "⌶"))

(provide 'apl-input)
;;; apl-input.el ends here
