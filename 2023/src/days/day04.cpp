#include <fstream>
#include <iostream>
#include <regex>
#include <string_view>
#include <vector>

namespace
{
    const std::string g_input_path = "../2023/inputs/day04.txt";

    using Numbers = std::vector<int>;

    void print(const Numbers & numbers)
    {
        for(const auto number: numbers)
        {
            std::cout << number << " ";
        }
    }

    struct Record
    {
        Numbers winning_numbers;
        Numbers numbers;

        void print()
        {
            std::cout << "Winning numbers: ";
            ::print(winning_numbers);
            std::cout << "| Numbers: ";
            ::print(numbers);
            std::cout << std::endl;
        }
    };
    using ScratchCards = std::vector<Record>;

    Numbers parse_numbers(std::string_view numbers)
    {
        Numbers res;
        const auto number = std::regex{"[\\d]+"};
        auto number_it = std::regex_iterator<std::string_view::iterator>
                {
                        numbers.begin(), numbers.end(),
                        number
                };

        for(decltype(number_it) last; number_it != last; number_it++)
        {
            res.push_back(std::stoi(number_it->str()));
        }
        std::sort(res.begin(), res.end());
        return res;
    }

    ScratchCards parse_file(const std::string & path)
    {
        std::ifstream ifs(path, std::ifstream::in);
        std::string line;

        ScratchCards table;
        while(std::getline(ifs, line)){
            if (line != ""){
                const auto start_pos = line.find_first_of(":");
                const auto numbers_delimiter_pos = line.find_first_of("|");
                const auto winning_numbers_sv = std::string_view{line}.substr(start_pos, numbers_delimiter_pos - start_pos);
                const auto numbers_sv = std::string_view{line}.substr(numbers_delimiter_pos, std::string::npos);
                Record game_record{parse_numbers(winning_numbers_sv), parse_numbers(numbers_sv)};
                table.push_back(std::move(game_record));
            }
        }
        ifs.close();
        return table;
    }

    int calculate_matches_for_record(const Record & record)
    {
        Numbers v_intersection;
        std::set_intersection(record.winning_numbers.begin(), record.winning_numbers.end(),
                              record.numbers.begin(), record.numbers.end(),
                              std::back_inserter(v_intersection));
        return v_intersection.size();
    }

} // namespace

int main(){
    const auto scratchcards = parse_file(g_input_path);
    int points_sum = 0;
    for(const auto & card_record: scratchcards)
    {
        const auto matches_number = calculate_matches_for_record(card_record);
        points_sum += matches_number? 1 << (matches_number - 1): 0;
    };
    std::cout << "--Part 1 Output " << points_sum << std::endl;

    int total_amount_of_cards = 0;
    std::vector<int> cards_instances_count(scratchcards.size(), 1);
    for(int i = 0; i < scratchcards.size(); i++)
    {
        total_amount_of_cards += cards_instances_count[i];
        const auto matches_number = calculate_matches_for_record(scratchcards[i]);
        for(int j = i + 1; j < i + matches_number + 1; j++)
        {
            cards_instances_count[j] += cards_instances_count[i];
        }
    };
    std::cout << "--Part 2 Output " << total_amount_of_cards << std::endl;

    return 0;
}
