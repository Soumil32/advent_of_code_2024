#include <iostream>
#include <fstream>
#include <string>
#include <sstream>
#include <vector>
#include <algorithm>
#include <ranges>

using std::string;

std::vector<string> splitLine(const string& line, const char del) {
    std::vector<string> splits;
    std::stringstream line_stream(line);
    string split;
    while (getline(line_stream, split, del)) {
        if (split.empty()) continue;
        splits.push_back(split);
    }
    return splits;
}

int partOne(const string& input) {
    std::vector<int> left_list;
    std::vector<int> right_list;
    std::stringstream input_stream(input);
    string line;
    while (getline(input_stream, line)) {
        auto splits = splitLine(line, ' ');
        if (splits.size() != 2) {
            std::cerr << "Line contains more than 2 values" << std::endl;
            return -1; // clear that something has gone wrong
        }
        left_list.push_back(stoi(splits.at(0)));
        right_list.push_back(stoi(splits.at(1)));
    }
    std::ranges::sort(left_list);
    std::ranges::sort(right_list);

    int total_distance = 0;
    for (int i = 0; i<left_list.size(); ++i) {
        const auto left_num = left_list.at(i);
        const auto right_num = right_list.at(i);
        const auto difference = std::abs(left_num - right_num);
        total_distance += difference;
    }
    return total_distance;
}

int partTwo(const string& input) {
    std::vector<int> left_list;
    std::vector<int> right_list;
    std::stringstream input_stream(input);
    string line;
    while (getline(input_stream, line)) {
        auto splits = splitLine(line, ' ');
        if (splits.size() != 2) {
            std::cerr << "Line contains more than 2 values" << std::endl;
            return -1; // clear that something has gone wrong
        }
        left_list.push_back(stoi(splits.at(0)));
        right_list.push_back(stoi(splits.at(1)));
    }
    int total_similarity = 0;
    for (int left_num: left_list) {
        total_similarity += static_cast<int>(std::ranges::count(right_list, left_num)) * left_num;
    }
    return total_similarity;
}

int main() {
    std::ifstream input_file("./input.txt");
    if (!input_file) {
        std::cerr << "Could not open input file" << std::endl;
        return 1;
    }
    std::stringstream buffer;
    buffer << input_file.rdbuf();
    string input = buffer.str();
    std::cout << partOne(input) << std::endl;
    std::cout << partTwo(input) << std::endl;
    return 0;
}