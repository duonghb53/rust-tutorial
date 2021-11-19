Nội dung:

Ownership là gì?
Đây là 1 phần rất quan trọng của ngôn ngữ Rust, vì khi hiểu được bản chất được Ownership bạn có thể làm rất nhiều điều và hiểu sâu về cách sử dụng trong Rust
Tất cả các chương trình khi chạy đều phải quản lý cách chúng sử dụng bộ nhớ.
- Một vài ngôn ngữ có các bộ gom rác (garbage collection) liên tục tìm kiếm các vùng nhớ lâu không được sử dụng khi chương trình chạy(C#, Java, Python).
- Quản lý thủ công: khi sử dụng chúng ta cần cấp phát và giải phóng chúng(ví dụ: C/C++).
- Rust sử dụng cách tiếp cận thứ 3: quản lý bộ nhớ thông qua ownership tại thời điểm biên dịch.

|                       |            Ưu điểm             |               Nhược điểm                  |        Ngôn ngữ          |
|:---------------------:|:------------------------------:|:-----------------------------------------:|:------------------------:|
|   Bộ gom rác          |   Không quan tâm đến việc quản |   Không thể quản lý bộ nhớ                |  C#, Java, Python..      |
|   (Garbage collectiom)|   lý bộ nhớ                    |   Thời gian biên dịch và chạy lâu hơn     |                          |
|                       |   Không lỗi, giảm thời gian    |   Dung lượng file lớn hơn                 |                          |
|                       |   viết code                    |                                           |                          |
|:---------------------:|:------------------------------:|:-----------------------------------------:|:------------------------:|
|   Quản lý thủ công    |   Có quyển kiếm soát bộ nhớ    |   Gặp lỗi nhiều khi sử dụng               |  C/C++                   |
|   Bộ gom rác          |   Chương trình chạy nhanh hơn  |   Viết code lâu hơn và khó hơn            |                          |
|   (Garbage collectiom)|   Dung lượng file nhỏ          |                                           |                          |
|:---------------------:|:------------------------------:|:-----------------------------------------:|:------------------------:|
|     Onwership Model   |   Có quyển kiếm soát bộ nhớ    |  Viết code lâu                            |  Rust, C version 11      |
|                       |   Chương trình chạy nhanh hơn  |  Học và viết code khác với các ngôn ngữ # |                          |
|                       |   Dung lượng file nhỏ          |                                           |                          |
|                       |   Không lỗi                    |                                           |                          |
|=======================|================================|===========================================|==========================|



Ownership là 1 khái niệm mới với nhiều lập trình viên nên sẽ mất thời gian để hiểu nó, khi bạn sử dụng nó càng thuần thục thì code của bạn càng an toàn và hiệu.
Trong chap này bạn sẽ tìm hiểu cách hoạt động của Ownership và ví dụ về strings(chuỗi)

Nhắc lại 1 chút về Stack và Heap:
- Stack: ngăn xếp theo kiểu LIFO( last in first out: vào sau ra trước): bạn tưởng tượng như 1 chồng đĩa khi bạn xếp các đĩa sau lần lượt lên thì bạn cần lấy đĩa trên cùng ra đầu tiên.
Như vậy chúng ta chỉ có thể truy cập, thêm, xoá ở phần tử cuối cùng.
Kích thước ở stack thì là cố định
- Heap: Kích thước thay đổi, khi bạn sử dụng dữ liệu trên Heap thì cần yêu cầu cho HĐH 1 khoảng memory nhất định.
Ví dụ khi bạn đi vào nhà hàng và muốn đặt bàn 8 chỗ, thì bồi bàn cần kiểm tra và tìm cho bạn 1 bàn có >= 8 chỗ ngồi, trên máy tính cũng vậy bạn cần 1 vùng nhớ 1MB thì máy tính sẽ kiểm tra xem có vùng nhớ nào đang free >= 1MB sẽ cấp phát cho bạn.
Khi Push dữ liệu vào stack thì tốc độ nhanh hơn vì nó push lên trên cùng còn head thì máy tính lại cần tìm 1 không gian mới copy dữ liệu cũ -> thêm 1 giá trị mới vào.
Truy cập vào head cũng tốn thời gian hơn vì nó cần tìm đến vị trí con trỏ chứa Heap.
Vì tốc độ(CPU) và bộ nhớ(RAM) của các máy tính hiện tại rất mạnh và các ứng dụng hay viết ở tầng Application(Desktop App, Mobile App...) nên khi viết c.trình các lập trình viên thường không để ý đến việc cấp phát và phân bổ bộ nhớ.
Tuy nhiên khi viết c.trình tương tác với máy(C/C++) thì phần quản lý bộ nhớ lại rất quan trọng, bạn cần theo dõi những dữ liệu nào sử dụng heap, dữ liệu nào sử dụng stack để tránh số lượng dữ liệu trùng lặp trên heap, và giải phóng dữ liệu không sử dụng.
Trong Rust khi bạn sử dụng Ownership, bạn không cần quan tâm đến heap và stack.

Ownership Rules:
- Mỗi giá trị trong Rust có 1 biến được gọi là owner của nó
- Tại 1 thời điểm chỉ có 1 owner
- Owner chỉ có hiệu lực trong scope của nó(tức là bên trong {})

Rồi ok đi vào code với các ví dụ sử dụng Ownership trên String

Reference: https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html