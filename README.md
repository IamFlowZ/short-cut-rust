just getting my toes wet with rust

you can accomplish the same and source it with two lines of shell. albeit you'll need to tweak the rc file and second line depending on your shell.

```bash
sc() {
  echo "export $1=$(pwd)" >> $HOME/.zshrc
  source "${functions_source[$0]}"
}
```