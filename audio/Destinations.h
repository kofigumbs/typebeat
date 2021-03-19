struct Destinations {
    struct Entry {
        float* zone;
        float min;
        float max;

        float get() {
            return *zone;
        }

        void set(float value) {
            *zone = std::clamp(value, min, max);
        }
    };

    Destinations() : entries() {}

    void add(int voice, std::string name, float* zone, float min, float max) {
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
