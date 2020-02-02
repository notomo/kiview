
let s:helper = KiviewTestHelper()
let s:suite = s:helper.suite('autoload.kiview.complete')
let s:assert = s:helper.assert

let s:_cursor_position = 8888

function! s:get_enums(path, name) abort
    let pattern = '^pub enum ' . a:name
    let in_enum_block = v:false
    let enum = []

    for line in readfile(a:path)
        if in_enum_block && line =~? '}$'
            return enum
        endif

        if in_enum_block
            let member = substitute(trim(line), ',', '', 'g')
            if member ==? 'Unknown'
                continue
            endif
            call add(enum, member)
            continue
        endif

        if line =~? pattern
            let in_enum_block = v:true
        endif
    endfor
endfunction

function! s:pascal_to_snake(str) abort
    let str = tolower(a:str[0]) . a:str[1:]
    return substitute(str, '\v(\u)', '_\l\1', 'g')
endfunction

function! s:suite.get_with_empty() abort
    let got = kiview#complete#get('', 'Kiview ', s:_cursor_position)
    let got_names = split(got, "\n")

    call themis#log('[log] ' . string(got_names))

    call s:assert.contains(got_names, 'new')
    call s:assert.not_contains(got_names, 'unknown')

    " NOTE: compare enum directly for check not to foget updating candidates.
    let path = 'src/src/command/command.rs'
    let enum = map(s:get_enums(path, 'CommandName'), { _, v -> s:pascal_to_snake(v) })

    call s:assert.equals(enum, got_names)
endfunction

function! s:suite.get_with_uncompleted_cmd() abort
    let got = kiview#complete#get('rem', 'Kiview rem', s:_cursor_position)
    let names = split(got, "\n")

    call themis#log('[log] ' . string(names))

    call s:assert.contains(names, 'new')
    call s:assert.not_contains(names, 'unknown')
endfunction

function! s:suite.get_with_cmd() abort
    let got = kiview#complete#get('', 'Kiview new ', s:_cursor_position)
    let names = split(got, "\n")

    call themis#log('[log] ' . string(names))

    call s:assert.not_contains(names, 'new')
endfunction
