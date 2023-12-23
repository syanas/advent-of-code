#include <fstream>
#include <iostream>
#include <map>
#include <optional>
#include <regex>
#include <vector>

const std::string g_input_path = "../2023/inputs/day05.txt";
const auto g_number_regex = std::regex{"\\d+"};
const auto g_seeds = "seeds";
const auto g_map_names_regex = std::regex{"(\\w+)-to-(\\w+) map:"};

using NumberType = int64_t;
enum class Domain
{
    FERTILIZER,
    HUMIDITY,
    LIGHT,
    LOCATION,
    SEED,
    SOIL,
    TEMPERATURE,
    WATER
};

Domain to_enum(std::string_view name)
{
    if(name == "soil")
        return Domain::SOIL;
    else if(name == "seed")
        return Domain::SEED;
    else if(name == "fertilizer")
        return Domain::FERTILIZER;
    else if(name == "water")
        return Domain::WATER;
    else if(name == "light")
        return Domain::LIGHT;
    else if(name == "temperature")
        return Domain::TEMPERATURE;
    else if(name == "humidity")
        return Domain::HUMIDITY;
    else if(name == "location")
        return Domain::LOCATION;
    else
    {
        throw std::invalid_argument("to_enum: Domain enum values have changed.");
    }
}

std::string to_string(const Domain& d) {
    switch(d) {
        case Domain::SEED: return "seed";
        case Domain::SOIL: return "soil";
        case Domain::FERTILIZER: return "fertilizer";
        case Domain::WATER: return "water";
        case Domain::LIGHT: return "light";
        case Domain::TEMPERATURE: return "temperature";
        case Domain::HUMIDITY: return "humidity";
        case Domain::LOCATION: return "location";
        default: throw std::invalid_argument("to_string: Domain enum values have changed.");
    }
}

struct SegmentMapping
{
    NumberType destination_begin_id;
    NumberType source_begin_id;
    NumberType length;

    void print() const
    {
        std::cout << destination_begin_id << " " << source_begin_id << " " << length << std::endl;
    }
};

using MappingName = std::pair<Domain, Domain>;
using Mapping = std::vector<SegmentMapping>;
using MappingDescription = std::pair<MappingName, Mapping>;
using Numbers = std::vector<NumberType>;

std::tuple<bool, NumberType> apply_transformation(NumberType id, const SegmentMapping & segment);

struct Range;
using Ranges = std::vector<Range>;

struct Range
{
    NumberType begin_id;
    NumberType length;

    NumberType end_id() const
    {
        return begin_id + length - 1;
    }

    bool intersects(const Range & other) const
    {
        return std::min(end_id(), other.end_id()) >= std::max(begin_id, other.begin_id);
    }

    Ranges split(const Mapping & mapping) const
    {
        Ranges res{};
        std::optional<Range> not_split_range;
        bool range_is_split = false;
        for(const auto & triplet: mapping)
        {
            if(end_id() < triplet.source_begin_id)
            {
                range_is_split = true;
                res.push_back(*this);
                break;
            }
            if (intersects({triplet.source_begin_id, triplet.length}))
            {
                range_is_split = true;
                const auto triplet_end = triplet.source_begin_id + triplet.length - 1;
                if (triplet_end >= end_id())
                {
                    if (triplet.source_begin_id > begin_id)
                    {
                        res.push_back({begin_id, triplet.source_begin_id - begin_id});
                        res.push_back({triplet.source_begin_id, end_id() - triplet.source_begin_id + 1});
                    }
                    else
                    {
                        res.push_back(*this);
                    }
                }
                else
                {
                    if (triplet.source_begin_id > begin_id)
                    {
                        res.push_back({begin_id, triplet.source_begin_id - begin_id});
                        res.push_back({triplet.source_begin_id, triplet.length});
                    }
                    else
                    {
                        res.push_back({begin_id, triplet_end - begin_id + 1});
                    }
                    not_split_range = {triplet_end + 1, end_id() - triplet_end};
                }
                break;
            }
        }
        if (!range_is_split)
        {
            res.push_back(*this);
        }
        if (not_split_range)
        {
            auto range_end = not_split_range->split(mapping);
            res.insert(res.end(), range_end.begin(), range_end.end());
        }
        return res;
    }

    //range should be previously split, i.e. it fully belongs to one of the mapping segments
    Range apply_mapping(const Mapping & mapping) const
    {
        Range res{begin_id, length};
        for (const auto & triplet: mapping)
        {
            if (res.intersects({triplet.source_begin_id, triplet.length}))
            {
                const auto range_transformed = apply_transformation(begin_id, triplet);
                res.begin_id = std::get<1>(range_transformed);
                break;
            }
        }
        return res;
    }
};

void print(std::string_view str, const Ranges & ranges)
{
    std::cout << str << std::endl;
    for (const auto & item: ranges)
    {
        std::cout << "from: " << item.begin_id << "; length: " << item.length << std::endl;
    }
}

const std::vector<MappingName> g_sequence_of_mappings{
        MappingName{Domain::SEED, Domain::SOIL},
        MappingName{Domain::SOIL, Domain::FERTILIZER},
        MappingName{Domain::FERTILIZER, Domain::WATER},
        MappingName{Domain::WATER, Domain::LIGHT},
        MappingName{Domain::LIGHT, Domain::TEMPERATURE},
        MappingName{Domain::TEMPERATURE, Domain::HUMIDITY},
        MappingName{Domain::HUMIDITY, Domain::LOCATION}
};

struct Almanac
{
    std::vector<NumberType> seeds;
    // Mapping is sorted by source
    std::map<MappingName, Mapping> domain_mappings;

    void print() const
    {
        std::cout << "Seeds: ";
        for(const auto & item: seeds)
        {
            std::cout << item << " " << std::endl;
        }
        for(const auto & [name, mapping]: domain_mappings)
        {
            std::cout << to_string(name.first) << "-to-" << to_string(name.second) << " map:" << std::endl;
            for(const auto item: mapping)
            {
                item.print();
            }
        }
    }
};

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

SegmentMapping parse_triplet(std::string_view triplet)
{
    const auto numbers = parse_numbers(triplet);
    return {.destination_begin_id = numbers[0], .source_begin_id = numbers[1], .length = numbers[2]};
}

MappingName parse_map_names(std::string_view name)
{
    MappingName res;
    std::match_results<std::string_view::const_iterator> match;
    if (std::regex_match(name.begin(), name.end(), match, g_map_names_regex))
    {
        const auto first = match[1].str();
        res.first = to_enum(first);
        const auto second = match[2].str();
        res.second = to_enum(second);

    }
    return res;
}

Almanac parse_file(const std::string & path) {
    std::ifstream ifs(path, std::ifstream::in);
    std::string line;

    Almanac almanac;
    std::optional<MappingDescription> description;
    while (std::getline(ifs, line)) {
        if (line != "") {
            const auto start_pos = line.find_first_of(":");
            if (start_pos != std::string::npos) {
                const auto state_sv = std::string_view{line}.substr(0, start_pos);
                if (state_sv == g_seeds) {
                    almanac.seeds = parse_numbers(std::string_view{line}.substr(start_pos + 1, std::string::npos));
                } else {
                    if (description) {
                        almanac.domain_mappings[description->first] = description->second;
                    }
                    description = MappingDescription{parse_map_names({line}), Mapping{}};
                }
            } else {
                if (description) {
                    description->second.push_back(parse_triplet({line}));
                }
            }
        }
    }
    if (description) {
        almanac.domain_mappings[description->first] = description->second;
    }
    for (auto &[name, mapping]: almanac.domain_mappings)
    {
       std::sort(mapping.begin(), mapping.end(), [](SegmentMapping & l, SegmentMapping & r)
       {
           return l.source_begin_id < r.source_begin_id;
       });
    }
    return almanac;
}

std::tuple<bool, NumberType> apply_transformation(NumberType id, const SegmentMapping & segment)
{
    NumberType res = id;
    bool applied = false;
    if(id >= segment.source_begin_id && id < segment.source_begin_id + segment.length)
    {
        res = segment.destination_begin_id + (id - segment.source_begin_id);
        applied = true;
    }
    return {applied, res};
}

NumberType apply_mapping(NumberType id, const Mapping & mapping)
{
    NumberType res = id;
    bool applied = false;
    for(const auto & segment: mapping)
    {
        std::tie(applied, res) = apply_transformation(res, segment);
        if (applied)
        {
            break;
        }
    }
    return res;
}

Ranges apply_mapping_2(const Ranges & ranges, const Mapping & mapping)
{
    Ranges res{};
    for (const Range & range: ranges)
    {
        const auto split_ranges = range.split(mapping);
        for(const auto & split_range: split_ranges)
        {
            res.push_back(split_range.apply_mapping(mapping));
        }
    }
    return res;
}

int main()
{
    const auto almanac = parse_file(g_input_path);
    std::vector<NumberType> location_ids;
    for(const auto seed_id: almanac.seeds)
    {
        auto tmp_id = seed_id;
        for(const auto & item: g_sequence_of_mappings)
        {
            tmp_id = apply_mapping(tmp_id, almanac.domain_mappings.at(item));
        };
        location_ids.push_back(tmp_id);
    }
    std::sort(location_ids.begin(), location_ids.end());
    std::cout << "--Part 1 Output " << location_ids[0] << std::endl;

    Ranges seed_ranges{};
    for(auto i = 0; i < almanac.seeds.size(); i += 2)
        seed_ranges.emplace_back(Range{almanac.seeds[i], almanac.seeds[i+1]});
    std::sort(seed_ranges.begin(), seed_ranges.end(), [](auto & l, auto & r){return l.begin_id < r.begin_id;});
    Ranges locations{};
    Ranges tmp_ranges = seed_ranges;
    for(const auto & item: g_sequence_of_mappings)
    {
        auto res = apply_mapping_2(tmp_ranges, almanac.domain_mappings.at(item));
        tmp_ranges = res;
    };
    locations.insert(locations.end(), tmp_ranges.cbegin(), tmp_ranges.cend());
    std::sort(locations.begin(), locations.end(), [](auto & l, auto & r){return l.begin_id < r.begin_id;});
    std::cout << "--Part 2 Output " << locations.front().begin_id << std::endl;
}
