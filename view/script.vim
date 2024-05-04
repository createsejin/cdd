let s:so_save = &g:so | let s:siso_save = &g:siso | setg so=0 siso=0 | setl so=-1 siso=-1
arglocal
%argdel
$argadd ~/Projects/cdd/script
$argadd ~/Documents/scripts/.scripts/cdd.sh
$argadd ~/Projects/cdd/cdd.sh
$argadd ~/Projects/cdd/cdd_data.txt
$argadd ~/Documents/configs/.zshrc
if bufexists(fnamemodify("~/Projects/cdd/cdd_data.txt", ":p")) | buffer ~/Projects/cdd/cdd_data.txt | else | edit ~/Projects/cdd/cdd_data.txt | endif
if &buftype ==# 'terminal'
  silent file ~/Projects/cdd/cdd_data.txt
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
let s:l = 5 - ((4 * winheight(0) + 14) / 28)
if s:l < 1 | let s:l = 1 | endif
keepjumps exe s:l
normal! zt
keepjumps 5
normal! 05|
let &g:so = s:so_save | let &g:siso = s:siso_save
set hlsearch
nohlsearch
doautoall SessionLoadPost
" vim: set ft=vim :
