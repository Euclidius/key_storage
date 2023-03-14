# key_storage
This is minimal product which can work. There are some necessary things, which I need to add, such as cyphering, keygen, etc.

## How to use

Firstly, you need to initiate the storage in your current directory.
```
./key_storage init
```

Adding:
```
./key_storage add <site> <login> <password>
```

Showing:
```
./key_storage show <site>
```
will show you all the logins and passwords from \<site\>

Removing:
```
./key_storage remove <site> <login> <password>
```

## TODO

1. Add an ability to cyphering the passwords (or file)
2. Add an ability to generate secure passwords
3. Add an ability to store not only passwords but hashes, keys, tokens, etc.
4. (maybe) Add a gui
