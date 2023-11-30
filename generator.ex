day = System.argv() |> List.first() |> String.pad_leading(2, "0")

defmodule Generator do
  def main(day) do
    File.read!("src/main.rs")
    |> String.split("\n")
    |> Enum.map(fn line ->
      if(line == "    ];") do
        "      day_#{day},\n" <> line
      else
        line
      end
    end)
    |> Enum.join("\n")
    |> (&File.write!("src/main.rs", &1)).()
  end

  def lib(day) do
    File.read!("src/lib.rs")
    |> (fn original -> "#{original}mod day_#{day};
pub use day_#{day}::run as day_#{day};\n" end).()
    |> (&File.write!("src/lib.rs", &1)).()
  end



  def code(day) do
    "pub fn run_a(input: &str) {
  let result = \"Not known yet\";
  println!(\"Day #{day}a: {}\", result);
}
pub fn run_b(input: &str) {
  let result = \"Not known yet\";
  println!(\"Day #{day}b: {}\", result);
}

pub fn run() {
  let _challenge_input = include_str!(\"../inputs/day_#{day}/challenge.txt\");
  let test_input = include_str!(\"../inputs/day_#{day}/test.txt\");
  run_a(test_input);
  run_b(test_input);
}
" |> (&File.write!("src/day_#{day}.rs", &1)).()
  end

  def input(day) do
    File.mkdir_p!("inputs/day_#{day}")
    File.write!("inputs/day_#{day}/test.txt", "")
    File.write!("inputs/day_#{day}/challenge.txt", "")
  end
end

spawn(Generator, :main, [day])
spawn(Generator, :lib, [day])
spawn(Generator, :code, [day])
spawn(Generator, :input, [day])
