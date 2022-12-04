:syntax on

filetype plugin on
set omnifunc=syntaxcomplete#Complete

inoremap jk <ESC>
let mapleader = ","

" turn relative line numbers on
:set relativenumber
:set rnu

" backspace works to delete
:set backspace=indent,eol,start

call pathogen#infect()
call pathogen#helptags()

" colorscheme
:colorscheme molokai
let g:molokai_original = 1

