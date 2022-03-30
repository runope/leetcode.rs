# LeetCode.rs
Download leetcode problem to local and generate the appropriate rust file. You can code directly locally and enjoy code completion.

Download problems which contains questions, samples, descriptions, and code templete in the problem_sets folder
<img src="./img/problem.png">
When solved a problem, you can solve $id. The problem will move to the solution_sets folder, and README.md will add the problem automatically.
<img src="./img/solved.png">

## usage
cargo run -- -h, --help Print help information <br>
cargo run -- -i, --id <id>       generate problem by a frontend problem <id> <br>
cargo run -- -r, --random        generate a random problem <br>
cargo run -- -s, --solve <id>    solve the problem's <id> <br>

## Resolved
<table id="leetcode" class="table-auto">
  <thead>
    <tr>
      <th>id</th>
      <th>Leetcode</th>
      <th>Level</th>
    </tr>
  </thead>
  <tbody>
   <tr>
        <td>0001</td>
        <td><a href="./src/solution_sets/s0001_two_sum.rs"> two-sum</a></td>
        <td>Easy</td>
    </tr>
   <tr>
        <td>0172</td>
        <td><a href="./src/solution_sets/s0172_factorial_trailing_zeroes.rs"> factorial-trailing-zeroes</a></td>
        <td>Medium</td>
    </tr>
   <tr>
        <td>1925</td>
        <td><a href="./src/solution_sets/s1925_count_square_sum_triples.rs"> count-square-sum-triples</a></td>
        <td>Easy</td>
    </tr>
