struct Destinations {
    struct Entry {
        float* zone;
        float min;
        float max;

        float read() {
            return *zone;
        }

        void write(float value) {
            *zone = std::clamp(value, min, max);
        }
    };

    std::set<std::string> names;

    Destinations() : names(), entries() {}

    void add(int voice, std::string name, float* zone, float min, float max) {
        names.insert(name);
        while (voice >= entries.size())
            entries.push_back({});
        entries[voice][name] = { zone, min, max };
    }

    Entry* get(int voice, std::string name) {
        return entries[voice].count(name) ? &entries[voice][name] : nullptr;
    }

  private:
    std::vector<std::unordered_map<std::string, Entry>> entries;
};
