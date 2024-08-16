;;; irl-mode.el --- Major mode for editing IRL files -*- lexical-binding: t; -*-

;; Author: hyouteki
;; URL: https://github.com/hyouteki/irl/blob/master/editor_plugins/irl-mode.el
;; Version: 0.1
;; Keywords: languages
;; Package-Requires: ((emacs "24.3"))

(defvar irl-mode-hook nil)

(defvar irl-mode-syntax-table
  (let ((table (make-syntax-table)))
	(modify-syntax-entry ?/ ". 124b" table)
    (modify-syntax-entry ?\n "> b" table)
	table))

(defvar irl-keywords
  '("function" "arg" "goto" "label" "if" "param" "ret" "call" "alloc" "load" "store"))

(defvar irl-operators
  '("=" "\\+" "-" "\\*" "/" "<=" "==" "!=" "<" ">" ">="))

(defvar irl-iden-regexp "\\b[a-zA-Z][a-zA-Z0-9]*\\b")
(defvar irl-num-regexp "\\b[0-9]+\\b")
(defvar irl-keyword-regexp (regexp-opt irl-keywords 'words))
(defvar irl-operator-regexp (regexp-opt irl-operators))

(defvar irl-font-lock-keywords
  `((,irl-keyword-regexp . font-lock-keyword-face)
    (,irl-operator-regexp . font-lock-operator-face)
    (,irl-iden-regexp . font-lock-variable-name-face)
    (,irl-num-regexp . font-lock-constant-face)
    ))

(setq irl-keywords-regexp nil)
(setq irl-operators-regexp nil)

(define-derived-mode irl-mode prog-mode "IRL"
  "Major mode for editing IRL language files."
  :syntax-table irl-mode-syntax-table
  (setq font-lock-defaults '((irl-font-lock-keywords)))
  (setq irl-keyword nil)
  (setq irl-operator nil)
  (setq-local comment-start "// "))

(provide 'irl-mode)
