if exists('g:loaded_kiview')
    finish
endif
let g:loaded_kiview = 1

command! -range -nargs=* Kiview call kiview#main(<q-args>, {'range': [<line1>, <line2>]})
