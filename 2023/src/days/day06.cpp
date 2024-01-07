#include <cmath>
#include <fstream>
#include <iomanip>
#include <iostream>
#include <regex>
#include <string_view>
#include <vector>

namespace
{
    const std::string g_input_path = "../2023/inputs/day06.txt";
    const auto g_time = "Time";
    const auto g_distance = "Distance";
    const auto g_number_regex = std::regex{"[\\d]+"};

    using Number = double;
    using Numbers = std::vector<Number>;
    using Times = Numbers;
    using Distances = Numbers;

    void print(const Numbers & numbers)
    {
        for(const auto number: numbers)
        {
            std::cout << number << " ";
        }
        std::cout << std::endl;
    }

    Numbers parse_numbers(std::string_view numbers)
    {
        Numbers res;
        auto number_it = std::regex_iterator<std::string_view::iterator>
                {
                        numbers.begin(), numbers.end(),
                        g_number_regex
                };

        for(decltype(number_it) last; number_it != last; number_it++)
        {
            res.push_back(std::stoll(number_it->str()));
        }
        return res;
    }

    std::tuple<Times, Distances> parse_file(const std::string & path)
    {
        std::ifstream ifs(path);
        std::string line;

        Times times{};
        Distances distances{};
        while(std::getline(ifs, line)){
            if (line != ""){
                const auto start_pos = line.find_first_of(":");
                if (start_pos != std::string::npos) {
                    const auto state_sv = std::string_view{line}.substr(0, start_pos);
                    if (state_sv == g_time) {
                        times = parse_numbers(std::string_view{line}.substr(start_pos + 1, std::string::npos));
                    } else if (state_sv == g_distance)
                    {
                        distances = parse_numbers(std::string_view{line}.substr(start_pos + 1, std::string::npos));
                    }
                    else
                    {
                        std::cout << "Invalid input.";
                    }
                }
            }
        }
        return {times, distances};
    }

    std::tuple<Times, Distances> parse_file_2(const std::string & path)
    {
        std::ifstream ifs(path);
        std::string line;

        Times times{};
        Distances distances{};
        while(std::getline(ifs, line)){
            if (line != ""){
                const auto start_pos = line.find_first_of(":");
                if (start_pos != std::string::npos) {
                    const auto state_sv = std::string_view{line}.substr(0, start_pos);
                    line.erase(std::remove(line.begin(),line.end(), ' '), line.end());
                    if (state_sv == g_time) {
                        times = parse_numbers(std::string_view{line}.substr(start_pos + 1, std::string::npos)

                        );
                    } else if (state_sv == g_distance)
                    {
                        distances = parse_numbers(std::string_view{line}.substr(start_pos + 1, std::string::npos));
                    }
                    else
                    {
                        std::cout << "Invalid input.";
                    }
                }
            }
        }
        return {times, distances};
    }

    Number calculate_possible_solutions(Number time, Number distance)
    {
        Number res = 0;
        const auto D = time * time - 4 * distance;
        if (D < 0){
            return res;
        }
        const auto D_sqrt = std::sqrt(D);
        const auto x_1 = (time + D_sqrt) / 2.0;
        const auto x_2 = (time - D_sqrt) / 2.0;
        return ceil(x_1 - 1) - floor(x_2 + 1) + 1;
    }

} // namespace

int main(){
    const auto [times, distances] = parse_file(g_input_path);
    Number possible_solutions_mul = 1;
    for(int i = 0; i < times.size(); i++)
    {
        const auto count = calculate_possible_solutions(times[i], distances[i]);
        std::cout << "iter: " << i << "; res: " << count << std::endl;
        possible_solutions_mul *= count;
    }
    std::cout << "--Part 1 Output " << std::setprecision(15) << possible_solutions_mul << std::endl;

    const auto [times_2, distances_2] = parse_file_2(g_input_path);
    Number possible_solutions_mul_2 = calculate_possible_solutions(times_2.front(), distances_2.front());;
    std::cout << "--Part 2 Output " << std::setprecision(15) << possible_solutions_mul_2 << std::endl;

    return 0;
}
