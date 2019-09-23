
let s:suite = themis#suite('plugin.kiview')
let s:assert = themis#helper('assert')

function! s:suite.before_each()
    call KiviewTestBeforeEach()
    filetype on
    syntax enable
endfunction

function! s:suite.after_each()
    call KiviewTestAfterEach()
    filetype off
    syntax off
endfunction

function! s:lines() abort
    return getbufline('%', 1, '$')
endfunction

function! s:syntax_name() abort
    return synIDattr(synID(line('.'), col('.'), v:true), 'name')
endfunction

function! s:count_window() abort
    return tabpagewinnr(tabpagenr(), '$')
endfunction

function! s:count_tab() abort
    return tabpagenr('$')
endfunction

function! s:file_name() abort
    return fnamemodify(bufname('%'), ':t')
endfunction

function! s:assert.contains(haystack, needle) abort
    call s:assert.true(count(a:haystack, a:needle) != 0, a:needle . ' must be in the haystack')
endfunction

function! s:assert.not_contains(haystack, needle) abort
    call s:assert.false(count(a:haystack, a:needle) != 0, a:needle . ' must not be in the haystack')
endfunction

function! s:main(arg) abort
    let line = line('.')
    return kiview#main([line, line], a:arg)
endfunction

function! s:suite.create()
    let command = s:main('')
    call command.wait()

    let lines = s:lines()
    call s:assert.not_empty(lines)
    call s:assert.equals(lines[0], '..')
    call s:assert.contains(lines, 'autoload/')
    call s:assert.not_contains(lines, '')
    call s:assert.equals(&filetype, 'kiview')
    call s:assert.false(&modifiable)

    call s:assert.equals(s:syntax_name(), 'KiviewNode')
    call search('autoload\/')
    call s:assert.equals(s:syntax_name(), 'KiviewNode')
endfunction

function! s:suite.do_parent_child()
    cd ./test/plugin

    let command = s:main('')
    call command.wait()

    let lines = s:lines()
    call s:assert.not_empty(lines)
    call s:assert.contains(lines, 'kiview.vim')
    call s:assert.not_contains(lines, '')
    call s:assert.equals(&filetype, 'kiview')

    let command = s:main('parent')
    call command.wait()

    let test_lines = s:lines()
    call s:assert.not_empty(test_lines)
    call s:assert.equals(lines[0], '..')
    call s:assert.contains(test_lines, 'plugin/')
    call s:assert.not_contains(test_lines, '')
    call s:assert.equals(&filetype, 'kiview')
    call s:assert.false(&modifiable)

    let command = s:main('parent')
    call command.wait()

    let lines = s:lines()
    call s:assert.not_empty(lines)
    call s:assert.contains(lines, 'autoload/')
    call s:assert.not_contains(lines, '')
    call s:assert.equals(&filetype, 'kiview')

    call search('test/')
    let command = s:main('child')
    call command.wait()

    let lines = s:lines()
    call s:assert.not_empty(lines)
    call s:assert.equals(lines[0], '..')
    call s:assert.contains(test_lines, 'plugin/')
    call s:assert.not_contains(test_lines, '')
    call s:assert.equals(&filetype, 'kiview')
    call s:assert.equals(lines, test_lines)

    call search('\.themisrc')
    let command = s:main('child')
    call command.wait()

    call s:assert.equals(s:file_name(), '.themisrc')
    call s:assert.equals(&filetype, 'vim')
endfunction

function! s:suite.quit()
    let command = s:main('')
    call command.wait()

    call s:assert.equals(&filetype, 'kiview')
    call s:assert.equals(s:count_window(), 2)

    let command = s:main('quit')
    call command.wait()

    call s:assert.not_equals('kiview', &filetype)
    call s:assert.equals(s:count_window(), 1)
endfunction

function! s:suite.quit_option()
    let command = s:main('')
    call command.wait()

    call search('Makefile')
    let command = s:main('child -quit')
    call command.wait()

    call s:assert.equals(s:file_name(), 'Makefile')
    call s:assert.equals(s:count_window(), 1)
endfunction

function! s:suite.tab_open()
    let command = s:main('')
    call command.wait()

    call search('Makefile')
    let command = s:main('child -layout=tab')
    call command.wait()

    call s:assert.equals(s:file_name(), 'Makefile')
    call s:assert.equals(s:count_tab(), 2)
endfunction

function! s:suite.vertical()
    let command = s:main('')
    call command.wait()

    call search('Makefile')
    let command = s:main('child -layout=vertical')
    call command.wait()

    call s:assert.equals(s:file_name(), 'Makefile')
    call s:assert.equals(s:count_window(), 3)
endfunction

function! s:suite.history()
    cd ./src

    let command = s:main('')
    call command.wait()

    call search('src')
    let command = s:main('child')
    call command.wait()

    call search('repository')
    let command = s:main('child')
    call command.wait()

    let command = s:main('parent')
    call command.wait()
    let command = s:main('parent')
    call command.wait()
    let command = s:main('parent')
    call command.wait()

    let command = s:main('child')
    call command.wait()
    let command = s:main('child')
    call command.wait()

    let lines = s:lines()
    call s:assert.contains(lines, 'repository/')
endfunction

function! s:suite.no_error_with_continuous()
    cd ./src/src/repository

    let create_command = s:main('')
    let parent_command1 = s:main('parent')
    let parent_command2 = s:main('parent')

    call create_command.wait()
    call parent_command1.wait()
    call parent_command2.wait()
endfunction

function! s:suite.nop_logger()
    call kiview#logger#clear()

    let command = s:main('')
    call command.wait()

    let command = s:main('parent')
    call command.wait()
endfunction

function! s:suite.range()
    cd ./src

    let command = s:main('')
    call command.wait()

    let line = search('Cargo\.toml')
    let command = kiview#main([line, line + 1], 'child -layout=tab')
    call command.wait()

    call s:assert.equals(s:count_tab(), 3)
endfunction

function! s:suite.parent_marker()
    cd ./src

    let command = s:main('')
    call command.wait()

    let command = s:main('child')
    call command.wait()

    let lines = s:lines()
    call s:assert.contains(lines, 'autoload/')
endfunction
