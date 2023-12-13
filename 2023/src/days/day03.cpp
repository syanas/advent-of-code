#include <fstream>
#include <iostream>
#include <map>
#include <regex>
#include <tuple>
#include <vector>

namespace
{
    const std::string g_input_path = "../2023/inputs/day03.txt";
    using Pos = std::tuple<int, int>;
    using Len = int;
    using Number = std::tuple<Pos, Len, int>;
    using Numbers = std::vector<Number>;
    using Symbols = std::map<Pos, char>;

    std::tuple<Numbers, Symbols> parse_file(const std::string & path)
    {
        std::ifstream ifs(path, std::ifstream::in);
        Numbers numbers{};
        Symbols symbols{};
        std::string line;
        size_t i = 0;
        auto figure = std::regex{"[\\d]+"};
        auto symbol = std::regex{"[^\\d.]"};
        while(std::getline(ifs, line)){  // stops at eof
            if (line != ""){
                auto figure_it = std::regex_iterator<std::string::iterator>
                        {
                                line.begin(), line.end(),
                                figure
                        };

                for(decltype(figure_it) last; figure_it != last; figure_it++)
                {
                    numbers.push_back({{i, figure_it->position()}, figure_it->length(), std::stoi(figure_it->str())});
                }

                auto symbol_it = std::regex_iterator<std::string::iterator>
                        {
                                line.begin(), line.end(),
                                symbol
                        };
                for(decltype(symbol_it) last; symbol_it != last; symbol_it++)
                {
                    symbols.insert({{i, symbol_it->position()}, symbol_it->str()[0]});
                }
                i++;
            }
        }
        ifs.close();
        return {numbers, symbols};
    }

    bool is_part_number(const Number & number, const Symbols & symbols)
    {
        auto [position, length, value] = number;
        auto row = std::get<0>(position);
        auto column_begin = std::get<1>(position) - 1;
        auto column_end = column_begin + length + 1;
        for(auto column = column_begin; column <= column_end; column++)
        {
            if(symbols.count({row - 1, column}) || symbols.count({row + 1, column}))
            {
                return true;
            }
        }
        if(symbols.count({row, column_begin}) || symbols.count({row, column_end}))
        {
            return true;
        }
        return false;
    }

} // namespace

int main(){
    auto [numbers, symbols] = parse_file(g_input_path);

    int part_numbers_sum = 0;
    for(auto number: numbers)
    {
        if (is_part_number(number, symbols))
        {
            part_numbers_sum += std::get<2>(number);
        }
    };
    std::cout << "--Part 1 Output " << part_numbers_sum << std::endl;

    return 0;
}
