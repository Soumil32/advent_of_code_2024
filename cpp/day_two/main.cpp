#include <iostream>
#include <fstream>
#include <string>
#include <sstream>
#include <vector>

using std::string;
using std::stringstream;
using std::vector;

int partOne(const string& input) {
    stringstream input_stream(input);
    int num_safe_reports = 0;
    string line;
    while (getline(input_stream, line)) {
        stringstream line_stream(line);
        vector<int> report_numbers;
        string current_number;
        while (getline(line_stream, current_number, ' ')) {
            report_numbers.push_back(std::stoi(current_number));
        }
        bool is_increasing = report_numbers.at(1) > report_numbers.at(0);
        bool failed = false;
        if (report_numbers.at(0) == report_numbers.at(1)) continue; // failed
        for (int i=1; i<report_numbers.size(); ++i) {
            if (report_numbers.at(i) > report_numbers.at(i-1) != is_increasing) {
                failed = true;
            }
            int const difference = std::abs(report_numbers.at(i) - report_numbers.at(i-1));
            if (difference < 1 || difference > 3) {
                failed = true;
            }
        }
        if (!failed) num_safe_reports++;
    }
    return num_safe_reports;
}

int main() {
    const std::ifstream input_file("input.txt");
    if (!input_file) {
        std::cerr << "Could not open input" << std::endl;
        return 1;
    }

    stringstream buffer;
    buffer << input_file.rdbuf();
    string input = buffer.str();
    std::cout << partOne(input) << std::endl;

    return 0;
}
