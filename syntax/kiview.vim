if exists('b:current_syntax')
    finish
endif

syntax match KiviewNode ".*\/$"
syntax match KiviewNode "^\.\.$"
