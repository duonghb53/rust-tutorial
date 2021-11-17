Trong bài này thì có 1 số khái niệm cần lưu ý:

I. Variable & Mutability
1. Khai báo biến dạng immutable(không thể thay đổi):
- let variable_name: type_data = ... thì sau khi khai báo không thể thay đổi giá trị của biến
- let mut variable_name: type_data = ... thì có thể thay đổi giá trị của biến

2. Shadowing: 
- Nghĩa là khi khai báo 1 biến
let x = 5;
let x = x + 1;
Thì giá trị x của biến đầu sẽ bị biến thứ 2 "che đi" hay đè lên

Shadowing khác với mut: mut cho phép thay đổi giá trị của biến đó nhưng shadowing thì tạo ra 1 biến mới và xoá vùng nhớ biến cũ đi.
Trong các trường hợp làm việc với dữ liệu lớn struct, array... thì mut sẽ cho tốc độ nhanh hơn vì nó gán trực tiếp giá trị chứ không tạo ra 1 giá trị mới như shadowing

