#include <algorithm>
#include <fstream>
#include <iostream>
#include <string>
#include <unordered_map>
#include <vector>

namespace
{
    const std::string g_input_path = "../2023/inputs/day07.txt";

    using CharTable = std::unordered_map<char, int>;
    const CharTable g_card_strength_table =
            {
                    {'A', 13},
                    {'K', 12},
                    {'Q', 11},
                    {'J', 10},
                    {'T', 9},
                    {'9', 8},
                    {'8', 7},
                    {'7', 6},
                    {'6', 5},
                    {'5', 4},
                    {'4', 3},
                    {'3', 2},
                    {'2', 1}
            };
    const CharTable g_card_strength_table_with_wild_card =
            {
                    {'A', 13},
                    {'K', 12},
                    {'Q', 11},
                    {'T', 10},
                    {'9', 9},
                    {'8', 8},
                    {'7', 7},
                    {'6', 6},
                    {'5', 5},
                    {'4', 4},
                    {'3', 3},
                    {'2', 2},
                    {'J', 1},
            };

    using Number = int;
    enum class HandType
    {
        FIVE_OF_A_KIND  = 7,    // 1 distinct
        FOUR_OF_A_KIND  = 6,    // 2 distinct, one with frequency 4
        FULL_HOUSE      = 5,    // 2 distinct
        THREE_OF_A_KIND = 4,    // 3 distinct, one with frequency 3
        TWO_PAIR        = 3,    // 3 distinct
        ONE_PAIR        = 2,    // 4 distinct
        HIGH_CARD       = 1     // 5 distinct
    };

    std::ostream& operator<<(std::ostream& out, const HandType value){
        return out << [value]{
#define PROCESS_VAL(p) case(p): return #p;
            switch(value){
                PROCESS_VAL(HandType::FIVE_OF_A_KIND);
                PROCESS_VAL(HandType::FOUR_OF_A_KIND);
                PROCESS_VAL(HandType::FULL_HOUSE);
                PROCESS_VAL(HandType::THREE_OF_A_KIND);
                PROCESS_VAL(HandType::TWO_PAIR);
                PROCESS_VAL(HandType::ONE_PAIR);
                PROCESS_VAL(HandType::HIGH_CARD);
            }
#undef PROCESS_VAL
        }();
    }

    struct Record
    {
        std::string hand;
        Number bid;

        [[nodiscard]] HandType calculate_type(const std::unordered_map<char, int> & hand_view) const
        {
            int distinct_numbers = hand_view.size();
            switch (distinct_numbers)
            {
                case 1:
                    return HandType::FIVE_OF_A_KIND;
                case 2:
                    if (std::any_of(hand_view.cbegin(), hand_view.cend(), [](auto & item) { return item.second == 4; }))
                        return HandType::FOUR_OF_A_KIND;
                    else
                        return HandType::FULL_HOUSE;
                case 3:
                    if (std::any_of(hand_view.cbegin(), hand_view.cend(), [](auto & item){ return item.second == 3; }))
                        return HandType::THREE_OF_A_KIND;
                    else
                        return HandType::TWO_PAIR;
                case 4:
                    return HandType::ONE_PAIR;
                case 5:
                    return HandType::HIGH_CARD;
                default:
                    throw std::invalid_argument("Check hand input!");
            }
        }

        [[nodiscard]] bool is_stronger_for_equal_types(const Record & other, const CharTable & table) const
        {
            for(int i = 0; i < hand.size(); i++)
            {
                const auto first_strength = table.at(hand[i]);
                const auto second_strength = table.at(other.hand[i]);
                if(first_strength == second_strength)
                    continue;
                else if (first_strength <= second_strength)
                    return true;
                else
                    return false;
            }
        }

        [[nodiscard]] std::unordered_map<char, int> get_hand_view(bool use_wildcard) const
        {
            std::unordered_map<char, int> hand_view{};
            for (char symbol: hand)
            {
                if (hand_view.find(symbol) == hand_view.end())
                {
                    hand_view.insert(std::make_pair(symbol, 1));
                }
                else
                {
                    hand_view[symbol]++;
                }
            }

            if (use_wildcard && hand_view.count('J') && hand_view['J'] < 5)
            {
                const auto item = hand_view.extract('J');
                auto max_it = std::max_element(hand_view.begin(), hand_view.end(), [](auto & l, auto & r)
                {
                    return l.second < r.second;
                });
                max_it->second += item.mapped();
            }

            return hand_view;
        }

    };
    using Game = std::vector<Record>;

    void print(const Game & game)
    {
        for(const auto & record: game)
        {
            std::cout << "Hand: " << record.hand << "; Bid: " << record.bid << std::endl;
        }
        std::cout << std::endl;
    }

    Game parse_file(const std::string & path)
    {
        std::ifstream ifs(path);
        std::string line;

        Game game{};
        while(std::getline(ifs, line)){
            if (line != ""){
                const auto start_pos = line.find_first_of(' ');
                game.push_back({line.substr(0, start_pos), std::stoi(line.substr(start_pos + 1, std::string::npos))});
            }
        }
        return game;
    }

    Game order_hands_by_strength(const Game & game, bool use_wildcard = false)
    {
        Game res = game;
        const auto & table = use_wildcard? g_card_strength_table_with_wild_card: g_card_strength_table;
        std::sort(res.begin(), res.end(),
            [use_wildcard, &table](auto & l, auto & r)
            {
                const auto l_type = l.calculate_type(l.get_hand_view(use_wildcard));
                const auto r_type = r.calculate_type(r.get_hand_view(use_wildcard));
                if(l_type != r_type)
                    return l_type < r_type;
                else
                    return l.is_stronger_for_equal_types(r, table);
            });
        return res;
    }

    long long get_total_winnings(const Game & game)
    {
        long long sum = 0;
        for(int i = 0; i < game.size(); i++)
        {
            sum += game[i].bid * (i+1);
        }
        return sum;
    }

} // namespace

int main(){
    const auto game_input = parse_file(g_input_path);

    const auto game_sorted_1 = order_hands_by_strength(game_input);
    std::cout << "--Part 1 Output " << get_total_winnings(game_sorted_1) << std::endl;

    const auto game_sorted_2 = order_hands_by_strength(game_input, true);
    std::cout << "--Part 2 Output " << get_total_winnings(game_sorted_2) << std::endl;

    return 0;
}
