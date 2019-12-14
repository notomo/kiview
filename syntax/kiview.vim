if exists('b:current_syntax')
    finish
endif

highlight default link KiviewNodeClosed String
highlight default KiviewNodeOpen term=NONE guifg=#a9dd9d ctermfg=150 gui=bold
highlight default link KiviewSelected Statement

syntax match KiviewNodeClosed ".*\/$"
syntax match KiviewNodeClosed "^\.\.$"
