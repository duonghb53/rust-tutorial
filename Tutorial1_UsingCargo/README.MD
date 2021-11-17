Bài 1: Cargo và tạo project đầu tiên
1. cargo new [options] path: 
Câu lệnh sẽ tạo 1 project mới với thư mục giống tên project bao gồm:
- File template Cargo.toml
- Folder src chứa source code là các file .rs(Rust)

[options] là phần tuỳ chọn không bắt buộc phải có. Nếu không có thì tự động tạo 1 project binary

VD1: cargo new hello_cargo
cargo sẽ tự tạo ra 1 project hello_cargo và tạo các file trong folder có tên giống tên project
Trong project bao gồm:
- File Cargo.toml
- Folder src và file src/main.rs bên trong, thư mục này sẽ chưa các file code khi project biên dịch, ở đây mặc định không [options] thì nó sẽ tạo ra 1 project build kiểu bin

VD2: cargo new --lib hello_lib
Trong project bao gồm:
- File Cargo.toml
- Folder src và file src/lib.rs

Project sử dụng --bin sẽ build ra file binary
Project sử dụng --lib sẽ build ra file lib( ,rlib)

Cụ thể các [options]:
--bin: tạo 1 project binary target(src/main.rs), mặc định
--lib: tạo 1 project library target(src/lib.rs)
--edition edition: Chỉ định phiên bản Rust: mặc định hiện tại 2021, có thể sử dụng: 2015, 2018, 2021
--name name: đặt tên package, mặc định là tên thư mục
Ngoài ra còn 1 số các [options] khác nhưng chúng ta chỉ sử dụng chính các [options] trên

2. cargo build [options]: Compile package hiện tại