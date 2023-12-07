#include <cstdlib>
#include <fstream>
#include <functional>
#include <iostream>
#include <map>
#include <numeric>
#include <ranges>
#include <regex>
#include <string>

using namespace std;

int q2(const string &line) {
  regex r(": ");
  sregex_token_iterator it(line.begin(), line.end(), r, -1);
  string s = *(++it);

  r = regex("; ");
  it = sregex_token_iterator(s.begin(), s.end(), r, -1);

  vector<string> reveals{it, {}};

  map<string, int> m;

  for (auto reveal : reveals) {
    regex r(", ");
    sregex_token_iterator it(reveal.begin(), reveal.end(), r, -1);
    vector<string> balls{it, {}};

    for (auto ball : balls) {
      regex r(" ");
      sregex_token_iterator it(ball.begin(), ball.end(), r, -1);
      vector<string> info{it, {}};

      string color = info[1];
      m[color] = max(stoi(info[0]), m[color]);
    }
  }

  auto t =
      m | views::transform([](auto a) { return a.second; }) | views::common;
  return accumulate(t.begin(), t.end(), 1, std::multiplies<int>());
}

int main(int ac, char **argv) {
  if (ac < 2) {
    cout << "Usage: ./main file_path" << endl;
    exit(1);
  }

  auto f = fstream(argv[1]);
  int total = 0;
  while (!f.eof()) {
    string line;
    getline(f, line);
    if (line.empty())
      continue;

    total += q2(line);
  }

  cout << total << endl;
}
