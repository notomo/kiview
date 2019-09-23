if exists('g:loaded_kiview')
    finish
endif
let g:loaded_kiview = 1

command! -range -nargs=* Kiview call kiview#main([<line1>, <line2>], <q-args>)

highlight default link KiviewNode String
