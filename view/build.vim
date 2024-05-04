let s:so_save = &g:so | let s:siso_save = &g:siso | setg so=0 siso=0 | setl so=-1 siso=-1
arglocal
%argdel
$argadd ~/Projects/cdd/build
$argadd ~/Projects/cdd/Cargo.toml
$argadd ~/Projects/cdd/src/main.rs
$argadd ~/Projects/cdd/src/test/mod.rs
if bufexists(fnamemodify("~/Projects/cdd/src/main.rs", ":p")) | buffer ~/Projects/cdd/src/main.rs | else | edit ~/Projects/cdd/src/main.rs | endif
if &buftype ==# 'terminal'
  silent file ~/Projects/cdd/src/main.rs
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
let s:l = 3 - ((2 * winheight(0) + 13) / 27)
if s:l < 1 | let s:l = 1 | endif
keepjumps exe s:l
normal! zt
keepjumps 3
normal! 011|
let &g:so = s:so_save | let &g:siso = s:siso_save
set hlsearch
nohlsearch
doautoall SessionLoadPost
" vim: set ft=vim :
