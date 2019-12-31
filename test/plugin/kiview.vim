
let s:helper = KiviewTestHelper('plugin.kiview')
let s:suite = s:helper.suite()
let s:assert = s:helper.assert()

function! s:main(arg) abort
    let line = line('.')
    return kiview#main([line, line], a:arg, bufnr('%'))
endfunction

function! s:sync_main(arg) abort
    let command = s:main(a:arg)
    call command.wait()
    return command
endfunction

function! s:suite.create_one()
    let cwd = getcwd()

    call s:sync_main('')

    let lines = s:helper.lines()
    call s:assert.not_empty(lines)
    call s:assert.equals(lines[0], '..')
    call s:assert.contains(lines, 'autoload/')
    call s:assert.not_contains(lines, '')
    call s:assert.filetype('kiview')
    call s:assert.false(&modifiable)
    call s:assert.line_number(2)
    call s:assert.working_dir(cwd)
    call s:assert.buffer_name('kiview')

    normal! gg
    call s:assert.syntax_name('KiviewNodeClosed')

    call search('autoload\/')
    call s:assert.syntax_name('KiviewNodeClosed')
    call s:assert.ends_with(kiview#get().path, 'autoload')
endfunction

function! s:suite.multiple_create()
    call s:sync_main('')
    call s:sync_main('') " nop
    call s:sync_main('-create')

    call s:assert.window_count(3)
endfunction

function! s:suite.do_parent_child()
    let cwd = getcwd()
    cd ./test/plugin

    call s:sync_main('')

    let lines = s:helper.lines()
    call s:assert.not_empty(lines)
    call s:assert.contains(lines, 'kiview.vim')
    call s:assert.not_contains(lines, '')
    call s:assert.filetype('kiview')
    call s:assert.working_dir(cwd . '/test/plugin')

    call s:sync_main('parent')

    let test_lines = s:helper.lines()
    call s:assert.not_empty(test_lines)
    call s:assert.equals(lines[0], '..')
    call s:assert.contains(test_lines, 'plugin/')
    call s:assert.not_contains(test_lines, '')
    call s:assert.filetype('kiview')
    call s:assert.false(&modifiable)
    call s:assert.working_dir(cwd . '/test')

    call s:sync_main('parent')

    let lines = s:helper.lines()
    call s:assert.not_empty(lines)
    call s:assert.contains(lines, 'autoload/')
    call s:assert.not_contains(lines, '')
    call s:assert.filetype('kiview')
    call s:assert.working_dir(cwd)

    call search('test/')
    call s:sync_main('child')

    let lines = s:helper.lines()
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

    let lines = s:helper.lines()
    call s:assert.contains(lines, 'repository/')

    call search('repository')
    call s:sync_main('child')

    call s:sync_main('parent')
    call s:sync_main('parent')
    call s:sync_main('parent')

    call s:sync_main('child')
    call s:sync_main('child')

    let lines = s:helper.lines()
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
    let command = kiview#main([line, line + 1], 'child -layout=tab', bufnr('%'))
    call command.wait()

    call s:assert.tab_count(3)
endfunction

function! s:suite.parent_marker()
    cd ./src

    call s:sync_main('')

    normal! gg
    call s:sync_main('child')

    let lines = s:helper.lines()
    call s:assert.contains(lines, 'autoload/')
    call s:assert.line_number(2)
endfunction

function! s:suite.go()
    call s:sync_main('')
    call s:sync_main('go -path=./autoload')

    let lines = s:helper.lines()
    call s:assert.contains(lines, 'kiview/')
endfunction

function! s:suite.__new__() abort
    let suite = s:helper.sub_suite('new')

    function! suite.before_each()
        call s:helper.before_each()

        call s:helper.new_directory('tree')
        call s:helper.new_file('tree/file_in_tree')
        call s:helper.new_directory('tree2')

        call s:helper.new_file_with_content('already', ['has contents'])
    endfunction

    function! suite.after_each()
        call s:helper.after_each()
    endfunction

    function! suite.new_one()
        let cwd = getcwd()
        cd ./test/plugin/_test_data

        call s:helper.set_input('new/')

        call s:sync_main('')
        call s:sync_main('new')

        call search('new\/')
        call s:sync_main('child')
        call s:assert.working_dir(cwd . '/test/plugin/_test_data/new')

        call s:helper.set_input('new_file')

        call s:sync_main('new')

        let lines = s:helper.lines()
        call s:assert.contains(lines, 'new_file')

        call search('new_file')
        call s:sync_main('child')

        call s:assert.file_name('new_file')
    endfunction

    function! suite.cancel_new()
        call s:helper.set_input('')

        call s:sync_main('')
        call s:sync_main('new')

        let lines = s:helper.lines()
        call s:assert.contains(lines, 'autoload/')
    endfunction

    function! suite.new_already_exists()
        cd ./test/plugin/_test_data

        call s:helper.set_input('already')

        call s:sync_main('')
        call s:sync_main('new')

        call s:assert.file_not_empty('already')
    endfunction

    function! suite.new_in_tree()
        cd ./test/plugin/_test_data

        call s:sync_main('')

        call search('tree/')
        call s:sync_main('toggle_tree')
        call search('file_in_tree')

        call s:helper.set_input('new_in_tree')
        call s:sync_main('new')

        let lines = s:helper.lines()
        call s:assert.contains(lines, '  new_in_tree')
        call s:assert.contains(lines, 'tree2/')
        call s:assert.count_contains(lines, '  file_in_tree', 1)
    endfunction

    function! suite.new_multiple()
        let cwd = getcwd()
        cd ./test/plugin/_test_data
        call s:helper.set_input('new_file1 new_file2')

        call s:sync_main('')
        call s:sync_main('new')

        let lines = s:helper.lines()
        call s:assert.contains(lines, 'new_file1')
        call s:assert.contains(lines, 'new_file2')
    endfunction

endfunction

function! s:suite.__remove__() abort
    let suite = s:helper.sub_suite('remove')

    function! suite.before_each()
        call s:helper.before_each()

        call s:helper.new_file('removed_file1')
        call s:helper.new_file('removed_file2')
        call s:helper.new_file('removed_cancel_file')

        call s:helper.new_directory('tree')
        call s:helper.new_file('tree/file_in_tree1')
        call s:helper.new_file('tree/file_in_tree2')

        call s:helper.new_directory('tree2')
        call s:helper.new_file('tree2/file_in_tree')

        call s:helper.new_directory('removed_dir')
        call s:helper.new_file('removed_dir/file')
    endfunction

    function! suite.after_each()
        call s:helper.after_each()
    endfunction

    function! suite.remove_one()
        cd ./test/plugin/_test_data
        call s:helper.set_input('y', 'y')

        call s:sync_main('')

        let first_line = search('removed_file1')
        let last_line = search('removed_file2')
        let command = kiview#main([first_line, last_line], 'remove', bufnr('%'))
        call command.wait()

        let lines = s:helper.lines()
        call s:assert.not_contains(lines, 'removed_file1')
        call s:assert.not_contains(lines, 'removed_file2')

        call search('removed_dir\/')
        call s:sync_main('remove')

        let lines = s:helper.lines()
        call s:assert.not_contains(lines, 'removed_dir/')
    endfunction

    function! suite.remove_in_tree()
        cd ./test/plugin/_test_data
        call s:helper.set_input('y')

        call s:sync_main('')

        call search('tree\/')
        call s:sync_main('toggle_tree')
        call search('file_in_tree1')
        call s:sync_main('toggle_selection')

        call s:sync_main('remove')

        let lines = s:helper.lines()
        call s:assert.not_contains(lines, '  file_in_tree1')
        call s:assert.contains(lines, '  file_in_tree2')
    endfunction

    function! suite.remove_parent_and_child()
        cd ./test/plugin/_test_data
        call s:helper.set_input('y')

        call s:sync_main('')

        call search('tree\/')
        call s:sync_main('toggle_tree')
        call search('tree2\/')
        call s:sync_main('toggle_tree')

        call search('tree\/')
        call s:sync_main('toggle_selection')
        call search('file_in_tree1')
        call s:sync_main('toggle_selection')
        call search('file_in_tree$')
        call s:sync_main('toggle_selection')

        call s:sync_main('remove')

        let lines = s:helper.lines()
        call s:assert.not_contains(lines, '  file_in_tree1')
        call s:assert.not_contains(lines, '  file_in_tree2')
        call s:assert.not_contains(lines, 'tree/')
        call s:assert.not_contains(lines, '  file_in_tree')
    endfunction

    function! suite.cancel_remove()
        cd ./test/plugin/_test_data
        call s:helper.set_input('')

        call s:sync_main('')

        call search('removed_cancel_file')
        call s:sync_main('remove')

        let lines = s:helper.lines()
        call s:assert.contains(lines, 'removed_cancel_file')
    endfunction

    function! suite.no_remove()
        cd ./test/plugin/_test_data
        call s:helper.set_input('n')

        call s:sync_main('')

        call search('removed_cancel_file')
        call s:sync_main('remove')

        let lines = s:helper.lines()
        call s:assert.contains(lines, 'removed_cancel_file')
    endfunction

endfunction

function! s:suite.__copy_cut_paste__() abort
    let suite = s:helper.sub_suite('copy_cut_paste')

    function! suite.before_each()
        call s:helper.before_each()

        call s:helper.new_file('copy_file')
        call s:helper.new_file('cut_file')
        call s:helper.new_directory('paste')
        call s:helper.new_directory('tree')
        call s:helper.new_file('tree/file_in_tree')

        call s:helper.new_file_with_content('already', ['has contents'])

        call s:helper.new_directory('has_already')
        call s:helper.new_file('has_already/already')
    endfunction

    function! suite.after_each()
        call s:helper.after_each()
    endfunction

    function! suite.copy_and_paste()
        call s:sync_main('go -path=test/plugin/_test_data')

        let messenger = s:helper.messenger()

        call search('copy_file')
        call s:sync_main('cut')
        call s:sync_main('copy') " copy disables cut

        call s:assert.contains(messenger.msg, 'Copied')
        call s:assert.contains(messenger.msg, 'test/plugin/_test_data/copy_file')

        call search('paste\/')
        call s:sync_main('child')

        call s:sync_main('paste')

        let lines = s:helper.lines()
        call s:assert.contains(lines, 'copy_file')

        call search('copy_file')
        call s:sync_main('child')

        call s:assert.file_name('copy_file')

        wincmd p
        call s:sync_main('parent')

        let lines = s:helper.lines()
        call s:assert.contains(lines, 'copy_file')
    endfunction

    function! suite.cut_and_paste()
        call s:sync_main('go -path=test/plugin/_test_data')

        let messenger = s:helper.messenger()

        call search('cut_file')
        call s:sync_main('cut')

        call s:assert.contains(messenger.msg, 'Cut')
        call s:assert.contains(messenger.msg, 'test/plugin/_test_data/cut_file')

        call search('paste\/')
        call s:sync_main('child')

        call s:sync_main('paste')

        let lines = s:helper.lines()
        call s:assert.contains(lines, 'cut_file')

        call search('cut_file')
        call s:sync_main('child')

        call s:assert.file_name('cut_file')

        wincmd p
        call s:sync_main('parent')

        let lines = s:helper.lines()
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

        let lines = s:helper.lines()
        call s:assert.contains(lines, '  copy_file')
        call s:assert.contains(lines, 'copy_file')
    endfunction

    function! suite.copy_and_renamed_paste()
        call s:sync_main('go -path=test/plugin/_test_data')

        call search('copy_file')
        call s:sync_main('copy')

        call s:helper.set_input('r', 'renamed_copy_file')
        call s:sync_main('paste')

        let lines = s:helper.lines()
        call s:assert.contains(lines, 'copy_file')
        call s:assert.contains(lines, 'renamed_copy_file')
    endfunction

    function! suite.cancel_renamed_paste()
        call s:sync_main('go -path=test/plugin/_test_data')
        let test_lines = s:helper.lines()

        call search('copy_file')
        call s:sync_main('copy')

        call s:helper.set_input('r', '')
        call s:sync_main('paste')

        call s:assert.lines(test_lines)
    endfunction

    function! suite.cancel_paste_on_already_exists()
        call s:sync_main('go -path=test/plugin/_test_data/has_already')

        call search('already')
        call s:sync_main('copy')
        call s:sync_main('parent')

        call s:helper.set_input('n')
        call s:sync_main('paste')

        let lines = s:helper.lines()
        call s:assert.contains(lines, 'already')
        call s:assert.file_not_empty('already')
    endfunction

    function! suite.force_paste_on_already_exists()
        call s:sync_main('go -path=test/plugin/_test_data/has_already')

        call search('already')
        call s:sync_main('copy')
        call s:sync_main('parent')

        call s:helper.set_input('f')
        call s:sync_main('paste')

        let lines = s:helper.lines()
        call s:assert.contains(lines, 'already')
        call s:assert.file_empty('already')
    endfunction

    function! suite.share_clipboard()
        let cwd = getcwd()

        call s:sync_main('go -path=test/plugin/_test_data')

        call search('copy_file')
        call s:sync_main('copy')

        call s:sync_main('-create')
        call search('paste\/')
        call s:sync_main('child')

        call s:sync_main('paste')

        let lines = s:helper.lines()
        call s:assert.working_dir(cwd . '/test/plugin/_test_data/paste')
        call s:assert.contains(lines, 'copy_file')

        wincmd p
        let lines = s:helper.lines()
        call s:assert.working_dir(cwd . '/test/plugin/_test_data')
        call s:assert.contains(lines, 'copy_file')
    endfunction

endfunction

function! s:suite.__rename__() abort
    let suite = s:helper.sub_suite('rename')

    function! suite.before_each()
        call s:helper.before_each()

        call s:helper.new_file('rename_file')

        call s:helper.new_file_with_content('already', ['has contents'])

        call s:helper.new_directory('tree')
        call s:helper.new_file('tree/file_in_tree')
    endfunction

    function! suite.after_each()
        call s:helper.after_each()
    endfunction

    function! suite.rename_one()
        cd ./test/plugin/_test_data

        call s:helper.set_input('renamed_file')

        call s:sync_main('')

        call search('rename_file')
        call s:sync_main('rename')

        let lines = s:helper.lines()
        call s:assert.contains(lines, 'renamed_file')
        call s:assert.not_contains(lines, 'rename_file')
    endfunction

    function! suite.rename_in_tree()
        cd ./test/plugin/_test_data

        call s:helper.set_input('renamed_file')

        call s:sync_main('')

        call search('tree/')
        call s:sync_main('toggle_tree')

        call search('file_in_tree')
        call s:sync_main('rename')

        let lines = s:helper.lines()
        call s:assert.contains(lines, '  renamed_file')
        call s:assert.not_contains(lines, '  rename_file')
    endfunction

    function! suite.rename_already_exists()
        cd ./test/plugin/_test_data

        call s:sync_main('')

        call search('rename_file')
        call s:helper.set_input('already')
        call s:sync_main('rename')

        call s:assert.file_not_empty('already')
    endfunction

    function! suite.multiple_rename_one()
        cd ./test/plugin/_test_data

        call s:sync_main('')

        call search('rename_file')
        call s:sync_main('multiple_rename')
        call s:assert.modified(v:false)

        let lines = s:helper.lines()
        call s:assert.contains(lines, 'rename_file')

        call setbufline('%', 2, 'renamed_file')

        write
        let command = kiview#last_command()
        call command.wait(1000)

        call s:assert.modified(v:false)
        call s:assert.window_count(3)

        quit
        call s:sync_main('')
        let lines = s:helper.lines()
        call s:assert.contains(lines, 'renamed_file')
        call s:assert.not_contains(lines, 'rename_file')
    endfunction

    function! suite.multiple_rename_already_exists()
        cd ./test/plugin/_test_data

        call s:sync_main('')

        call search('rename_file')
        call s:sync_main('toggle_selection')
        call search('already')
        call s:sync_main('toggle_selection')
        call s:sync_main('multiple_rename')

        call setbufline('%', 2, 'renamed_file')
        call setbufline('%', 3, 'renamed_file')

        write
        let command = kiview#last_command()
        call command.wait(1000)

        call s:assert.window_count(3)
        call s:assert.modified(v:true)
        q!
    endfunction

    function! suite.multiple_rename_twice()
        cd ./test/plugin/_test_data

        call s:sync_main('')

        call search('rename_file')
        call s:sync_main('multiple_rename')

        call setbufline('%', 2, 'renamed_file')

        write
        let command = kiview#last_command()
        call command.wait(1000)

        call s:assert.modified(v:false)
        call s:assert.window_count(3)

        let lines = s:helper.lines()
        call s:assert.contains(lines, 'renamed_file')

        call search('renamed_file')
        call setbufline('%', 2, 'twice_renamed_file')

        write
        let command = kiview#last_command()
        call command.wait(1000)
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

    let lines = s:helper.lines()
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

    let lines = s:helper.lines()

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

function! s:suite.toggle_parent_and_child()
    call s:sync_main('')

    call search('autoload\/')
    call s:sync_main('toggle_selection')
    call s:sync_main('toggle_tree')

    call search('kiview\/')
    call s:sync_main('toggle_selection')

    call s:sync_main('toggle_tree')
endfunction

function! s:suite.__toggle_deep__() abort
    let suite = s:helper.sub_suite('toggle_deep')

    function! suite.before_each()
        call s:helper.before_each()

        call s:helper.new_directory('depth0')
        call s:helper.new_directory('depth0/depth1')
    endfunction

    function! suite.after_each()
        call s:helper.after_each()
    endfunction

    function! suite.toggle_last_dir()
        call s:sync_main('go -path=./test/plugin/_test_data/depth0')

        let lines = s:helper.lines()

        call search('depth1\/')
        call s:sync_main('toggle_tree')
        call s:sync_main('toggle_tree')

        call s:assert.lines(lines)
    endfunction

    function! suite.toggle_single_dir()
        call s:sync_main('go -path=./test/plugin/_test_data')

        call search('depth0\/')
        call s:sync_main('toggle_tree')

        let lines = s:helper.lines()
        call search('depth1\/')
        call s:sync_main('toggle_tree')
        call s:sync_main('toggle_tree')

        call s:assert.lines(lines)
    endfunction

endfunction

function! s:suite.open_root()
    call s:sync_main('go -path=/')
    let lines = s:helper.lines()

    normal! 2j
    let line_number = line('.')

    call s:sync_main('parent')

    call s:assert.lines(lines)
    call s:assert.line_number(line_number)
endfunction

function! s:suite.toggle_selection()
    call s:sync_main('go -path=src')

    call search('\.gitignore')
    call s:sync_main('toggle_selection')

    call search('Cargo\.toml')
    call s:sync_main('toggle_selection')

    call s:sync_main('child -layout=tab')

    call s:assert.tab_count(3)
    call s:assert.file_name('Cargo.toml')
endfunction

function! s:suite.toggle_all_selection()
    call s:sync_main('go -path=src')

    call search('src\/')
    call s:sync_main('toggle_selection')
    call search('target\/')
    call s:sync_main('toggle_selection')

    call s:sync_main('toggle_all_selection')

    call s:sync_main('child -layout=tab')

    call s:assert.tab_count(5)
    call s:assert.file_name('Makefile')

    call s:sync_main('parent')
    call s:sync_main('toggle_all_selection')
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
    call s:assert.working_dir(cwd . '/test')

    call search('plugin\/')
    call s:sync_main('child')

    call s:assert.working_dir(cwd . '/test/plugin')

    tabprevious
    call s:assert.working_dir(cwd)
endfunction
