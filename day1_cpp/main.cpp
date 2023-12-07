#include <fstream>
#include <iostream>
#include <string>

bool starts_with(const std::string &str, const std::string &needle) {
  for (int i = 0; i < needle.size(); i++) {
    if (i >= str.size())
      return false;
    if (str[i] != needle[i])
      return false;
  }
  return true;
}

int get(const std::string &row, int s, int e, int inc) {
  for (int i = s; i != e; i += inc) {
    const std::string &sub = row.substr(i);

    if (starts_with(sub, "one"))
      return 1;
    if (starts_with(sub, "two"))
      return 2;
    if (starts_with(sub, "three"))
      return 3;
    if (starts_with(sub, "four"))
      return 4;
    if (starts_with(sub, "five"))
      return 5;
    if (starts_with(sub, "six"))
      return 6;
    if (starts_with(sub, "seven"))
      return 7;
    if (starts_with(sub, "eight"))
      return 8;
    if (starts_with(sub, "nine"))
      return 9;

    if (row[i] >= '0' && row[i] <= '9') {
      return row[i] - '0';
    }
  }

  return 0;
}

int main(int ac, char **av) {
  if (ac < 2) {
    std::cerr << "Usage: ./main file_path" << std::endl;
    exit(1);
  }

  std::fstream fs(av[1]);
  int total = 0;
  while (!fs.eof()) {
    std::string row;
    fs >> row;

    if (row.empty()) {
      break;
    }

    int D = 10 * get(row, 0, row.size(), 1);
    int U = get(row, row.size() - 1, -1, -1);

    total += D;
    total += U;
  }

  std::cout << total << std::endl;
}
