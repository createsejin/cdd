let s:so_save = &g:so | let s:siso_save = &g:siso | setg so=0 siso=0 | setl so=-1 siso=-1
arglocal
%argdel
$argadd ~/Projects/cdd/rust
$argadd ~/Projects/cdd/src/operator.rs
$argadd ~/Projects/cdd/src/test/test001.rs
$argadd ~/Projects/rust_study_001/study001/src/leetcode/leet002.rs
$argadd ~/Projects/rust_study_001/minigrep/src/main.rs
$argadd ~/Projects/rust_study_001/minigrep/src/lib.rs
if bufexists(fnamemodify("~/Projects/cdd/src/operator.rs", ":p")) | buffer ~/Projects/cdd/src/operator.rs | else | edit ~/Projects/cdd/src/operator.rs | endif
if &buftype ==# 'terminal'
  silent file ~/Projects/cdd/src/operator.rs
endif
setlocal fdm=manual
setlocal fde=0
setlocal fmr={{{,}}}
setlocal fdi=#
setlocal fdl=0
setlocal fml=1
setlocal fdn=20
setlocal fen
silent! normal! zE
let &fdl = &fdl
let s:l = 9 - ((8 * winheight(0) + 14) / 28)
if s:l < 1 | let s:l = 1 | endif
keepjumps exe s:l
normal! zt
keepjumps 9
normal! 04|
let &g:so = s:so_save | let &g:siso = s:siso_save
set hlsearch
nohlsearch
doautoall SessionLoadPost
" vim: set ft=vim :
