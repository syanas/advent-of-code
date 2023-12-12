#include <algorithm>
#include <fstream>
#include <iostream>
#include <vector>

namespace
{
    const std::string g_input_path = "../2023/inputs/day02.txt";

    struct CubeSet
    {
        int red = 0;
        int green = 0;
        int blue = 0;

        void print()
        {
            std::cout << "Red: " << red << "; Green: " << green << "; Blue: " << blue << "\n";
        }
    };

    struct Game{
        int id = 0;
        std::vector<CubeSet> rounds;

        void print()
        {
            std::cout << "Game " << id << "\n";
            for(int i = 0; i < rounds.size(); ++i)
            {
                rounds[i].print();
            }
        }
    };

    const CubeSet g_bag_contents{12, 13, 14};

    void update_round(const std::string & color, int quantity, CubeSet & round)
    {
        if(color == "red")
        {
            round.red = quantity;
        }
        else if(color == "green")
        {
            round.green = quantity;
        }
        else if(color == "blue")
        {
            round.blue = quantity;
        }
        else
        {
            std::cout << "Impossible color:" << color << "\n";
        }
    }

    CubeSet parse_round(const std::string & round)
    {
        std::string::size_type start = 0;
        auto end = round.find_first_of(",");
        CubeSet res;
        while(end != std::string::npos)
        {
            auto delimiter = round.find_first_of(" ", start + 1);
            auto quantity = std::stoi(round.substr(start + 1, delimiter - start - 1));
            auto color = round.substr(delimiter + 1, end - delimiter - 1);
            update_round(color, quantity, res);
            start = end + 1;
            end = round.find_first_of(",", start);
        }

        auto delimiter = round.find_first_of(" ", start + 1);
        auto quantity = std::stoi(round.substr(start + 1, delimiter - start - 1));
        auto color = round.substr(delimiter + 1, round.size() - delimiter - 1);
        update_round(color, quantity, res);
        return res;
    }

    Game parse_game(const std::string & line)
    {
        auto start = line.find_first_of(" ");
        auto end = line.find_first_of(":");

        Game res;
        res.id = std::stoi(line.substr(start + 1, end - start - 1));
        start = end;
        end = line.find_first_of(";", start);
        while(end != std::string::npos)
        {
            const auto round = line.substr(start + 1, end - start - 1);
            res.rounds.push_back(parse_round(round));
            start = end;
            end = line.find_first_of(";", start + 1);
        }
        const auto round = line.substr(start + 1, line.size() - start);
        res.rounds.push_back(parse_round(round));
        return res;
    }

    bool game_is_possible(const Game & game, const CubeSet & bag_contents)
    {
        return std::all_of(game.rounds.begin(), game.rounds.end(), [& bag_contents](const CubeSet & round) -> bool
        {
            return round.red <= bag_contents.red && round.green <= bag_contents.green && round.blue <= bag_contents.blue;
        });
    }

    CubeSet get_min_bag_contents_for_game(const Game & game)
    {
        CubeSet bag_contents{};
        std::for_each(game.rounds.begin(), game.rounds.end(), [&bag_contents](const CubeSet & round) -> void
        {
            bag_contents.red = std::max(round.red, bag_contents.red);
            bag_contents.green = std::max(round.green, bag_contents.green);
            bag_contents.blue = std::max(round.blue, bag_contents.blue);
        });
        return bag_contents;
    }

    void part_1()
    {
        std::ifstream ifs(g_input_path, std::ifstream::in);
        std::string line;
        int sum = 0;
        while(std::getline(ifs, line)){  // stops at eof
            if (line != ""){
                auto game = parse_game(line);
                if (game_is_possible(game, g_bag_contents))
                sum += game.id;
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
                auto game = parse_game(line);
                auto min_bag_contents = get_min_bag_contents_for_game(game);
                sum += min_bag_contents.red * min_bag_contents.green * min_bag_contents.blue;
            }
        }
        ifs.close();
        std::cout << "--Part 1 Output " << sum << std::endl;
    }
} // namespace

int main(){
    part_1();
    part_2();
    return 0;
}
