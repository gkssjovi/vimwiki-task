# Description

Create the files structure for new tasks when using vimwiki.

# Install 

```sh
cargo build --release

ln -s $PWD/target/release/vimwiki-task /usr/local/bin/vimwiki-task
ln -s $PWD/config.yaml ~/.config/vimwiki-task/config.yaml
ln -s $PWD/template.md ~/.config/vimwiki-task/template.md
ln -s $PWD/doc_template.md ~/.config/vimwiki-task/doc_template.md
```

# Help
```sh
vimwiki-task --help
```

# Usage
```sh
# The override ture is for test only
vimwiki-task ~/vimwiki/src/tasks/index.md --line-number 0 --override true
# Inset on the current line number
vimwiki-task ~/vimwiki/src/tasks/index.md --line-number 10 --override false
```

# Vim implementation
`vim ~/.config/nvim/init.vim`

```vim
function! VimwikiCreateNewTask()
  let extension = expand('%:e')
  if extension == 'md'
    :echo  extension
    let file_name = expand('%:p')
    let line_number = line(".")
    :echo system(printf('vimwiki-task %s --line-number %s --override false', file_name, line_number))
  endif
endfunction

command! VimwikiCreateNewTask :call VimwikiCreateNewTask()
nmap <leader>wt :VimwikiCreateNewTask<CR><CR>
```

# Templates variables

`{title}` \
`{branch_name}` \
`{date_time}` \
`{date}` \
`{target_name}` \
`{docs_dir_name}` \
`{assets_dir_name}` 
