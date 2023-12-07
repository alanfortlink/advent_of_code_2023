import 'dart:io';

final values = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine", ];

int get(String line, int s, int e, int ic){
  for(int i = s; i != e; i += ic){
    for(final key in values){
      if (line.substring(i).startsWith(key)) {
        return values.indexOf(key) + 1; 
      }
    }
    if("0123456789".contains(line[i])) return int.parse(line[i]);
  }

  return 0;
}

int q2(String line){
  int D = get(line, 0, line.length, 1);
  int U = get(line, line.length-1, -1, -1);
  return 10 * D + U;
}

int main(List<String> args) {
  if (args.length < 1) {
    print("Usage: ./main file_path");
    exit(1);
  }

  print(File(args.first).readAsLinesSync().map(q2).reduce((v, e) => v + e));

  return 0;
}
