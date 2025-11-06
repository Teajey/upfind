# Examples

Finding all Helix config files in the current scope:
```
$ upf '.helix/*.toml' '.config/helix/*.toml'
/Users/myself/code/rust/upfind/.helix/languages.toml
/Users/myself/.config/helix/config.toml
/Users/myself/.config/helix/languages.toml
```
Find all .env files in the current scope
```
$ upf .env '.env.*'
/Users/myself/code/my-mono-repo/my-project/.env.dev.local
/Users/myself/code/my-mono-repo/my-project/.env.test
/Users/myself/code/my-mono-repo/.env
/Users/myself/code/my-mono-repo/.env.prod.local
/Users/myself/code/my-mono-repo/.env.test.local
/Users/myself/code/.env
```
Or just the local ones
```
$ upf '.env*.local'
/Users/myself/code/my-mono-repo/my-project/.env.dev.local
/Users/myself/code/my-mono-repo/.env.prod.local
/Users/myself/code/my-mono-repo/.env.test.local
```
