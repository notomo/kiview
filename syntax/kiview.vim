if exists('b:current_syntax')
    finish
endif

highlight default link KiviewNode String
highlight default link KiviewSelected Statement

syntax match KiviewNode ".*\/$"
syntax match KiviewNode "^\.\.$"
