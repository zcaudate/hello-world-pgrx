# hello-world-pgrx

This project follows instructions on https://github.com/pgcentralfoundation/pgrx/blob/develop/cargo-pgrx/README.md for generating a sample extension and testing it on postgres. 


Usage:


```
select hello_hello_world_pgrx('hello');
=>  1c8aff950685c2ed4bc3174f3472287b56d9517b9c948127319a09a7a36deac8

select hello_hello_world_pgrx('hello worldoooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo');
=> e534b0e4446bbccddf7d35facd71686dd2594e52c89f4609156be762d793102a
```    
