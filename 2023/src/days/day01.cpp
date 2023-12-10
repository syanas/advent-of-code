#include <algorithm>
#include <fstream>
#include <iostream>
#include <string>
#include <unordered_map>

namespace
{

const std::string g_input_path = "../2023/inputs/day01.txt";
const std::string digits = "123456789";
const std::unordered_map<char, std::string> digits_comparison_table =
{
    {'1', "one"},
    {'2', "two"},
    {'3', "three"},
    {'4', "four"},
    {'5', "five"},
    {'6', "six"},
    {'7', "seven"},
    {'8', "eight"},
    {'9', "nine"}
};

int get_calibration_value(const std::string & line)
{
    const auto first_pos = line.find_first_of(digits);
    const auto last_pos = line.find_last_of(digits);
    const auto first_digit = line[first_pos] - '0';
    const auto last_digit = line[last_pos] - '0';
    return first_digit * 10 + last_digit;
}

using PositionToDigitType = std::pair<std::string::size_type, char>;

int get_calibration_value_using_table(const std::string & line)
{
    PositionToDigitType first_pos_digit = {std::string::npos, '0'};
    auto find_first_calibration_value = [&first_pos_digit, line](char digit)
    {
        std::string::size_type pos = std::min(line.find(digit), line.find(digits_comparison_table.at(digit)));
        if (pos != std::string::npos && pos < first_pos_digit.first)
        {
            first_pos_digit = PositionToDigitType{pos, digit};
        }
    };

    std::for_each(digits.begin(),digits.end(),find_first_calibration_value);
    const auto first_digit = first_pos_digit.second - '0';

    PositionToDigitType last_pos_digit = {0, '0'};;
    auto find_last_calibration_value = [&last_pos_digit, line](char digit)
    {
        std::string::size_type pos_char = line.rfind(digit);
        std::string::size_type pos_word = line.rfind(digits_comparison_table.at(digit));
        std::string::size_type pos = std::string::npos;
        if(pos_char == std::string::npos || pos_word == std::string::npos)
        {
            pos = std::min(pos_char, pos_word);
        } else if (pos_char != std::string::npos && pos_word != std::string::npos)
        {
            pos = std::max(pos_char, pos_word);
        }

        if (pos != std::string::npos && pos >= last_pos_digit.first)
        {
            last_pos_digit = PositionToDigitType{pos, digit};
        }
    };

    std::for_each(digits.begin(),digits.end(),find_last_calibration_value);
    const auto last_digit = last_pos_digit.second - '0';

    return first_digit * 10 + last_digit;
}

void part_1()
{
    std::ifstream ifs(g_input_path, std::ifstream::in);
    std::string line;
    int sum = 0;
    while(std::getline(ifs, line)){  // stops at eof
        if (line != ""){
            sum += get_calibration_value(line);
        }
    }
    ifs.close();
    std::cout << "--Part 1 Output " << sum << std::endl;
}

void part_2()
{
    std::ifstream ifs(g_input_path, std::ifstream::in);
    std::string line;
    int sum = 0;
    while(std::getline(ifs, line)){  // stops at eof
        if (line != ""){
            sum += get_calibration_value_using_table(line);
        }
    }
    ifs.close();
    std::cout << "--Part 2 Output " << sum << std::endl;
}

} // namespace

int main(){
    part_1();
    part_2();
    return 0;
}