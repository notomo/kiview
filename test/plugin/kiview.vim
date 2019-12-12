
let s:suite = themis#suite('plugin.kiview')
let s:assert = KiviewTestAssert()

function! s:suite.before_each()
    call KiviewTestBeforeEach()
endfunction

function! s:suite.after_each()
    call KiviewTestAfterEach()
endfunction

function! s:lines() abort
    return getbufline('%', 1, '$')
endfunction

function! s:input_reader(answer) abort
    let input_reader = {'answer': a:answer}
    function! input_reader.read(msg) abort
        call themis#log('[prompt] ' . a:msg . self.answer)
        return self.answer
    endfunction
    call kiview#input_reader#set(input_reader)

    return input_reader
endfunction

function! s:main(arg) abort
    let line = line('.')
    return kiview#main([line, line], a:arg)
endfunction

function! s:sync_main(arg) abort
    let command = s:main(a:arg)
    call command.wait()
    return command
endfunction

function! s:suite.create()
    let cwd = getcwd()

    call s:sync_main('')

    let lines = s:lines()
    call s:assert.not_empty(lines)
    call s:assert.equals(lines[0], '..')
    call s:assert.contains(lines, 'autoload/')
    call s:assert.not_contains(lines, '')
    call s:assert.filetype('kiview')
    call s:assert.false(&modifiable)
    call s:assert.line_number(2)
    call s:assert.dir(cwd)
    call s:assert.buffer_name('kiview')

    call s:assert.syntax_name('KiviewNode')
    call search('autoload\/')
    call s:assert.syntax_name('KiviewNode')
endfunction

function! s:suite.do_parent_child()
    let cwd = getcwd()
    cd ./test/plugin

    call s:sync_main('')

    let lines = s:lines()
    call s:assert.not_empty(lines)
    call s:assert.contains(lines, 'kiview.vim')
    call s:assert.not_contains(lines, '')
    call s:assert.filetype('kiview')
    call s:assert.dir(cwd . '/test/plugin')

    call s:sync_main('parent')

    let test_lines = s:lines()
    call s:assert.not_empty(test_lines)
    call s:assert.equals(lines[0], '..')
    call s:assert.contains(test_lines, 'plugin/')
    call s:assert.not_contains(test_lines, '')
    call s:assert.filetype('kiview')
    call s:assert.false(&modifiable)
    call s:assert.dir(cwd . '/test')

    call s:sync_main('parent')

    let lines = s:lines()
    call s:assert.not_empty(lines)
    call s:assert.contains(lines, 'autoload/')
    call s:assert.not_contains(lines, '')
    call s:assert.filetype('kiview')
    call s:assert.dir(cwd)

    call search('test/')
    call s:sync_main('child')

    let lines = s:lines()
    call s:assert.not_empty(lines)
    call s:assert.equals(lines[0], '..')
    call s:assert.contains(test_lines, 'plugin/')
    call s:assert.not_contains(test_lines, '')
    call s:assert.filetype('kiview')
    call s:assert.lines(test_lines)

    call search('\.themisrc')
    call s:sync_main('child')

    call s:assert.file_name('.themisrc')
    call s:assert.filetype('vim')
endfunction

function! s:suite.quit()
    call s:sync_main('')

    call s:assert.filetype('kiview')
    call s:assert.window_count(2)

    call s:sync_main('quit')

    call s:assert.filetype('')
    call s:assert.window_count(1)
endfunction

function! s:suite.quit_option()
    call s:sync_main('')

    call search('Makefile')
    call s:sync_main('child -quit')

    call s:assert.file_name('Makefile')
    call s:assert.window_count(1)
endfunction

function! s:suite.tab_open()
    call s:sync_main('')

    call search('Makefile')
    call s:sync_main('child -layout=tab')

    call s:assert.file_name('Makefile')
    call s:assert.tab_count(2)
endfunction

function! s:suite.vertical_open()
    call s:sync_main('')

    call search('Makefile')
    call s:sync_main('child -layout=vertical')

    call s:assert.file_name('Makefile')
    call s:assert.window_count(3)

    wincmd h
    call s:assert.filetype('kiview')
endfunction

function! s:suite.horizontal_open()
    call s:sync_main('')

    call search('Makefile')
    call s:sync_main('child -layout=horizontal')

    call s:assert.file_name('Makefile')
    call s:assert.window_count(3)

    wincmd h
    call s:assert.filetype('kiview')
endfunction

function! s:suite.history()
    cd ./src

    call s:sync_main('')

    call search('src')
    call s:sync_main('child')

    let lines = s:lines()
    call s:assert.contains(lines, 'repository/')

    call search('repository')
    call s:sync_main('child')

    call s:sync_main('parent')
    call s:sync_main('parent')
    call s:sync_main('parent')

    call s:sync_main('child')
    call s:sync_main('child')

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

    call s:sync_main('')

    call s:sync_main('parent')
endfunction

function! s:suite.range()
    cd ./src

    call s:sync_main('')

    let line = search('Cargo\.toml')
    let command = kiview#main([line, line + 1], 'child -layout=tab')
    call command.wait()

    call s:assert.tab_count(3)
endfunction

function! s:suite.parent_marker()
    cd ./src

    call s:sync_main('')

    normal! gg
    call s:sync_main('child')

    let lines = s:lines()
    call s:assert.contains(lines, 'autoload/')
    call s:assert.line_number(2)
endfunction

function! s:suite.go()
    call s:sync_main('')
    call s:sync_main('go -path=./autoload')

    let lines = s:lines()
    call s:assert.contains(lines, 'kiview/')
endfunction

function! s:suite.__new__() abort
    let suite = themis#suite('kiview.plugin.new')

    function! suite.before_each()
        call mkdir('./test/plugin/_test_data/tree', 'p')
        call system(['touch', './test/plugin/_test_data/tree/file_in_tree'])
        call mkdir('./test/plugin/_test_data/tree2', 'p')

        call KiviewTestBeforeEach()
    endfunction

    function! suite.after_each()
        call KiviewTestAfterEach()

        call delete('./test/plugin/_test_data/tree', 'rf')
        call delete('./test/plugin/_test_data/tree2', 'rf')
        call delete('./test/plugin/_test_data/new', 'rf')
        call delete('./test/plugin/_test_data/new_file', 'rf')
    endfunction

    function! suite.new()
        cd ./test/plugin/_test_data

        let input_reader = s:input_reader('new/')

        call s:sync_main('')
        call s:sync_main('new')

        call search('new\/')
        call s:sync_main('child')

        let input_reader = s:input_reader('new_file')

        call s:sync_main('new')

        let lines = s:lines()
        call s:assert.contains(lines, 'new_file')

        call search('new_file')
        call s:sync_main('child')

        call s:assert.file_name('new_file')
    endfunction

    function! suite.cancel_new()
        let input_reader = s:input_reader('')

        call s:sync_main('')
        call s:sync_main('new')

        let lines = s:lines()
        call s:assert.contains(lines, 'autoload/')
    endfunction

    function! suite.new_in_tree()
        cd ./test/plugin/_test_data

        call s:sync_main('')

        call search('tree/')
        call s:sync_main('toggle_tree')
        call search('file_in_tree')

        let input_reader = s:input_reader('new_in_tree')
        call s:sync_main('new')

        let lines = s:lines()
        call s:assert.contains(lines, '  new_in_tree')
        call s:assert.contains(lines, 'tree2/')
        call s:assert.count_contains(lines, '  file_in_tree', 1)
    endfunction

endfunction

function! s:suite.__remove__() abort
    let suite = themis#suite('plugin.kiview.remove')

    function! suite.before_each()
        call system(['touch', './test/plugin/_test_data/removed_file1'])
        call system(['touch', './test/plugin/_test_data/removed_file2'])
        call system(['touch', './test/plugin/_test_data/removed_cancel_file'])

        call mkdir('./test/plugin/_test_data/removed_dir', 'p')
        call system(['touch', './test/plugin/_test_data/removed_dir/file'])

        call KiviewTestBeforeEach()
    endfunction

    function! suite.after_each()
        call KiviewTestAfterEach()

        call delete('./test/plugin/_test_data/removed_dir', 'rf')
        call delete('./test/plugin/_test_data/removed_file1')
        call delete('./test/plugin/_test_data/removed_file2')

        call delete('./test/plugin/_test_data/new', 'rf')
        call delete('./test/plugin/_test_data/removed_cancel_file')
    endfunction

    function! suite.remove()
        cd ./test/plugin/_test_data

        let input_reader = s:input_reader('y')

        call s:sync_main('')

        let first_line = search('removed_file1')
        let last_line = search('removed_file2')
        let command = kiview#main([first_line, last_line], 'remove')
        call command.wait()

        let lines = s:lines()
        call s:assert.not_contains(lines, 'removed_file1')
        call s:assert.not_contains(lines, 'removed_file2')

        call search('removed_dir\/')
        call s:sync_main('remove')

        let lines = s:lines()
        call s:assert.not_contains(lines, 'removed_dir/')
    endfunction

    function! suite.cancel_remove()
        cd ./test/plugin/_test_data

        let input_reader = s:input_reader('')

        call s:sync_main('')

        call search('removed_cancel_file')
        call s:sync_main('remove')

        let lines = s:lines()
        call s:assert.contains(lines, 'removed_cancel_file')
    endfunction

    function! suite.no_remove()
        cd ./test/plugin/_test_data

        let input_reader = s:input_reader('n')

        call s:sync_main('')

        call search('removed_cancel_file')
        call s:sync_main('remove')

        let lines = s:lines()
        call s:assert.contains(lines, 'removed_cancel_file')
    endfunction

endfunction

function! s:suite.__copy_cut_paste__() abort
    let suite = themis#suite('plugin.kiview.copy_cut_paste')

    function! suite.before_each()
        call system(['touch', './test/plugin/_test_data/copy_file'])
        call system(['touch', './test/plugin/_test_data/cut_file'])
        call mkdir('./test/plugin/_test_data/paste', 'p')
        call mkdir('./test/plugin/_test_data/tree', 'p')
        call system(['touch', './test/plugin/_test_data/tree/file_in_tree'])
        call KiviewTestBeforeEach()
    endfunction

    function! suite.after_each()
        call KiviewTestAfterEach()
        call delete('./test/plugin/_test_data/copy_file')
        call delete('./test/plugin/_test_data/cut_file')
        call delete('./test/plugin/_test_data/paste', 'rf')
        call delete('./test/plugin/_test_data/tree', 'rf')
    endfunction

    function! suite.copy_and_paste()
        call s:sync_main('go -path=test/plugin/_test_data')

        call search('copy_file')
        call s:sync_main('cut')
        call s:sync_main('copy') " copy disables cut

        call search('paste\/')
        call s:sync_main('child')

        call s:sync_main('paste')

        let lines = s:lines()
        call s:assert.contains(lines, 'copy_file')

        call search('copy_file')
        call s:sync_main('child')

        call s:assert.file_name('copy_file')

        wincmd p
        call s:sync_main('parent')

        let lines = s:lines()
        call s:assert.contains(lines, 'copy_file')
    endfunction

    function! suite.cut_and_paste()
        call s:sync_main('go -path=test/plugin/_test_data')

        call search('cut_file')
        call s:sync_main('cut')

        call search('paste\/')
        call s:sync_main('child')

        call s:sync_main('paste')

        let lines = s:lines()
        call s:assert.contains(lines, 'cut_file')

        call search('cut_file')
        call s:sync_main('child')

        call s:assert.file_name('cut_file')

        wincmd p
        call s:sync_main('parent')

        let lines = s:lines()
        call s:assert.not_contains(lines, 'cut_file')
    endfunction

    function! suite.paste_in_tree()
        call s:sync_main('go -path=test/plugin/_test_data')

        call search('copy_file')
        call s:sync_main('cut')
        call s:sync_main('copy') " copy disables cut

        call search('tree/')
        call s:sync_main('toggle_tree')

        call search('file_in_tree')
        call s:sync_main('paste')

        let lines = s:lines()
        call s:assert.contains(lines, '  copy_file')
        call s:assert.contains(lines, 'copy_file')
    endfunction

endfunction

function! s:suite.__rename__() abort
    let suite = themis#suite('plugin.kiview.rename')

    function! suite.before_each()
        call system(['touch', './test/plugin/_test_data/rename_file'])
        call KiviewTestBeforeEach()
    endfunction

    function! suite.after_each()
        call KiviewTestAfterEach()
        call delete('./test/plugin/_test_data/rename_file')
        call delete('./test/plugin/_test_data/renamed_file')
    endfunction

    function! suite.rename()
        cd ./test/plugin/_test_data

        let input_reader = s:input_reader('renamed_file')

        call s:sync_main('')

        call search('rename_file')
        call s:sync_main('rename')

        let lines = s:lines()
        call s:assert.contains(lines, 'renamed_file')
        call s:assert.not_contains(lines, 'rename_file')
    endfunction

endfunction

function! s:suite.go_error()
    call s:sync_main('')

    let f = {'called': ''}
    function! f.echo(message) abort
        call kiview#logger#new('output').log(': ' . a:message)
        let self.called = a:message
    endfunction
    call kiview#messenger#set_func({ msg -> f.echo(msg) })

    call s:sync_main('go -path=./not_found')
    call s:assert.not_empty(f.called)
endfunction

function! s:suite.unknown_command()
    let f = {'called': ''}
    function! f.echo(message) abort
        call kiview#logger#new('output').log(': ' . a:message)
        let self.called = a:message
    endfunction
    call kiview#messenger#set_func({ msg -> f.echo(msg) })

    call s:sync_main('invalid_command_name')
    call s:assert.not_empty(f.called)
endfunction

function! s:suite.toggle_tree()
    call s:sync_main('')

    call search('autoload/')

    let lines = s:lines()
    call s:sync_main('toggle_tree')
    call s:sync_main('toggle_tree')
    call s:assert.lines(lines)

    call s:sync_main('toggle_tree')

    call search('kiview.vim')
    call s:sync_main('child')

    call s:assert.file_name('kiview.vim')
endfunction

function! s:suite.cannot_toggle_parent_node()
    call s:sync_main('')

    let lines = s:lines()

    normal! gg
    call s:sync_main('toggle_tree')

    call s:assert.lines(lines)
endfunction

function! s:suite.toggle_multi_trees()
    call s:sync_main('')

    call search('autoload\/')
    call s:sync_main('toggle_tree')

    call search('plugin\/')
    call s:sync_main('toggle_tree')

    call search('kiview\.vim')
    call s:sync_main('child')

    call s:assert.path('plugin/kiview.vim')
endfunction

function! s:suite.toggle_last_dir()
    call s:sync_main('go -path=./test/plugin/_test_data/depth0')

    let lines = s:lines()

    call search('depth1\/')
    call s:sync_main('toggle_tree')
    call s:sync_main('toggle_tree')

    call s:assert.lines(lines)
endfunction

function! s:suite.toggle_single_dir()
    call s:sync_main('go -path=./test/plugin/_test_data')

    call search('depth0\/')
    call s:sync_main('toggle_tree')

    let lines = s:lines()
    call search('depth1\/')
    call s:sync_main('toggle_tree')
    call s:sync_main('toggle_tree')

    call s:assert.lines(lines)
endfunction

function! s:suite.open_root()
    call s:sync_main('go -path=/')
    let lines = s:lines()

    normal! 2j
    let line_number = line('.')

    call s:sync_main('parent')

    call s:assert.lines(lines)
    call s:assert.line_number(line_number)
endfunction

function! s:suite.toggle_selection()
    call s:sync_main('')

    call search('src\/')
    call s:sync_main('child')

    call search('\.gitignore')
    call s:sync_main('toggle_selection')

    call search('Cargo\.toml')
    call s:sync_main('toggle_selection')

    call s:sync_main('child -layout=tab')

    call s:assert.tab_count(3)
    call s:assert.file_name('Cargo.toml')
endfunction

function! s:suite.vertical_rightbelow()
    call s:sync_main('-split=vertical:rightbelow')

    call s:assert.filetype('kiview')

    wincmd h
    call s:assert.filetype('')
    call s:assert.window_count(2)
endfunction

function! s:suite.split_horizontal_leftabove()
    call s:sync_main('-split=horizontal:leftabove')

    call s:assert.filetype('kiview')

    wincmd j
    call s:assert.filetype('')
    call s:assert.window_count(2)
endfunction

function! s:suite.split_horizontal_rightbelow()
    call s:sync_main('-split=horizontal:rightbelow')

    call s:assert.filetype('kiview')

    wincmd k
    call s:assert.filetype('')
    call s:assert.window_count(2)
endfunction

function! s:suite.split_tab()
    call s:sync_main('-split=tab')

    call s:assert.filetype('kiview')
    call s:assert.window_count(1)
    call s:assert.tab_count(2)
endfunction

function! s:suite.no_split()
    call s:sync_main('-split=no')

    call s:assert.filetype('kiview')
    call s:assert.window_count(1)
endfunction

function! s:suite.clear_selection_on_error()
    call s:sync_main('go -path=./src')

    call search('\.gitignore')
    call s:sync_main('toggle_selection')

    call s:sync_main('go -path=./not_found')

    call search('Makefile')
    call s:sync_main('child')

    call s:assert.file_name('Makefile')
endfunction

function! s:suite.open_same_path()
    call s:sync_main('')

    let line_number = line('.')
    call s:sync_main('')

    call s:assert.line_number(line_number)

    normal! 2j
    let line_number = line('.')
    call s:sync_main('')

    call s:assert.line_number(line_number)
endfunction

function! s:suite.tab_open_group_node()
    let cwd = getcwd()

    call s:sync_main('')
    call search('test\/')
    call s:sync_main('toggle_selection')
    call search('Makefile')
    call s:sync_main('toggle_selection')
    call s:sync_main('child -layout=tab')

    call s:assert.tab_count(3)

    call s:assert.file_name('Makefile')

    tabprevious
    call s:assert.filetype('kiview')
    call s:assert.dir(cwd . '/test')

    call search('plugin\/')
    call s:sync_main('child')

    call s:assert.dir(cwd . '/test/plugin')

    tabprevious
    call s:assert.dir(cwd)
endfunction
