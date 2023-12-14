#include <cstdlib>
#include <fstream>
#include <iostream>
#include <map>
#include <optional>
#include <regex>
#include <tuple>
#include <vector>

namespace
{
    const std::string g_input_path = "../2023/inputs/day03.txt";
    using GearRatio = int;
    using Row = int;
    using Column = int;
    using Pos = std::tuple<Row, Column>;
    using Len = int;
    using Value = int;
    struct Number
    {
        Len len = 0;
        Value value = 0;
    };
    using Numbers = std::vector<std::map<Column, Number>>;
    using Symbols = std::vector<std::map<Column, char>>;

    std::tuple<Numbers, Symbols> parse_file(const std::string & path)
    {
        std::ifstream ifs(path, std::ifstream::in);
        Numbers numbers{};
        Symbols symbols{};
        std::string line;
        auto figure = std::regex{"[\\d]+"};
        auto symbol = std::regex{"[^\\d.]"};
        while(std::getline(ifs, line)){  // stops at eof
            if (line != ""){
                auto figure_it = std::regex_iterator<std::string::iterator>
                        {
                                line.begin(), line.end(),
                                figure
                        };

                std::map<Column, Number> line_figures;
                for(decltype(figure_it) last; figure_it != last; figure_it++)
                {
                    line_figures.insert({figure_it->position(), {static_cast<Len>(figure_it->length()), std::stoi(figure_it->str())}});
                }
                numbers.push_back(line_figures);

                std::map<Column, char> line_symbols;
                auto symbol_it = std::regex_iterator<std::string::iterator>
                        {
                                line.begin(), line.end(),
                                symbol
                        };
                for(decltype(symbol_it) last; symbol_it != last; symbol_it++)
                {
                    line_symbols.insert({symbol_it->position(), symbol_it->str()[0]});
                }
                symbols.push_back(line_symbols);
            }
        }
        ifs.close();
        return {numbers, symbols};
    }

    bool is_adjacent(const Pos & symbol_position, const Pos & number_position, Len length)
    {
        auto & [symbol_row, symbol_column] = symbol_position;
        auto & [number_row, number_column] = number_position;
        return (abs(symbol_row - number_row) < 2)
            && symbol_column >= number_column - 1
            && symbol_column <= number_column + length;
    }

    bool is_part_number(const Number & number, Pos position, const Symbols & symbols)
    {
        [[maybe_unused]] auto [length, dummy1] = number;
        [[maybe_unused]] auto [row, dummy2] = position;

        auto row_begin = std::max(0, row - 1);
        auto row_end = std::min(static_cast<int>(symbols.size() - 1), row + 1);
        for (int row_index = row_begin; row_index <= row_end; row_index++)
        {
            for(auto &[symbol_column, _]: symbols[row_index])
            {
                if(is_adjacent({row_index, symbol_column}, position, length))
                {
                    return true;
                }
            }
        }
        return false;
    }

    std::optional<GearRatio> is_gear_symbol(char symbol, Pos position, const Numbers & numbers)
    {
        if (symbol != '*')
        {
            return std::nullopt;
        }
        auto &[row, column] = position;

        std::vector<Value> adjacent_numbers;
        for(int i = std::max(row - 1, 0); i <= std::min(row + 1, static_cast<int>(numbers.size() - 1)); i++)
        {
            auto &row_numbers = numbers[i];
            for (auto &[number_column, number]: row_numbers)
            {
                if (is_adjacent(position, {i, number_column}, number.len))
                {
                    adjacent_numbers.push_back(number.value);
                }
            }
        }

        if (adjacent_numbers.size() == 2)
        {
            return {adjacent_numbers[0] * adjacent_numbers[1]};
        }
        else
        {
            return std::nullopt;
        }
    }

} // namespace

int main(){
    auto [numbers, symbols] = parse_file(g_input_path);
    int part_numbers_sum = 0;
    for(auto i = 0; i < numbers.size(); i++)
    {
        for(auto [column, number]: numbers[i])
        {
            if (is_part_number(number, {i, column}, symbols))
            {
                part_numbers_sum += number.value;
            }
        }
    };
    std::cout << "--Part 1 Output " << part_numbers_sum << std::endl;

    int gears_ratio_sum = 0;
    for(auto i = 0; i < symbols.size(); i++)
    {
        for(auto [column, symbol]: symbols[i])
        {
            auto gear_ratio = is_gear_symbol(symbol, {i, column}, numbers);
            if (gear_ratio)
            {
                gears_ratio_sum += *gear_ratio;
            }
        }
    };
    std::cout << "--Part 2 Output " << gears_ratio_sum << std::endl;

    return 0;
}
