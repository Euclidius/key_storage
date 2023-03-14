# key_storage
This is minimal product which can work. There are some necessary things, which I need to add, such as cyphering, keygen, etc.

## How to use

```
./key_storage help
```

Firstly, you need to initiate the storage in your current directory.
```
./key_storage init
```

Adding:
```
./key_storage add <site> <login> <password>
```
adds the given line

Showing:
```
./key_storage show <site>
```
will show you all the logins and passwords from \<site\>

Removing:
```
./key_storage remove <site> <login> <password>
```
removes the given line

Generating a password:
```
./key_storage keygen <len>
```
generates the password with given len

## TODO

1. Add an ability to cyphering the passwords (or file)
2. Add an ability to change settings for password generating
3. Add an ability to store not only passwords but hashes, keys, tokens, etc.
4. (maybe) Add a gui
