struct Destinations {
    Destinations() : entries() {}

    void add(int track, std::string name, float* zone, float min, float max) {
        if (entries.find(track) == entries.end())
            entries[track] = {};
        entries[track][name] = std::make_tuple(zone, min, max);
    }

  private:
    std::unordered_map<int, std::unordered_map<std::string, std::tuple<float*, float, float>>> entries;
};
