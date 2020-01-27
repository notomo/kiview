
let s:helper = KiviewTestHelper()
let s:suite = s:helper.suite('autoload.kiview.complete')
let s:assert = s:helper.assert()

let s:_cursor_position = 8888

function! s:suite.get_with_empty() abort
    let got = kiview#complete#get('', 'Kiview ', s:_cursor_position)
    let names = split(got, "\n")

    call themis#log('[log] ' . string(names))

    call s:assert.contains(names, 'new')
    call s:assert.not_contains(names, 'unknown')
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
